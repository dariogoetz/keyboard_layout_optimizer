use keyboard_layout::layout::Layout;
use keyboard_layout_optimizer::common;
use layout_evaluation::{cache::Cache, results::EvaluationResult};

use clap::Parser;
use rayon::prelude::*;
use serde::Serialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Serialize)]
struct LayoutEvaluation {
    details: EvaluationResult,
    total_cost: f64,
}

impl From<EvaluationResult> for LayoutEvaluation {
    fn from(details: EvaluationResult) -> Self {
        let total_cost = details.total_cost();
        Self {
            details,
            total_cost,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(name = "Keyboard layout evaluation")]
struct Options {
    /// List of Layout keys from left to right, top to bottom
    layout_str: Vec<String>,

    /// Do not remove whitespace from layout strings
    #[clap(long)]
    do_not_remove_whitespace: bool,

    /// Read layouts from file and append to command line layouts
    #[clap(long)]
    from_file: Option<String>,

    /// General parameters
    #[clap(flatten)]
    general_parameters: common::Options,

    /// If to only output the results as JSON to stdout
    #[clap(long)]
    json: bool,

    /// Print only total costs
    #[clap(long)]
    only_total_costs: bool,

    /// Sort results by total costs
    #[clap(long)]
    sort: bool,
}

fn main() {
    dotenv::dotenv().ok();
    let options = Options::parse();
    if !options.json {
        // if the "json" option is set, we do not want any other log messages
        env_logger::init();
    }

    let (layout_generator, evaluator) = common::init(&options.general_parameters);

    // collect layout strings to a vec
    let mut layout_strings = options.layout_str.to_vec();
    if let Some(filename) = &options.from_file {
        match File::open(filename) {
            Ok(file) => {
                layout_strings.append(&mut BufReader::new(file).lines().flatten().collect());
            }
            Err(e) => {
                log::error!("Error reading layouts file {}: {:?}", filename, e);
                panic!("{:?}", e);
            }
        }
    }

    let result_cache: Cache<EvaluationResult> = Cache::new();

    // evaluate layouts
    let mut results: Vec<(String, Layout, EvaluationResult)> = layout_strings
        .par_iter()
        .map(|layout_str| {
            let layout_str: String = layout_str
                .chars()
                .filter(|c| options.do_not_remove_whitespace || !c.is_whitespace())
                .collect();
            let layout = match layout_generator.generate(&layout_str) {
                Ok(layout) => layout,
                Err(e) => {
                    log::error!("Error in generating layout: {:?}", e);
                    panic!("{:?}", e);
                }
            };
            let evaluation_result =
                result_cache.get_or_insert_with(&layout_str, || evaluator.evaluate_layout(&layout));
            (layout_str, layout, evaluation_result)
        })
        .collect();

    // sort if required
    if options.sort {
        results.sort_by(|(_, _, c1), (_, _, c2)| {
            c1.total_cost().partial_cmp(&c2.total_cost()).unwrap()
        });
    }

    // print results
    if options.json {
        let results: Vec<LayoutEvaluation> =
            results.into_iter().map(|(_, _, res)| res.into()).collect();
        println!("{}", serde_json::to_string(&results).unwrap());
    } else {
        for (layout_str, layout, evaluation_result) in results {
            if !options.only_total_costs {
                println!("Layout (layer 1):\n{}", layout.plot_layer(0));
                println!("Layout string (layer 1):\n{}\n", layout);
                println!("{}", evaluation_result);
            } else {
                println!("{} {:4.2}", layout_str, evaluation_result.total_cost());
            }
        }
    }
}
