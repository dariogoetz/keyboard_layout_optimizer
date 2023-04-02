use crate::keyboard::KeyboardYAML;
use crate::neo_layout_generator::BaseLayoutYAML;

use anyhow::Result;
use serde::Deserialize;
use std::error::Error;
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
        cfg.validate()?;

        Ok(cfg)
    }

    pub fn validate(&self) -> Result<()> {
        self.keyboard.validate()?;
        self.base_layout.validate()?;
        Ok(())
    }
}

impl FromStr for LayoutConfig {
    type Err = Box<dyn Error>;
    fn from_str(layout_config_str: &str) -> Result<Self, Self::Err> {
        let cfg: LayoutConfig = serde_yaml::from_str(layout_config_str)?;
        cfg.validate()?;

        Ok(cfg)
    }
}
