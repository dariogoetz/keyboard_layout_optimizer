use super::common::PermutationLayoutGenerator;
use keyboard_layout::layout::Layout;
use keyboard_layout::layout_generator::NeoLayoutGenerator;
use layout_evaluation::{evaluation::Evaluator, results::EvaluationResult};

use argmin::prelude::*;
use argmin::solver::simulatedannealing::{SATempFunc, SimulatedAnnealing};
use serde::{Deserialize, Serialize};
//use argmin_testfunctions::rosenbrock;
use rand::distributions::Uniform;
use rand::prelude::*;
//use rand_xoshiro::Xoshiro256PlusPlus;
use std::default::Default;
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize)]
struct SomeStruct {
    current_layout: Layout,
    evaluator: Arc<Evaluator>,
    layout_generator: PermutationLayoutGenerator,
}

/* impl Default for Rosenbrock {
    fn default() -> Self {
        let lower_bound: Vec<f64> = vec![-5.0, -5.0];
        let upper_bound: Vec<f64> = vec![5.0, 5.0];
        Rosenbrock::new(1.0, 100.0, lower_bound, upper_bound)
    }
} */

/* impl SomeStruct {
    /// Constructor
    pub fn new() -> Self {
        SomeStruct {
            current_layout: Layout,
        }
    }
} */

impl ArgminOp for SomeStruct {
    type Param = Layout;
    type Output = f64;
    type Hessian = ();
    type Jacobian = ();

    fn apply(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let evaluation_result = self.evaluator.evaluate_layout(&self.current_layout);
        Ok(evaluation_result.optimization_score() as f64)
    }

    /// This function is called by the annealing function
    fn modify(&self, param: &Self::Param, temp: f64) -> Result<Self::Param, Error> {
        Ok(self.layout_generator.switch_n_keys(&self.current_layout, 1))
    }
}

fn optimize(
    //params: &Parameters,
    evaluator: &Evaluator,
    layout_str: &str,
    layout_generator: &NeoLayoutGenerator,
) /* -> Layout */
{
    /* // Define bounds
    let lower_bound: Vec<f64> = vec![-5.0, -5.0];
    let upper_bound: Vec<f64> = vec![5.0, 5.0];

    // Define cost function
    let operator = SomeStruct::new(1.0, 100.0, lower_bound, upper_bound);

    // Define initial parameter vector
    let init_param: Vec<f64> = vec![1.0, 1.2];*/

    // Define initial temperature
    let temp = 15.0;

    // Seed RNG
    //let rng = Xoshiro256PlusPlus::from_entropy();

    // Set up simulated annealing solver
    let solver = SimulatedAnnealing::new(temp)
    /* ?
        // Optional: Define temperature function (defaults to `SATempFunc::TemperatureFast`)
        .temp_func(SATempFunc::Boltzmann)
        /////////////////////////
        // Stopping criteria   //
        /////////////////////////
        // Optional: stop if there was no new best solution after 1000 iterations
        .stall_best(1000)
        // Optional: stop if there was no accepted solution after 1000 iterations
        .stall_accepted(1000)
        /////////////////////////
        // Reannealing         //
        /////////////////////////
        // Optional: Reanneal after 1000 iterations (resets temperature to initial temperature)
        .reannealing_fixed(1000)
        // Optional: Reanneal after no accepted solution has been found for `iter` iterations
        .reannealing_accepted(500)
        // Optional: Start reannealing after no new best solution has been found for 800 iterations
        .reannealing_best(800) */;

    /////////////////////////
    // Run solver          //
    /////////////////////////
    let res = Executor::new(operator, solver, init_param)
        // Optional: Attach a observer
        .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
        // Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
        .max_iters(10_000)
        // Optional: Set target cost function value (defaults to `std::f64::NEG_INFINITY`)
        .target_cost(0.0)
        .run()?;

    // Wait a second (lets the logger flush everything before printing again)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Print result
    println!("{}", res);
    Ok(());
}
