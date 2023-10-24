//! The layout module provides structs representing a keyboard layout and
//! its relation to the individual keys required to generate the layout's symbols.
//! These provide the core objects that are evaluated in the `layout_evaluation` crate.

use crate::key::{Hand, Key, MatrixPosition};
use crate::keyboard::{KeyIndex, Keyboard};

use ahash::AHashMap;
use anyhow::Result;
use colored::Colorize;
use core::slice;
use serde::Deserialize;
use smallmap::Map;
use std::{fmt, sync::Arc};

/// The index of a [`LayerKey`] in the `layerkeys` vec of a [`Layout`]
///
/// This type is used as the key for hashmaps in unigrams, bigrams, and trigrams and thus
/// directly impacts performance of the evaluation (hashing can take a large chunk of the computation time).
/// Therefore, this is not a [`usize`] or larger.
pub type LayerKeyIndex = u16;

/// Enum for specifying the location of a modifier relative to the keyboard.
///
/// This can be a `MatrixPosition` provided by the keyboard or a symbol that a corresponding layout
/// can genenrate (this should not belong to the same layer that the modifier is used for).
/// If multiple locations in the layout generate that symbol, one of those on the lowest layer is
/// used.
/// Note that if `Symbol(char)` is used, the modifier location may move with the symbol during an
/// optimization.
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum ModifierLocation {
    Position(MatrixPosition),
    Symbol(char),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LayerModifierType {
    None,
    Hold,
    OneShot,
    LongPress,
}

impl Default for LayerModifierType {
    fn default() -> Self {
        Self::None
    }
}

impl LayerModifierType {
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn is_hold(&self) -> bool {
        matches!(self, Self::Hold)
    }

    pub fn is_one_shot(&self) -> bool {
        matches!(self, Self::OneShot)
    }

    pub fn is_long_press(&self) -> bool {
        matches!(self, Self::LongPress)
    }
}

/// Enum for configuring the way how the modifiers shall be used to access a layer.
/// (e.g. whether the modifiers has to be held or tapped for activating a layer)
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum LayerModifierLocations {
    Hold(Vec<ModifierLocation>),
    OneShot(Vec<ModifierLocation>),
    LongPress,
}

impl LayerModifierLocations {
    pub fn iter(&self) -> slice::Iter<'_, ModifierLocation> {
        match self {
            Self::Hold(v) => v.iter(),
            Self::OneShot(v) => v.iter(),
            Self::LongPress => [].iter(),
        }
    }
    pub fn layer_modifier_type(&self) -> LayerModifierType {
        match self {
            Self::Hold(_) => LayerModifierType::Hold,
            Self::OneShot(_) => LayerModifierType::OneShot,
            Self::LongPress => LayerModifierType::LongPress,
        }
    }
}

/// Enumeration describing the various modifier types (e.g. whether the modifier has to be held or tapped
/// for activating a layer)
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LayerModifiers {
    Hold(Vec<LayerKeyIndex>),
    OneShot(Vec<LayerKeyIndex>),
    LongPress,
}

impl LayerModifiers {
    pub fn layerkey_indices(&self) -> &[LayerKeyIndex] {
        match self {
            Self::Hold(v) => v,
            Self::OneShot(v) => v,
            Self::LongPress => &[],
        }
    }
}

impl Default for LayerModifiers {
    fn default() -> Self {
        Self::Hold(Vec::new())
    }
}

/// Representation of a symbol that can be generated with a layout.
/// It consist of a key that needs to be pressed and a layer of the layout that produces the symbol
/// and contains various other useful properties, e.g. a list of modifiers required to reach given layer.
///
/// This struct serves as major input to evaluation metrics in the `layout_evaluation` crate.
#[derive(Clone, PartialEq, Debug)]
pub struct LayerKey {
    /// Layer of the layout which the symbol belongs to
    pub layer: u8,
    /// Key to press for the symbol
    pub key: Key,
    /// Symbol belonging to a layout
    pub symbol: char,
    /// Vec of modifiers required to activate the layer (in terms of a [`LayerKeyIndex`] for a layout)
    pub modifiers: LayerModifiers,
    /// If the key shall not be permutated for optimization
    pub is_fixed: bool,
    /// If the symbol itself is a modifier
    pub is_modifier: LayerModifierType,
}

impl fmt::Display for LayerKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_modifier.is_some() {
            write!(f, "[{}]", self.symbol.escape_debug())
        } else {
            write!(f, "{}", self.symbol.escape_debug())
        }
    }
}

