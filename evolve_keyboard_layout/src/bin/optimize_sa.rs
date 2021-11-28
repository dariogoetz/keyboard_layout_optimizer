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
    start_layout: Option<String>,
    //
    /// Do not cache intermediate results
    #[structopt(long)]
    no_cache_results: bool,

    /// Set the init_temp to 0.0, turning the Simulated Annealing algorithm into a greedy one.
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

    // If it was provided, use the [start_layout] as [fix_from].
    let fix_from = options
        .start_layout
        .as_ref()
        .unwrap_or(&options.fix_from)
        .to_string();

    loop {
        let layout = optimization_sa::optimize(
            &optimization_params,
            &fix_from,
            &options.fix.clone().unwrap_or_else(|| "".to_string()),
            &layout_generator,
            options.start_layout.is_some(),
            &evaluator,
            options.greedy,
            !options.no_cache_results,
        );

        println!("{}", layout.plot());
        println!("{}", layout.plot_compact());

        let evaluation_result = evaluator.evaluate_layout(&layout);
        println!("{}", evaluation_result);

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

        if !options.run_forever {
            break;
        }
    }
}
