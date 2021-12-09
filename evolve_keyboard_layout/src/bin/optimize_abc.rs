use structopt::StructOpt;

use evolve_keyboard_layout::common;
use layout_optimization::optimization_abc;

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

    /// Publish found layout to webservice under this name
    #[structopt(long)]
    publish_as: Option<String>,

    /// Publish found layout to webservice at this url
    #[structopt(
        long,
        default_value = "https://keyboard-layout-optimizer.herokuapp.com/api"
    )]
    publish_to: String,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let optimization_params =
        optimization_abc::Parameters::from_yaml(&options.optimization_parameters).expect(&format!(
            "Could not read optimization parameters from {}.",
            &options.optimization_parameters,
        ));

    let fix_from = options.fix_from.to_string();

    for new_best in optimization_abc::optimize(
        &optimization_params,
        &evaluator,
        &fix_from,
        &layout_generator,
        &options.fix.clone().unwrap_or_else(|| "".to_string()),
        !options.no_cache_results,
    ) {
        let layout = new_best.solution;
        println!("{}", layout.plot());
        println!("{}", layout.plot_compact());

        let evaluation_result = evaluator.evaluate_layout(&layout);
        println!("{}", evaluation_result);

        // Log solution to file.
        if let Some(filename) = &options.append_solutions_to {
            common::append_to_file(&layout, filename);
        }

        // Publish to webservice.
        if let Some(publish_name) = &options.publish_as {
            common::publish_to_webservice(&layout, publish_name, &options.publish_to);
        }
    }
}
