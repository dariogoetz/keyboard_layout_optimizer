#[macro_use]
extern crate rocket;

use keyboard_layout::{
    config::LayoutConfig, keyboard::Keyboard, neo_layout_generator::NeoLayoutGenerator,
};
use layout_evaluation::{
    config::EvaluationParameters,
    evaluation::Evaluator,
    ngram_mapper::on_demand_ngram_mapper::OnDemandNgramMapper,
    ngrams::{Bigrams, Trigrams, Unigrams},
};

use ahash::AHashMap;
use rocket::{fairing::AdHoc, fs::FileServer};
use serde::Deserialize;
use std::{path::Path, sync::Arc};

mod api;

#[derive(Clone, Deserialize, Debug)]
struct Options {
    /// Path to ngram files
    pub ngrams: String,

    /// Filename of evaluation configuration file to use
    pub eval_parameters: String,

    /// Identifiers and filenames of layout configuration file to use
    pub layout_configs: Vec<(String, String)>,

    /// Default layout config to use if unspecified
    pub default_layout_config: String,

    /// Directory with static content to serve
    pub static_dir: String,

    /// Secret for performing admin actions
    pub secret: String,

    /// CORS allowed origins
    pub allowed_cors_origins: String,
}

use async_trait::async_trait;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

pub struct Cors {
    options: Options,
}

#[async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            self.options.allowed_cors_origins.to_owned(),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let options: Options = figment.extract().expect("config");

    let mut layout_generators: AHashMap<String, NeoLayoutGenerator> = AHashMap::default();
    for (config_id, layout_config) in &options.layout_configs {
        let layout_config = LayoutConfig::from_yaml(layout_config)
            .unwrap_or_else(|e| panic!("Could not load config file '{}': {}", &layout_config, e));

        let keyboard = Arc::new(Keyboard::from_yaml_object(layout_config.keyboard));
        let layout_generator = NeoLayoutGenerator::from_object(layout_config.base_layout, keyboard);
        layout_generators.insert(config_id.to_owned(), layout_generator);
    }

    let eval_params =
        EvaluationParameters::from_yaml(&options.eval_parameters).unwrap_or_else(|_| {
            panic!(
                "Could not read evaluation yaml file '{}'",
                &options.eval_parameters
            )
        });
    let p = Path::new(&options.ngrams).join("1-grams.txt");
    let unigrams = Unigrams::from_file(p.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Could not read 1-gramme file from '{:?}'.", &p));
    let p = Path::new(&options.ngrams).join("2-grams.txt");
    let bigrams = Bigrams::from_file(p.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Could not read 2-gramme file from '{:?}'.", &p));
    let p = Path::new(&options.ngrams).join("3-grams.txt");
    let trigrams = Trigrams::from_file(p.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Could not read 3-gramme file from '{:?}'.", &p));
    let ngram_mapper_config = eval_params.ngram_mapper.clone();
    let ngram_mapper =
        OnDemandNgramMapper::with_ngrams(unigrams, bigrams, trigrams, ngram_mapper_config);

    let evaluator =
        Evaluator::default(Box::new(ngram_mapper)).default_metrics(&eval_params.metrics);

    rocket
        .manage(evaluator)
        .manage(layout_generators)
        .attach(AdHoc::config::<Options>())
        .attach(api::stage())
        .attach(Cors {
            options: options.clone(),
        })
        .mount("/", FileServer::from(&options.static_dir))
}
