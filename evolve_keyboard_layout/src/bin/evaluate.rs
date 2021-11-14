use layout_evaluation::results::EvaluationResult;
use structopt::StructOpt;

use evolve_keyboard_layout::common;

#[derive(StructOpt, Debug)]
#[structopt(name = "Keyboard layout optimization")]
struct Options {
    /// List of Layout keys from left to right, top to bottom
    layout_str: Vec<String>,

    /// Evaluation parameters
    #[structopt(flatten)]
    evaluation_parameters: common::Options,

    /// If to only output the results as JSON to stdout
    #[structopt(long)]
    return_json: bool,
}

fn main() {
    dotenv::dotenv().ok();
    let options = Options::from_args();
    if !options.return_json {
        env_logger::init();
    }

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let mut results: Vec<EvaluationResult> = Vec::new();
    for layout_str in options.layout_str.iter() {
        let layout = match layout_generator.generate(layout_str) {
            Ok(layout) => layout,
            Err(e) => {
                log::error!("Error in generating layout: {:?}", e);
                panic!("{:?}", e);
            }
        };
        let evaluation_result = evaluator.evaluate_layout(&layout);
        results.push(evaluation_result.clone());
        if !options.return_json {
            println!("Layout (layer 1):\n{}", layout.plot_layer(0));
            println!("Layout compact (layer 1):\n{}", layout.plot_compact());
            println!("{}", evaluation_result);
        }
    }
    if options.return_json {
        println!("{}", serde_json::to_string(&results).unwrap());
    }
}
