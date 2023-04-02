use keyboard_layout::{layout::Layout, layout_generator::LayoutGenerator};
use layout_evaluation::{cache::Cache, evaluation::Evaluator};

use layout_optimization_common::LayoutPermutator;

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{fs::File, sync::Arc};

use genevo::{
    genetic::{Children, FitnessFunction, Parents},
    operator::{prelude::*, CrossoverOp, GeneticOperator},
    population::Population,
    prelude::*,
    random::SliceRandom,
    simulation::simulator::Simulator,
    types::fmt::Display,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub population_size: usize,
    pub generation_limit: u64,
    pub num_individuals_per_parents: usize,
    pub selection_ratio: f64,
    pub mutation_rate: f64,
    pub reinsertion_ratio: f64,
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
        let f = File::open(filename)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

// The genotype
type Genotype = Vec<usize>;

/// The fitness function for [`Genotype`]s.
#[derive(Clone, Debug)]
pub struct FitnessCalc {
    evaluator: Arc<Evaluator>,
    permutator: LayoutPermutator,
    layout_generator: Box<dyn LayoutGenerator>,
    result_cache: Option<Cache<usize>>,
}

impl FitnessFunction<Genotype, usize> for FitnessCalc {
    fn fitness_of(&self, genome: &Genotype) -> usize {
        let layout_str = self.permutator.generate_string(genome);
        let l = self.layout_generator.generate(&layout_str).unwrap();

        // Get & return the evaluation-result
        match &self.result_cache {
            Some(result_cache) => result_cache.get_or_insert_with(&layout_str, || {
                self.evaluator.evaluate_layout(&l).optimization_score()
            }),
            None => self.evaluator.evaluate_layout(&l).optimization_score(),
        }
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
    fn with_permutable_layout(layout_prototype: &LayoutPermutator) -> Self {
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

struct FromGivenLayoutBuilder {
    indices: Vec<usize>,
}

impl FromGivenLayoutBuilder {
    fn with_permutable_layout(layout_prototype: &LayoutPermutator) -> Self {
        Self {
            indices: layout_prototype.get_permutable_indices(),
        }
    }
}

impl GenomeBuilder<Vec<usize>> for FromGivenLayoutBuilder {
    fn build_genome<R>(&self, _: usize, _rng: &mut R) -> Vec<usize>
    where
        R: Rng + Sized,
    {
        // start with initial layout
        self.indices.clone()
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
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

// Crossover method as used in https://github.com/Coletronix/Genetic-Keyboard-Generator
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct MyCrossover {}
impl MyCrossover {
    pub fn new() -> Self {
        MyCrossover {}
    }
}
impl GeneticOperator for MyCrossover {
    fn name() -> String {
        "My-Crossover".to_string()
    }
}
impl CrossoverOp<Vec<usize>> for MyCrossover {
    fn crossover<R>(&self, parents: Parents<Vec<usize>>, rng: &mut R) -> Children<Vec<usize>>
    where
        R: Rng + Sized,
    {
        parents
            .iter()
            .zip(parents.iter().skip(1).cycle())
            .map(|(p1, p2)| {
                let len = p1.len();
                let mut offspring: Vec<Option<usize>> = vec![None; len];

                // determine cycle and take from parent 1
                let start_idx = rng.gen_range(0..len);
                offspring[start_idx] = Some(p1[start_idx]);

                // find index in p1 of value that sits in p2 at start_idx
                let mut idx = p1.iter().position(|v| *v == p2[start_idx]).unwrap().clone();
                while idx != start_idx {
                    offspring[idx] = Some(p1[idx]);

                    // find index in p1 of value that sits in p2 at idx
                    idx = p1.iter().position(|v| *v == p2[idx]).unwrap().clone();
                }

                // rest is copied from parent 2
                p2.iter()
                    .zip(offspring.iter_mut())
                    .for_each(|(p_val, o_val)| {
                        if o_val.is_none() {
                            *o_val = Some(p_val.clone());
                        }
                    });

                offspring.iter().map(|val| val.unwrap()).collect()
            })
            .collect()
    }
}

pub type MySimulator = Simulator<
    GeneticAlgorithm<
        Vec<usize>,
        usize,
        FitnessCalc,
        MaximizeSelector,
        // PartiallyMappedCrossover,
        // MyCrossover,
        NoOpCrossover,
        SwapOrderMutator,
        UniformReinserter, // we do not use an elitist reinserter due to performance reasons (non-parallelized evaluation)
    >,
    GenerationLimit,
>;

pub fn init_optimization(
    params: &Parameters,
    evaluator: &Evaluator,
    layout_str: &str,
    layout_generator: &Box<dyn LayoutGenerator>,
    fixed_characters: &str,
    start_with_layout: bool,
    cache_results: bool,
) -> (MySimulator, LayoutPermutator) {
    let pm = LayoutPermutator::new(layout_str, fixed_characters);
    let initial_population: Population<Genotype> = if start_with_layout {
        build_population()
            .with_genome_builder(FromGivenLayoutBuilder::with_permutable_layout(&pm))
            .of_size(params.population_size)
            .uniform_at_random()
    } else {
        build_population()
            .with_genome_builder(LayoutBuilder::with_permutable_layout(&pm))
            .of_size(params.population_size)
            .uniform_at_random()
    };

    let result_cache = if cache_results {
        Some(Cache::new())
    } else {
        None
    };

    let sim = simulate(
        genetic_algorithm()
            .with_evaluation(FitnessCalc {
                evaluator: Arc::new(evaluator.clone()),
                permutator: pm.clone(),
                layout_generator: layout_generator.clone(),
                result_cache,
            })
            .with_selection(MaximizeSelector::new(
                params.selection_ratio,
                params.num_individuals_per_parents,
            ))
            // .with_crossover(PartiallyMappedCrossover::new())
            // .with_crossover(MyCrossover::new())
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
    layout_generator: &Box<dyn LayoutGenerator>,
    fixed_characters: &str,
    start_with_layout: bool,
    cache_results: bool,
) -> (String, Layout) {
    let (mut sim, pm) = init_optimization(
        params,
        evaluator,
        layout_str,
        layout_generator,
        fixed_characters,
        start_with_layout,
        cache_results,
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
                        let layout_str = pm.generate_string(&best_solution.solution.genome);
                        let layout = layout_generator.generate(&layout_str).unwrap();

                        let evaluation_result = evaluator.evaluate_layout(&layout);
                        println!(
                            "{}: {} (score: {})\n{}",
                            format!("New best in generation {}:", step.iteration)
                                .yellow()
                                .bold(),
                            layout_str,
                            format!("{}", evaluation_result.total_cost()).yellow(),
                            layout.plot(),
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
                log::info!(
                    "{}, average_fitness: {}, \
                     best fitness: {}, all time best: {}, duration: {}, processing_time: {}, generation's best: {}",
                    format!("Generation {}:", step.iteration).yellow().bold(),
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    all_time_best.as_ref().unwrap().0,
                    step.duration.fmt(),
                    step.processing_time.fmt(),
                    pm.generate_string(&best_solution.solution.genome)
                );
            }
            Ok(SimResult::Final(step, processing_time, duration, _stop_reason)) => {
                let layout_str = pm.generate_string(&all_time_best.as_ref().unwrap().1);
                let layout = layout_generator.generate(&layout_str).unwrap();
                println!(
                    "{} after generation {}, duration {}, processing time {}\n\n{}\n\n{}\n{}",
                    "Final result".green().bold(),
                    step.iteration,
                    duration.fmt(),
                    processing_time.fmt(),
                    layout_str,
                    layout.plot_compact(),
                    layout.plot()
                );
                break;
            }
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
    }

    let best_layout_str = pm.generate_string(&all_time_best.as_ref().unwrap().1);
    let best_layout = layout_generator.generate(&best_layout_str).unwrap();

    (best_layout_str, best_layout)
}
