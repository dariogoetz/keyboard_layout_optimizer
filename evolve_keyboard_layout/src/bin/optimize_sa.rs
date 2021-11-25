use evolve_keyboard_layout::common;

fn main() {
    // No idea what those are used for.
    dotenv::dotenv().ok();
    env_logger::init();

    let (layout_generator, evaluator) = common::init(&options.evaluation_parameters);
}
