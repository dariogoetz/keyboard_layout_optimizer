//! This module provides an implementation of unigram mapping functionalities
//! used by the [`OnDemandNgramMapper`].

use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};

use crate::ngrams::Unigrams;

use ahash::AHashMap;
use keyboard_layout::layout::{LayerKey, LayerKeyIndex, LayerModifiers, Layout};

// Before passing the resulting LayerKey-based ngrams as a result, smaller LayerKeyIndex-based
// ones are used because they are smaller than a reference (u16 vs usize) and yield better
// hashing performance.
type UnigramIndices = AHashMap<LayerKeyIndex, f64>;
type UnigramIndicesVec = Vec<(LayerKeyIndex, f64)>;

/// Turns the [`Unigrams`]'s characters into their indices, returning a [`UnigramIndicesVec`].
fn map_unigrams(unigrams: &Unigrams, layout: &Layout) -> (UnigramIndicesVec, f64) {
    let mut not_found_weight = 0.0;
    let mut unigrams_vec = Vec::with_capacity(unigrams.grams.len());

    unigrams_vec.extend(
        unigrams
            .grams
            .iter()
            //.filter(|(c, _weight)| !c.is_whitespace())
            .filter_map(|(c, weight)| {
                let layerkeyidx = match layout.get_layerkey_index_for_symbol(c) {
                    Some(idx) => idx,
                    None => {
                        not_found_weight += *weight;
                        return None;
                    }
                };

                Some((layerkeyidx, *weight))
            }),
    );

    (unigrams_vec, not_found_weight)
}

/// Generates [`LayerKey`]-based unigrams from char-based unigrams. Optionally resolves modifiers
/// for higher-layer symbols of the layout.
#[derive(Clone, Debug)]
pub struct OnDemandUnigramMapper {
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandUnigramMapper {
    pub fn new(split_modifiers: SplitModifiersConfig) -> Self {
        Self { split_modifiers }
    }

    /// For a given [`Layout`] generate [`LayerKeyIndex`]-based unigrams, optionally resolving modifiers for higer-layer symbols.
    pub fn layerkey_indices(&self, unigrams: &Unigrams, layout: &Layout) -> (UnigramIndices, f64) {
        let (mut unigram_keys_vec, not_found_weight) = map_unigrams(unigrams, layout);

        if layout.has_one_shot_layers() {
            unigram_keys_vec = self.process_one_shot_modifiers(unigram_keys_vec, layout);
        }

        let unigram_keys = if self.split_modifiers.enabled && layout.has_hold_layers() {
            Self::process_hold_modifiers(unigram_keys_vec, layout)
        } else {
            unigram_keys_vec.into_iter().collect()
        };

        (unigram_keys, not_found_weight)
    }

    /// Resolve &[`LayerKey`] references for [`LayerKeyIndex`]
    pub fn get_layerkeys<'s>(
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
    fn process_hold_modifiers(unigrams: UnigramIndicesVec, layout: &Layout) -> UnigramIndices {
        let mut idx_w_map = AHashMap::with_capacity(unigrams.len() / 3);
        unigrams.into_iter().for_each(|(k, w)| {
            let (base, mods) = layout.resolve_modifiers(&k);

            let (key, mods) = match mods {
                LayerModifiers::Hold(mods) => (base, mods),
                _ => (k, Vec::new()),
            };

            // Make sure we don't have any duplicate unigrams by adding them up.
            TakeOneLayerKey::new(key, &mods, w)
                .for_each(|(idx, w)| idx_w_map.insert_or_add_weight(idx, w));

            // if base.symbol == ' ' {
            // println!(
            //     "{:>3} -> {}",
            //     k,
            //     v.iter()
            //         .map(|(t1, w)| format!(
            //             "{:>3} (weight: {:>12.2}) ",
            //             t1,
            //             w
            //         ))
            //         .collect::<String>(),
            // );
            // }
        });

        idx_w_map
    }

    fn process_one_shot_modifiers(
        &self,
        unigrams: UnigramIndicesVec,
        layout: &Layout,
    ) -> UnigramIndicesVec {
        let mut processed_unigrams = Vec::with_capacity(unigrams.len());

        unigrams.into_iter().for_each(|(k, w)| {
            let (base, mods) = layout.resolve_modifiers(&k);
            if let LayerModifiers::OneShot(mods) = mods {
                processed_unigrams.extend(mods.iter().map(|m| (*m, w)));
                processed_unigrams.push((base, w));
            } else {
                processed_unigrams.push((k, w));
            }
        });

        processed_unigrams
    }
}
