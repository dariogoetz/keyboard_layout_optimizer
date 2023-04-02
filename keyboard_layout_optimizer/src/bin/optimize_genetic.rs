use keyboard_layout_optimizer::common;
use layout_evaluation::cache::Cache;
use layout_optimization_genetic::optimization;

use clap::Parser;
use std::{env, process};

#[derive(Parser, Debug)]
#[clap(name = "Keyboard layout optimization - Genetic Algorithm")]
struct Options {
    /// Evaluation parameters
    #[clap(flatten)]
    evaluation_parameters: common::Options,

    /// Do not optimize those keys (wrt. --start-layout or --fix-from)
    #[clap(short, long)]
    fix: Option<String>,

    /// Fix the keys from this layout (will be overwritten by --start-layout)
    #[clap(long, default_value = "xvlcwkhgfqyßuiaeosnrtdüöäpzbm,.j")]
    fix_from: String,

    /// Filename of optimization configuration file
    #[clap(short, long, default_value = "config/optimization/genetic.yml")]
    optimization_parameters: String,

    /// Start optimization from this layout (keys from left to right, top to bottom)
    #[clap(short, long)]
    start_layout: Option<String>,

    /// Do not remove whitespace from layout strings
    #[clap(long)]
    do_not_remove_whitespace: bool,

    /// Do not cache intermediate results
    #[clap(long)]
    no_cache_results: bool,

    /// Maximum number of generations
    #[clap(long)]
    generation_limit: Option<u64>,

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

    let start_layout = options.start_layout.as_ref().map(|s| {
        s.chars()
            .filter(|c| options.do_not_remove_whitespace || !c.is_whitespace())
            .collect::<String>()
    });

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

    if let Some(generation_limit) = options.generation_limit {
        optimization_params.generation_limit = generation_limit
    }

    let fix_from = start_layout.as_ref().unwrap_or(&fix_from).to_string();

    loop {
        let (layout_str, layout) = optimization::optimize(
            &optimization_params,
            &evaluator,
            &fix_from,
            &layout_generator,
            &options.fix.clone().unwrap_or_else(|| "".to_string()),
            start_layout.is_some(),
            !options.no_cache_results,
        );
        let evaluation_result = evaluator.evaluate_layout(&layout);
        let cost = evaluation_result.total_cost();
        let _ = final_results.get_or_insert_with(&layout_str, || cost);

        println!(
            "{}\n\n{}\n",
            evaluation_result,
            final_results.highlighted_fmt(Some(&layout_str), 10)
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

        if !options.run_forever {
            break;
        }
    }
}
