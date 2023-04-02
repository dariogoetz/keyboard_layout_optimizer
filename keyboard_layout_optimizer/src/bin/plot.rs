use clap::Parser;

use keyboard_layout_optimizer::common;

#[derive(Parser, Debug)]
#[clap(name = "Keyboard layout plotting")]
struct Options {
    /// Layout keys from left to right, top to bottom
    layout_str: String,

    /// Do not remove whitespace from layout strings
    #[clap(long)]
    do_not_remove_whitespace: bool,

    /// Filename of layout configuration file to use
    #[clap(short, long, default_value = "config/keyboard/standard.yml")]
    layout_config: String,

    /// Interpred given layout string using the "grouped" logic
    #[clap(long)]
    pub grouped_layout_generator: bool,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = Options::parse();

    let layout_str: String = options
        .layout_str
        .chars()
        .filter(|c| options.do_not_remove_whitespace || !c.is_whitespace())
        .collect();
    let layout_generator =
        common::init_layout_generator(&options.layout_config, options.grouped_layout_generator);

    let layout = match layout_generator.generate(&layout_str) {
        Ok(layout) => layout,
        Err(e) => {
            log::error!("{:?}", e);
            panic!("{:?}", e);
        }
    };
    let max_layer = layout.layerkeys.iter().map(|k| k.layer).max().unwrap_or(0);
    for layer in 0..max_layer + 1 {
        println!(
            "Layout '{}' (layer {}):\n{}",
            layout_str,
            layer + 1,
            layout.plot_layer(layer as usize)
        );
    }
    println!("Layout compact: \n{}", layout.plot_compact());
    println!("Layout as text: \n{}", layout);
}
