use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use structopt::StructOpt;

use layout_optimization::optimization;
use evolve_keyboard_layout::common;

const PUBLISH_URL: &str = "http://localhost:8000/";

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
    #[structopt(short, long, default_value = "optimization_parameters.yml")]
    optimization_parameters: String,

    /// Start optimization from this layout (keys from left to right, top to bottom)
    #[structopt(short, long)]
    start_layout: Option<String>,

    /// Do not cache intermediate results
    #[structopt(long)]
    no_cache_results: bool,

    /// Maximum number of generations
    #[structopt(long)]
    generation_limit: Option<u64>,

    /// Append found layout to file
    #[structopt(long)]
    append_solution_to: Option<String>,

    /// Publish found layout to webservice under this name
    #[structopt(long)]
    publish_as: Option<String>,

    /// Repeat optimizations indefinitely
    #[structopt(long)]
    run_forever: bool,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let mut optimization_params = optimization::Parameters::from_yaml(&options.optimization_parameters)
        .expect(&format!(
            "Could not read optimization parameters from {}.",
            &options.optimization_parameters,
        ));

    if let Some(generation_limit) = options.generation_limit {
        optimization_params.generation_limit = generation_limit
    }

    let fix_from = options
        .start_layout
        .as_ref()
        .unwrap_or(&options.fix_from)
        .to_string();

    loop {
        let layout = optimization::optimize(
            &optimization_params,
            &evaluator,
            &fix_from,
            &layout_generator,
            &options.fix.clone().unwrap_or_else(|| "".to_string()),
            options.start_layout.is_some(),
            !options.no_cache_results,
        );

        let evaluation_result = evaluator.evaluate_layout(&layout);
        println!("{}", evaluation_result);

        if let Some(filename) = &options.append_solution_to {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(filename)
                .unwrap();

            if let Err(e) = writeln!(file, "{}", layout.as_text()) {
                log::error!("Couldn't write to file: {}", e);
            }
        }

        if let Some(publish_name) = &options.publish_as {

            let client = reqwest::blocking::Client::new();
            let mut body = HashMap::new();
            body.insert("published_by", publish_name.to_string());
            body.insert("layout", layout.as_text());

            let resp = client
                .post(PUBLISH_URL)
                .json(&body)
                .send()
                .ok();

            if let Some(resp) = resp {
                if resp.status().as_str() == "200" {
                    log::info!("Published layout '{}' to {}", layout.as_text(), PUBLISH_URL);
                } else {
                    log::error!("Could not publish result to webservice: {:?}", &resp.text());
                }
            } else {
                log::error!("Could not publish result to webservice");
            }
        }

        if !options.run_forever {
            break
        }
    }
}
