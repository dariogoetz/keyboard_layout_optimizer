//! The layout module provides structs representing a keyboard layout and
//! its relation to the individual keys required to generate the layout's symbols.
//! These provide the core objects that are evaluated in the `layout_evaluation` crate.

use crate::key::{Hand, Key};
use crate::keyboard::{KeyIndex, Keyboard};

use anyhow::Result;
use rustc_hash::FxHashMap;
use std::{fmt, sync::Arc};

/// The index of a [`LayerKey`] in the `layerkeys` vec of a [`Layout`]
///
/// This type is used as the key for hashmaps in unigrams, bigrams, and trigrams and thus
/// directly impacts performance of the evaluation (hashing can take a large chunk of the computation time).
/// Therefore, this is not a [`usize`] or larger.
pub type LayerKeyIndex = u16;

/// Representation of a symbol that can be generated with a layout.
/// It consist of a key that needs to be pressed and a layer of the layout that produces the symbol
/// and contains various other useful properties, e.g. a list of modifiers required to reach given layer.
///
/// This struct serves as  major input to evaluation metrics in the `layout_evaluation` crate.
#[derive(Clone, PartialEq, Debug)]
pub struct LayerKey {
    /// Layer of the layout which the symbol belongs to
    pub layer: u8,
    /// Key to press for the symbol
    pub key: Key,
    /// Symbol belonging to a layout
    pub symbol: char,
    /// Vec of modifiers required to activate the layer (in terms of a [`LayerKeyIndex`] for a layout)
    pub modifiers: Vec<LayerKeyIndex>,
    /// If the key shall not be permutated for optimization
    pub is_fixed: bool,
    /// If the symbol itself is a modifier
    pub is_modifier: bool,
}

