//! The layout module provides structs representing a keyboard layout and
//! its relation to the individual keys required to generate the layout's symbols.
//! These provide the core objects that are evaluated in the `layout_evaluation` crate.

use crate::key::Key;
use crate::keyboard::{KeyIndex, Keyboard};

use rustc_hash::FxHashMap;
use std::sync::Arc;

/// The index of a `LayerKey` in the `layerkeys` vec of a `Layout`
///
/// This type ist used as key for hashmaps in unigrams, bigrams, and trigrams and
/// thus directly impacts performance of the evaluation (hashing can take a large chunk of the computation time).
/// Therefore, this is not a `usize` or larger.
pub type LayerKeyIndex = u16;

/// Representation of a symbol that can be generated with a layout.
/// It consist of a key that needs to be pressed and a layer of the layout that produces the symbol
/// and contains various other useful properties, e.g. a list of modifiers required to reach given layer.
///
/// This struct serves as  major input to evaluation metrics in the `layout_evaluation` crate.
#[derive(Clone, PartialEq, Debug)]
pub struct LayerKey {
    /// Layer of the layout which the symbol belongs to
    pub layer: usize,
    /// Key to press for the symbol
    pub key: Key,
    /// Symbol belonging to a layout
    pub symbol: char,
    /// Vec of modifiers required to activate the layer (in terms of a `LayerKeyIndex` for a layout)
    pub modifiers: Vec<LayerKeyIndex>,
    /// If the key shall not be permutated for optimization
    pub is_fixed: bool,
    /// If the symbol itself is a modifier
    pub is_modifier: bool,
    key_index: KeyIndex, // is used for determining corresponding base layer key
}

impl LayerKey {
    pub fn new(
        layer: usize,
        key: Key,
        symbol: char,
        modifiers: Vec<LayerKeyIndex>,
        is_fixed: bool,
        is_modifier: bool,
        key_index: KeyIndex,
    ) -> Self {
        Self {
            layer,
            key,
            key_index,
            symbol,
            modifiers,
            is_fixed,
            is_modifier,
        }
    }
}

/// A layout represents a collection of symbols (chars) that can be generated with a keyboard.
/// To achieve a higher number of symbols than there are keys on the keyboard, each key can be
/// associated with several layers. The layers are activated by pressing (combinations of) modifier keys.
///
/// The layout is represented as a Vec of `LayerKey` objects with their indexes in the Vec being
/// called `LayerKeyIndex`.
/// A major task of the `Layout` object is to map given symbols (e.g. from a text) to corresponding
/// `LayerKey` objects that describe which key(s) is (are) required to generate it (and then analyse
/// corresponding efforts).
#[derive(Debug)]
pub struct Layout {
    /// Vec of `LayerKey` objects representing all symbols that can be generated with the layout
    pub layerkeys: Vec<LayerKey>,
    /// The underlying keyboard providing the keys
    pub keyboard: Arc<Keyboard>,
    key_layers: Vec<Vec<LayerKeyIndex>>,
    key_map: FxHashMap<char, LayerKeyIndex>,
    layer_costs: Vec<f64>,
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_text())
    }
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

    /// Get a `LayerKey` for a given index
    #[inline(always)]
    pub fn get_layerkey(&self, layerkey_index: &LayerKeyIndex) -> &LayerKey {
        &self.layerkeys[*layerkey_index as usize]
    }

    /// Get a `LayerKey` for a given symbol, if it can be generated with the layout
    #[inline(always)]
    pub fn get_layerkey_for_symbol(&self, c: &char) -> Option<&LayerKey> {
        self.key_map
            .get(c)
            .cloned()
            .map(|idx| self.get_layerkey(&idx))
    }

    /// Get the index of a `LayerKey` for a given symbol, if it can be generated with the layout
    #[inline(always)]
    pub fn get_layerkey_index_for_symbol(&self, c: &char) -> Option<LayerKeyIndex> {
        self.key_map.get(c).cloned()
    }

    /// Get the index of the "base" symbol (the one on the base layer, e.g. "A" -> "a") for a given `LayerKeyIndex`
    #[inline(always)]
    pub fn get_base_layerkey_index(&self, layerkey_index: &LayerKeyIndex) -> LayerKeyIndex {
        let layerkey = self.get_layerkey(layerkey_index);
        self.key_layers[layerkey.key_index as usize][0]
    }

    /// Get a list of modifiers required to generate a given `LayerKey` as a Vec of `LayerKey`s
    #[inline(always)]
    pub fn resolve_modifiers(&self, k: &LayerKeyIndex) -> (LayerKeyIndex, Vec<LayerKeyIndex>) {
        let base = self.get_base_layerkey_index(k);
        let k = self.get_layerkey(k);
        let mods = k.modifiers.to_vec();
        (base, mods)
    }

    /// Get the cost that are associated with a layer
    #[inline(always)]
    pub fn get_layer_cost(&self, layer: usize) -> f64 {
        *self.layer_costs.get(layer).unwrap_or(&0.0)
    }

    /// Plot a graphical representation of a layer
    pub fn plot_layer(&self, layer: usize) -> String {
        let keys_strings: Vec<String> = self
            .key_layers
            .iter()
            .map(|c| {
                if c.len() > layer {
                    self.get_layerkey(&c[layer])
                        .symbol
                        .to_string()
                        .replace("\n", "\u{23ce}")
                        .replace("\t", "\u{21e5}")
                        .replace("", "\u{2327}")
                        .replace("␡", " ")
                } else if !c.is_empty() {
                    self.get_layerkey(&c[c.len() - 1])
                        .symbol
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

    /// Plot a graphical representation of the base (first) layer
    pub fn plot(&self) -> String {
        self.plot_layer(0)
    }

    /// Plot a compact graphical representation (without borders and only non-fixed keys) of the base (first) layer
    pub fn plot_compact(&self) -> String {
        let keys_strings: Vec<String> = self
            .key_layers
            .iter()
            .map(|layerkeys| self.get_layerkey(&layerkeys[0]))
            .filter(|c| !c.is_fixed)
            .map(|k| k.symbol.to_string())
            .collect();
        let keys_str: Vec<&str> = keys_strings.iter().map(|s| s.as_str()).collect();
        self.keyboard.plot_compact(&keys_str)
    }

    /// Concatenate all non-fixed keys into a string without any whitespace
    pub fn as_text(&self) -> String {
        self.key_layers
            .iter()
            .map(|layerkeys| self.get_layerkey(&layerkeys[0]))
            .filter(|c| !c.is_fixed)
            .map(|k| k.symbol)
            .collect()
    }
}
