use keyboard_layout::{layout::Layout, layout_generator::NeoLayoutGenerator};
use layout_evaluation::{cache::Cache, evaluation::Evaluator};

use layout_optimization_common::PermutationLayoutGenerator;

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
    layout_generator: PermutationLayoutGenerator,
    result_cache: Option<Cache<usize>>,
}

impl FitnessFunction<Genotype, usize> for FitnessCalc {
    fn fitness_of(&self, genome: &Genotype) -> usize {
        let l = self.layout_generator.generate_layout(genome);
        let layout_str = self.layout_generator.generate_string(genome);

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

struct FromGivenLayoutBuilder {
    indices: Vec<usize>,
}

impl FromGivenLayoutBuilder {
    fn with_permutable_layout(layout_prototype: &PermutationLayoutGenerator) -> Self {
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
        UniformReinserter, // we do not use an elitist reinserter due to performance reasons (non-parallelized evaluation)
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
    cache_results: bool,
) -> (MySimulator, PermutationLayoutGenerator) {
    let pm = PermutationLayoutGenerator::new(layout_str, fixed_characters, layout_generator);
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
                layout_generator: pm.clone(),
                result_cache,
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
    cache_results: bool,
) -> Layout {
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
                        let layout = pm.generate_layout(&best_solution.solution.genome);
                        let evaluation_result = evaluator.evaluate_layout(&layout);
                        println!(
                            "{}: {} (score: {})\n{}",
                            format!("New best in generation {}:", step.iteration)
                                .yellow()
                                .bold(),
                            layout,
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
                    pm.generate_layout(&best_solution.solution.genome)
                );
            }
            Ok(SimResult::Final(step, processing_time, duration, _stop_reason)) => {
                let layout = pm.generate_layout(&all_time_best.as_ref().unwrap().1);
                println!(
                    "{} after generation {}, duration {}, processing time {}\n\n{}\n\n{}\n{}",
                    "Final result".green().bold(),
                    step.iteration,
                    duration.fmt(),
                    processing_time.fmt(),
                    layout,
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

    pm.generate_layout(&all_time_best.as_ref().unwrap().1)
}
