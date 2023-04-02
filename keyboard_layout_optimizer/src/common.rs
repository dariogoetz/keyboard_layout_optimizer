use keyboard_layout::{
    config::LayoutConfig, grouped_layout_generator::GroupedLayoutGenerator, keyboard::Keyboard,
    layout::Layout, layout_generator::LayoutGenerator, neo_layout_generator::NeoLayoutGenerator,
};
use layout_evaluation::{
    config::EvaluationParameters,
    evaluation::Evaluator,
    ngram_mapper::on_demand_ngram_mapper::OnDemandNgramMapper,
    ngrams::{Bigrams, Trigrams, Unigrams},
};

use ahash::AHashMap;
use clap::Parser;
use itertools::Itertools;
use std::{
    fs::{self, OpenOptions},
    io::prelude::*,
    path::Path,
    sync::Arc,
};

#[derive(Parser, Debug)]
#[clap(name = "Keyboard layout evaluation")]
pub struct Options {
    /// Path to ngram files
    #[clap(
        short,
        long,
        default_value = "ngrams/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4"
    )]
    pub ngrams: String,

    /// Filename of evaluation configuration file to use
    #[clap(short, long, default_value = "config/evaluation/default.yml")]
    pub eval_parameters: String,

    /// Filename of layout configuration file to use
    #[clap(short, long, default_value = "config/keyboard/standard.yml")]
    pub layout_config: String,

    /// Filename of corpus file to use instead of ngram files
    #[clap(short, long)]
    pub corpus: Option<String>,

    /// Evaluate given text instead of corpus file or ngram files
    #[clap(short, long)]
    pub text: Option<String>,

    /// Only consider the top ngrams up to the given fraction
    #[clap(long)]
    pub tops: Option<f64>,

    /// Only consider ngrams that do not contain any of the given characters
    #[clap(long)]
    pub exclude_chars: Option<String>,

    /// Do not split modifiers
    #[clap(long)]
    pub no_split_modifiers: bool,

    /// Do not increase weight of common ngrams
    #[clap(long)]
    pub no_increase_common_ngrams: bool,

    /// Interpred given layout string using the "grouped" logic
    #[clap(long)]
    pub grouped_layout_generator: bool,
}

#[derive(Parser, Debug)]
#[clap(name = "Keyboard layout publication")]
pub struct PublishingOptions {
    /// Publish found layout to webservice under this name.
    /// This option is required if you want to publish your layout(s)!
    #[clap(long)]
    pub publish_as: Option<String>,

    /// Publish the layout only if its cost is lower (better) than this value
    #[clap(long, requires = "publish-as")]
    pub publish_if_cost_below: Option<f64>,

    /// Publish found layout to webservice for this layout config
    #[clap(long, default_value = "standard")]
    pub publish_layout_config: String,

    /// Publish found layout to webservice at this url
    #[clap(long, default_value = "https://keyboard-layout-optimizer.fly.dev/api")]
    pub publish_to: String,
}

pub fn init(options: &Options) -> (Box<dyn LayoutGenerator>, Evaluator) {
    (
        init_layout_generator(&options.layout_config, options.grouped_layout_generator),
        init_evaluator(options),
    )
}

pub fn init_layout_generator(
    layout_config: &str,
    grouped_layout_generator: bool,
) -> Box<dyn LayoutGenerator> {
    let layout_config = LayoutConfig::from_yaml(layout_config)
        .unwrap_or_else(|e| panic!("Could not load config file {}: {}", layout_config, e));

    let keyboard = Arc::new(Keyboard::from_yaml_object(layout_config.keyboard));
    log::info!("A-priori estimations from key_costs:");
    log::info!(
        "Finger loads (thumbs set to 0.00): {}",
        keyboard.estimated_finger_loads(true)
    );
    let mut messages = Vec::new();
    keyboard
        .estimated_row_loads()
        .iter()
        .sorted_by_key(|(row, _)| *row)
        .for_each(|(row, load)| {
            let msg = format!("Row {}: {:>.2}", row, load);
            messages.push(msg);
        });
    let message = messages.join(" ");
    log::info!("Row loads: {}", message);

    if grouped_layout_generator {
        Box::new(GroupedLayoutGenerator::from_object(
            layout_config.base_layout,
            keyboard,
        ))
    } else {
        Box::new(NeoLayoutGenerator::from_object(
            layout_config.base_layout,
            keyboard,
        ))
    }
}

