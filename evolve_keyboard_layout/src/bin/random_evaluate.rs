use structopt::StructOpt;

use rand::seq::SliceRandom;
use rand::thread_rng;

mod common;

#[derive(StructOpt, Debug)]
#[structopt(name = "Random keyboard layout evaluation")]
struct Options {
    /// Evaluation parameters
    #[structopt(flatten)]
    evaluation_parameters: common::Options,

    /// Number of samples
    #[structopt(short, long, default_value = "1000")]
    number_of_samples: usize,
}
fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = Options::from_args();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);

    let layout_str = "abcdefghijklmnopqrstuvwxyzäöüß,.";
    let mut best_cost: Option<f64> = None;
    let mut best_layout: String = "".into();

    for _ in 0..options.number_of_samples {
        let mut rng = thread_rng();
        let mut s: Vec<char> = layout_str.chars().collect();
        s.shuffle(&mut rng);
        let s: String = s.iter().collect();

        let layout = match layout_generator.generate(&s) {
            Ok(layout) => layout,
            Err(e) => {
                log::error!("Error in generating layout: {:?}", e);
                panic!("{:?}", e);
            }
        };

        let evaluation_result = evaluator.evaluate_layout(&layout);

        let cost = evaluation_result.total_cost();
        best_cost = Some(best_cost.unwrap_or(cost));

        if cost < best_cost.unwrap() {
            best_layout = s.clone();
            best_cost = Some(cost);
        };

        log::info!("Evaluated {}: {}", s, cost);
    }
    log::info!("Best: {}: {}", best_layout, best_cost.unwrap_or(0.0));
    // for layout_str in options.layout_str.iter() {
    //     let layout = match layout_generator.generate(layout_str) {
    //         Ok(layout) => layout,
    //         Err(e) => {
    //             log::error!("Error in generating layout: {:?}", e);
    //             panic!("{:?}", e);
    //         }
    //     };
    //     println!("Layout (layer 1):\n{}", layout.plot_layer(0));
    //     let metric_costs = evaluator.evaluate_layout(&layout);
    //     let mut cost = 0.0;
    //     for mc in metric_costs.iter().filter(|mc| mc.metric_costs.len() > 0) {
    //         cost += mc.total_cost();
    //         mc.print();
    //     }

    //     println!(
    //         "Cost: {:.4} (optimization score: {})\n",
    //         cost,
    //         (1e8 / cost) as usize
    //     );
    // }
}
