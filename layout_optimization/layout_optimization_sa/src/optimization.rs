use keyboard_layout::{layout::Layout, layout_generator::LayoutGenerator};
use layout_evaluation::{cache::Cache, evaluation::Evaluator};

use layout_optimization_common::LayoutPermutator;

use anyhow::Result;
use colored::Colorize;
use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256PlusPlus};
use serde::Deserialize;
use std::{fs::File, sync::Arc};

use argmin::{
    core::{
        observers::{Observe, ObserverMode},
        CostFunction, Error, Executor, IterState, State, KV,
    },
    solver::simulatedannealing::{Anneal, SATempFunc, SimulatedAnnealing},
};

#[derive(Deserialize, Debug)]
pub struct Parameters {
    /// Initial temperature. Gets eventually lowered down to (almost) zero during optimization.
    pub init_temp: Option<f64>,

    /// In each modification of the layout, swap this many key-pairs.
    pub key_switches: usize,

    // Parameters for the solver.
    /// Stop if there was no accepted solution after this many iterations
    pub stall_accepted: u64,

    // Parameters for the [Executor].
    /// Set maximum number of iterations (defaults to `std::u64::MAX`)
    pub max_iters: u64,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            init_temp: Some(150.0),
            key_switches: 1,
            // Parameters for the solver.
            stall_accepted: 5000,
            // Parameters for the [Executor].
            max_iters: 100_000,
        }
    }
}

impl Parameters {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = File::open(filename)?;
        Ok(serde_yaml::from_reader(f)?)
    }
    /// Makes sure that [self.init_temp] is greater than zero.
    /// => Negative values and zero get turned into `f64::MIN_POSITIVE`.
    pub fn correct_init_temp(&mut self) {
        if let Some(init_temp) = self.init_temp {
            let corrected_init_temp = if init_temp <= 0.0 {
                f64::MIN_POSITIVE
            } else {
                init_temp
            };
            self.init_temp = Some(corrected_init_temp);
        }
    }
}

pub struct AnnealingStruct {
    evaluator: Arc<Evaluator>,
    permutator: LayoutPermutator,
    layout_generator: Box<dyn LayoutGenerator>,
    key_switches: usize,
    result_cache: Option<Cache<f64>>,
}

impl CostFunction for AnnealingStruct {
    type Param = Vec<usize>;
    type Output = f64;

    /// Evaluate param (= the layout-vector).
    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let evaluate_layout_str = |layout_str: &str| -> f64 {
            let l = self.layout_generator.generate(layout_str).unwrap();
            self.evaluator.evaluate_layout(&l).total_cost()
        };

        let layout_string = self.permutator.generate_string(param);
        let evaluation_result = match &self.result_cache {
            Some(result_cache) => result_cache
                .get_or_insert_with(&layout_string, || evaluate_layout_str(&layout_string)),
            None => evaluate_layout_str(&layout_string),
        };

        Ok(evaluation_result)
    }
}

impl Anneal for AnnealingStruct {
    type Param = Vec<usize>;
    type Output = Vec<usize>;
    type Float = f64;

    /// Anneal a parameter vector, slightly changing it.
    fn anneal(&self, param: &Self::Param, _temp: f64) -> Result<Self::Output, Error> {
        Ok(self.permutator.perform_n_swaps(param, self.key_switches))
    }
}

pub type SaIterState = IterState<Vec<usize>, (), (), (), f64>;

/// An observer that outputs important information in a more human-readable format than `Argmin`'s original implementation.
struct BestObserver {
    id: String,
    permutator: LayoutPermutator,
}

impl Observe<SaIterState> for BestObserver {
    fn observe_iter(&mut self, state: &SaIterState, _kv: &KV) -> Result<(), Error> {
        let reason = match state.iter {
            0 => "First tested layout:".blue(),
            _ => "New best:".green(),
        };
        let best_layout = self
            .permutator
            .generate_string(state.best_param.as_ref().unwrap());
        log::info!(
            "{} {} {} ({:>6.1})",
            format!("{}:", self.id).yellow().bold(),
            reason,
            best_layout,
            state.best_cost,
        );
        Ok(())
    }
}

