use keyboard_layout::layout::Layout;
use keyboard_layout::layout_generator::NeoLayoutGenerator;
use layout_evaluation::evaluation::Evaluator;
use layout_evaluation::results::MetricResults;

use anyhow::Result;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::usize;

use genevo::genetic::{Children, FitnessFunction, Parents};
use genevo::operator::{prelude::*, CrossoverOp, GeneticOperator};
use genevo::population::Population;
use genevo::prelude::*;
use genevo::random::SliceRandom;
// use genevo::recombination::order::PartiallyMappedCrossover;
use genevo::simulation::simulator::Simulator;
use genevo::types::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct Parameters {
    population_size: usize,
    generation_limit: u64,
    num_individuals_per_parents: usize,
    selection_ratio: f64,
    mutation_rate: f64,
    reinsertion_ratio: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            population_size: 100,
            generation_limit: 2000,
            num_individuals_per_parents: 2,
            selection_ratio: 0.7,
            mutation_rate: 0.1,
            reinsertion_ratio: 0.7,
        }
    }
}

impl Parameters {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

// The genotype
type Genotype = Vec<usize>;

#[derive(Clone, Debug)]
pub struct PermutationLayoutGenerator {
    perm_keys: Vec<char>,
    perm_indices: Vec<usize>,
    fixed_keys: Vec<char>,
    fixed_indices: Vec<usize>,
    layout_generator: NeoLayoutGenerator,
}

impl PermutationLayoutGenerator {
    fn new(layout: &str, fixed: &str, layout_generator: &NeoLayoutGenerator) -> Self {
        let mut perm_keys = Vec::new();
        let mut perm_indices = Vec::new();
        let mut fixed_keys = Vec::new();
        let mut fixed_indices = Vec::new();

        for (i, c) in layout.chars().enumerate() {
            if fixed.contains(c) {
                fixed_keys.push(c);
                fixed_indices.push(i);
            } else {
                perm_keys.push(c);
                perm_indices.push(i);
            }
        }
        Self {
            perm_keys,
            perm_indices,
            fixed_keys,
            fixed_indices,
            layout_generator: layout_generator.clone(),
        }
    }

    fn generate_string(&self, permutation: &[usize]) -> String {
        let mut res: Vec<char> = vec!['-'; self.fixed_keys.len() + self.perm_keys.len()];

        self.fixed_indices
            .iter()
            .zip(self.fixed_keys.iter())
            .for_each(|(i, c)| res[*i] = *c);

        permutation
            .iter()
            .zip(self.perm_keys.iter())
            .for_each(|(i, c)| res[*i] = *c);

        res.iter().collect()
    }

    fn generate(&self, permutation: &[usize]) -> Layout {
        let s = self.generate_string(permutation);
        self.layout_generator.generate(&s).unwrap()
    }

