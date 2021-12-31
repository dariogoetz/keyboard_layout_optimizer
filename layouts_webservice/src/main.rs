#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

use keyboard_layout::{
    keyboard::{Keyboard, KeyboardYAML},
    layout_generator::{BaseLayoutYAML, NeoLayoutGenerator},
};
use layout_evaluation::{
    evaluation::{Evaluator, MetricParameters},
    ngram_mapper::on_demand_ngram_mapper::{NgramMapperConfig, OnDemandNgramMapper},
    ngrams::{Bigrams, Trigrams, Unigrams},
};

use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;
use std::path::Path;

mod api;


#[derive(Clone, Deserialize, Debug)]
struct Options {
    /// Path to ngram files
    pub ngrams: String,

    /// Filename of evaluation configuration file to use
    pub eval_parameters: String,

    /// Filename of layout configuration file to use
    pub layout_config: String,

    /// Directory with static content to serve
    pub static_dir: String,

    /// Secret for performing admin actions
    pub secret: String,

    /// CORS allowed origins
    pub allowed_cors_origins: String,
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

use async_trait::async_trait;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

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

    async fn on_response<'r>(&self,
        _request: &'r Request<'_>,
        response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            self.options.allowed_cors_origins.to_owned(),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PATCH, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "*"
        ));
    }
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let options: Options = figment.extract().expect("config");

    let layout_config = LayoutConfig::from_yaml(&options.layout_config).expect(&format!(
        "Could not load config file '{}'",
        &options.layout_config
    ));
    let keyboard = Arc::new(Keyboard::from_yaml_object(layout_config.keyboard));
    let layout_generator = NeoLayoutGenerator::from_object(layout_config.base_layout, keyboard);
    let eval_params = EvaluationParameters::from_yaml(&options.eval_parameters).expect(&format!(
        "Could not read evaluation yaml file '{}'",
        &options.eval_parameters
    ));
    let p = Path::new(&options.ngrams).join(eval_params.ngrams.unigrams);
    let unigrams = Unigrams::from_file(&p.to_str().unwrap()).expect(&format!(
        "Could not read 1-gramme file from '{:?}'.",
        &p
    ));
    let p = Path::new(&options.ngrams).join(eval_params.ngrams.bigrams);
    let bigrams = Bigrams::from_file(&p.to_str().unwrap()).expect(&format!(
        "Could not read 2-gramme file from '{:?}'.",
        &p
    ));
    let p = Path::new(&options.ngrams).join(eval_params.ngrams.trigrams);
    let trigrams = Trigrams::from_file(&p.to_str().unwrap()).expect(&format!(
        "Could not read 3-gramme file from '{:?}'.",
        &p
    ));
    let ngram_mapper_config = eval_params.ngram_mapper.clone();
    let ngram_mapper = OnDemandNgramMapper::with_ngrams(unigrams, bigrams, trigrams, ngram_mapper_config);

    let evaluator =
        Evaluator::default(Box::new(ngram_mapper)).default_metrics(&eval_params.metrics);


    rocket
        .manage(evaluator)
        .manage(layout_generator)
        .attach(AdHoc::config::<Options>())
        .attach(api::stage())
        .attach(Cors { options: options.clone() })
        .mount("/", FileServer::from(&options.static_dir))
}
