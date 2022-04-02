//! This module provides an implementation of trigram mapping functionalities
//! used by the [`OnDemandNgramMapper`].

use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};
use super::TrigramIndices;

use crate::ngrams::Trigrams;

use keyboard_layout::layout::{LayerKey, Layout};

use ahash::AHashMap;

/// Turns the [`Trigrams`]'s characters into their indices, returning a [`TrigramIndices`].
fn map_trigrams(
    trigrams: &Trigrams,
    layout: &Layout,
    exclude_line_breaks: bool,
) -> (TrigramIndices, f64) {
    let mut not_found_weight = 0.0;
    let mut trigram_keys = AHashMap::with_capacity(trigrams.grams.len());

    trigrams
        .grams
        .iter()
        //.filter(|((c1, c2, c3), _weight)| {
        //    !c1.is_whitespace() && !c2.is_whitespace() && !c3.is_whitespace()
        //})
        .filter(|((c1, c2, c3), _weight)| {
            // Exclude trigrams that contain a line break, followed by a non-line-break character
            !(exclude_line_breaks && ((*c1 == '\n' && *c2 != '\n') || (*c2 == '\n' && *c3 != '\n')))
        })
        .for_each(|((c1, c2, c3), weight)| {
            let idx1 = match layout.get_layerkey_index_for_symbol(c1) {
                Some(idx) => idx,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            let idx2 = match layout.get_layerkey_index_for_symbol(c2) {
                Some(idx) => idx,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            let idx3 = match layout.get_layerkey_index_for_symbol(c3) {
                Some(idx) => idx,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            trigram_keys.insert_or_add_weight((idx1, idx2, idx3), *weight);
        });

    (trigram_keys, not_found_weight)
}

/// Generates [`LayerKey`]-based trigrams from char-based unigrams. Optionally resolves modifiers
/// for higher-layer symbols of the layout.
#[derive(Clone, Debug)]
pub struct OnDemandTrigramMapper {
    trigrams: Trigrams,
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandTrigramMapper {
    pub fn new(trigrams: Trigrams, split_modifiers: SplitModifiersConfig) -> Self {
        Self {
            trigrams,
            split_modifiers,
        }
    }

    /// For a given [`Layout`] generate [`LayerKeyIndex`]-based unigrams, optionally resolving modifiers for higer-layer symbols.
    pub fn layerkey_indices(
        &self,
        layout: &Layout,
        exclude_line_breaks: bool,
    ) -> (TrigramIndices, f64, f64) {
        let (trigram_keys, not_found_weight) =
            map_trigrams(&self.trigrams, layout, exclude_line_breaks);

        let trigram_keys = if self.split_modifiers.enabled {
            self.split_trigram_modifiers(trigram_keys, layout)
        } else {
            trigram_keys
        };

        let found_weight: f64 = trigram_keys.values().sum();

        (trigram_keys, found_weight, not_found_weight)
    }

    /// Resolve &[`LayerKey`] references for [`LayerKeyIndex`]
    pub fn layerkeys<'s>(
        trigrams: &TrigramIndices,
        layout: &'s Layout,
    ) -> Vec<((&'s LayerKey, &'s LayerKey, &'s LayerKey), f64)> {
        trigrams
            .iter()
            .map(|((k1, k2, k3), w)| {
                (
                    (
                        layout.get_layerkey(k1),
                        layout.get_layerkey(k2),
                        layout.get_layerkey(k3),
                    ),
                    *w,
                )
            })
            .collect()
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
        trigrams: TrigramIndices,
        layout: &Layout,
    ) -> TrigramIndices {
        let mut trigram_w_map = AHashMap::with_capacity(trigrams.len());
        trigrams.into_iter().for_each(|((k1, k2, k3), w)| {
            let (base1, mods1) = layout.resolve_modifiers(&k1);
            let (base2, mods2) = layout.resolve_modifiers(&k2);
            let (base3, mods3) = layout.resolve_modifiers(&k3);

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
                            //     e1.symbol.escape_debug(),
                            //     e2.symbol.escape_debug(),
                            //     e3.symbol.escape_debug(),
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
                        //     e1.symbol.escape_debug(),
                        //     e2.symbol.escape_debug(),
                        //     e3.symbol.escape_debug(),
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
                        //     e1.symbol.escape_debug(),
                        //     e2.symbol.escape_debug(),
                        //     e3.symbol.escape_debug(),
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
                        //     e1.symbol.escape_debug(),
                        //     e2.symbol.escape_debug(),
                        //     e3.symbol.escape_debug(),
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
                        //     e1.symbol.escape_debug(),
                        //     e2.symbol.escape_debug(),
                        //     e3.symbol.escape_debug(),
                        // );
                        trigram_w_map.insert_or_add_weight((e1, e2, e3), w1);
                    }
                });
            });

            TakeThreeLayerKey::new(base1, &mods1, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of first:              {}{}{}",
                    //     e.0.symbol.escape_debug(),
                    //     e.1.symbol.escape_debug(),
                    //     e.2.symbol.escape_debug(),
                    // );
                    trigram_w_map.insert_or_add_weight(e, w);
                });

            TakeThreeLayerKey::new(base2, &mods2, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of second:             {}{}{}",
                    //     e.0.symbol.escape_debug(),
                    //     e.1.symbol.escape_debug(),
                    //     e.2.symbol.escape_debug(),
                    // );
                    trigram_w_map.insert_or_add_weight(e, w);
                });

            TakeThreeLayerKey::new(base3, &mods3, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of third:              {}{}{}",
                    //     e.0.symbol.escape_debug(),
                    //     e.1.symbol.escape_debug(),
                    //     e.2.symbol.escape_debug(),
                    // );
                    trigram_w_map.insert_or_add_weight(e, w);
                });

            // log::debug!(
            //     "{:>3}{:^3}{:<3} -> {}",
            //     k1.symbol.escape_debug().to_string(),
            //     k2.symbol.escape_debug().to_string(),
            //     k3.symbol.escape_debug().to_string(),
            //     v.iter()
            //         .map(|((t1, t2, t3), w)| format!(
            //             "\n{}{}{} (weight: {:>12.2}) ",
            //             t1.symbol.escape_debug(),
            //             t2.symbol.escape_debug(),
            //             t3.symbol.escape_debug(),
            //             w
            //         ))
            //         .collect::<String>(),
            // );
        });

        trigram_w_map
    }
}
