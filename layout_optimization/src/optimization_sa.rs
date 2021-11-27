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
    pub stall_best: u64,
    pub stall_accepted: u64,
    pub reannealing_fixed: u64,
    pub reannealing_accepted: u64,
    pub reannealing_best: u64,
    // Parameters for the [Executor].
    pub max_iters: u64,
    pub target_cost: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            // Parameters for the solver.
            stall_best: 1000,
            stall_accepted: 1000,
            reannealing_fixed: 1000,
            reannealing_accepted: 500,
            reannealing_best: 800,
            // Parameters for the [Executor].
            max_iters: 10_000,
            target_cost: 0.0,
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

pub fn optimize(
    params: &Parameters,
    layout_str: &str,
    fixed_characters: &str,
    layout_generator: &NeoLayoutGenerator,
    evaluator: &Evaluator,
    cache_results: bool,
) -> Layout {
    log::info!("Starting optimization with: {:?}", params);
    let pm = PermutationLayoutGenerator::new(layout_str, fixed_characters, layout_generator);

    //let init_param = pm.generate_layout(layout_str);
    let problem = AnnealingStruct {
        evaluator: Arc::new(evaluator.clone()),
        layout_generator: &pm,
    };
    // create new SA solver with some parameters (see docs for details)
    // This essentially just prepares the SA solver. It is not run yet, nor does it know anything about the problem it is about to solve.
    let solver = SimulatedAnnealing::new(25.0)
        .unwrap()
        // Optional: Define temperature function (defaults to `SATempFunc::TemperatureFast`)
        .temp_func(SATempFunc::Boltzmann)
        /////////////////////////
        // Stopping criteria   //
        /////////////////////////
        // Optional: stop if there was no new best solution after 1000 iterations
        .stall_best(params.stall_best)
        // Optional: stop if there was no accepted solution after 1000 iterations
        .stall_accepted(params.stall_accepted);
    /////////////////////////
    // Reannealing         //
    /////////////////////////
    // Optional: Reanneal after 1000 iterations (resets temperature to initial temperature)
    //.reannealing_fixed(params.reannealing_fixed)
    // Optional: Reanneal after no accepted solution has been found for `iter` iterations
    //.reannealing_accepted(params.reannealing_accepted)
    // Optional: Start reannealing after no new best solution has been found for 800 iterations
    //.reannealing_best(params.reannealing_best);

    let init_param = pm.generate_random();

    // Create and run the executor, which will apply the solver to the problem, given a starting point (`init_param`)
    let res = Executor::new(problem, solver, init_param)
        // Optional: Attach a observer
        .add_observer(ArgminSlogLogger::term(), ObserverMode::NewBest)
        // Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
        .max_iters(params.max_iters)
        // Optional: Set target cost function value (defaults to `std::f64::NEG_INFINITY`)
        .target_cost(params.target_cost)
        .run()
        .unwrap();

    // Wait a second (lets the logger flush everything before printing again)
    //std::thread::sleep(std::time::Duration::from_secs(1));

    //println!("\n\n\n\n{}", res);
    //println!("\n\n\n\n{:?}", res.operator());
    //println!("\n\n\n\n{:?}", res.state());
    //let lay = res.param;

    //Ok(())
    let best_layout_vec = res.state().get_best_param();
    pm.generate_layout(&best_layout_vec)
}