impl LayerKey {
    pub fn new(
        layer: u8,
        key: Key,
        symbol: char,
        modifiers: LayerModifiers,
        is_fixed: bool,
        is_modifier: LayerModifierType,
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
    key_map: Map<char, LayerKeyIndex>,
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
        modifiers: Vec<AHashMap<Hand, LayerModifierLocations>>,
    ) -> Result<Self> {
        // generate layer keys
        let mut layerkeys = Vec::new();
        let mut layerkey_to_key_index = Vec::new();
        let mut char2layerkey_index: AHashMap<char, LayerKeyIndex> = AHashMap::default();
        let mut pos2layerkey_index: AHashMap<MatrixPosition, LayerKeyIndex> = AHashMap::default();
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
                            LayerModifiers::default(),
                            *fixed,
                            LayerModifierType::None,
                        ));
                        layerkey_to_key_index.push(key_index as KeyIndex);

                        pos2layerkey_index
                            .entry(key.matrix_position)
                            .or_insert(layerkey_index);

                        // use layerkey with lowest layer for char2layerkey_index
                        let entry = char2layerkey_index.entry(*c).or_insert(layerkey_index);
                        let entry_layerkey = &layerkeys[*entry as usize];
                        if layer_id < entry_layerkey.layer as usize {
                            char2layerkey_index.insert(*c, layerkey_index);
                        }

                        layerkey_index += 1;
                        layerkey_index - 1
                    })
                    .collect();

                indices
            })
            .collect();

        // a vec that provides the `modifiers` in terms of LayerKeyIndex for each layer
        let mut mod_map: Vec<AHashMap<Hand, LayerModifiers>> = Vec::with_capacity(modifiers.len());

        // add modifier keys as layerkeys
        let mut pos2mod_index: AHashMap<(LayerModifierType, MatrixPosition), LayerKeyIndex> =
            AHashMap::default();
        let mut char2mod_index: AHashMap<(LayerModifierType, char), LayerKeyIndex> =
            AHashMap::default();
        for mods_per_hand in modifiers.iter() {
            let mut resolved_mods_per_hand = AHashMap::default();
            for (hand, mods) in mods_per_hand.iter() {
                let mut resolved_mods_vec = Vec::new();
                for mp in mods.iter() {
                    let layer_modifier_type = mods.layer_modifier_type();
                    match mp {
                        ModifierLocation::Position(mp) => {
                            let base_key_idx = *pos2layerkey_index
                                .get(mp)
                                .ok_or(format!("Modifier position '{:?}' not found", mp))
                                .map_err(anyhow::Error::msg)?;
                            let mod_idx = *pos2mod_index
                                .entry((layer_modifier_type, *mp))
                                .or_insert_with(|| {
                                    let base_layerkey = &layerkeys[base_key_idx as usize];
                                    layerkeys.push(LayerKey::new(
                                        0,
                                        base_layerkey.key.clone(),
                                        base_layerkey.symbol,
                                        LayerModifiers::default(),
                                        base_layerkey.is_fixed,
                                        layer_modifier_type,
                                    ));
                                    layerkey_to_key_index
                                        .push(layerkey_to_key_index[base_key_idx as usize]);

                                    layerkey_index += 1;
                                    layerkey_index - 1
                                });
                            resolved_mods_vec.push(mod_idx);
                        }
                        ModifierLocation::Symbol(c) => {
                            let base_key_idx = *char2layerkey_index
                                .get(c)
                                .ok_or(format!("Modifier char '{:?}' not found", c))
                                .map_err(anyhow::Error::msg)?;
                            let mod_idx = *char2mod_index
                                .entry((layer_modifier_type, *c))
                                .or_insert_with(|| {
                                    let base_layerkey = &layerkeys[base_key_idx as usize];
                                    layerkeys.push(LayerKey::new(
                                        base_layerkey.layer,
                                        base_layerkey.key.clone(),
                                        base_layerkey.symbol,
                                        base_layerkey.modifiers.clone(),
                                        base_layerkey.is_fixed,
                                        layer_modifier_type,
                                    ));
                                    layerkey_to_key_index
                                        .push(layerkey_to_key_index[base_key_idx as usize]);

                                    layerkey_index += 1;
                                    layerkey_index - 1
                                });

                            resolved_mods_vec.push(mod_idx);
                        }
                    }
                }
                let resolved_mods = match mods {
                    LayerModifierLocations::Hold(_) => LayerModifiers::Hold(resolved_mods_vec),
                    LayerModifierLocations::OneShot(_) => {
                        LayerModifiers::OneShot(resolved_mods_vec)
                    }
                    LayerModifierLocations::LongPress => LayerModifiers::LongPress,
                };
                resolved_mods_per_hand.insert(*hand, resolved_mods);
            }
            mod_map.push(resolved_mods_per_hand);
        }

        // resolve each Single LayerKey's modifiers
        layerkeys.iter_mut().for_each(|k| {
            let mods = if k.layer > 0 && k.layer < (modifiers.len() + 1) as u8 {
                mod_map
                    .get((k.layer - 1) as usize)
                    .unwrap() // can not fail due to above check
                    .get(&k.key.hand.other())
                    .cloned()
                    .unwrap_or_default()
            } else {
                LayerModifiers::default()
            };

            k.modifiers = mods;
        });

        let key_map = Self::gen_key_map(&layerkeys);

        Ok(Self {
            layerkeys,
            key_layers,
            keyboard,
            layerkey_to_key_index,
            key_map,
        })
    }

    fn gen_key_map(layerkeys: &[LayerKey]) -> Map<char, LayerKeyIndex> {
        let mut m = Map::default();
        layerkeys
            .iter()
            .enumerate()
            .for_each(|(layerkey_index, layerkey)| {
                // modifiers do not generate symbols themselves -> return
                if layerkey.is_modifier.is_some() {
                    return;
                };

                // cast usize layerkey_index to LayerKeyIndex
                let layerkey_index = layerkey_index as LayerKeyIndex;
                let entry = m.entry(layerkey.symbol).or_insert(layerkey_index);
                let entry_layerkey = &layerkeys[*entry as usize]; // is layerkey or existing one from map m

                let entry_modifier_cost: f64 = entry_layerkey
                    .modifiers
                    .layerkey_indices()
                    .iter()
                    .map(|i| layerkeys[*i as usize].key.cost)
                    .sum();

                let new_modifier_cost: f64 = layerkey
                    .modifiers
                    .layerkey_indices()
                    .iter()
                    .map(|i| layerkeys[*i as usize].key.cost)
                    .sum();

                // NOTE: In contrast to ArneBab's version, here the layer costs are not multiplied by 3
                let entry_cost = entry_layerkey.key.cost + entry_modifier_cost;
                let new_cost = layerkey.key.cost + new_modifier_cost;

                // if key already exists use the representation with lowest key cost
                // if costs are identical, use lowest layer
                if new_cost < entry_cost
                    || ((new_cost - entry_cost).abs() < 0.01
                        && layerkey.layer < entry_layerkey.layer)
                {
                    m.insert(layerkey.symbol, layerkey_index);
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
    pub fn resolve_modifiers(&self, k: &LayerKeyIndex) -> (LayerKeyIndex, LayerModifiers) {
        let lk = self.get_layerkey(k);

        if matches!(lk.modifiers, LayerModifiers::LongPress) {
            // long press does not resolve to underlying base LayerKey
            return (*k, LayerModifiers::LongPress);
        }

        let base = self.get_base_layerkey_index(k);
        let mods = lk.modifiers.clone();
        (base, mods)
    }

    /// If the layout has at least one layer configured as hold layer
    pub fn has_hold_layers(&self) -> bool {
        self.layerkeys
            .iter()
            .any(|lk| std::matches!(lk.modifiers, LayerModifiers::Hold(_)))
    }

    /// If the layout has at least one layer configured as one-shot layer
    pub fn has_one_shot_layers(&self) -> bool {
        self.layerkeys
            .iter()
            .any(|lk| std::matches!(lk.modifiers, LayerModifiers::OneShot(_)))
    }

    /// Plot a graphical representation of a layer
    pub fn plot_layer(&self, layer: usize) -> String {
        let fmt_char = |c: char| -> char {
            match c {
                ' ' => '␣',
                '\n' => '\u{23ce}',
                '\t' => '\u{21e5}',
                '' => '\u{2327}',
                normal_char => normal_char,
            }
        };
        let key_chars: Vec<String> = self
            .key_layers
            .iter()
            .map(|layers| {
                if layers.is_empty() {
                    return " ".to_string();
                }
                // layers may have less items than given "layer"
                let k = self.get_layerkey(&layers[layer.min(layers.len() - 1)]);

                if layer >= layers.len() && !k.is_fixed {
                    // for non-fixed, show empty field if no symbol is in layers
                    " ".to_string()
                } else {
                    // if no symbol is in layers, show last layers value if it is fixed
                    let mut s = fmt_char(k.symbol).to_string();
                    if !k.is_fixed {
                        s = s.yellow().bold().to_string();
                    }
                    s
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
        let key_chars: Vec<String> = self
            .key_layers
            .iter()
            .filter_map(|layerkeys| layerkeys.first().map(|lk| self.get_layerkey(lk)))
            .filter(|k| !k.is_fixed)
            .map(|k| k.symbol.to_string())
            .collect();
        self.keyboard.plot_compact(&key_chars)
    }

    /// Concatenate all non-fixed keys into a string without any whitespace
    pub fn as_text(&self) -> String {
        self.key_layers
            .iter()
            .filter_map(|layerkeys| layerkeys.first().map(|lk| self.get_layerkey(lk)))
            .filter(|k| !k.is_fixed)
            .map(|k| k.symbol.to_string())
            .collect()
    }
}
