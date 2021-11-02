use structopt::StructOpt;

use evolve_keyboard_layout::common;


#[derive(StructOpt, Debug)]
#[structopt(name = "Keyboard layout ptimization")]
struct Options {
    /// Layout keys from left to right, top to bottom
    layout_str: String,

    /// Filename of layout configuration file to use
    #[structopt(short, long, default_value = "standard_keyboard.yml")]
    layout_config: String,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = Options::from_args();

    let layout_generator = common::init_layout_generator(&options.layout_config);

    let layout = match layout_generator.generate(&options.layout_str) {
        Ok(layout) => layout,
        Err(e) => {
            log::error!("{:?}", e);
            panic!("{:?}", e);
        }
    };
    println!(
        "Layout '{}' (layer 1):\n{}",
        options.layout_str,
        layout.plot_layer(0)
    );
    println!(
        "Layout '{}' (layer 2):\n{}",
        options.layout_str,
        layout.plot_layer(1)
    );
    println!(
        "Layout '{}' (layer 3):\n{}",
        options.layout_str,
        layout.plot_layer(2)
    );
    println!(
        "Layout '{}' (layer 4):\n{}",
        options.layout_str,
        layout.plot_layer(3)
    );
    println!(
        "Layout '{}' (layer 5):\n{}",
        options.layout_str,
        layout.plot_layer(4)
    );
    println!(
        "Layout '{}' (layer 6):\n{}",
        options.layout_str,
        layout.plot_layer(5)
    );
    println!("Layout compact: \n{}", layout.plot_compact());
    println!("Layout as text: \n{}", layout);
}
