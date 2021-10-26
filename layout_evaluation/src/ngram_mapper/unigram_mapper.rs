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
            let layerkey = match layout.get_layerkey_index_for_char(c) {
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

#[derive(Clone, Debug)]
pub struct OnDemandUnigramMapper {
    unigrams: Unigrams,
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandUnigramMapper {
    pub fn new(unigrams: &Unigrams, split_modifiers: SplitModifiersConfig) -> Self {
        Self {
            unigrams: unigrams.clone(),
            split_modifiers,
        }
    }

    pub fn layerkey_indices(&self, layout: &Layout) -> (UnigramIndices, f64, f64) {
        let (mut unigram_keys, not_found_weight) = mapped_unigrams(&self.unigrams, layout);

        if self.split_modifiers.enabled {
            unigram_keys = Self::split_unigram_modifiers(&unigram_keys, layout);
        }

        let found_weight = unigram_keys.iter().map(|(_, w)| w).sum();

        (unigram_keys, found_weight, not_found_weight)
    }

    pub fn layerkeys<'s>(
        unigrams: &[(LayerKeyIndex, f64)],
        layout: &'s Layout,
    ) -> Vec<(&'s LayerKey, f64)> {
        unigrams
            .iter()
            .map(|(k1, w)| (layout.get_layerkey(k1), *w))
            .collect()
    }

    fn split_unigram_modifiers(
        unigrams: &[(LayerKeyIndex, f64)],
        layout: &Layout,
    ) -> UnigramIndices {
        unigrams
            .iter()
            .map(|(k, w)| {
                let (base, mods) = layout.resolve_modifiers(k);

                take_one_layerkey(base, &mods, *w)

                // if base.char == ' ' {
                // println!(
                //     "{:>3} -> {}",
                //     k.char.escape_debug().to_string(),
                //     v.iter()
                //         .map(|(t1, w)| format!(
                //             "{:>3} (weight: {:>12.2}) ",
                //             t1.char.escape_debug().to_string(),
                //             w
                //         ))
                //         .collect::<String>(),
                // );
                // }
            })
            .flatten()
            .collect()
    }
}
