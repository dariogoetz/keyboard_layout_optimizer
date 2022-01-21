use colored::Colorize;
use structopt::StructOpt;

use evolve_keyboard_layout::common;
use layout_optimization_abc::optimization;

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
    #[structopt(short, long, default_value = "config/optimization_parameters_abc.yml")]
    optimization_parameters: String,

    /// Do not cache intermediate results
    #[structopt(long)]
    no_cache_results: bool,

    /// Append found layouts to file
    #[structopt(long)]
    append_solutions_to: Option<String>,

    /// Publishing options
    #[structopt(flatten)]
    publishing_options: common::PublishingOptions,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // disable storing worst ngrams for speed boost
    std::env::set_var("SHOW_WORST", "false");

    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let optimization_params = optimization::Parameters::from_yaml(&options.optimization_parameters)
        .expect(&format!(
            "Could not read optimization parameters from {}.",
            &options.optimization_parameters,
        ));

    let fix_from = options.fix_from.to_string();

    for new_best in optimization::optimize(
        &optimization_params,
        &evaluator,
        &fix_from,
        &layout_generator,
        &options.fix.clone().unwrap_or_else(|| "".to_string()),
        !options.no_cache_results,
    ) {
        let layout = new_best.solution;
        let evaluation_result = evaluator.evaluate_layout(&layout);
        println!(
            "{}\n\n{}\n\n{}\n{}\n{}\n",
            "New best layout:".yellow().bold(),
            layout,
            layout.plot_compact(),
            layout.plot(),
            evaluation_result
        );

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
    }
}
