use crate::keyboard::KeyboardYAML;
use crate::layout_generator::BaseLayoutYAML;

use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, str::FromStr};

#[derive(Deserialize, Debug)]
pub struct LayoutConfig {
    pub keyboard: KeyboardYAML,
    pub base_layout: BaseLayoutYAML,
}

impl LayoutConfig {
    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = File::open(filename)?;
        let cfg: LayoutConfig = serde_yaml::from_reader(f)?;

        Ok(cfg)
    }
}

impl FromStr for LayoutConfig {
    type Err = serde_yaml::Error;
    fn from_str(layout_config_str: &str) -> Result<Self, Self::Err> {
        let cfg: LayoutConfig = serde_yaml::from_str(layout_config_str)?;

        Ok(cfg)
    }
}
