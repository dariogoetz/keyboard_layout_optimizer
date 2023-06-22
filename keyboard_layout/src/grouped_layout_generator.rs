use crate::key::Hand;
use crate::keyboard::Keyboard;
use crate::layout::{LayerModifierLocations, Layout};
use crate::layout_generator::LayoutGenerator;
use crate::neo_layout_generator::BaseLayoutYAML;

use ahash::{AHashMap, AHashSet};
use anyhow::Result;
use std::{fs::File, iter::FromIterator, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("Invalid keyboard layout: Unsupported characters in provided layout (not in a level of `base_layout` corresponding to a multiple of `grouped_layers` and `fixed_keys` with value `false`): '{0}'")]
    UnsupportedChars(String),
    #[error("Invalid keyboard layout: Number of characters in provided layout needs to be a multiple of the number of non-fixed keys. Provided {0} keys, {1} keys are non-fixed.")]
    WrongKeyNumber(usize, usize),
}

/// Provides functionalities for generating Neo layout variants from given string representations
/// of their base layer.
#[derive(Clone, Debug)]
pub struct GroupedLayoutGenerator {
    base_layout_symbols: Vec<Vec<char>>,
    fixed_keys: Vec<bool>,
    permutable_key_map: AHashMap<char, (u8, u8)>,
    grouped_layers: u8,
    modifiers: Vec<AHashMap<Hand, LayerModifierLocations>>,
    keyboard: Arc<Keyboard>,
}

impl GroupedLayoutGenerator {
    /// Generate a [`GroupedLayoutGenerator`] from a [`BaseLayoutYAML`] object
    pub fn from_object(base: BaseLayoutYAML, keyboard: Arc<Keyboard>) -> Self {
        let base_layout_symbols: Vec<Vec<char>> = base
            .keys
            .iter()
            .flatten()
            .map(|layers| layers.iter().filter_map(|l| l.chars().next()).collect())
            .collect();
        let fixed_keys: Vec<bool> = base.fixed_keys.iter().flatten().cloned().collect();

        let mut permutable_key_map: AHashMap<char, (u8, u8)> = AHashMap::default();
        base_layout_symbols
            .iter()
            .zip(fixed_keys.iter())
            .enumerate()
            .filter(|(_key_idx, (_key_layers, fixed))| !*fixed)
            .for_each(|(key_idx, (key_layers, _fixed))| {
                if !key_layers.is_empty() {
                    key_layers
                        .iter()
                        .enumerate()
                        .step_by(base.grouped_layers as usize)
                        .for_each(|(layer, c)| {
                            permutable_key_map
                                .entry(*c)
                                .or_insert((key_idx as u8, layer as u8));
                        });
                }
            });

        GroupedLayoutGenerator {
            base_layout_symbols,
            fixed_keys,
            permutable_key_map,
            grouped_layers: base.grouped_layers,
            modifiers: base.modifiers,
            keyboard,
        }
    }

    /// Generate a [`GroupedLayoutGenerator`] from a YAML file
    pub fn from_yaml_file(filename: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let f = File::open(filename)?;
        let base: BaseLayoutYAML = serde_yaml::from_reader(f)?;
        Ok(GroupedLayoutGenerator::from_object(base, keyboard))
    }

    /// Generate a [`GroupedLayoutGenerator`] from a YAML string
    pub fn from_yaml_str(data: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let base: BaseLayoutYAML = serde_yaml::from_str(data)?;
        Ok(GroupedLayoutGenerator::from_object(base, keyboard))
    }

    /// Generate a [`Layout`] from given string representation of individual layers.
    /// Does not check whether the given string is valid (sufficient, correct and unique characters).
    /// This is useful for plotting unfinished or invalid layouts.
    pub fn generate_unchecked(&self, layout_keys: &str) -> Result<Layout> {
        let chars: Vec<char> = layout_keys.chars().collect();

        // assemble a Vec<Vec<char>> representation of the layer for the given layout string
        let n_fixed = self.fixed_keys.iter().filter(|fixed| !**fixed).count();
        let n_iter = chars.len() / n_fixed;

        let mut given_chars = chars.iter();
        let mut key_chars = Vec::with_capacity(self.fixed_keys.len());
        for iter in 0..n_iter {
            for (key_idx, (key_layers, fixed)) in self
                .base_layout_symbols
                .iter()
                .zip(self.fixed_keys.iter())
                .enumerate()
            {
                if *fixed {
                    if iter == 0 {
                        key_chars.push(key_layers.clone());
                    }
                } else {
                    let given_char = given_chars.next();
                    if given_char.is_none() {
                        // number of given layout keys are insufficient
                        log::warn!("Number of given symbols in layout ({}) string is smaller than a multiple of the number of non-fixed keys ({})",
                        chars.len(), n_fixed);
                        break;
                    }
                    let given_char = given_char.unwrap();

                    let (base_key_idx, base_key_layer) = self
                        .permutable_key_map
                        .get(given_char)
                        .ok_or_else(|| LayoutError::UnsupportedChars(given_char.to_string()))?;
                    let given_key_layers = &self.base_layout_symbols[*base_key_idx as usize];

                    // take a group of <self.grouped_layer> chars from the base layout
                    let new_key_layers: Vec<char> = given_key_layers
                        .iter()
                        .skip(*base_key_layer as usize)
                        .take(self.grouped_layers as usize)
                        // make sure that the result has "self.grouped_layers" items
                        // if insufficient symbols are in base_layout by cycling
                        .cycle()
                        .take(self.grouped_layers as usize)
                        .cloned()
                        .collect();

                    if iter == 0 {
                        // add new key
                        key_chars.push(new_key_layers);
                    } else {
                        // append to existing key
                        key_chars[key_idx].extend(new_key_layers);
                    }
                }
            }
        }

        Layout::new(
            key_chars,
            self.fixed_keys.clone(),
            self.keyboard.clone(),
            self.modifiers.clone(),
        )
    }
}

impl LayoutGenerator for GroupedLayoutGenerator {
    /// Generate a Neo variant [`Layout`] from a given string representation of its base layer (only non-fixed keys)
    fn generate(&self, layout_keys: &str) -> Result<Layout> {
        let chars: Vec<char> = layout_keys.chars().collect();

        let n_fixed = self.fixed_keys.iter().filter(|fixed| !**fixed).count();
        if chars.len() % n_fixed != 0 {
            return Err(LayoutError::WrongKeyNumber(chars.len(), n_fixed).into());
        }

        let char_set: AHashSet<char> = AHashSet::from_iter(chars);
        let layout_set: AHashSet<char> =
            AHashSet::from_iter(self.permutable_key_map.keys().cloned());

        let mut unsupported_chars: Vec<char> = char_set.difference(&layout_set).cloned().collect();
        // let mut missing_chars: Vec<char> = layout_set.difference(&char_set).cloned().collect();

        unsupported_chars.sort_unstable();
        // missing_chars.sort_unstable();

        if !unsupported_chars.is_empty() {
            return Err(LayoutError::UnsupportedChars(unsupported_chars.iter().collect()).into());
        }
        // if !missing_chars.is_empty() {
        //     return Err(LayoutError::MissingChars(missing_chars.iter().collect()).into());
        // }

        self.generate_unchecked(layout_keys)
    }
}
