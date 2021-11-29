//! This module provides a layout generator that can generate Neo variant layouts
//! from given string representations of its base layer.

use crate::key::Hand;
use crate::keyboard::Keyboard;
use crate::layout::Layout;

use anyhow::Result;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("Invalid keyboard layout: Duplicate characters in layout {0}")]
    DuplicateChars(String),
    #[error("Invalid keyboard layout: Missing characters: {0}")]
    MissingChars(String),
    #[error("Invalid keyboard layout: Unsupported characters: {0}")]
    UnsupportedChars(String),
}

/// A collection of data (configuration) regarding the Neo layout (and its family)
/// required to generate Neo layout variants.
///
/// Corresponds to (parts of) a YAML configuration file.
#[derive(Deserialize, Debug)]
pub struct BaseLayoutYAML {
    keys: Vec<Vec<Vec<String>>>,
    fixed_keys: Vec<Vec<bool>>,
    fixed_layers: Vec<usize>,
    modifiers: Vec<FxHashMap<Hand, Vec<char>>>,
    layer_costs: Vec<f64>,
}

/// Provides functionalities for generating Neo layout variants from given string representations
/// of their base layer.
#[derive(Clone, Debug)]
pub struct NeoLayoutGenerator {
    keys: Vec<Vec<char>>,
    fixed_keys: Vec<bool>,
    permutable_key_map: FxHashMap<char, usize>,
    fixed_layers: Vec<usize>,
    modifiers: Vec<FxHashMap<Hand, Vec<char>>>,
    layer_costs: Vec<f64>,
    keyboard: Arc<Keyboard>,
}

impl NeoLayoutGenerator {
    /// Generate a `NeoLayoutGenerator` from a `BaseLayoutYAML` object
    pub fn from_object(base: BaseLayoutYAML, keyboard: Arc<Keyboard>) -> Self {
        let keys: Vec<Vec<char>> = base
            .keys
            .iter()
            .flatten()
            .map(|layers| {
                layers
                    .iter()
                    .map(|l| l.chars().next().unwrap_or('␡'))
                    .collect()
            })
            .collect();
        let fixed_keys: Vec<bool> = base.fixed_keys.iter().flatten().cloned().collect();

        let mut permutable_key_map: FxHashMap<char, usize> = FxHashMap::default();
        keys.iter()
            .zip(fixed_keys.iter())
            .enumerate()
            .filter(|(_i, (_key_layers, fixed))| !*fixed)
            .for_each(|(i, (key_layers, _fixed))| {
                if !key_layers.is_empty() {
                    permutable_key_map.entry(key_layers[0]).or_insert(i);
                }
            });

        NeoLayoutGenerator {
            keys,
            fixed_keys,
            permutable_key_map,
            fixed_layers: base.fixed_layers,
            modifiers: base.modifiers,
            layer_costs: base.layer_costs,
            keyboard,
        }
    }

    /// Generate a `NeoLayoutGenerator` from a YAML file
    pub fn from_yaml_file(filename: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let base: BaseLayoutYAML = serde_yaml::from_reader(f)?;
        Ok(NeoLayoutGenerator::from_object(base, keyboard))
    }

    /// Generate a `NeoLayoutGenerator` from a YAML string
    pub fn from_yaml_str(data: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let base: BaseLayoutYAML = serde_yaml::from_str(data)?;
        Ok(NeoLayoutGenerator::from_object(base, keyboard))
    }

    /// Generate a Neo variant `Layout` from a given string representation of its base layer (only non-fixed keys)
    pub fn generate(&self, layout_keys: &str) -> Result<Layout> {
        let chars: Vec<char> = layout_keys.chars().filter(|c| !c.is_whitespace()).collect();

        let char_set: HashSet<char> = HashSet::from_iter(chars.clone());
        let layout_set: HashSet<char> = HashSet::from_iter(self.permutable_key_map.keys().cloned());

        // Check for duplicate chars
        if char_set.len() != chars.len() {
            return Err(LayoutError::DuplicateChars(layout_keys.to_string()).into());
        }

        let mut unsupported_chars: Vec<char> = char_set.difference(&layout_set).cloned().collect();
        let mut missing_chars: Vec<char> = layout_set.difference(&char_set).cloned().collect();

        unsupported_chars.sort();
        missing_chars.sort();

        if !unsupported_chars.is_empty() {
            return Err(LayoutError::UnsupportedChars(unsupported_chars.iter().collect()).into());
        }
        if !missing_chars.is_empty() {
            return Err(LayoutError::MissingChars(missing_chars.iter().collect()).into());
        }

        // assemble a Vec<Vec<char>> representation of the layer for the given layout string
        let mut given_chars = chars.iter();

        let mut key_chars = Vec::new();
        self.keys
            .iter()
            .zip(self.fixed_keys.iter())
            .for_each(|(key_layers, fixed)| {
                if *fixed {
                    key_chars.push(key_layers.clone());
                } else {
                    let mut new_key_layers = Vec::new();
                    let given_char = given_chars.next().unwrap();
                    let given_key_layers =
                        &self.keys[*self.permutable_key_map.get(given_char).unwrap()];
                    given_key_layers
                        .iter()
                        .enumerate()
                        .for_each(|(layer_id, c)| {
                            if !self.fixed_layers.contains(&layer_id) {
                                new_key_layers.push(*c);
                            } else {
                                new_key_layers.push(*key_layers.get(layer_id).unwrap_or(&'␡'));
                            }
                        });
                    key_chars.push(new_key_layers);
                }
            });

        Ok(Layout::new(
            key_chars,
            self.fixed_keys.clone(),
            self.keyboard.clone(),
            self.modifiers.clone(),
            self.layer_costs.clone(),
        ))
    }
}