pub struct CustomObserver(pub Box<dyn Observe<SaIterState>>);

/// Necessary to avoid errors when importing a `custom_observer` to `optimize()`.
impl Observe<SaIterState> for CustomObserver {
    fn observe_iter(&mut self, state: &SaIterState, kv: &KV) -> Result<(), Error> {
        self.0.observe_iter(state, kv)
    }
}

/// An observer that outputs important information in a more human-readable format than `Argmin`'s original implementation.
struct IterationObserver {
    id: String,
    permutator: LayoutPermutator,
    log_everything: bool,
}

impl Observe<SaIterState> for IterationObserver {
    fn observe_iter(&mut self, state: &SaIterState, kv: &KV) -> Result<(), Error> {
        if state.iter > 0 {
            let layout = self
                .permutator
                .generate_string(state.param.as_ref().unwrap());
            let best_layout = self
                .permutator
                .generate_string(state.best_param.as_ref().unwrap());
            /* Structure of ArgminKV.kv: Vec<(&'static str, String)>
            t: 111.38906945299198
            new_be: true
            acc: true
            st_i_be: 0
            st_i_ac: 0
            ra_i_fi: 1
            ra_i_be: 0
            ra_i_ac: 0
            ra_fi: false
            ra_be: false
            ra_ac: false
            time: 0.533206799 */
            let mut temperature = String::from("Not found.");
            let mut accepted = String::from("Not found");
            for (key, value) in &kv.kv {
                match *key {
                    "t" => temperature = format!("{:.5}", value),
                    "acc" => accepted = value.to_string(),
                    _ => {}
                }
            }
            let mut output = format!(
                "{} {} {:>3}, {} {} ({:>6.1}), {} {} ({:>6.1}), {} {}°",
                format!("{}:", self.id).yellow().bold(),
                "n:".bold(),
                state.iter,
                "current:".bold(),
                layout,
                state.cost,
                "best:".bold(),
                best_layout,
                state.best_cost,
                "temp:".bold(),
                temperature,
            );
            if self.log_everything {
                let is_better = state.cost < state.prev_cost;
                output.push_str(&format!(
                    " {} {}{} {} {}",
                    "better:".bold(),
                    is_better,
                    if is_better { " " } else { "" }, // Used to perserve alignment. {:.5} doesn't work.
                    "acc:".bold(),
                    accepted
                ));
            }
            log::info!("{}", output);
        }
        Ok(())
    }
}

/// Calculates the mean of a vec containing f64-values.
fn mean(list: &[f64]) -> f64 {
    let sum: f64 = list.iter().sum();
    sum / (list.len() as f64)
}

/// Calculates the [Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation)
/// for the cost of some amount of Layouts, then returns it.
///
/// This value can then be used as the initial temperature in Simulated annealing.
/// Reference: https://link.springer.com/content/pdf/10.1007/s10732-007-9012-8.pdf
fn get_cost_sd(
    initial_indices: &[usize],
    evaluator: Arc<Evaluator>,
    permutator: &LayoutPermutator,
    layout_generator: &Box<dyn LayoutGenerator>,
    key_pair_switches: usize,
) -> f64 {
    const USED_NEIGHBORS: u16 = 100;

    // Calculate initial temperature.
    let mut sd = 0.0;
    let mut costs: Vec<f64> = vec![];
    let mut current_indices = initial_indices.to_owned();

    for _ in 0..USED_NEIGHBORS {
        let layout = layout_generator
            .generate(&permutator.generate_string(&current_indices))
            .unwrap();
        let evaluation_result = evaluator.evaluate_layout(&layout);
        costs.push(evaluation_result.total_cost());
        current_indices = permutator.perform_n_swaps(&current_indices, key_pair_switches);
    }
    let average: f64 = mean(&costs);

    for cost in &costs {
        let difference = cost - average;
        sd += difference.powi(2);
    }
    sd /= USED_NEIGHBORS as f64;
    sd = sd.sqrt();
    sd
}

