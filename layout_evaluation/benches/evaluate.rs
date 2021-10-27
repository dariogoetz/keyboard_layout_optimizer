use keyboard_layout::{
    keyboard::{Keyboard, KeyboardYAML},
    layout_generator::{BaseLayoutYAML, NeoLayoutGenerator},
};
use layout_evaluation::{
    evaluation::{Evaluator, MetricParameters},
    ngrams::{
        ngrams::{Bigrams, Trigrams, Unigrams},
        on_demand_ngram_mapper::OnDemandNgramMapper,
    },
};

use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Deserialize, Debug)]
pub struct NGramConfig {
    pub unigrams: String,
    pub bigrams: String,
    pub trigrams: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct EvaluationParameters {
    pub corpus: NGramConfig,
    pub metrics: MetricParameters,
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

pub fn evaluate_bench(c: &mut Criterion) {
    let layout_config = LayoutConfig::from_yaml(&"../standard_keyboard.yml").expect(&format!(
        "Could not load config file 'standard_keyboard.yml'",
    ));

    let keyboard = Arc::new(Keyboard::from_yaml_object(layout_config.keyboard));

    let layout_generator = NeoLayoutGenerator::from_object(layout_config.base_layout, keyboard);

    let eval_params = EvaluationParameters::from_yaml(&"../evaluation_parameters.yml").expect(
        &format!("Could not read evaluation yaml file 'evaluation_parameters.yml'"),
    );

    log::info!("Reading unigram file: '{}'", &eval_params.corpus.unigrams);
    let unigrams =
        Unigrams::from_file(&("../".to_string() + &eval_params.corpus.unigrams)).expect(&format!(
            "Could not read 1-gramme file from '{}'.",
            &eval_params.corpus.unigrams
        ));
    log::info!("Reading bigram file: '{}'", &eval_params.corpus.bigrams);
    let bigrams =
        Bigrams::from_file(&("../".to_string() + &eval_params.corpus.bigrams)).expect(&format!(
            "Could not read 2-gramme file from '{}'.",
            &eval_params.corpus.bigrams
        ));
    log::info!("Reading trigram file: '{}'", &eval_params.corpus.trigrams);
    let trigrams =
        Trigrams::from_file(&("../".to_string() + &eval_params.corpus.trigrams)).expect(&format!(
            "Could not read 3-gramme file from '{}'.",
            &eval_params.corpus.trigrams
        ));

    let ngram_provider = OnDemandNgramMapper::with_ngrams(&unigrams, &bigrams, &trigrams, true);

    let evaluator =
        Evaluator::default(Box::new(ngram_provider)).default_metrics(&eval_params.metrics);

    let layout = match layout_generator.generate("jduaxphlmwqßctieobnrsgfvüäöyz,.k") {
        Ok(layout) => layout,
        Err(e) => {
            log::error!("Error in generating layout: {:?}", e);
            panic!("{:?}", e);
        }
    };
    c.bench_function("evaluate", |b| {
        b.iter(|| evaluator.evaluate_layout(&layout));
    });
}

criterion_group!(benches, evaluate_bench);
criterion_main!(benches);
