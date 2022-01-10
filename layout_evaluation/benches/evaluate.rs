use keyboard_layout::{
    config::LayoutConfig, keyboard::Keyboard, layout_generator::NeoLayoutGenerator,
};
use layout_evaluation::{
    config::EvaluationParameters,
    evaluation::Evaluator,
    ngram_mapper::on_demand_ngram_mapper::OnDemandNgramMapper,
    ngrams::{Bigrams, Trigrams, Unigrams},
};

use criterion::{criterion_group, criterion_main, Criterion};
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;

const NGRAMS: &str = "../corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4";
const LAYOUT_CONFIG: &str = "../config/standard_keyboard.yml";
const EVALUATION_PARAMETERS: &str = "../config/evaluation_parameters.yml";

#[derive(Clone, Deserialize, Debug)]
pub struct NGramConfig {
    pub unigrams: String,
    pub bigrams: String,
    pub trigrams: String,
}

pub fn evaluate_bench(c: &mut Criterion) {
    let layout_config = LayoutConfig::from_yaml(LAYOUT_CONFIG).expect(&format!(
        "Could not load config file 'standard_keyboard.yml'",
    ));

    let keyboard = Arc::new(Keyboard::from_yaml_object(layout_config.keyboard));

    let layout_generator = NeoLayoutGenerator::from_object(layout_config.base_layout, keyboard);

    let eval_params = EvaluationParameters::from_yaml(EVALUATION_PARAMETERS).expect(&format!(
        "Could not read evaluation yaml file 'evaluation_parameters.yml'"
    ));

    let p = Path::new(NGRAMS).join("1-grams.txt");
    log::info!("Reading unigram file: '{:?}'", p);
    let unigrams = Unigrams::from_file(&p.to_str().unwrap())
        .expect(&format!("Could not read 1-gramme file from '{:?}'.", p));

    let p = Path::new(NGRAMS).join("2-grams.txt");
    log::info!("Reading bigram file: '{:?}'", p);
    let bigrams = Bigrams::from_file(&p.to_str().unwrap())
        .expect(&format!("Could not read 2-gramme file from '{:?}'.", p));

    let p = Path::new(NGRAMS).join("3-grams.txt");
    log::info!("Reading trigram file: '{:?}'", p);
    let trigrams = Trigrams::from_file(&p.to_str().unwrap())
        .expect(&format!("Could not read 3-gramme file from '{:?}'.", p));

    let ngram_mapper_config = eval_params.ngram_mapper.clone();

    let ngram_provider =
        OnDemandNgramMapper::with_ngrams(unigrams, bigrams, trigrams, ngram_mapper_config);

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