/// Performs one run of Simulated Annealing, then returns the best layout found.
#[allow(clippy::too_many_arguments)]
pub fn optimize(
    process_name: &str,
    params: &Parameters,
    layout_str: &str,
    fixed_characters: &str,
    layout_generator: &Box<dyn LayoutGenerator>,
    start_with_layout: bool,
    evaluator: &Evaluator,
    log_everything: bool,
    result_cache: Option<Cache<f64>>,
    custom_observer: Option<CustomObserver>,
) -> (String, Layout) {
    let pm = LayoutPermutator::new(layout_str, fixed_characters);
    // Get initial Layout.
    let initial_indices = match start_with_layout {
        true => pm.get_permutable_indices(),
        false => pm.generate_random(),
    };

    /* // Test 10_000 Layouts to get a good default initial temperature.
    let mut init_temp_vec: Vec<f64> = vec![];
    const TESTED_LAYOUT_NR: u8 = 100;
    for i in 0..TESTED_LAYOUT_NR {
        let l = pm.generate_random();
        let init_temp = get_cost_sd(&l, Arc::new(evaluator.clone()), &pm, params.key_switches);
        println!("init_temp {:>2}: {}", i, init_temp);
        init_temp_vec.push(init_temp);
    }
    println!("Average init_temp: {}", mean(&init_temp_vec)); */

    let init_temp = match params.init_temp {
        Some(t) => t,
        None => {
            log::info!(
                "{} Calculating initial temperature",
                format!("{}:", process_name).yellow().bold(),
            );
            let init_temp = get_cost_sd(
                &initial_indices,
                Arc::new(evaluator.clone()),
                &pm,
                layout_generator,
                params.key_switches,
            );
            log::info!(
                "{} Initial temperature = {}°",
                format!("{}:", process_name).yellow().bold(),
                init_temp,
            );
            init_temp
        }
    };
    let problem = AnnealingStruct {
        evaluator: Arc::new(evaluator.clone()),
        permutator: pm.clone(),
        layout_generator: layout_generator.clone(),
        key_switches: params.key_switches,
        result_cache,
    };

    // Create new SA solver with some parameters (see docs for details)
    // This essentially just prepares the SA solver. It is not run yet, nor does it know anything about the problem it is about to solve.
    let rng = Xoshiro256PlusPlus::from_entropy();
    let solver = SimulatedAnnealing::new_with_rng(init_temp, rng)
        .unwrap()
        // Optional: Define temperature function (defaults to `SATempFunc::TemperatureFast`)
        .with_temp_func(SATempFunc::Exponential(0.998))
        /////////////////////////
        // Stopping criteria   //
        /////////////////////////
        // Optional: stop if there was no accepted solution after [params.stall_accepted] iterations
        .with_stall_accepted(params.stall_accepted);

    // Create and run the executor, which will apply the solver to the problem, given a starting point (`init_param`)
    let mut executor = Executor::new(problem, solver)
        .configure(|state| {
            state
                // Set initial starting-param (~staring layout)
                .param(initial_indices)
                // Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
                .max_iters(params.max_iters)
        })
        .timer(false);
    match custom_observer {
        // If a custom Observer was supplied, only use that Observer.
        Some(observer) => {
            executor = executor.add_observer(observer, ObserverMode::Always);
        }
        // If no custom Observer was supplied, use the default setup.
        None => {
            let best_observer = BestObserver {
                id: process_name.to_string(),
                permutator: pm.clone(),
            };
            let iter_observer = IterationObserver {
                id: process_name.to_string(),
                permutator: pm.clone(),
                log_everything,
            };
            let iter_observer_mode = if log_everything {
                ObserverMode::Always
            } else {
                ObserverMode::Every(100)
            };
            // Optional: Attach a observer
            executor = executor
                .add_observer(best_observer, ObserverMode::NewBest)
                .add_observer(iter_observer, iter_observer_mode);
        }
    }

    log::info!(
        "{} Starting optimization with: initial_temperature: {:.2}°, {:?}",
        format!("{}:", process_name).yellow().bold(),
        init_temp,
        params,
    );
    let res = executor.run().unwrap();

    let best_layout_param = res.state().get_best_param().unwrap();
    let best_layout_str = pm.generate_string(best_layout_param);
    let best_layout = layout_generator.generate(&best_layout_str).unwrap();

    (best_layout_str, best_layout)
}