    fn get_permutable_indices(&self) -> Vec<usize> {
        self.perm_indices.clone()
    }
}

/// The fitness function for `Genotype`s.
#[derive(Clone, Debug)]
pub struct FitnessCalc {
    evaluator: Arc<Evaluator>,
    layout_generator: PermutationLayoutGenerator,
    result_cache: Arc<Mutex<FxHashMap<String, Vec<MetricResults>>>>,
}

impl FitnessFunction<Genotype, usize> for FitnessCalc {
    fn fitness_of(&self, genome: &Genotype) -> usize {
        let l = self.layout_generator.generate(genome);
        let layout_str = self.layout_generator.generate_string(genome);
        let cache_val;
        {
            let cache = self.result_cache.lock().unwrap();
            cache_val = cache.get(&layout_str).map(|v| v.to_vec());
        }
        let metric_costs = match cache_val {
            Some(res) => res,
            None => {
                let res = self.evaluator.evaluate_layout(&l);
                {
                    let mut cache = self.result_cache.lock().unwrap();
                    cache.insert(layout_str, res.clone());
                }

                res
            }
        };
        let cost = metric_costs
            .iter()
            .fold(0.0, |acc, metric_cost| acc + metric_cost.total_cost());
        (1e8 / cost) as usize
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum::<usize>() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        100
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

struct LayoutBuilder {
    indices: Vec<usize>,
}

impl LayoutBuilder {
    fn with_permutable_layout(layout_prototype: &PermutationLayoutGenerator) -> Self {
        Self {
            indices: layout_prototype.get_permutable_indices(),
        }
    }
}

impl GenomeBuilder<Vec<usize>> for LayoutBuilder {
    fn build_genome<R>(&self, _: usize, rng: &mut R) -> Vec<usize>
    where
        R: Rng + Sized,
    {
        let mut s: Vec<usize> = self.indices.clone();
        s.shuffle(rng);
        s
    }
}

struct FixedLayoutBuilder {
    prototype: PermutationLayoutGenerator,
}

impl FixedLayoutBuilder {
    fn with_permutable_layout(layout_prototype: &PermutationLayoutGenerator) -> Self {
        Self {
            prototype: layout_prototype.clone(),
        }
    }
}

impl GenomeBuilder<Vec<usize>> for FixedLayoutBuilder {
    fn build_genome<R>(&self, _: usize, _rng: &mut R) -> Vec<usize>
    where
        R: Rng + Sized,
    {
        self.prototype.get_permutable_indices()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct NoOpCrossover {}
impl NoOpCrossover {
    pub fn new() -> Self {
        NoOpCrossover {}
    }
}
impl GeneticOperator for NoOpCrossover {
    fn name() -> String {
        "No-Op-Crossover".to_string()
    }
}
impl CrossoverOp<Vec<usize>> for NoOpCrossover {
    fn crossover<R>(&self, parents: Parents<Vec<usize>>, _rng: &mut R) -> Children<Vec<usize>>
    where
        R: Rng + Sized,
    {
        parents
    }
}

pub type MySimulator = Simulator<
    GeneticAlgorithm<
        Vec<usize>,
        usize,
        FitnessCalc,
        MaximizeSelector,
        //PartiallyMappedCrossover,
        NoOpCrossover,
        SwapOrderMutator,
        UniformReinserter,
    >,
    GenerationLimit,
>;

pub fn init_optimization(
    params: &Parameters,
    evaluator: &Evaluator,
    layout_str: &str,
    layout_generator: &NeoLayoutGenerator,
    fixed_characters: &str,
    start_with_layout: bool,
) -> (MySimulator, PermutationLayoutGenerator) {
    let pm = PermutationLayoutGenerator::new(layout_str, fixed_characters, layout_generator);
    let initial_population: Population<Genotype> = if start_with_layout {
        build_population()
            .with_genome_builder(FixedLayoutBuilder::with_permutable_layout(&pm))
            .of_size(params.population_size)
            .uniform_at_random()
    } else {
        build_population()
            .with_genome_builder(LayoutBuilder::with_permutable_layout(&pm))
            .of_size(params.population_size)
            .uniform_at_random()
    };

    let result_cache = Mutex::new(FxHashMap::default());

    let sim = simulate(
        genetic_algorithm()
            .with_evaluation(FitnessCalc {
                evaluator: Arc::new(evaluator.clone()),
                layout_generator: pm.clone(),
                result_cache: Arc::new(result_cache),
            })
            .with_selection(MaximizeSelector::new(
                params.selection_ratio,
                params.num_individuals_per_parents,
            ))
            //.with_crossover(PartiallyMappedCrossover::new())
            .with_crossover(NoOpCrossover::new())
            .with_mutation(SwapOrderMutator::new(params.mutation_rate))
            .with_reinsertion(UniformReinserter::new(params.reinsertion_ratio))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(GenerationLimit::new(params.generation_limit))
    .build();

    (sim, pm)
}

pub fn optimize(
    params: &Parameters,
    evaluator: &Evaluator,
    layout_str: &str,
    layout_generator: &NeoLayoutGenerator,
    fixed_characters: &str,
    start_with_layout: bool,
) -> Layout {
    let (mut sim, pm) = init_optimization(
        params,
        evaluator,
        layout_str,
        layout_generator,
        fixed_characters,
        start_with_layout,
    );

    log::info!("Starting optimization with: {:?}", params);
    let mut all_time_best: Option<(usize, Genotype)> = None;

    loop {
        let result = sim.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                if let Some(king) = &all_time_best {
                    if best_solution.solution.fitness > king.0 {
                        println!(
                            "New best:\n{}\n\n{}\n{}",
                            pm.generate(&best_solution.solution.genome).as_text(),
                            pm.generate(&best_solution.solution.genome).plot_compact(),
                            pm.generate(&best_solution.solution.genome).plot()
                        );
                        all_time_best = Some((
                            best_solution.solution.fitness,
                            best_solution.solution.genome.clone(),
                        ));
                    }
                } else {
                    all_time_best = Some((
                        best_solution.solution.fitness,
                        best_solution.solution.genome.clone(),
                    ));
                }
                println!(
                    "Step: generation: {}, average_fitness: {}, \
                     best fitness: {}, all time best: {}, duration: {}, processing_time: {}, generation's best: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    all_time_best.as_ref().unwrap().0,
                    step.duration.fmt(),
                    step.processing_time.fmt(),
                    pm.generate(&best_solution.solution.genome).as_text()
                );
            }
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                println!("{}", stop_reason);
                println!(
                    "Final result after {}: generation: {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    processing_time.fmt()
                );
                println!(
                    "\n{}",
                    pm.generate(&all_time_best.as_ref().unwrap().1).as_text()
                );
                println!(
                    "\n{}",
                    pm.generate(&all_time_best.as_ref().unwrap().1)
                        .plot_compact()
                );
                println!(
                    "\n{}",
                    pm.generate(&all_time_best.as_ref().unwrap().1).plot()
                );
                break;
            }
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
    }

    pm.generate(&all_time_best.as_ref().unwrap().1)
}
