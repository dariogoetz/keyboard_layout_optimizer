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
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    for layout_str in options.layout_str.iter() {
        let layout = match layout_generator.generate(layout_str) {
            Ok(layout) => layout,
            Err(e) => {
                log::error!("Error in generating layout: {:?}", e);
                panic!("{:?}", e);
            }
        };
        println!("Layout (layer 1):\n{}", layout.plot_layer(0));
        println!("Layout compact (layer 1):\n{}", layout.plot_compact());
        let evaluation_result = evaluator.evaluate_layout(&layout);
        println!("{}", evaluation_result);
    }
}
