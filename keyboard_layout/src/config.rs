use crate::keyboard::KeyboardYAML;
use crate::layout_generator::BaseLayoutYAML;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LayoutConfig {
    pub keyboard: KeyboardYAML,
    pub base_layout: BaseLayoutYAML,
}

impl LayoutConfig {
    pub fn from_str(layout_config_str: &str) -> Result<Self> {
        let cfg: LayoutConfig = serde_yaml::from_str(layout_config_str)?;

        Ok(cfg)
    }

    pub fn from_yaml(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let cfg: LayoutConfig = serde_yaml::from_reader(f)?;

        Ok(cfg)
    }
}
