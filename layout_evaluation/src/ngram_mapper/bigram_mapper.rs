//! This module provides an implementation of bigram mapping functionalities
//! used by the [`OnDemandNgramMapper`].
//!
//! Note: In contrast to ArneBab's algorithm, here all trigrams will be used
//! for secondary bigrams. Not only those that lead to same-hand bigrams.

use super::BigramIndices;
use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};

use crate::ngrams::Bigrams;

use keyboard_layout::layout::{LayerKey, LayerKeyIndex, Layout};

use ahash::AHashMap;
use rustc_hash::FxHashSet;
use serde::Deserialize;

type BigramIndicesVec = Vec<((LayerKeyIndex, LayerKeyIndex), f64)>;

/// Configuration parameters for process of increasing the weight of common bigrams.
#[derive(Debug, Clone, Deserialize)]
pub struct IncreaseCommonBigramsConfig {
    /// Whether to increase the weight of common bigrams even further.
    pub enabled: bool,
    /// The critical fraction above which a bigram's weight will be increased.
    pub critical_fraction: f64,
    /// The slope with which the bigram's weight will be increased.
    /// The increment is performed linearly starting from the critical fraction,
    /// i.e. a bigram with weight equal the critical fraction is actually not affected.
    pub factor: f64,
    /// A minimum total weight (of all bigrams) that needs to be achieved. Otherwise no
    /// increment takes place.
    pub total_weight_threshold: f64,
}

impl Default for IncreaseCommonBigramsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            critical_fraction: 0.001,
            factor: 2.0,
            total_weight_threshold: 20.0,
        }
    }
}

/// Increase the weight of bigrams that already have a weight exceeding a threshold even further.
pub fn increase_common_bigrams(
    bigram_keys: &mut BigramIndices,
    config: &IncreaseCommonBigramsConfig,
) {
    if !config.enabled {
        return;
    }

    let total_weight: f64 = bigram_keys.values().sum();
    let critical_point = config.critical_fraction * total_weight;

    bigram_keys.values_mut().for_each(|weight| {
        if *weight > critical_point && total_weight > config.total_weight_threshold {
            *weight += (*weight - critical_point) * (config.factor - 1.0);
        }
    });
}

/// Configuration parameters for adding secondary bigrams from trigrams.
#[derive(Debug, Clone, Deserialize)]
pub struct SecondaryBigramsFromTrigramsConfig {
    /// Whether to add secondary bigrams from trigrams.
    pub enabled: bool,
    /// Factor to apply to a trigram's weight before assigning it to the secondary bigram if the trigram involves no handswitch.
    pub factor_no_handswitch: f64,
    /// Factor to apply to a trigram's weight before assigning it to the secondary bigram if the trigram involves a handswitch.
    pub factor_handswitch: f64,
    /// Exclude secondary bigrams for trigrams containing at least one of the given symbols
    pub exclude_containing: FxHashSet<char>,
}

impl Default for SecondaryBigramsFromTrigramsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            factor_no_handswitch: 0.7,
            factor_handswitch: 0.8,
            exclude_containing: FxHashSet::default(),
        }
    }
}

/// Add secondary bigrams from the first and third symbol of a trigram (if they belong to the same hand).
pub fn add_secondary_bigrams_from_trigrams(
    bigram_keys: &mut BigramIndices,
    trigram_keys: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
    config: &SecondaryBigramsFromTrigramsConfig,
) {
    if !config.enabled {
        return;
    }

    // there are many duplicates in the secondary bigrams -> using a hashmap is cheaper
    trigram_keys
        .iter()
        .filter(|((layerkey1, layerkey2, layerkey3), _)| {
            !config.exclude_containing.contains(&layerkey1.symbol)
                && !config.exclude_containing.contains(&layerkey2.symbol)
                && !config.exclude_containing.contains(&layerkey3.symbol)
        })
        .for_each(|((layerkey1, layerkey2, layerkey3), weight)| {
            let factor = if layerkey1.key.hand == layerkey2.key.hand
                && layerkey2.key.hand == layerkey3.key.hand
            {
                config.factor_no_handswitch
            } else {
                config.factor_handswitch
            };

            bigram_keys.insert_or_add_weight((layerkey1.index, layerkey3.index), weight * factor);
        });
}

/// Turns the [`Bigrams`]'s characters into their indices, returning a [`BigramIndicesVec`].
fn map_bigrams(
    bigrams: &Bigrams,
    layout: &Layout,
    exclude_line_breaks: bool,
) -> (BigramIndicesVec, f64) {
    let mut not_found_weight = 0.0;
    let bigrams_vec = bigrams
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
        })
        .collect();

    (bigrams_vec, not_found_weight)
}

/// Generates [`LayerKey`]-based [Bigrams] from char-based unigrams. Optionally resolves modifiers
/// for higher-layer symbols of the layout.
#[derive(Clone, Debug)]
pub struct OnDemandBigramMapper {
    bigrams: Bigrams,
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandBigramMapper {
    pub fn new(bigrams: Bigrams, split_modifiers: SplitModifiersConfig) -> Self {
        Self {
            bigrams,
            split_modifiers,
        }
    }

    /// For a given [`Layout`] generate [`LayerKeyIndex`]-based unigrams, optionally resolving modifiers for higer-layer symbols.
    pub fn layerkey_indices(
        &self,
        layout: &Layout,
        exclude_line_breaks: bool,
    ) -> (BigramIndices, f64, f64) {
        let (bigram_keys_vec, not_found_weight) =
            map_bigrams(&self.bigrams, layout, exclude_line_breaks);

        let bigram_keys = if self.split_modifiers.enabled {
            self.split_bigram_modifiers(bigram_keys_vec, layout)
        } else {
            bigram_keys_vec.into_iter().collect()
        };

        let found_weight: f64 = bigram_keys.values().sum();

        // bigram_keys
        //     .iter()
        //     .filter(|((c1, c2), _)| c1.symbol == 'l' && c2.symbol == 'r')
        //     .for_each(|((_, _), w)| {
        //         println!("After split: {}", w);
        //     });

        (bigram_keys, found_weight, not_found_weight)
    }

    /// Resolve &[`LayerKey`] references for [`LayerKeyIndex`]
    pub fn layerkeys<'s>(
        bigrams: &BigramIndices,
        layout: &'s Layout,
    ) -> Vec<((&'s LayerKey, &'s LayerKey), f64)> {
        bigrams
            .iter()
            .map(|((k1, k2), w)| ((layout.get_layerkey(k1), layout.get_layerkey(k2)), *w))
            .collect()
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
