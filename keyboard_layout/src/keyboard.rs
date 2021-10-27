//! This module provides a struct representing a keyboard.

use crate::key::{Finger, Hand, Key, MatrixPosition, Position};

use anyhow::Result;
use serde::Deserialize;
use string_template::Template;

/// The index of a `Key` in the `keys` vec of a `Keyboard
pub type KeyIndex = u16;

/// A struct representing a keyboard as a list of keys
#[derive(Clone, Debug)]
pub struct Keyboard {
    /// The keys of the keyboard
    pub keys: Vec<Key>,
    plot_template: String,
    plot_template_short: String,
}

/// A collection of all relevant properties for the keys on a keyboard.
/// Corresponds to a YAML configuration file.
#[derive(Deserialize, Debug)]
pub struct KeyboardYAML {
    matrix_positions: Vec<Vec<MatrixPosition>>,
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
    /// Generate a Keyboard from a `KeyboardYAML` object
    pub fn from_yaml_object(k: KeyboardYAML) -> Self {
        let keys = k
            .hands
            .into_iter()
            .flatten()
            .zip(k.fingers.into_iter().flatten())
            .zip(k.matrix_positions.into_iter().flatten())
            .zip(k.positions.into_iter().flatten())
            .zip(k.symmetries.into_iter().flatten())
            .zip(k.key_costs.into_iter().flatten())
            .zip(k.unbalancing_positions.into_iter().flatten())
            .map(
                |(
                    (((((hand, finger), matrix_position), position), symmetry_index), cost),
                    unbalancing,
                )| Key {
                    hand,
                    finger,
                    matrix_position,
                    position,
                    symmetry_index,
                    cost,
                    unbalancing,
                },
            )
            .collect();

        Keyboard {
            keys,
            plot_template: k.plot_template,
            plot_template_short: k.plot_template_short,
        }
    }

    /// Generate a Keyboard from a YAML file
    pub fn from_yaml_file(filename: &str) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let k: KeyboardYAML = serde_yaml::from_reader(f)?;
        Ok(Keyboard::from_yaml_object(k))
    }

    /// Generate a Keyboard from a YAML string
    pub fn from_yaml_str(data: &str) -> Result<Self> {
        let k: KeyboardYAML = serde_yaml::from_str(data)?;
        Ok(Keyboard::from_yaml_object(k))
    }

    /// Plot a graphical representation of the keyboard with given key labels
    pub fn plot(&self, key_labels: &[&str]) -> String {
        let template = Template::new(&self.plot_template);
        template.render_positional(key_labels)
    }

    /// Plot a compact graphical representation of the keyboard with given key labels without borders (compatible with ArneBab's input strings)
    pub fn plot_compact(&self, key_labels: &[&str]) -> String {
        let template = Template::new(&self.plot_template_short);
        template.render_positional(key_labels)
    }
}
