use structopt::StructOpt;

use evolve_keyboard_layout::common;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = common::Options::from_args();

    let (layout_generator, evaluator) = common::init(&options);

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
