use keyboard_layout_optimizer::common;
use layout_evaluation::cache::Cache;
use layout_optimization_sa::optimization;

use clap::Parser;
use colored::Colorize;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{env, process};

#[derive(Parser, Debug)]
#[clap(name = "Keyboard layout optimization - Simulated Annealing")]
struct Options {
    /// Evaluation parameters
    #[clap(flatten)]
    evaluation_parameters: common::Options,

    /// Do not optimize those keys (wrt. --start-layout or --fix-from)
    #[clap(short, long)]
    fix: Option<String>,

    /// Fix the keys from this layout (will be overwritten by --start-layout)
    #[clap(long, default_value = "xvlcwkhgfqßuiaeosnrtdyüöäpzbm,.j")]
    fix_from: String,

    /// Filename of optimization configuration file
    #[clap(short, long, default_value = "config/optimization/sa.yml")]
    optimization_parameters: String,

    /// Start optimization from this layout (keys from left to right, top to bottom)
    #[clap(short, long)]
    start_layouts: Vec<String>,

    /// Do not remove whitespace from layout strings
    #[clap(long)]
    do_not_remove_whitespace: bool,

    /// Do not cache intermediate results
    #[clap(long)]
    no_cache_results: bool,

    /// Set the initial temperature (Will be overwritten by --greedy)
    #[clap(long)]
    init_temp: Option<f64>,

    /// Set the init_temp to 0.0, turning the Simulated Annealing algorithm into a greedy one
    #[clap(short, long)]
    greedy: bool,

    /// If used, log every single iteration instead of every 100th.
    #[clap(long)]
    log_everything: bool,

    /// Append found layouts to file
    #[clap(long)]
    append_solutions_to: Option<String>,

    /// Repeat optimizations indefinitely
    #[clap(long)]
    run_forever: bool,

    /// Publishing options
    #[clap(flatten)]
    publishing_options: common::PublishingOptions,
}

/// An iterator for layouts to feed into the optimizer.
/// If `run_forever` is true, it iterates over the given layouts indefinitely.
struct LayoutIterator {
    layouts: Vec<String>,
    run_forever: bool,
    i: usize,
}

impl LayoutIterator {
    fn new<T: AsRef<str>>(layouts: &[T], run_forever: bool) -> Self {
        Self {
            layouts: layouts.iter().map(|s| s.as_ref().to_string()).collect(),
            run_forever,
            i: 0,
        }
    }
}

impl Iterator for LayoutIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.layouts.len() {
            // There are still elements left to give
            let res = self.layouts[self.i].clone();
            self.i += 1;

            Some(res)
        } else {
            // All elements of this.layouts have been given
            if self.run_forever {
                // Loop around and start anew
                self.i = 0;

                Some(self.layouts[self.i].clone())
            } else {
                // Finish iteration
                None
            }
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // Disable storing worst ngrams for speed boost
    if env::var("SHOW_WORST").is_err() {
        env::set_var("SHOW_WORST", "false");
    };

    let final_results: Cache<f64> = Cache::new();

    // Handle Ctrl+C
    let cloned_final_results = final_results.clone();
    ctrlc::set_handler(move || {
        // Display a summary of the optimization.
        println!("\n\n{}\n", cloned_final_results);
        // Stop execution
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let options = Options::parse();

    let fix_from: String = options
        .fix_from
        .chars()
        .filter(|c| options.do_not_remove_whitespace || !c.is_whitespace())
        .collect();

    let start_layouts: Vec<String> = options
        .start_layouts
        .iter()
        .map(|s| {
            s.chars()
                .filter(|c| options.do_not_remove_whitespace || !c.is_whitespace())
                .collect::<String>()
        })
        .collect();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let mut optimization_params = optimization::Parameters::from_yaml(
        &options.optimization_parameters,
    )
    .unwrap_or_else(|_| {
        panic!(
            "Could not read optimization parameters from {}.",
            &options.optimization_parameters
        )
    });
    if options.greedy {
        optimization_params.init_temp = Some(f64::MIN_POSITIVE);
    } else if options.init_temp.is_some() {
        optimization_params.init_temp = options.init_temp;
    }
    optimization_params.correct_init_temp();

    let mut layouts: Vec<String> = start_layouts.to_vec();
    if layouts.is_empty() {
        layouts = vec![fix_from];
    }
    let layout_iterator = LayoutIterator::new(&layouts, options.run_forever);
    let start_from_layout = !start_layouts.is_empty();

    let cache: Option<Cache<f64>> = match !options.no_cache_results {
        true => Some(Cache::new()),
        false => None,
    };

    layout_iterator
        .enumerate()
        .par_bridge()
        .for_each(|(i, fix_from)| {
            let process_id = format!("Process {:>3}", i);
            if start_from_layout {
                log::info!(
                    "{} Starting optimization from {}",
                    format!("{}:", process_id).yellow().bold(),
                    fix_from
                );
            } else {
                log::info!(
                    "{} Starting optimization",
                    format!("{}:", process_id).yellow().bold(),
                );
            }

            // Perform the optimization.
            let (layout_str, layout) = optimization::optimize(
                &process_id,
                &optimization_params,
                &fix_from,
                &options.fix.clone().unwrap_or_default(),
                &layout_generator,
                start_from_layout,
                &evaluator,
                options.log_everything,
                cache.clone(),
                None,
            );
            let evaluation_result = evaluator.evaluate_layout(&layout);
            let cost = evaluation_result.total_cost();
            let _ = final_results.get_or_insert_with(&layout_str, || cost);

            // Plot some information regarding the layout.
            println!(
                "{} {}\n\n{}\n\n{}\n{}\n{}\n\n{}\n",
                format!("{}:", process_id).yellow().bold(),
                "Final result:".green().bold(),
                layout,
                layout.plot_compact(),
                layout.plot(),
                evaluation_result,
                final_results.highlighted_fmt(Some(&layout_str), 10),
            );

            // Log solution to file.
            if let Some(filename) = &options.append_solutions_to {
                common::append_to_file(&layout_str, filename);
            }

            // Publish to webservice.
            let o = &options.publishing_options;
            if o.publish_as.is_some() && cost < o.publish_if_cost_below.unwrap_or(f64::INFINITY) {
                common::publish_to_webservice(
                    &layout_str,
                    o.publish_as.as_ref().unwrap(),
                    &o.publish_to,
                    &o.publish_layout_config,
                );
            }
        });
}
