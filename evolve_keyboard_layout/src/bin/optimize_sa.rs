use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use structopt::StructOpt;

use evolve_keyboard_layout::common;
use layout_optimization::optimization_sa;

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

    /// Append found layouts to file
    #[structopt(long)]
    append_solutions_to: Option<String>,

    /// Publish found layout to webservice under this name
    #[structopt(long)]
    publish_as: Option<String>,

    /// Publish found layout to webservice at this url
    #[structopt(
        long,
        default_value = "https://keyboard-layout-optimizer.herokuapp.com/api"
    )]
    publish_to: String,

    /// Repeat optimizations indefinitely
    #[structopt(long)]
    run_forever: bool,
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
    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let optimization_params =
        optimization_sa::Parameters::from_yaml(&options.optimization_parameters).expect(&format!(
            "Could not read optimization parameters from {}.",
            &options.optimization_parameters,
        ));

    let mut layouts: Vec<String> = options.start_layouts.to_vec();
    if layouts.is_empty() {
        layouts = vec![options.fix_from.clone()];
    }
    let layout_iterator = LayoutIterator::new(&layouts, options.run_forever);

    let start_from_layout = !options.start_layouts.is_empty();

    let init_temp: Option<f64>;
    if options.greedy {
        init_temp = Some(f64::MIN_POSITIVE);
    } else {
        init_temp = match options.init_temp {
            Some(t) => {
                if t > 0.0 {
                    Some(t)
                } else {
                    println!("Please input an initial-temperature that is bigger than 0.");
                    None
                }
            }
            None => None,
        };
    }

    layout_iterator
        .enumerate()
        .par_bridge()
        .for_each(|(i, fix_from)| {
            if start_from_layout {
                log::info!("Starting optimization {} from {}", i, fix_from);
            } else {
                log::info!("Starting optimization {}", i);
            }

            // Perform the optimization.
            let layout = optimization_sa::optimize(
                &format!("Process {}", i),
                &optimization_params,
                &fix_from,
                &options.fix.clone().unwrap_or_else(|| "".to_string()),
                &layout_generator,
                start_from_layout,
                &evaluator,
                init_temp,
                !options.no_cache_results,
            );

            // Plot some information regarding the layout.
            println!("{}", layout.plot());
            println!("{}", layout.plot_compact());
            let evaluation_result = evaluator.evaluate_layout(&layout);
            println!("{}", evaluation_result);

            // Log solution to file.
            if let Some(filename) = &options.append_solutions_to {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(filename)
                    .unwrap();
                if let Err(e) = writeln!(file, "{}", layout.as_text()) {
                    log::error!("Couldn't write to file: {}", e);
                } else {
                    log::info!("Appended layout '{}' to '{}'", layout.as_text(), filename);
                }
            }

            // Publish to webservice.
            if let Some(publish_name) = &options.publish_as {
                let client = reqwest::blocking::Client::new();
                let mut body = HashMap::new();
                body.insert("published_by", publish_name.to_string());
                body.insert("layout", layout.as_text());
                let resp = client.post(&options.publish_to).json(&body).send().ok();
                if let Some(resp) = resp {
                    if resp.status().is_success() {
                        log::info!(
                            "Published layout '{}' to {}",
                            layout.as_text(),
                            &options.publish_to
                        );
                    } else {
                        log::error!("Could not publish result to webservice: {:?}", &resp.text());
                    }
                } else {
                    log::error!("Could not publish result to webservice");
                }
            }
        });
}
