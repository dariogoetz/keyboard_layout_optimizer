use keyboard_layout::layout::Layout;
use keyboard_layout::layout_generator::NeoLayoutGenerator;
use layout_evaluation::evaluation::Evaluator;
use layout_evaluation::results::EvaluationResult;

use super::common::PermutationLayoutGenerator;

use anyhow::Result;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::usize;

use abc::{scaling, Candidate, Context, HiveBuilder};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Deserialize, Debug)]
pub struct Parameters {
    retries: usize,
    n_switches: usize,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            retries: 1000,
            n_switches: 4,
        }
    }
}

impl Parameters {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

/// The fitness function for `Genotype`s.
#[derive(Clone, Debug)]
pub struct FitnessCalc {
    evaluator: Arc<Evaluator>,
    layout_generator: PermutationLayoutGenerator,
    result_cache: Option<Arc<Mutex<FxHashMap<String, EvaluationResult>>>>,
    n_switches: usize,
}

impl Context for FitnessCalc {
    type Solution = Vec<usize>;

    fn make(&self) -> Self::Solution {
        self.layout_generator.generate_random()
    }

    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        let l = self.layout_generator.generate(solution);
        let layout_str = self.layout_generator.generate_string(solution);
        let mut cache_val = None;
        if let Some(result_cache) = &self.result_cache {
            let cache = result_cache.lock().unwrap();
            cache_val = cache.get(&layout_str).map(|v| v.clone());
        }
        let evaluation_result = match cache_val {
            Some(res) => res,
            None => {
                let res = self.evaluator.evaluate_layout(&l);
                if let Some(result_cache) = &self.result_cache {
                    let mut cache = result_cache.lock().unwrap();
                    cache.insert(layout_str, res.clone());
                }

                res
            }
        };

        evaluation_result.optimization_score() as f64
    }

    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        let mut core = field[n].solution.clone();

        core.partial_shuffle(&mut thread_rng(), self.n_switches);

        core
    }
}

pub fn optimize(
    params: &Parameters,
    evaluator: &Evaluator,
    layout_str: &str,
    layout_generator: &NeoLayoutGenerator,
    fixed_characters: &str,
    cache_results: bool,
) -> Layout {
    let pm = PermutationLayoutGenerator::new(layout_str, fixed_characters, layout_generator);

    let result_cache = if cache_results {
        Some(Arc::new(Mutex::new(FxHashMap::default())))
    } else {
        None
    };

    let core = FitnessCalc {
        evaluator: Arc::new(evaluator.clone()),
        layout_generator: pm.clone(),
        result_cache,
        n_switches: params.n_switches,
    };

    let ncpus = num_cpus::get();
    let hive = HiveBuilder::<FitnessCalc>::new(core, ncpus)
        .set_threads(ncpus)
        .set_retries(params.retries)
        .set_scaling(scaling::proportionate());
    // .set_scaling(scaling::power_rank(10_f64));

    let mut best_layout = pm.generate_random();
    for candidate in hive.build().unwrap().stream().iter() {
        let layout = pm.generate(&candidate.solution);
        println!("{}", layout.plot());
        println!("{}", layout.plot_compact());
        println!("{}", evaluator.evaluate_layout(&layout));
        best_layout = candidate.solution;
    }

    pm.generate(&best_layout)
}
