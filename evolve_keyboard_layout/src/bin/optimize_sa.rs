use colored::Colorize;
use rayon::iter::{ParallelBridge, ParallelIterator};
use structopt::StructOpt;

use evolve_keyboard_layout::common;
use layout_evaluation::cache::Cache;
use layout_optimization_sa::optimization;

#[derive(StructOpt, Debug)]
#[structopt(name = "Keyboard layout optimization")]
struct Options {
    /// Evaluation parameters
    #[structopt(flatten)]
    evaluation_parameters: common::Options,

    /// Do not optimize those keys (wrt. --start-layout or --fix-from)
    #[structopt(short, long)]
    fix: Option<String>,

    /// Fix the keys from this layout (will be overwritten by --start-layout)
    #[structopt(long, default_value = "xvlcwkhgfqßuiaeosnrtdyüöäpzbm,.j")]
    fix_from: String,

    /// Filename of optimization configuration file
    #[structopt(short, long, default_value = "config/optimization_parameters_sa.yml")]
    optimization_parameters: String,

    /// Start optimization from this layout (keys from left to right, top to bottom)
    #[structopt(short, long)]
    start_layouts: Vec<String>,

    /// Do not cache intermediate results
    #[structopt(long)]
    no_cache_results: bool,

    /// Set the initial temperature (Will be overwritten by --greedy)
    #[structopt(long)]
    init_temp: Option<f64>,

    /// Set the init_temp to 0.0, turning the Simulated Annealing algorithm into a greedy one
    #[structopt(short, long)]
    greedy: bool,

    /// If used, log every single iteration instead of every 100th.
    #[structopt(long)]
    log_everything: bool,

    /// Append found layouts to file
    #[structopt(long)]
    append_solutions_to: Option<String>,

    /// Repeat optimizations indefinitely
    #[structopt(long)]
    run_forever: bool,

    /// Publishing options
    #[structopt(flatten)]
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
        let res = if self.i < self.layouts.len() {
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
        };

        res
    }
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // disable storing worst ngrams for speed boost
    std::env::set_var("SHOW_WORST", "false");

    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let mut optimization_params =
        optimization::Parameters::from_yaml(&options.optimization_parameters).expect(&format!(
            "Could not read optimization parameters from {}.",
            &options.optimization_parameters,
        ));
    if options.greedy {
        optimization_params.init_temp = Some(f64::MIN_POSITIVE);
    } else if options.init_temp.is_some() {
        optimization_params.init_temp = options.init_temp;
    }
    optimization_params.correct_init_temp();

    let mut layouts: Vec<String> = options.start_layouts.to_vec();
    if layouts.is_empty() {
        layouts = vec![options.fix_from.clone()];
    }
    let layout_iterator = LayoutIterator::new(&layouts, options.run_forever);

    let start_from_layout = !options.start_layouts.is_empty();

    let cache: Option<Cache<f64>> = match !options.no_cache_results {
        true => Some(Cache::new()),
        false => None,
    };
    let result_cache: Cache<f64> = Cache::new();

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
            let layout = optimization::optimize(
                &process_id,
                &optimization_params,
                &fix_from,
                &options.fix.clone().unwrap_or_else(|| "".to_string()),
                &layout_generator,
                start_from_layout,
                &evaluator,
                options.log_everything,
                cache.clone(),
                None,
            );

            // Plot some information regarding the layout.
            let evaluation_result = evaluator.evaluate_layout(&layout);
            let mut output_string = format!(
                "{} {}\n\n{}\n\n{}\n{}\n{}",
                format!("{}:", process_id).yellow().bold(),
                "Final result:".green().bold(),
                layout,
                layout.plot_compact(),
                layout.plot(),
                evaluation_result
            );
            if options.run_forever {
                // Simulated Annealing can produce a high number of layouts.
                // To combat this chaos, an overview of all layouts found so far is shown after every cycle, if optimization is run_forever.
                let current_layout = layout.as_text();
                let _ = result_cache
                    .get_or_insert_with(&current_layout, || evaluation_result.total_cost());
                output_string.push_str(&format!(
                    "\n{}",
                    result_cache.highlighted_fmt(Some(&current_layout)),
                ));
            }
            println!("{}\n", output_string);

            // Log solution to file.
            if let Some(filename) = &options.append_solutions_to {
                common::append_to_file(&layout, filename);
            }

            // Publish to webservice.
            if let Some(publish_name) = &options.publishing_options.publish_as {
                common::publish_to_webservice(
                    &layout,
                    publish_name,
                    &options.publishing_options.publish_to,
                    &options.publishing_options.publish_layout_config,
                );
            }
        });
}
