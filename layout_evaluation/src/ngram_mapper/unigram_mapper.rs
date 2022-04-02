//! This module provides an implementation of unigram mapping functionalities
//! used by the [`OnDemandNgramMapper`].

use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};
use super::UnigramIndices;

use crate::ngrams::Unigrams;

use ahash::AHashMap;
use keyboard_layout::layout::{LayerKey, Layout};

/// Turns the [`Unigrams`]'s characters into their indices, returning a [`UnigramIndices`].
fn map_unigrams(unigrams: &Unigrams, layout: &Layout) -> (UnigramIndices, f64) {
    let mut unigram_keys = AHashMap::with_capacity(unigrams.grams.len());
    let mut not_found_weight = 0.0;
    unigrams
        .grams
        .iter()
        //.filter(|(c, _weight)| !c.is_whitespace())
        .for_each(|(c, weight)| {
            let layerkeyidx = match layout.get_layerkey_index_for_symbol(c) {
                Some(idx) => idx,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };
            unigram_keys.insert_or_add_weight(layerkeyidx, *weight);
        });

    (unigram_keys, not_found_weight)
}

/// Generates [`LayerKey`]-based unigrams from char-based unigrams. Optionally resolves modifiers
/// for higher-layer symbols of the layout.
#[derive(Clone, Debug)]
pub struct OnDemandUnigramMapper {
    unigrams: Unigrams,
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandUnigramMapper {
    pub fn new(unigrams: Unigrams, split_modifiers: SplitModifiersConfig) -> Self {
        Self {
            unigrams,
            split_modifiers,
        }
    }

    /// For a given [`Layout`] generate [`LayerKeyIndex`]-based unigrams, optionally resolving modifiers for higer-layer symbols.
    pub fn layerkey_indices(&self, layout: &Layout) -> (UnigramIndices, f64, f64) {
        let (unigram_keys, not_found_weight) = map_unigrams(&self.unigrams, layout);

        let unigram_keys = if self.split_modifiers.enabled {
            Self::split_unigram_modifiers(unigram_keys, layout)
        } else {
            unigram_keys
        };

        let found_weight: f64 = unigram_keys.values().sum();

        (unigram_keys, found_weight, not_found_weight)
    }

    /// Resolve &[`LayerKey`] references for [`LayerKeyIndex`]
    pub fn layerkeys<'s>(
        unigrams: &UnigramIndices,
        layout: &'s Layout,
    ) -> Vec<(&'s LayerKey, f64)> {
        unigrams
            .iter()
            .map(|(k1, w)| (layout.get_layerkey(k1), *w))
            .collect()
    }

    /// Map all unigrams to base-layer unigrams, potentially generating multiple unigrams
    /// with modifiers for those with higer-layer keys.
    ///
    /// Each unigram of a higher-layer symbol will transform into a unigram with the base-layer key and one
    /// for each modifier involved in accessing the higher layer.
    fn split_unigram_modifiers(unigrams: UnigramIndices, layout: &Layout) -> UnigramIndices {
        let mut idx_w_map = AHashMap::with_capacity(unigrams.len());
        unigrams.into_iter().for_each(|(k, w)| {
            let (base, mods) = layout.resolve_modifiers(&k);

            // Make sure we don't have any duplicate unigrams by adding them up.
            TakeOneLayerKey::new(base, &mods, w)
                .for_each(|(idx, w)| idx_w_map.insert_or_add_weight(idx, w));

            // if base.symbol == ' ' {
            // println!(
            //     "{:>3} -> {}",
            //     k.symbol.escape_debug().to_string(),
            //     v.iter()
            //         .map(|(t1, w)| format!(
            //             "{:>3} (weight: {:>12.2}) ",
            //             t1.symbol.escape_debug().to_string(),
            //             w
            //         ))
            //         .collect::<String>(),
            // );
            // }
        });

        idx_w_map
    }
}
