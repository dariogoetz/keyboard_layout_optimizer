use crate::{
    evaluation::MetricParameters, ngram_mapper::on_demand_ngram_mapper::NgramMapperConfig, ngrams::NgramsConfig,
};

use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, str::FromStr};

#[derive(Clone, Deserialize, Debug)]
pub struct EvaluationParameters {
    pub metrics: MetricParameters,
    pub ngrams: NgramsConfig,
    pub ngram_mapper: NgramMapperConfig,
}

impl EvaluationParameters {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = File::open(filename)?;
        let k: EvaluationParameters = serde_yaml::from_reader(f)?;

        Ok(k)
    }
}

impl FromStr for EvaluationParameters {
    type Err = serde_yaml::Error;
    fn from_str(evaluation_params_str: &str) -> Result<Self, Self::Err> {
        let cfg: EvaluationParameters = serde_yaml::from_str(evaluation_params_str)?;

        Ok(cfg)
    }
}
