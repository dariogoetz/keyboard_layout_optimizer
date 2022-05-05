//! This module provides an implementation of bigram mapping functionalities
//! used by the [`OnDemandNgramMapper`].
//!
//! Note: In contrast to ArneBab's algorithm, here all trigrams will be used
//! for secondary bigrams. Not only those that lead to same-hand bigrams.

use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};

use crate::ngrams::Bigrams;

use keyboard_layout::layout::{LayerKey, LayerKeyIndex, Layout};

use ahash::AHashMap;

// Before passing the resulting LayerKey-based ngrams as a result, smaller LayerKeyIndex-based
// ones are used because they are smaller than a reference (u16 vs usize) and yield better
// hashing performance.
type BigramIndices = AHashMap<(LayerKeyIndex, LayerKeyIndex), f64>;
type BigramIndicesVec = Vec<((LayerKeyIndex, LayerKeyIndex), f64)>;

/// Turns the [`Bigrams`]'s characters into their indices, returning a [`BigramIndicesVec`].
fn map_bigrams(
    bigrams: &Bigrams,
    layout: &Layout,
    exclude_line_breaks: bool,
) -> (BigramIndicesVec, f64) {
    let mut not_found_weight = 0.0;
    let mut bigrams_vec: BigramIndicesVec = Vec::with_capacity(bigrams.grams.len());

    bigrams_vec.extend(
        bigrams
            .grams
            .iter()
            //.filter(|((c1, c2), _weight)| !c1.is_whitespace() && !c2.is_whitespace())
            .filter_map(|((c1, c2), weight)| {
                // Exclude bigrams that contain a line break, followed by a non-line-break character
                if exclude_line_breaks && *c1 == '\n' && *c2 != '\n' {
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

                Some(((idx1, idx2), *weight))
            }),
    );

    (bigrams_vec, not_found_weight)
}

/// Generates [`LayerKey`]-based [Bigrams] from char-based unigrams. Optionally resolves modifiers
/// for higher-layer symbols of the layout.
#[derive(Clone, Debug)]
pub struct OnDemandBigramMapper {
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandBigramMapper {
    pub fn new(split_modifiers: SplitModifiersConfig) -> Self {
        Self { split_modifiers }
    }

    /// For a given [`Layout`] generate [`LayerKeyIndex`]-based unigrams, optionally resolving modifiers for higer-layer symbols.
    pub fn layerkey_indices(
        &self,
        bigrams: &Bigrams,
        layout: &Layout,
        exclude_line_breaks: bool,
    ) -> (BigramIndices, f64) {
        let (bigram_keys_vec, not_found_weight) = map_bigrams(bigrams, layout, exclude_line_breaks);

        let bigram_keys = if self.split_modifiers.enabled {
            self.split_bigram_modifiers(bigram_keys_vec, layout)
        } else {
            bigram_keys_vec.into_iter().collect()
        };

        // bigram_keys
        //     .iter()
        //     .filter(|((c1, c2), _)| c1.symbol == 'l' && c2.symbol == 'r')
        //     .for_each(|((_, _), w)| {
        //         println!("After split: {}", w);
        //     });

        (bigram_keys, not_found_weight)
    }

    /// Resolves &[`LayerKey`] references for [`LayerKeyIndex`] and filters bigrams that contain
    /// repeating identical modifiers.
    pub fn get_filtered_layerkeys<'s>(
        bigrams: &BigramIndices,
        layout: &'s Layout,
    ) -> Vec<((&'s LayerKey, &'s LayerKey), f64)> {
        let mut layerkeys = Vec::with_capacity(bigrams.len());

        layerkeys.extend(bigrams.iter().filter_map(|((idx1, idx2), w)| {
            let k1 = layout.get_layerkey(idx1);

            // If the same modifier appears consecutively, it is usually "hold" instead of repeatedly pressed
            // --> remove
            match k1.is_modifier && idx1 == idx2 {
                false => Some((
                    (
                        k1,                        // LayerKey 1
                        layout.get_layerkey(idx2), // LayerKey 2
                    ),
                    *w,
                )),
                true => None,
            }
        }));

        layerkeys
    }

    /// Map all bigrams to base-layer bigrams, potentially generating multiple bigrams
    /// with modifiers for those with higer-layer keys.
    ///
    /// Each bigram of higher-layer symbols will transform into a series of bigrams with permutations of
    /// the involved base-keys and modifers. However, the base-key will always be after its modifier.
    fn split_bigram_modifiers(&self, bigrams: BigramIndicesVec, layout: &Layout) -> BigramIndices {
        let mut bigram_w_map = AHashMap::with_capacity(bigrams.len() / 3);

        bigrams.into_iter().for_each(|((k1, k2), w)| {
            let (base1, mods1) = layout.resolve_modifiers(&k1);
            let (base2, mods2) = layout.resolve_modifiers(&k2);

            bigram_w_map.insert_or_add_weight((base1, base2), w);

            mods1.iter().for_each(|mod1| {
                // mix mods of k1 with base of k2
                bigram_w_map.insert_or_add_weight((*mod1, base2), w * 0.5);

                // mix mods of k1 and k2
                mods2.iter().for_each(|mod2| {
                    if mod1 != mod2 {
                        bigram_w_map.insert_or_add_weight((*mod1, *mod2), w);
                    }
                });
            });

            mods2.iter().for_each(|mod2| {
                // mix mods of k2 with base of k1
                bigram_w_map.insert_or_add_weight((base1, *mod2), w * 2.0);
            });

            // same key mods
            TakeTwoLayerKey::new(base1, &mods1, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    bigram_w_map.insert_or_add_weight(e, w);
                });

            TakeTwoLayerKey::new(base2, &mods2, w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    bigram_w_map.insert_or_add_weight(e, w);
                });

            // log::debug!(
            //     "{:>3}{:<3} -> {}",
            //     k1.symbol.escape_debug().to_string(),
            //     k2.symbol.escape_debug().to_string(),
            //     v.iter()
            //         .map(|((t1, t2), w)| format!(
            //             "{}{} (weight: {:>12.3}) ",
            //             t1.symbol.escape_debug(),
            //             t2.symbol.escape_debug(),
            //             w
            //         ))
            //         .collect::<String>(),
            // );
        });

        bigram_w_map
    }
}
