//! This module provides an implementation of bigram mapping functionalities
//! used by the `OnDemandNgramMapper`.
//!
//! Note: In contrast to ArneBab's algorithm, here all trigrams will be used
//! for secondary bigrams. Not only those that lead to same-hand bigrams.

use super::BigramIndices;
use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};

use crate::ngrams::Bigrams;

use keyboard_layout::layout::{LayerKey, Layout};

use rustc_hash::FxHashMap;
use serde::Deserialize;

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
    bigram_keys: &BigramIndices,
    config: &IncreaseCommonBigramsConfig,
) -> BigramIndices {
    if !config.enabled {
        return bigram_keys.to_vec();
    }

    // here we need to collect all mapped trigrams per key in order to successfully increase weights in next step
    let mut m = FxHashMap::default();
    bigram_keys.iter().for_each(|(k, w)| {
        *m.entry(*k).or_insert(0.0) += *w;
    });

    let total_weight: f64 = m.values().sum();
    let critical_point = config.critical_fraction * total_weight;

    m.iter_mut().for_each(|(_c, weight)| {
        let mut new_weight = *weight;
        if *weight > critical_point && total_weight > config.total_weight_threshold {
            new_weight += (new_weight - critical_point) * (config.factor - 1.0);
        }
        *weight = new_weight;
    });
    m.into_iter().collect()
}

fn layerkey_indices(bigrams: &Bigrams, layout: &Layout) -> (BigramIndices, f64) {
    let mut not_found_weight = 0.0;
    let mut bigram_keys = Vec::with_capacity(bigrams.grams.len());

    bigrams
        .grams
        .iter()
        //.filter(|((c1, c2), _weight)| !c1.is_whitespace() && !c2.is_whitespace())
        .for_each(|((c1, c2), weight)| {
            let layerkey1 = match layout.get_layerkey_index_for_symbol(c1) {
                Some(k) => k,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };
            let layerkey2 = match layout.get_layerkey_index_for_symbol(c2) {
                Some(k) => k,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            bigram_keys.push(((layerkey1, layerkey2), *weight));
        });

    (bigram_keys, not_found_weight)
}

/// Generates `LayerKey`-based bigrams from char-based unigrams. Optionally resolves modifiers
/// for higher-layer symbols of the layout.
#[derive(Clone, Debug)]
pub struct OnDemandBigramMapper {
    bigrams: Bigrams,
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandBigramMapper {
    pub fn new(bigrams: &Bigrams, split_modifiers: SplitModifiersConfig) -> Self {
        Self {
            bigrams: bigrams.clone(),
            split_modifiers,
        }
    }

    /// For a given `Layout` generate `LayerKeyIndex`-based unigrams, optionally resolving modifiers for higer-layer symbols.
    pub fn layerkey_indices(&self, layout: &Layout) -> (BigramIndices, f64, f64) {
        // println!("Before split: {:?}", self.bigrams.grams.get(&('l', 'r')));
        let (mut bigram_keys, not_found_weight) = layerkey_indices(&self.bigrams, layout);

        if self.split_modifiers.enabled {
            bigram_keys = self.split_bigram_modifiers(&bigram_keys, layout);
        }

        let found_weight = bigram_keys.iter().map(|(_, w)| w).sum();
        // bigram_keys
        //     .iter()
        //     .filter(|((c1, c2), _)| c1.symbol == 'l' && c2.symbol == 'r')
        //     .for_each(|((_, _), w)| {
        //         println!("After split: {}", w);
        //     });

        (bigram_keys, found_weight, not_found_weight)
    }

    /// Resolve `&LayerKey` references for `LayerKeyIndex`
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
    fn split_bigram_modifiers(
        &self,
        bigrams: &BigramIndices,
        layout: &Layout,
    ) -> BigramIndices {
        let mut bigram_keys = Vec::with_capacity(2 * bigrams.len());

        bigrams.iter().for_each(|((k1, k2), w)| {
            let (base1, mods1) = layout.resolve_modifiers(k1);
            let (base2, mods2) = layout.resolve_modifiers(k2);

            bigram_keys.push(((base1, base2), *w));

            mods1.iter().for_each(|mod1| {
                // mix mods of k1 with base of k2
                bigram_keys.push(((*mod1, base2), *w * 0.5));

                // mix mods of k1 and k2
                mods2.iter().for_each(|mod2| {
                    if mod1 != mod2 {
                        bigram_keys.push(((*mod1, *mod2), *w));
                    }
                });
            });

            mods2.iter().for_each(|mod2| {
                // mix mods of k2 with base of k1
                bigram_keys.push(((base1, *mod2), *w * 2.0));
            });

            // same key mods
            TakeTwoLayerKey::new(base1, &mods1, *w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    bigram_keys.push((e, w));
                });

            TakeTwoLayerKey::new(base2, &mods2, *w, self.split_modifiers.same_key_mod_factor)
                .for_each(|(e, w)| {
                    bigram_keys.push((e, w));
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

        bigram_keys
    }
}
