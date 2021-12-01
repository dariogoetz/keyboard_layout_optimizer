use keyboard_layout::layout::Layout;
use keyboard_layout::layout_generator::NeoLayoutGenerator;
use layout_evaluation::evaluation::Evaluator;

use super::common::PermutationLayoutGenerator;

use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;

use argmin::prelude::{ArgminKV, ArgminOp, Error, Executor, IterState, Observe, ObserverMode};
use argmin::solver::simulatedannealing::{SATempFunc, SimulatedAnnealing};

#[derive(Deserialize, Debug)]
pub struct Parameters {
    /// In each modification of the layout, swap this many key-pairs.
    pub key_switches: usize,

    // Parameters for the solver.
    /// Stop if there was no new best solution after this many iterations
    pub stall_accepted: u64,
    /// Start reannealing after no new best solution has been found for this many iterations
    pub reannealing_best: u64,

    // Parameters for the [Executor].
    /// Set maximum number of iterations (defaults to `std::u64::MAX`)
    pub max_iters: u64,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            key_switches: 1,
            // Parameters for the solver.
            stall_accepted: 1000,
            reannealing_best: 4000,
            // Parameters for the [Executor].
            max_iters: 100_000,
        }
    }
}

impl Parameters {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

struct AnnealingStruct<'a> {
    evaluator: Arc<Evaluator>,
    layout_generator: &'a PermutationLayoutGenerator,
    key_switches: usize,
}

impl ArgminOp for AnnealingStruct<'_> {
    type Param = Vec<usize>;
    type Output = f64;
    type Hessian = ();
    type Jacobian = ();
    type Float = f64;

    /// Evaluate the param (~the layout).
    fn apply(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let layout = self.layout_generator.generate_layout(&param);
        let evaluation_result = self.evaluator.evaluate_layout(&layout);
        Ok(evaluation_result.total_cost())
    }

    /// Modify param (~the layout).
    fn modify(&self, param: &Self::Param, _temp: f64) -> Result<Self::Param, Error> {
        Ok(self
            .layout_generator
            .switch_n_keys(&param, self.key_switches))
    }
}

/// An observer that outputs important information in a more human-readable format than `Argmin`'s original implementation.
struct Observer {
    id: String,
    layout_generator: PermutationLayoutGenerator,
}

impl Observe<AnnealingStruct<'_>> for Observer {
    fn observe_init(&self, _name: &str, _kv: &ArgminKV) -> Result<(), Error> {
        Ok(())
    }

    fn observe_iter(
        &mut self,
        state: &IterState<AnnealingStruct<'_>>,
        kv: &ArgminKV,
    ) -> Result<(), Error> {
        let layout = self.layout_generator.generate_string(&state.param);
        let best_layout = self.layout_generator.generate_string(&state.best_param);
        let mut temperature = String::from("Not found.");
        for (key, value) in &kv.kv {
            if key == &"t" {
                temperature = format!("{:.5}", value);
            }
        }
        log::info!(
            "{}: n: {:>3}, current: {} ({:>6.1}), best: {} ({:>6.1}), temp: {}",
            self.id,
            state.iter,
            layout,
            state.cost,
            best_layout,
            state.best_cost,
            temperature, // Already is formatted.
        );
        Ok(())
    }
}

/// Calculates the [Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation)
/// for the cost of some amount of Layouts, then returns it.
///
/// This value can then be used as the initial temperature in Simulated annealing.
/// Reference: https://link.springer.com/content/pdf/10.1007/s10732-007-9012-8.pdf
fn get_cost_sd(
    initial_layout: &Vec<usize>,
    evaluator: Arc<Evaluator>,
    layout_generator: &PermutationLayoutGenerator,
    key_pair_switches: usize,
) -> f64 {
    const USED_NEIGHBORS: u16 = 100;

    // Calculate initial temperature.
    let mut sd = 0.0;
    let mut costs: Vec<f64> = vec![];
    let mut current_layout = initial_layout.clone();

    for _ in 0..USED_NEIGHBORS {
        let layout = layout_generator.generate_layout(&current_layout);
        let evaluation_result = evaluator.evaluate_layout(&layout);
        costs.push(evaluation_result.total_cost());

        current_layout = layout_generator.switch_n_keys(&current_layout, key_pair_switches);
    }

    let sum: f64 = costs.iter().sum();
    let average: f64 = sum / USED_NEIGHBORS as f64;

    for cost in &costs {
        let difference = cost - average;
        sd += difference.powi(2);
    }
    sd /= USED_NEIGHBORS as f64;
    sd = sd.sqrt();
    sd
}

/// Performs one run of Simulated Annealing, then returns the best layout found.
pub fn optimize(
    process_name: &str,
    params: &Parameters,
    layout_str: &str,
    fixed_characters: &str,
    layout_generator: &NeoLayoutGenerator,
    start_with_layout: bool,
    evaluator: &Evaluator,
    optional_init_temp: Option<f64>,
    cache_results: bool,
) -> Layout {
    let pm = PermutationLayoutGenerator::new(layout_str, fixed_characters, layout_generator);
    // Get initial Layout.
    let init_layout = match start_with_layout {
        true => pm.get_permutable_indices(),
        false => pm.generate_random(),
    };
    let init_temp = match optional_init_temp {
        Some(t) => {
            println!("\nWARNING: Currently, the option `--greedy` is bugged. The very first modification always gets accepted.\nFor more information, visit this GitHub-issue: https://github.com/argmin-rs/argmin/issues/150\n");
            std::thread::sleep(std::time::Duration::from_secs(7));
            t
        }
        None => {
            log::info!("{}: Calculating initial temperature.", process_name);
            let init_temp = get_cost_sd(
                &init_layout,
                Arc::new(evaluator.clone()),
                &pm,
                params.key_switches,
            );
            println!("{}: Initial temperature = {}", process_name, init_temp);
            init_temp
        }
    };
    let problem = AnnealingStruct {
        evaluator: Arc::new(evaluator.clone()),
        layout_generator: &pm,
        key_switches: params.key_switches,
    };
    // Create new SA solver with some parameters (see docs for details)
    // This essentially just prepares the SA solver. It is not run yet, nor does it know anything about the problem it is about to solve.
    let solver = SimulatedAnnealing::new(init_temp) // 200.0)
        .unwrap()
        // Optional: Define temperature function (defaults to `SATempFunc::TemperatureFast`)
        .temp_func(SATempFunc::Boltzmann)
        /////////////////////////
        // Stopping criteria   //
        /////////////////////////
        // Optional: stop if there was no accepted solution after [params.stall_accepted] iterations
        .stall_accepted(params.stall_accepted)
        /////////////////////////
        // Reannealing         //
        /////////////////////////
        // Optional: Start reannealing after no new best solution has been found for [params.reannealing_best] iterations
        .reannealing_best(params.reannealing_best);
    let observer = Observer {
        id: process_name.to_string(),
        layout_generator: pm.clone(),
    };

    log::info!(
        "{}: Starting optimization with: initial_temperature: {:.2}, {:?}",
        process_name,
        init_temp,
        params
    );
    // Create and run the executor, which will apply the solver to the problem, given a starting point (`init_param`)
    let res = Executor::new(problem, solver, init_layout)
        // Optional: Attach a observer
        .add_observer(observer, ObserverMode::NewBest) //ObserverMode::Always) //Every(100))
        // Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
        .max_iters(params.max_iters)
        .run()
        .unwrap();

    let best_layout_vec = res.state().get_best_param();
    pm.generate_layout(&best_layout_vec)
}
