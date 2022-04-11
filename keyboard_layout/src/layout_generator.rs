//! This module provides a layout generator that can generate Neo variant layouts
//! from given string representations of its base layer.

use crate::key::Hand;
use crate::keyboard::Keyboard;
use crate::layout::Layout;

use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Deserialize;
use std::{fs::File, iter::FromIterator, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("Invalid keyboard layout: Duplicate characters in layout '{0}': '{1}'")]
    DuplicateChars(String, String),
    #[error("Invalid keyboard layout: Missing characters: '{0}'")]
    MissingChars(String),
    #[error("Invalid keyboard layout: Unsupported characters: '{0}'")]
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
    fixed_layers: Vec<u8>,
    modifiers: Vec<FxHashMap<Hand, Vec<char>>>,
    layer_costs: Vec<f64>,
}

impl BaseLayoutYAML {
    /// Checks the [`KeyboardYAML`] for common errors.
    pub fn validate(&self) -> Result<(), String> {
        let flat_keys = self.keys.concat();
        let flat_fixed_keys = self.fixed_keys.concat();

        // Make sure that all settings that should have the same number of elements
        // do in fact have the same number of elements.
        if flat_keys.len() != flat_fixed_keys.len() {
            return Err("There are not the same number of `keys` as there are fixed/permutable entries `fixed_keys`.".to_string());
        }

        Ok(())
    }
}

/// Provides functionalities for generating Neo layout variants from given string representations
/// of their base layer.
#[derive(Clone, Debug)]
pub struct NeoLayoutGenerator {
    keys: Vec<Vec<char>>,
    fixed_keys: Vec<bool>,
    permutable_key_map: FxHashMap<char, u8>,
    fixed_layers: Vec<u8>,
    modifiers: Vec<FxHashMap<Hand, Vec<char>>>,
    layer_costs: Vec<f64>,
    keyboard: Arc<Keyboard>,
}

impl NeoLayoutGenerator {
    /// Generate a [`NeoLayoutGenerator`] from a [`BaseLayoutYAML`] object
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

        let mut permutable_key_map: FxHashMap<char, u8> = FxHashMap::default();
        keys.iter()
            .zip(fixed_keys.iter())
            .enumerate()
            .filter(|(_i, (_key_layers, fixed))| !*fixed)
            .for_each(|(i, (key_layers, _fixed))| {
                if !key_layers.is_empty() {
                    permutable_key_map.entry(key_layers[0]).or_insert(i as u8);
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

    /// Generate a [`NeoLayoutGenerator`] from a YAML file
    pub fn from_yaml_file(filename: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let f = File::open(filename)?;
        let base: BaseLayoutYAML = serde_yaml::from_reader(f)?;
        Ok(NeoLayoutGenerator::from_object(base, keyboard))
    }

    /// Generate a [`NeoLayoutGenerator`] from a YAML string
    pub fn from_yaml_str(data: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let base: BaseLayoutYAML = serde_yaml::from_str(data)?;
        Ok(NeoLayoutGenerator::from_object(base, keyboard))
    }

    /// Generate a Neo variant [`Layout`] from given string representation of its base layer.
    /// Does not check whether the given string is valid (sufficient, correct and unique charactors).
    /// This is useful for plotting unfinished or invalid layouts.
    pub fn generate_unchecked(&self, layout_keys: &str) -> Result<Layout> {
        let chars: Vec<char> = layout_keys.chars().filter(|c| !c.is_whitespace()).collect();

        // assemble a Vec<Vec<char>> representation of the layer for the given layout string
        let mut given_chars = chars.iter();

        let mut key_chars = Vec::with_capacity(self.fixed_keys.len());
        for (key_layers, fixed) in self.keys.iter().zip(self.fixed_keys.iter()) {
            if *fixed {
                key_chars.push(key_layers.clone());
            } else {
                let given_char = given_chars.next();
                if given_char.is_none() {
                    // number of given layout keys are insufficient
                    log::warn!("Number of given symbols in layout string is smaller than number of non-fixed keys");
                    break;
                }
                let given_char = given_char.unwrap();

                let key_idx = self
                    .permutable_key_map
                    .get(given_char)
                    .ok_or(format!(
                        "Unsupported symbol in given layout keys: '{}'",
                        given_char
                    ))
                    .map_err(anyhow::Error::msg)?;

                let given_key_layers = &self.keys[*key_idx as usize];
                let new_key_layers = given_key_layers
                    .iter()
                    .enumerate()
                    .map(|(layer_id, c)| {
                        if !self.fixed_layers.contains(&(layer_id as u8)) {
                            *c
                        } else {
                            *key_layers.get(layer_id).unwrap_or(&'␡')
                        }
                    })
                    .collect();
                key_chars.push(new_key_layers);
            }
        }

        Layout::new(
            key_chars,
            self.fixed_keys.clone(),
            self.keyboard.clone(),
            self.modifiers.clone(),
            self.layer_costs.clone(),
        )
    }

    /// Generate a Neo variant [`Layout`] from a given string representation of its base layer (only non-fixed keys)
    pub fn generate(&self, layout_keys: &str) -> Result<Layout> {
        let chars: Vec<char> = layout_keys.chars().filter(|c| !c.is_whitespace()).collect();

        let char_set: FxHashSet<char> = FxHashSet::from_iter(chars.clone());
        let layout_set: FxHashSet<char> =
            FxHashSet::from_iter(self.permutable_key_map.keys().cloned());

        // Check for duplicate chars
        if char_set.len() != chars.len() {
            let mut duplicates = FxHashSet::default();
            let mut seen_chars = FxHashSet::default();
            for char in chars.iter() {
                if seen_chars.contains(char) {
                    duplicates.insert(*char);
                } else {
                    seen_chars.insert(*char);
                }
            }
            return Err(LayoutError::DuplicateChars(
                layout_keys.to_string(),
                duplicates.iter().cloned().collect::<String>(),
            )
            .into());
        }

        let mut unsupported_chars: Vec<char> = char_set.difference(&layout_set).cloned().collect();
        let mut missing_chars: Vec<char> = layout_set.difference(&char_set).cloned().collect();

        unsupported_chars.sort_unstable();
        missing_chars.sort_unstable();

        if !unsupported_chars.is_empty() {
            return Err(LayoutError::UnsupportedChars(unsupported_chars.iter().collect()).into());
        }
        if !missing_chars.is_empty() {
            return Err(LayoutError::MissingChars(missing_chars.iter().collect()).into());
        }

        self.generate_unchecked(layout_keys)
    }

    /// Get the list of permutable symbols
    pub fn permutable_keys(&self) -> Vec<char> {
        self.permutable_key_map.keys().cloned().collect()
    }
}
