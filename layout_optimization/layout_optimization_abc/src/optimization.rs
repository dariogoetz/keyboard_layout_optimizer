use keyboard_layout::{layout::Layout, layout_generator::LayoutGenerator};
use layout_evaluation::{cache::Cache, evaluation::Evaluator};

use layout_optimization_common::LayoutPermutator;

use anyhow::Result;
use serde::Deserialize;
use std::{
    env,
    fs::File,
    sync::{mpsc::Receiver, Arc},
    thread,
};

use abc::{scaling, Candidate, Context, HiveBuilder};

#[derive(Deserialize, Debug)]
pub struct Parameters {
    retries: usize,
    n_switches: usize,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            retries: 1000,
            n_switches: 1,
        }
    }
}

impl Parameters {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = File::open(filename)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

/// The fitness function for [`Genotype`]s.
#[derive(Clone, Debug)]
pub struct FitnessCalc {
    evaluator: Arc<Evaluator>,
    permutator: LayoutPermutator,
    layout_generator: Box<dyn LayoutGenerator>,
    result_cache: Option<Cache<usize>>,
    n_switches: usize,
}

impl Context for FitnessCalc {
    type Solution = Layout;

    fn make(&self) -> Self::Solution {
        let indices = self.permutator.generate_random();
        self.layout_generator
            .generate(&self.permutator.generate_string(&indices))
            .unwrap()
    }

    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        let layout_str = solution.as_text();
        let evaluation_result = match &self.result_cache {
            Some(result_cache) => result_cache.get_or_insert_with(&layout_str, || {
                self.evaluator
                    .evaluate_layout(solution)
                    .optimization_score()
            }),
            None => self
                .evaluator
                .evaluate_layout(solution)
                .optimization_score(),
        };
        evaluation_result as f64
    }

    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        let layout_str = field[n].solution.as_text();
        let chars_orig: Vec<char> = layout_str.chars().collect();
        let mut chars: Vec<char> = layout_str.chars().collect();

        // only permutate indices of chars that are not fixed
        let indices = self.permutator.get_permutable_indices();
        let permutated_indices = self.permutator.perform_n_swaps(&indices, self.n_switches);

        indices
            .iter()
            .zip(permutated_indices.iter())
            .filter(|(i, pi)| i != pi)
            .for_each(|(i, pi)| {
                chars[*i] = chars_orig[*pi];
            });

        let permutated_layout_str: String = chars.iter().collect();
        self.layout_generator
            .generate(&permutated_layout_str)
            .unwrap()
    }
}

pub fn optimize(
    params: &Parameters,
    evaluator: &Evaluator,
    layout_str: &str,
    layout_generator: &Box<dyn LayoutGenerator>,
    fixed_characters: &str,
    cache_results: bool,
) -> Receiver<Candidate<Layout>> {
    let pm = LayoutPermutator::new(layout_str, fixed_characters);

    let result_cache = if cache_results {
        Some(Cache::new())
    } else {
        None
    };

    let core = FitnessCalc {
        evaluator: Arc::new(evaluator.clone()),
        permutator: pm,
        layout_generator: layout_generator.clone(),
        result_cache,
        n_switches: params.n_switches,
    };

    let ncpus: usize = env::var("RAYON_NUM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| thread::available_parallelism().unwrap().get());
    let hive = HiveBuilder::<FitnessCalc>::new(core, ncpus)
        .set_threads(ncpus)
        .set_retries(params.retries)
        .set_scaling(scaling::proportionate());
    // .set_scaling(scaling::power_rank(10_f64));

    hive.build().unwrap().stream()
}
