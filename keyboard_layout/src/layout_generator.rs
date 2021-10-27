use crate::key::Hand;
use crate::keyboard::Keyboard;
use crate::layout::{KeyIndex, LayerKey, LayerKeyIndex, Layout};

use anyhow::Result;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("Invalid keyboard layout: {0}")]
    InvalidLayout(String),
}

#[derive(Deserialize, Debug)]
pub struct BaseLayoutYAML {
    keys: Vec<Vec<Vec<String>>>,
    fixed_keys: Vec<Vec<bool>>,
    fixed_layers: Vec<usize>,
    modifiers: Vec<FxHashMap<Hand, Vec<char>>>,
    layer_costs: Vec<f64>,
}

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

    pub fn from_yaml_file(filename: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let f = std::fs::File::open(filename)?;
        let base: BaseLayoutYAML = serde_yaml::from_reader(f)?;
        Ok(NeoLayoutGenerator::from_object(base, keyboard))
    }

    pub fn from_yaml_str(data: &str, keyboard: Arc<Keyboard>) -> Result<Self> {
        let base: BaseLayoutYAML = serde_yaml::from_str(data)?;
        Ok(NeoLayoutGenerator::from_object(base, keyboard))
    }

    pub fn generate(&self, layout_keys: &str) -> Result<Layout> {
        let chars: Vec<char> = layout_keys.chars().filter(|c| !c.is_whitespace()).collect();

        // TODO: assert that keys are unique (HashSet)
        let s: HashSet<char> = HashSet::from_iter(chars.clone());
        if s.len() != self.permutable_key_map.len() {
            return Err(LayoutError::InvalidLayout(layout_keys.to_string()).into());
        };
        // TODO: assert that number of provided keys equals number of permutable keys in generator

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

        // generate layer keys
        let mut layerkeys = Vec::new();
        let mut layerkey_index = 0;
        let key_layers: Vec<Vec<LayerKeyIndex>> = key_chars
            .iter()
            .zip(self.keyboard.keys.iter())
            .zip(self.fixed_keys.iter())
            .enumerate()
            .map(|(key_index, ((layer_chars, key), fixed))| {
                let indices: Vec<LayerKeyIndex> = layer_chars
                    .iter()
                    .enumerate()
                    .map(|(layer_id, c)| {
                        let layerkey = LayerKey::new(
                            layer_id,
                            key.clone(),
                            *c,
                            Vec::new(),
                            *fixed,
                            false,
                            key_index as KeyIndex,
                        );
                        layerkey_index += 1;
                        layerkeys.push(layerkey);

                        layerkey_index - 1
                    })
                    .collect();
                indices
            })
            .collect();

        let key_map = Self::gen_key_map(&layerkeys, &self.layer_costs);

        self.modifiers
            .iter()
            .for_each(|mods_per_hand| {
                mods_per_hand
                    .values()
                    .for_each(|mods| {
                        mods
                            .iter()
                            .for_each(|mc| {
                                layerkeys[*key_map.get(mc).unwrap() as usize].is_modifier = true;
                            });
                    });
            });

        layerkeys.iter_mut().for_each(|k| {
            let mods = if k.layer > 0 && k.layer < self.modifiers.len() + 1 {
                self.modifiers
                    .get(k.layer - 1)
                    .unwrap()
                    .get(&k.key.hand.other())
                    .map(|mods| mods.iter().map(|mc| *key_map.get(mc).unwrap()).collect())
                    .unwrap_or_default()
            } else {
                Vec::new()
            };

            k.modifiers = mods;
        });

        Ok(Layout::new(
            layerkeys,
            key_layers,
            self.keyboard.clone(),
            key_map,
            self.layer_costs.to_vec(),
        ))
    }

    fn gen_key_map(layerkeys: &[LayerKey], layer_costs: &[f64]) -> FxHashMap<char, LayerKeyIndex> {
        let mut m = FxHashMap::default();
        layerkeys
            .iter()
            .enumerate()
            .for_each(|(layerkey_index, layerkey)| {
                let new_layerkey_index = layerkey_index as LayerKeyIndex;
            let entry = m.entry(layerkey.symbol).or_insert(new_layerkey_index);
            let entry_layerkey = &layerkeys[*entry as usize]; // is layerkey or existing one from map m

            let entry_cost = entry_layerkey.key.cost
                + 3.0 * layer_costs[entry_layerkey.layer];
            let new_cost =
                layerkey.key.cost + 3.0 * layer_costs[layerkey.layer];

            // if key already exists use the representation with lowest key cost
            // if costs are identical, use lowest layer
            if new_cost < entry_cost
                || ((new_cost - entry_cost).abs() < 0.01 && layerkey.layer < entry_layerkey.layer)
            {
                m.insert(layerkey.symbol, new_layerkey_index);
            }
        });

        m
    }
}
