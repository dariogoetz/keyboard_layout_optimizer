use structopt::StructOpt;

use evolve_keyboard_layout::common;
use layout_evaluation::cache::Cache;
use layout_optimization_genetic::optimization;

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
    #[structopt(long, default_value = "xvlcwkhgfqyßuiaeosnrtdüöäpzbm,.j")]
    fix_from: String,

    /// Filename of optimization configuration file
    #[structopt(
        short,
        long,
        default_value = "config/optimization_parameters_genetic.yml"
    )]
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

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // Disable storing worst ngrams for speed boost
    std::env::set_var("SHOW_WORST", "false");

    let final_results: Cache<f64> = Cache::new();

    // Handle Ctrl+C
    let cloned_final_results = final_results.clone();
    ctrlc::set_handler(move || {
        // Display a summary of the optimization.
        println!("\n\n{}\n", cloned_final_results);
        // Stop execution
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let mut optimization_params =
        optimization::Parameters::from_yaml(&options.optimization_parameters).expect(&format!(
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
        let _ =
            final_results.get_or_insert_with(&layout.as_text(), || evaluation_result.total_cost());

        println!("{}", evaluation_result);

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

        if !options.run_forever {
            break;
        }
    }
    println!("\n{}\n", final_results);
}
