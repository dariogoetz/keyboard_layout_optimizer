use keyboard_layout::layout::Layout;
use keyboard_layout::layout_generator::NeoLayoutGenerator;
use layout_evaluation::evaluation::Evaluator;

use super::common::PermutationLayoutGenerator;

use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;

use argmin::prelude::{ArgminOp, ArgminSlogLogger, Error, Executor, ObserverMode};
use argmin::solver::simulatedannealing::{SATempFunc, SimulatedAnnealing};

#[derive(Deserialize, Debug)]
pub struct Parameters {
    // Parameters for the solver.
    /// Optional: stop if there was no new best solution after 1000 iterations
    pub stall_accepted: u64,

    // Parameters for the [Executor].
    /// Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
    pub max_iters: u64,
    // /// Optional: Set target cost function value (defaults to `std::f64::NEG_INFINITY`)
    // pub target_cost: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            // Parameters for the solver.
            stall_accepted: 1000,

            // Parameters for the [Executor].
            max_iters: 10_000,
            //target_cost: 0.0,
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
}

impl ArgminOp for AnnealingStruct<'_> {
    type Param = Vec<usize>;
    type Output = f64;
    type Hessian = ();
    type Jacobian = ();
    type Float = f64;

    fn apply(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let layout = self.layout_generator.generate_layout(&param);
        let evaluation_result = self.evaluator.evaluate_layout(&layout);
        Ok(evaluation_result.total_cost())
    }

    /// This function is called by the annealing function
    fn modify(&self, param: &Self::Param, _temp: f64) -> Result<Self::Param, Error> {
        // in the following, the 1 will be replaced by something `temp`-depending
        Ok(self.layout_generator.switch_n_keys(&param, 1))
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

        current_layout = layout_generator.switch_n_keys(&current_layout, 1);
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
    params: &Parameters,
    layout_str: &str,
    fixed_characters: &str,
    layout_generator: &NeoLayoutGenerator,
    evaluator: &Evaluator,
    greedy: bool,
    cache_results: bool,
) -> Layout {
    log::info!("Starting optimization with: {:?}", params);
    let pm = PermutationLayoutGenerator::new(layout_str, fixed_characters, layout_generator);
    // Get initial Layout.
    let init_layout = pm.generate_random();

    /* for _ in 0..10 {
        let init_temp = get_cost_sd(Arc::new(evaluator.clone()), &pm);
        println!("Standart Deviation: {}\n", init_temp);
    } */
    let init_temp = match greedy {
        true => f64::MIN_POSITIVE,
        false => {
            println!("\nCalculating initial temperature.");
            let init_temp = get_cost_sd(&init_layout, Arc::new(evaluator.clone()), &pm);
            println!("Initial temperature = {}\n", init_temp);
            init_temp
        }
    };

    let problem = AnnealingStruct {
        evaluator: Arc::new(evaluator.clone()),
        layout_generator: &pm,
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
        // Optional: stop if there was no accepted solution after 1000 iterations
        .stall_accepted(params.stall_accepted);

    // Create and run the executor, which will apply the solver to the problem, given a starting point (`init_param`)
    let res = Executor::new(problem, solver, init_layout)
        // Optional: Attach a observer
        .add_observer(ArgminSlogLogger::term(), ObserverMode::Always) //Every(100))
        // Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
        .max_iters(params.max_iters)
        // Optional: Set target cost function value (defaults to `std::f64::NEG_INFINITY`)
        //.target_cost(params.target_cost)
        .run()
        .unwrap();

    let best_layout_vec = res.state().get_best_param();
    pm.generate_layout(&best_layout_vec)
}
