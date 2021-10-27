use crate::key::Key;
use crate::keyboard::Keyboard;

use rustc_hash::FxHashMap;
use std::sync::Arc;

pub type LayerKeyIndex = u16;

#[derive(Clone, PartialEq, Debug)]
pub struct LayerKey {
    pub layer: usize,
    pub key: Key,
    pub char: char,
    pub modifiers: Vec<LayerKeyIndex>,
    pub is_fixed: bool,
    pub is_modifier: bool,
    pub index: LayerKeyIndex,
}

#[derive(Debug)]
pub struct Layout {
    pub layerkeys: Vec<LayerKey>,
    pub keyboard: Arc<Keyboard>,
    key_layers: Vec<Vec<LayerKeyIndex>>,
    key_map: FxHashMap<char, LayerKeyIndex>,
    layer_costs: Vec<f64>,
}

impl Layout {
    pub fn new(
        layerkeys: Vec<LayerKey>,
        key_layers: Vec<Vec<LayerKeyIndex>>,
        keyboard: Arc<Keyboard>,
        key_map: FxHashMap<char, LayerKeyIndex>,
        layer_costs: Vec<f64>,
    ) -> Self {
        Self {
            layerkeys,
            key_layers,
            keyboard,
            key_map,
            layer_costs,
        }
    }

    #[inline(always)]
    pub fn get_layerkey(&self, layerkey_index: &LayerKeyIndex) -> &LayerKey {
        &self.layerkeys[*layerkey_index as usize]
    }

    #[inline(always)]
    pub fn get_layerkey_for_char(&self, c: &char) -> Option<&LayerKey> {
        self.key_map
            .get(c)
            .cloned()
            .map(|idx| self.get_layerkey(&idx))
    }

    #[inline(always)]
    pub fn get_layerkey_index_for_char(&self, c: &char) -> Option<LayerKeyIndex> {
        self.key_map.get(c).cloned()
    }

    #[inline(always)]
    pub fn get_base_layerkey_index(&self, layerkey_index: &LayerKeyIndex) -> LayerKeyIndex {
        let layerkey = self.get_layerkey(layerkey_index);
        self.key_layers[layerkey.key.index][0]
    }

    #[inline(always)]
    pub fn resolve_modifiers(&self, k: &LayerKeyIndex) -> (LayerKeyIndex, Vec<LayerKeyIndex>) {
        let base = self.get_base_layerkey_index(k);
        let k = self.get_layerkey(k);
        let mods = k.modifiers.to_vec();
        (base, mods)
    }

    #[inline(always)]
    pub fn get_layer_cost(&self, layer: usize) -> f64 {
        *self.layer_costs.get(layer).unwrap_or(&0.0)
    }

    pub fn plot_layer(&self, layer: usize) -> String {
        let keys_strings: Vec<String> = self
            .key_layers
            .iter()
            .map(|c| {
                if c.len() > layer {
                    self.get_layerkey(&c[layer])
                        .char
                        .to_string()
                        .replace("\n", "\u{23ce}")
                        .replace("\t", "\u{21e5}")
                        .replace("", "\u{2327}")
                        .replace("␡", " ")
                } else if !c.is_empty() {
                    self.get_layerkey(&c[c.len() - 1])
                        .char
                        .to_string()
                        .replace("\n", "\u{23ce}")
                        .replace("\t", "\u{21e5}")
                        .replace("", "\u{2327}")
                        .replace("␡", " ")
                } else {
                    " ".to_string()
                }
            })
            .collect();

        let keys_str: Vec<&str> = keys_strings.iter().map(|s| s.as_str()).collect();
        self.keyboard.plot(&keys_str)
    }

    pub fn plot(&self) -> String {
        self.plot_layer(0)
    }

    pub fn plot_short(&self) -> String {
        let keys_strings: Vec<String> = self
            .key_layers
            .iter()
            .map(|layerkeys| self.get_layerkey(&layerkeys[0]))
            .filter(|c| !c.is_fixed)
            .map(|k| k.char.to_string())
            .collect();
        let keys_str: Vec<&str> = keys_strings.iter().map(|s| s.as_str()).collect();
        self.keyboard.plot_short(&keys_str)
    }

    pub fn as_text(&self) -> String {
        self.key_layers
            .iter()
            .map(|layerkeys| self.get_layerkey(&layerkeys[0]))
            .filter(|c| !c.is_fixed)
            .map(|k| k.char)
            .collect()
    }
}