pub fn init_evaluator(options: &Options) -> Evaluator {
    let eval_params =
        EvaluationParameters::from_yaml(&options.eval_parameters).unwrap_or_else(|e| {
            panic!(
                "Could not read evaluation yaml file {}: {:?}",
                options.eval_parameters, e
            )
        });

    let text = options.text.as_ref().cloned().or_else(|| {
        options.corpus.as_ref().map(|corpus_file| {
            fs::read_to_string(&corpus_file)
                .unwrap_or_else(|_| panic!("Could not read corpus file from {}.", corpus_file))
        })
    });

    let mut ngram_mapper_config = eval_params.ngram_mapper.clone();
    if options.no_split_modifiers {
        ngram_mapper_config.split_modifiers.enabled = false;
    }

    let mut ngrams_config = eval_params.ngrams.clone();
    if options.no_increase_common_ngrams {
        ngrams_config.increase_common_ngrams.enabled = false;
    }

    let (mut unigrams, mut bigrams, mut trigrams) = match text {
        Some(txt) => {
            let unigrams =
                Unigrams::from_text(&txt).expect("Could not generate unigrams from text.");
            let bigrams = Bigrams::from_text(&txt).expect("Could not generate bigrams from text.");
            let trigrams =
                Trigrams::from_text(&txt).expect("Could not generate trigrams from text.");

            (unigrams, bigrams, trigrams)
        }
        None => {
            let p = Path::new(&options.ngrams).join("1-grams.txt");
            log::info!("Reading unigram file: '{:?}'", p);
            let unigrams = Unigrams::from_file(p.to_str().unwrap())
                .unwrap_or_else(|_| panic!("Could not read 1-gramme file from '{:?}'.", &p));
            let p = Path::new(&options.ngrams).join("2-grams.txt");
            log::info!("Reading bigram file: '{:?}'", p);
            let bigrams = Bigrams::from_file(p.to_str().unwrap())
                .unwrap_or_else(|_| panic!("Could not read 2-gramme file from '{:?}'.", &p));
            let p = Path::new(&options.ngrams).join("3-grams.txt");
            log::info!("Reading trigram file: '{:?}'", p);
            let trigrams = Trigrams::from_file(p.to_str().unwrap())
                .unwrap_or_else(|_| panic!("Could not read 3-gramme file from '{:?}'.", &p));

            (unigrams, bigrams, trigrams)
        }
    };

    if let Some(exclude_chars) = &options.exclude_chars {
        for exclude_char in exclude_chars.chars() {
            unigrams = unigrams.exclude_char(&exclude_char);
            bigrams = bigrams.exclude_char(&exclude_char);
            trigrams = trigrams.exclude_char(&exclude_char);
        }
    }

    if ngrams_config.increase_common_ngrams.enabled {
        unigrams = unigrams.increase_common(&ngrams_config.increase_common_ngrams);
        bigrams = bigrams.increase_common(&ngrams_config.increase_common_ngrams);
        trigrams = trigrams.increase_common(&ngrams_config.increase_common_ngrams);
    }

    if let Some(tops) = options.tops {
        unigrams = unigrams.tops(tops);
        bigrams = bigrams.tops(tops);
        trigrams = trigrams.tops(tops);
    }

    let ngram_provider =
        OnDemandNgramMapper::with_ngrams(unigrams, bigrams, trigrams, ngram_mapper_config);

    Evaluator::default(Box::new(ngram_provider)).default_metrics(&eval_params.metrics)
}

/// Appends a layout-string to a file.
pub fn append_to_file(layout_str: &str, filename: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", layout_str) {
        log::error!("Couldn't write to file: {}", e);
    } else {
        log::info!("Appended layout '{}' to '{}'", layout_str, filename);
    }
}

/// Publishes the layout to a webservice.
pub fn publish_to_webservice(
    layout_str: &str,
    publish_name: &str,
    publish_to: &str,
    publish_layout_config: &str,
) {
    let client = reqwest::blocking::Client::new();
    let mut body = AHashMap::default();
    body.insert("published_by", publish_name.to_string());
    body.insert("layout", layout_str.to_string());
    body.insert("layout_config", publish_layout_config.to_string());

    let resp = client.post(publish_to).json(&body).send().ok();
    if let Some(resp) = resp {
        if resp.status().is_success() {
            log::info!("Published layout '{}' to {}", layout_str, publish_to);
        } else {
            log::error!("Could not publish result to webservice: {:?}", &resp.text());
        }
    } else {
        log::error!("Could not publish result to webservice");
    }
}
