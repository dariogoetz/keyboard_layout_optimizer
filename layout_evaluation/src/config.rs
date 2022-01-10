use crate::{
    evaluation::MetricParameters, ngram_mapper::on_demand_ngram_mapper::NgramMapperConfig,
};

use anyhow::Result;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct EvaluationParameters {
    pub metrics: MetricParameters,
    pub ngram_mapper: NgramMapperConfig,
}

impl EvaluationParameters {
    pub fn from_str(evaluation_params_str: &str) -> Result<Self> {
        let cfg: EvaluationParameters = serde_yaml::from_str(evaluation_params_str)?;

        Ok(cfg)
    }

    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let k: EvaluationParameters = serde_yaml::from_reader(f)?;

        Ok(k)
    }
}
