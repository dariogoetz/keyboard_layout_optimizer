use crate::key::{Finger, Hand, Key, Position};

use anyhow::Result;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Keyboard {
    pub keys: Vec<Key>,
    pub key_costs: Vec<f64>,
    pub unbalancing_positions: Vec<f64>,
    pub plot_template: String,
    pub plot_template_short: String,
}

#[derive(Deserialize, Debug)]
pub struct KeyboardYAML {
    positions: Vec<Vec<Position>>,
    hands: Vec<Vec<Hand>>,
    fingers: Vec<Vec<Finger>>,
    key_costs: Vec<Vec<f64>>,
    symmetries: Vec<Vec<usize>>,
    unbalancing_positions: Vec<Vec<f64>>,
    plot_template: String,
    plot_template_short: String,
}

impl Keyboard {
    pub fn from_yaml_object(k: KeyboardYAML) -> Self {
        let keys = k
            .hands
            .into_iter()
            .flatten()
            .zip(k.fingers.into_iter().flatten())
            .zip(k.positions.into_iter().flatten())
            .zip(k.symmetries.into_iter().flatten())
            .enumerate()
            .map(|(i, (((hand, finger), position), symmetry_key))| Key {
                index: i,
                hand,
                finger,
                position,
                symmetry_key,
            })
            .collect();

        Keyboard {
            keys,
            key_costs: k.key_costs.into_iter().flatten().collect(),
            unbalancing_positions: k.unbalancing_positions.into_iter().flatten().collect(),
            plot_template: k.plot_template,
            plot_template_short: k.plot_template_short,
        }
    }

    pub fn from_yaml_file(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let k: KeyboardYAML = serde_yaml::from_reader(f)?;
        Ok(Keyboard::from_yaml_object(k))
    }

    pub fn from_yaml_str(data: &str) -> Result<Self> {
        let k: KeyboardYAML = serde_yaml::from_str(data)?;
        Ok(Keyboard::from_yaml_object(k))
    }
}
