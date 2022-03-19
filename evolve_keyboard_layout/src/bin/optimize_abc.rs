use clap::Parser;
use colored::Colorize;

use evolve_keyboard_layout::common;
use layout_optimization_abc::optimization;

#[derive(Parser, Debug)]
#[clap(name = "Keyboard layout optimization - Artificial Bee Colony")]
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
    #[clap(short, long, default_value = "config/optimization/abc.yml")]
    optimization_parameters: String,

    /// Do not cache intermediate results
    #[clap(long)]
    no_cache_results: bool,

    /// Append found layouts to file
    #[clap(long)]
    append_solutions_to: Option<String>,

    /// Publishing options
    #[clap(flatten)]
    publishing_options: common::PublishingOptions,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // disable storing worst ngrams for speed boost
    if std::env::var("SHOW_WORST").is_err() {
        std::env::set_var("SHOW_WORST", "false");
    };

    let options = Options::parse();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let optimization_params = optimization::Parameters::from_yaml(&options.optimization_parameters)
        .unwrap_or_else(|_| {
            panic!(
                "Could not read optimization parameters from {}.",
                &options.optimization_parameters
            )
        });

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
        let o = &options.publishing_options;
        if o.publish_as.is_some()
            && evaluation_result.total_cost() < o.publish_if_cost_below.unwrap_or(f64::INFINITY)
        {
            common::publish_to_webservice(
                &layout,
                o.publish_as.as_ref().unwrap(),
                &o.publish_to,
                &o.publish_layout_config,
            );
        }
    }
}