impl LayerKey {
    pub fn new(
        layer: u8,
        key: Key,
        symbol: char,
        modifiers: Vec<LayerKeyIndex>,
        is_fixed: bool,
        is_modifier: bool,
    ) -> Self {
        Self {
            layer,
            key,
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
/// The layout is represented as a Vec of [`LayerKey`] objects with their indexes in the Vec being
/// called [`LayerKeyIndex`].
/// A major task of the [`Layout`] object is to map given symbols (e.g. from a text) to corresponding
/// [`LayerKey`] objects that describe which key(s) is (are) required to generate it (and then analyse
/// corresponding efforts).
#[derive(Debug, Clone)]
pub struct Layout {
    /// Vec of [`LayerKey`] objects representing all symbols that can be generated with the layout
    pub layerkeys: Vec<LayerKey>,
    /// The underlying keyboard providing the keys
    pub keyboard: Arc<Keyboard>,
    /// Vec of the [`KeyIndex`] corresponding to each [`LayerKey`] in `layerkeys`
    layerkey_to_key_index: Vec<KeyIndex>,
    /// Vec for each [`Key`] of the [`Keyboard`] containing a Vec of all [`LayerKey`] that are
    /// generaten with that [`Key`]
    key_layers: Vec<Vec<LayerKeyIndex>>,
    /// Map for retrieving the [`LayerKey`] for the symbol it generates
    key_map: FxHashMap<char, LayerKeyIndex>,
    /// Costs associated with each layer
    layer_costs: Vec<f64>,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

impl Layout {
    pub fn new(
        key_chars: Vec<Vec<char>>,
        fixed_keys: Vec<bool>,
        keyboard: Arc<Keyboard>,
        modifiers: Vec<FxHashMap<Hand, Vec<char>>>,
        layer_costs: Vec<f64>,
    ) -> Result<Self> {
        // generate layer keys
        let mut layerkeys = Vec::new();
        let mut layerkey_to_key_index = Vec::new();
        let mut layerkey_index = 0;
        let key_layers: Vec<Vec<LayerKeyIndex>> = key_chars
            .iter()
            .zip(keyboard.keys.iter())
            .zip(fixed_keys.iter())
            .enumerate()
            .map(|(key_index, ((layer_chars, key), fixed))| {
                let indices: Vec<LayerKeyIndex> = layer_chars
                    .iter()
                    .enumerate()
                    .take(modifiers.len() + 1) // only consider layers for which a modifier is available
                    .map(|(layer_id, c)| {
                        layerkeys.push(LayerKey::new(
                            layer_id as u8,
                            key.clone(),
                            *c,
                            Vec::new(),
                            *fixed,
                            false,
                        ));
                        layerkey_to_key_index.push(key_index as KeyIndex);

                        let old_layerkey_index = layerkey_index;
                        layerkey_index += 1;
                        old_layerkey_index
                    })
                    .collect();

                indices
            })
            .collect();

        let key_map = Self::gen_key_map(&layerkeys, &layer_costs);

        // a map that resolvers the `modifiers` chars to LayerKeyIndex
        let mut mod_map: Vec<FxHashMap<Hand, Vec<LayerKeyIndex>>> =
            Vec::with_capacity(modifiers.len());
        for mods_per_hand in modifiers.iter() {
            let mut resolved_mods_per_hand = FxHashMap::default();
            for (hand, mods) in mods_per_hand.iter() {
                let mut resolved_mods = Vec::new();
                for mc in mods.iter() {
                    let mod_idx = *key_map
                        .get(mc)
                        .ok_or(format!("Modifier '{}' is not a supported symbol", mc))
                        .map_err(anyhow::Error::msg)?;

                    resolved_mods.push(mod_idx);

                    // flag this layerkey as modifier
                    layerkeys[mod_idx as usize].is_modifier = true;
                }
                resolved_mods_per_hand.insert(*hand, resolved_mods);
            }
            mod_map.push(resolved_mods_per_hand);
        }

        layerkeys.iter_mut().for_each(|k| {
            let mods = if k.layer > 0 && k.layer < (modifiers.len() + 1) as u8 {
                mod_map
                    .get((k.layer - 1) as usize)
                    .unwrap() // can not fail due to above check
                    .get(&k.key.hand.other())
                    .map(|mods| mods.to_vec())
                    .unwrap_or_default() // default is an empty vec
            } else {
                Vec::new()
            };

            k.modifiers = mods;
        });

        Ok(Self {
            layerkeys,
            key_layers,
            keyboard,
            layerkey_to_key_index,
            key_map,
            layer_costs,
        })
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

                // NOTE: In contrast to ArneBab's version, here the layer costs are not multiplied by 3
                let entry_cost =
                    entry_layerkey.key.cost + layer_costs[entry_layerkey.layer as usize];
                let new_cost = layerkey.key.cost + layer_costs[layerkey.layer as usize];

                // if key already exists use the representation with lowest key cost
                // if costs are identical, use lowest layer
                if new_cost < entry_cost
                    || ((new_cost - entry_cost).abs() < 0.01
                        && layerkey.layer < entry_layerkey.layer)
                {
                    m.insert(layerkey.symbol, new_layerkey_index);
                }
            });

        m
    }

    /// Get a [`LayerKey`] for a given index
    #[inline(always)]
    pub fn get_layerkey(&self, layerkey_index: &LayerKeyIndex) -> &LayerKey {
        &self.layerkeys[*layerkey_index as usize]
    }

    /// Get a [`LayerKey`] for a given symbol, if it can be generated with the layout
    #[inline(always)]
    pub fn get_layerkey_for_symbol(&self, c: &char) -> Option<&LayerKey> {
        self.key_map.get(c).map(|idx| self.get_layerkey(idx))
    }

    /// Get the index of a [`LayerKey`] for a given symbol, if it can be generated with the layout
    #[inline(always)]
    pub fn get_layerkey_index_for_symbol(&self, c: &char) -> Option<LayerKeyIndex> {
        self.key_map.get(c).cloned()
    }

    /// Get the index of the "base" symbol (the one on the base layer, e.g. "A" -> "a") for a given [`LayerKeyIndex`]
    #[inline(always)]
    pub fn get_base_layerkey_index(&self, layerkey_index: &LayerKeyIndex) -> LayerKeyIndex {
        let key_index: usize = self.layerkey_to_key_index[*layerkey_index as usize] as usize;
        self.key_layers[key_index][0]
    }

    /// Get a list of modifiers required to generate a given [`LayerKey`] as a Vec of [`LayerKey`]s
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
        let fmt_char = |c: char| -> char {
            match c {
                ' ' => '␣',
                '\n' => '\u{23ce}',
                '\t' => '\u{21e5}',
                '' => '\u{2327}',
                '␡' => ' ',
                normal_char => normal_char,
            }
        };
        let key_chars: Vec<char> = self
            .key_layers
            .iter()
            .map(|c| {
                if c.len() > layer {
                    fmt_char(self.get_layerkey(&c[layer]).symbol)
                } else if !c.is_empty() {
                    fmt_char(self.get_layerkey(&c[c.len() - 1]).symbol)
                } else {
                    ' '
                }
            })
            .collect();

        self.keyboard.plot(&key_chars)
    }

    /// Plot a graphical representation of the base (first) layer
    pub fn plot(&self) -> String {
        self.plot_layer(0)
    }

    /// Plot a compact graphical representation (without borders and only non-fixed keys) of the base (first) layer
    pub fn plot_compact(&self) -> String {
        let key_chars: Vec<char> = self
            .key_layers
            .iter()
            .map(|layerkeys| self.get_layerkey(&layerkeys[0]))
            .filter(|k| !k.is_fixed)
            .map(|k| k.symbol)
            .collect();
        self.keyboard.plot_compact(&key_chars)
    }

    /// Concatenate all non-fixed keys into a string without any whitespace
    pub fn as_text(&self) -> String {
        self.key_layers
            .iter()
            .map(|layerkeys| self.get_layerkey(&layerkeys[0]))
            .filter(|k| !k.is_fixed)
            .map(|k| k.symbol)
            .collect()
    }
}
