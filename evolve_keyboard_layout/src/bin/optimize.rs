use std::sync::Arc;
use structopt::StructOpt;

use keyboard_layout::{
    keyboard::{Keyboard, KeyboardYAML},
    layout_generator::{BaseLayoutYAML, NeoLayoutGenerator},
};
use layout_evaluation::{
    evaluation::{Evaluator, MetricParameters},
    ngram_mapper::on_demand_ngram_mapper::{NgramMapperConfig, OnDemandNgramMapper},
    ngrams::{Bigrams, Trigrams, Unigrams},
};
use layout_optimization::optimization;

use anyhow::Result;
use serde::Deserialize;

#[derive(StructOpt, Debug)]
#[structopt(name = "Keyboard layout ptimization")]
struct Options {
    /// Filename of evaluation configuration file to use
    #[structopt(short, long, default_value = "evaluation_parameters.yml")]
    eval_parameters: String,

    /// Filename of layout configuration file to use
    #[structopt(short, long, default_value = "standard_keyboard.yml")]
    layout_config: String,

    /// Filename of corpus file to use
    #[structopt(short, long)]
    corpus: Option<String>,

    /// Evaluate given text instead of corpus file
    #[structopt(short, long)]
    text: Option<String>,

    /// Only consider the top ngrams up to the given fraction
    #[structopt(long)]
    tops: Option<f64>,

    /// Do not optimize those keys (wrt. starting layout or Neo2 layout otherwise)
    #[structopt(short, long)]
    fix: Option<String>,

    /// Filename of optimization configuration file
    #[structopt(short, long, default_value = "optimization_parameters.yml")]
    optimization_parameters: String,

    /// Start optimization from this layout (keys from left to right, top to bottom)
    #[structopt(short, long)]
    start_layout_str: Option<String>,

    /// Do not split modifiers
    #[structopt(long)]
    no_split_modifiers: bool,

    /// Do not add secondary bigrams from trigrams
    #[structopt(long)]
    no_add_secondary_bigrams: bool,

    /// Do not increase weight of common bigrams
    #[structopt(long)]
    no_increase_common_bigrams: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NGramConfig {
    pub unigrams: String,
    pub bigrams: String,
    pub trigrams: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct EvaluationParameters {
    pub metrics: MetricParameters,
    pub ngrams: NGramConfig,
    pub ngram_mapper: NgramMapperConfig,
}

impl EvaluationParameters {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let k: EvaluationParameters = serde_yaml::from_reader(f)?;
        Ok(k)
    }
}

#[derive(Deserialize, Debug)]
pub struct LayoutConfig {
    pub keyboard: KeyboardYAML,
    pub base_layout: BaseLayoutYAML,
}

impl LayoutConfig {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let cfg: LayoutConfig = serde_yaml::from_reader(f)?;

        Ok(cfg)
    }
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let options = Options::from_args();

    let layout_config = LayoutConfig::from_yaml(&options.layout_config).expect(&format!(
        "Could not load config file {}",
        &options.layout_config
    ));

    let keyboard = Arc::new(Keyboard::from_yaml_object(layout_config.keyboard));

    let layout_generator =
        NeoLayoutGenerator::from_object(layout_config.base_layout, keyboard);

    let eval_params = EvaluationParameters::from_yaml(&options.eval_parameters).expect(&format!(
        "Could not read evaluation yaml file {}",
        options.eval_parameters
    ));

    let text = match options.text {
        Some(txt) => Some(txt),
        None => options.corpus.map(|corpus_file| {
            std::fs::read_to_string(&corpus_file)
                .expect(&format!("Could not read corpus file from {}.", corpus_file,))
        }),
    };

    let mut ngram_mapper_config = eval_params.ngram_mapper.clone();
    if options.no_split_modifiers {
        ngram_mapper_config.split_modifiers.enabled = false;
    }
    if options.no_add_secondary_bigrams {
        ngram_mapper_config.secondary_bigrams_from_trigrams.enabled = false;
    }
    if options.no_increase_common_bigrams {
        ngram_mapper_config.increase_common_bigrams.enabled = false;
    }

    let ngram_provider = match text {
        Some(txt) => OnDemandNgramMapper::with_corpus(&txt, ngram_mapper_config),
        None => {
            log::info!("Reading unigram file: '{}'", &eval_params.ngrams.unigrams);
            let mut unigrams = Unigrams::from_file(&eval_params.ngrams.unigrams).expect(&format!(
                "Could not read 1-gramme file from '{}'.",
                &eval_params.ngrams.unigrams
            ));
            log::info!("Reading bigram file: '{}'", &eval_params.ngrams.bigrams);
            let mut bigrams = Bigrams::from_file(&eval_params.ngrams.bigrams).expect(&format!(
                "Could not read 2-gramme file from '{}'.",
                &eval_params.ngrams.bigrams
            ));
            log::info!("Reading trigram file: '{}'", &eval_params.ngrams.trigrams);
            let mut trigrams = Trigrams::from_file(&eval_params.ngrams.trigrams).expect(&format!(
                "Could not read 3-gramme file from '{}'.",
                &eval_params.ngrams.trigrams
            ));

            if let Some(tops) = options.tops {
                unigrams = unigrams.tops(tops);
                bigrams = bigrams.tops(tops);
                trigrams = trigrams.tops(tops);
            }

            OnDemandNgramMapper::with_ngrams(&unigrams, &bigrams, &trigrams, ngram_mapper_config)
        }
    };

    let evaluator =
        Evaluator::default(Box::new(ngram_provider)).default_metrics(&eval_params.metrics);

    let optimization_params = optimization::Parameters::from_yaml(&options.optimization_parameters)
        .expect(&format!(
            "Could not read optimization parameters from {}.",
            &options.optimization_parameters,
        ));
    let layout = optimization::optimize(
        &optimization_params,
        &evaluator,
        options
            .start_layout_str
            .as_ref()
            .unwrap_or(&"xvlcwkhgfqßuiaeosnrtdyüöäpzbm,.j".to_string()),
        &layout_generator,
        &options.fix.unwrap_or_else(|| "".to_string()),
        options.start_layout_str.is_some(),
    );

    let metric_costs = evaluator.evaluate_layout(&layout);
    let mut cost = 0.0;
    for mc in metric_costs.iter().filter(|mc| !mc.metric_costs.is_empty()) {
        cost += mc.total_cost();
        mc.print();
    }

    println!(
        "Cost: {:.4} (optmization score: {})",
        cost,
        (1e8 / cost) as usize
    );
}
