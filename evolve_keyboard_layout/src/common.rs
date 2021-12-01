use keyboard_layout::{
    keyboard::{Keyboard, KeyboardYAML},
    layout::Layout,
    layout_generator::{BaseLayoutYAML, NeoLayoutGenerator},
};
use layout_evaluation::{
    evaluation::{Evaluator, MetricParameters},
    ngram_mapper::on_demand_ngram_mapper::{NgramMapperConfig, OnDemandNgramMapper},
    ngrams::{Bigrams, Trigrams, Unigrams},
};

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::Arc;
use structopt::StructOpt;

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

#[derive(StructOpt, Debug)]
#[structopt(name = "Keyboard layout evaluation")]
pub struct Options {
    /// Filename of evaluation configuration file to use
    #[structopt(short, long, default_value = "config/evaluation_parameters.yml")]
    pub eval_parameters: String,

    /// Filename of layout configuration file to use
    #[structopt(short, long, default_value = "config/standard_keyboard.yml")]
    pub layout_config: String,

    /// Filename of corpus file to use instead of ngram files
    #[structopt(short, long)]
    pub corpus: Option<String>,

    /// Evaluate given text instead of corpus file or ngram files
    #[structopt(short, long)]
    pub text: Option<String>,

    /// Only consider the top ngrams up to the given fraction
    #[structopt(long)]
    pub tops: Option<f64>,

    /// Do not split modifiers
    #[structopt(long)]
    pub no_split_modifiers: bool,

    /// Do not add secondary bigrams from trigrams
    #[structopt(long)]
    pub no_add_secondary_bigrams: bool,

    /// Do not increase weight of common bigrams
    #[structopt(long)]
    pub no_increase_common_bigrams: bool,
}

pub fn init(options: &Options) -> (NeoLayoutGenerator, Evaluator) {
    (
        init_layout_generator(&options.layout_config),
        init_evaluator(options),
    )
}

pub fn init_layout_generator(layout_config: &str) -> NeoLayoutGenerator {
    let layout_config = LayoutConfig::from_yaml(layout_config)
        .expect(&format!("Could not load config file {}", layout_config));

    let keyboard = Arc::new(Keyboard::from_yaml_object(layout_config.keyboard));

    NeoLayoutGenerator::from_object(layout_config.base_layout, keyboard)
}

pub fn init_evaluator(options: &Options) -> Evaluator {
    let eval_params = EvaluationParameters::from_yaml(&options.eval_parameters).expect(&format!(
        "Could not read evaluation yaml file {}",
        options.eval_parameters
    ));

    let text = options.text.as_ref().cloned().or_else(|| {
        options.corpus.as_ref().map(|corpus_file| {
            std::fs::read_to_string(&corpus_file)
                .expect(&format!("Could not read corpus file from {}.", corpus_file,))
        })
    });

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

    Evaluator::default(Box::new(ngram_provider)).default_metrics(&eval_params.metrics)
}

/// Appends a layout-string to a file.
pub fn append_to_file(layout: &Layout, filename: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", layout.as_text()) {
        log::error!("Couldn't write to file: {}", e);
    } else {
        log::info!("Appended layout '{}' to '{}'", layout.as_text(), filename);
    }
}

/// Publishes the layout to a webservice.
pub fn publish_to_webservice(layout: &Layout, publish_name: &str, publish_to: &str) {
    let client = reqwest::blocking::Client::new();
    let mut body = HashMap::new();
    body.insert("published_by", publish_name.to_string());
    body.insert("layout", layout.as_text());
    let resp = client.post(publish_to).json(&body).send().ok();
    if let Some(resp) = resp {
        if resp.status().is_success() {
            log::info!("Published layout '{}' to {}", layout.as_text(), publish_to);
        } else {
            log::error!("Could not publish result to webservice: {:?}", &resp.text());
        }
    } else {
        log::error!("Could not publish result to webservice");
    }
}
