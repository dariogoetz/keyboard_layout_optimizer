//! This module provides an implementation of unigram mapping functionalities
//! used by the [`OnDemandNgramMapper`].

use super::on_demand_ngram_mapper::SplitModifiersConfig;
use super::{common::*, UnigramIndices};

use crate::ngrams::Unigrams;

use keyboard_layout::layout::{LayerKey, LayerKeyIndex, Layout};

fn mapped_unigrams(unigrams: &Unigrams, layout: &Layout) -> (UnigramIndices, f64) {
    let mut unigram_keys = Vec::with_capacity(unigrams.grams.len());
    let mut not_found_weight = 0.0;
    unigrams
        .grams
        .iter()
        //.filter(|(c, _weight)| !c.is_whitespace())
        .for_each(|(c, weight)| {
            let layerkey = match layout.get_layerkey_index_for_symbol(c) {
                Some(k) => k,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            unigram_keys.push((layerkey, *weight));
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
        let (mut unigram_keys, not_found_weight) = mapped_unigrams(&self.unigrams, layout);

        if self.split_modifiers.enabled {
            unigram_keys = Self::split_unigram_modifiers(&unigram_keys, layout);
        }

        let found_weight = unigram_keys.iter().map(|(_, w)| w).sum();

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
    fn split_unigram_modifiers(unigrams: &UnigramIndices, layout: &Layout) -> UnigramIndices {
        unigrams
            .iter()
            .flat_map(|(k, w)| {
                let (base, mods) = layout.resolve_modifiers(k);

                TakeOneLayerKey::new(base, &mods, *w).collect::<Vec<(LayerKeyIndex, f64)>>()

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
            })
            .collect()
    }
}
