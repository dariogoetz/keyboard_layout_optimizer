//! This module provides an implementation of trigram mapping functionalities
//! used by the [`OnDemandNgramMapper`].

use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};

use crate::ngrams::Trigrams;

use ahash::AHashMap;
use keyboard_layout::layout::{LayerKey, LayerKeyIndex, LayerModifiers, Layout};

// Before passing the resulting LayerKey-based ngrams as a result, smaller LayerKeyIndex-based
// ones are used because they are smaller than a reference (u16 vs usize) and yield better
// hashing performance.
pub type TrigramIndices = AHashMap<(LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64>;
type TrigramIndicesVec = Vec<((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64)>;

/// Turns the [`Trigrams`]'s characters into their indices, returning a [`TrigramIndicesVec`].
fn map_trigrams(
    trigrams: &Trigrams,
    layout: &Layout,
    exclude_line_breaks: bool,
) -> (TrigramIndicesVec, f64) {
    let mut not_found_weight = 0.0;
    let mut trigrams_vec = Vec::with_capacity(trigrams.grams.len());

    trigrams_vec.extend(
        trigrams
            .grams
            .iter()
            //.filter(|((c1, c2, c3), _weight)| {
            //    !c1.is_whitespace() && !c2.is_whitespace() && !c3.is_whitespace()
            //})
            .filter_map(|((c1, c2, c3), weight)| {
                // Exclude trigrams that contain a line break, followed by a non-line-break character
                if exclude_line_breaks
                    && ((*c1 == '\n' && *c2 != '\n') || (*c2 == '\n' && *c3 != '\n'))
                {
                    return None;
                }

                let idx1 = match layout.get_layerkey_index_for_symbol(c1) {
                    Some(idx) => idx,
                    None => {
                        not_found_weight += *weight;
                        return None;
                    }
                };
                let idx2 = match layout.get_layerkey_index_for_symbol(c2) {
                    Some(idx) => idx,
                    None => {
                        not_found_weight += *weight;
                        return None;
                    }
                };
                let idx3 = match layout.get_layerkey_index_for_symbol(c3) {
                    Some(idx) => idx,
                    None => {
                        not_found_weight += *weight;
                        return None;
                    }
                };

                Some(((idx1, idx2, idx3), *weight))
            }),
    );

    (trigrams_vec, not_found_weight)
}

/// Generates [`LayerKey`]-based trigrams from char-based unigrams. Optionally resolves modifiers
/// for higher-layer symbols of the layout.
#[derive(Clone, Debug)]
pub struct OnDemandTrigramMapper {
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandTrigramMapper {
    pub fn new(split_modifiers: SplitModifiersConfig) -> Self {
        Self { split_modifiers }
    }

    /// For a given [`Layout`] generate [`LayerKeyIndex`]-based unigrams, optionally resolving modifiers for higer-layer symbols.
    pub fn layerkey_indices(
        &self,
        trigrams: &Trigrams,
        layout: &Layout,
        exclude_line_breaks: bool,
    ) -> (TrigramIndices, f64) {
        let (mut trigram_keys_vec, not_found_weight) =
            map_trigrams(trigrams, layout, exclude_line_breaks);

        if layout.has_one_shot_layers() {
            trigram_keys_vec = self.process_one_shot_layers(trigram_keys_vec, layout);
        }

        let trigram_keys = if self.split_modifiers.enabled {
            self.split_trigram_modifiers(trigram_keys_vec, layout)
        } else {
            trigram_keys_vec.into_iter().collect()
        };

        (trigram_keys, not_found_weight)
    }

    /// Resolve &[`LayerKey`] references for [`LayerKeyIndex`] and filters trigrams that contain
    /// repeating identical modifiers.
    pub fn get_filtered_layerkeys<'s>(
        trigrams: &TrigramIndices,
        layout: &'s Layout,
    ) -> Vec<((&'s LayerKey, &'s LayerKey, &'s LayerKey), f64)> {
        let mut layerkeys = Vec::with_capacity(trigrams.len());

        layerkeys.extend(trigrams.iter().filter_map(|((idx1, idx2, idx3), w)| {
            let k2 = layout.get_layerkey(idx2);

            // If the same modifier appears consecutively, it is usually "hold" instead of repeatedly pressed
            // --> remove
            match k2.is_modifier.is_hold() && (idx1 == idx2 || idx2 == idx3) {
                false => Some((
                    (
                        layout.get_layerkey(idx1), // LayerKey 1
                        k2,                        // LayerKey 2
                        layout.get_layerkey(idx3), // LayerKey 3
                    ),
                    *w,
                )),
                true => None,
            }
        }));

        layerkeys
    }

    /// Map all trigrams to base-layer trigrams, potentially generating multiple trigrams
    /// with modifiers for those with higer-layer keys.
    ///
    /// Each trigram of higher-layer symbols will transform into a series of various trigrams with permutations
    /// of the involved base-keys and modifiers. Keys from the latter parts of the trigram will always be after
    /// former ones and modifers always come before their base key. The number of generated trigrams from a single
    /// trigram can be large (tens of trigrams) if multiple symbols of the trigram are accessed using multiple modifiers.

    // this is one of the most intensive functions of the layout evaluation
    fn split_trigram_modifiers(
        &self,
        trigrams: TrigramIndicesVec,
        layout: &Layout,
    ) -> TrigramIndices {
        let mut trigram_w_map = AHashMap::with_capacity(trigrams.len() / 3);
        trigrams.into_iter().for_each(|((k1, k2, k3), w)| {
            let (base1, mods1) = layout.resolve_modifiers(&k1);
            let (base2, mods2) = layout.resolve_modifiers(&k2);
            let (base3, mods3) = layout.resolve_modifiers(&k3);

            let mods1 = match mods1 {
                LayerModifiers::Hold(mods) => mods,
                _ => Vec::new(),
            };

            let mods2 = match mods2 {
                LayerModifiers::Hold(mods) => mods,
                _ => Vec::new(),
            };

            let mods3 = match mods3 {
                LayerModifiers::Hold(mods) => mods,
                _ => Vec::new(),
            };

            let k1_take_one = TakeOneLayerKey::new(base1, &mods1, w);
            let k2_take_one = TakeOneLayerKey::new(base2, &mods2, w);
            let k3_take_one = TakeOneLayerKey::new(base3, &mods3, w);

            let k1_take_two =
                TakeTwoLayerKey::new(base1, &mods1, w, self.split_modifiers.same_key_mod_factor);
            let k2_take_two =
                TakeTwoLayerKey::new(base2, &mods2, w, self.split_modifiers.same_key_mod_factor);
            let k3_take_two =
                TakeTwoLayerKey::new(base3, &mods3, w, self.split_modifiers.same_key_mod_factor);

            k1_take_one.clone().for_each(|(e1, _)| {
                k2_take_one.clone().for_each(|(e2, _)| {
                    k3_take_one.clone().for_each(|(e3, _)| {
                        if (e1 != e2) && (e2 != e3) {
                            // log::trace!(
                            //     "one each:                    {}{}{}",
                            //     layout.get_layerkey(&e1).symbol,
                            //     layout.get_layerkey(&e2).symbol,
                            //     layout.get_layerkey(&e3).symbol,
                            // );
                            trigram_w_map.insert_or_add_weight((e1, e2, e3), w);
                        }
                    });
                });
            });

            k1_take_two.for_each(|((e1, e2), w1)| {
                k2_take_one.clone().for_each(|(e3, _)| {
                    if (e1 != e2) && (e2 != e3) {
                        // log::trace!(
                        //     "two of first, one of second: {}{}{}",
                        //     layout.get_layerkey(&e1).symbol,
                        //     layout.get_layerkey(&e2).symbol,
                        //     layout.get_layerkey(&e3).symbol,
                        // );
                        trigram_w_map.insert_or_add_weight((e1, e2, e3), w1);
                    }
                });
            });

            k1_take_one.for_each(|(e1, _)| {
                k2_take_two.clone().for_each(|((e2, e3), w1)| {
                    if (e1 != e2) && (e2 != e3) {
                        // log::trace!(
                        //     "one of first, two of second: {}{}{}",
                        //     layout.get_layerkey(&e1).symbol,
                        //     layout.get_layerkey(&e2).symbol,
                        //     layout.get_layerkey(&e3).symbol,
                        // );
                        trigram_w_map.insert_or_add_weight((e1, e2, e3), w1);
                    }
                });
            });

            k2_take_two.for_each(|((e1, e2), w1)| {
                k3_take_one.clone().for_each(|(e3, _)| {
                    if (e1 != e2) && (e2 != e3) {
                        // log::trace!(
                        //     "two of second, one of third: {}{}{}",
                        //     layout.get_layerkey(&e1).symbol,
                        //     layout.get_layerkey(&e2).symbol,
                        //     layout.get_layerkey(&e3).symbol,
                        // );
                        trigram_w_map.insert_or_add_weight((e1, e2, e3), w1);
                    }
                });
            });

            k2_take_one.for_each(|(e1, _)| {
                k3_take_two.clone().for_each(|((e2, e3), w1)| {
                    if (e1 != e2) && (e2 != e3) {
                        // log::trace!(
                        //     "one of second, two of third: {}{}{}",
                        //     layout.get_layerkey(&e1).symbol,
                        //     layout.get_layerkey(&e2).symbol,
                        //     layout.get_layerkey(&e3).symbol,
                        // );
                        trigram_w_map.insert_or_add_weight((e1, e2, e3), w1);
                    }
                });
            });

            TakeThreeLayerKey::new(base1, &mods1, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of first:              {}{}{}",
                    //     layout.get_layerkey(&e.0).symbol,
                    //     layout.get_layerkey(&e.1).symbol,
                    //     layout.get_layerkey(&e.2).symbol,
                    // );
                    trigram_w_map.insert_or_add_weight(e, w);
                });

            TakeThreeLayerKey::new(base2, &mods2, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of second:             {}{}{}",
                    //     layout.get_layerkey(&e.0).symbol,
                    //     layout.get_layerkey(&e.1).symbol,
                    //     layout.get_layerkey(&e.2).symbol,
                    // );
                    trigram_w_map.insert_or_add_weight(e, w);
                });

            TakeThreeLayerKey::new(base3, &mods3, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of third:              {}{}{}",
                    //     layout.get_layerkey(&e.0).symbol,
                    //     layout.get_layerkey(&e.1).symbol,
                    //     layout.get_layerkey(&e.2).symbol,
                    // );
                    trigram_w_map.insert_or_add_weight(e, w);
                });
        });

        trigram_w_map
    }

    fn process_one_shot_layers(
        &self,
        trigrams: TrigramIndicesVec,
        layout: &Layout,
    ) -> TrigramIndicesVec {
        let mut processed_trigrams = Vec::with_capacity(trigrams.len());

        trigrams.into_iter().for_each(|((k1, k2, k3), w)| {
            let (base1, mods1) = layout.resolve_modifiers(&k1);
            let (base2, mods2) = layout.resolve_modifiers(&k2);
            let (base3, mods3) = layout.resolve_modifiers(&k3);

            let mut keys = Vec::new();

            if let LayerModifiers::OneShot(mods) = mods1 {
                keys.extend(mods);
                keys.push(base1);
            } else {
                keys.push(k1);
            };

            if let LayerModifiers::OneShot(mods) = mods2 {
                keys.extend(mods);
                keys.push(base2);
            } else {
                keys.push(k2);
            };

            if let LayerModifiers::OneShot(mods) = mods3 {
                keys.extend(mods);
                keys.push(base3);
            } else {
                keys.push(k3);
            };

            keys.iter()
                .zip(keys.iter().skip(1))
                .zip(keys.iter().skip(2))
                .for_each(|((lk1, lk2), lk3)| {
                    processed_trigrams.push(((*lk1, *lk2, *lk3), w));
                });
        });

        processed_trigrams
    }
}
