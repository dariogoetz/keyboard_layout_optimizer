use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};
use super::ngrams::Bigrams;
use super::BigramIndices;

use keyboard_layout::layout::{LayerKey, LayerKeyIndex, Layout};

use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct IncreaseCommonBigramsConfig {
    pub enabled: bool,
    pub critical_fraction: f64,
    pub factor: f64,
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

pub fn increase_common_bigrams(
    bigram_keys: &[((LayerKeyIndex, LayerKeyIndex), f64)],
    config: &IncreaseCommonBigramsConfig
) -> BigramIndices {
    if !config.enabled {
        return bigram_keys.to_vec()
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

#[derive(Debug, Clone, Deserialize)]
pub struct SecondaryBigramsFromTrigramsConfig {
    pub enabled: bool,
    pub factor_no_handswitch: f64,
    pub factor_handswitch: f64,
}

impl Default for SecondaryBigramsFromTrigramsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            factor_no_handswitch: 0.7,
            factor_handswitch: 0.8,
        }
    }
}

pub fn add_secondary_bigrams_from_trigrams(
    bigram_keys: &mut BigramIndices,
    trigram_keys: &[((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64)],
    config: &SecondaryBigramsFromTrigramsConfig,
    layout: &Layout,
) {
    if !config.enabled {
        return
    }

    // there are many duplicates in the secondary bigrams -> using a hashmap is cheaper
    let mut m = FxHashMap::with_capacity_and_hasher(trigram_keys.len(), Default::default());
    trigram_keys
        .iter()
        .map(|((idx1, idx2, idx3), w)| {
            (
                (
                    layout.get_layerkey(idx1),
                    layout.get_layerkey(idx2),
                    layout.get_layerkey(idx3),
                ),
                w,
            )
        })
        .filter(|((layerkey1, _, layerkey3), _)| layerkey1.key.hand == layerkey3.key.hand)
        .for_each(|((layerkey1, layerkey2, layerkey3), weight)| {
            let factor = if layerkey1.key.hand == layerkey2.key.hand {
                config.factor_no_handswitch
            } else {
                config.factor_handswitch
            };

            *m.entry((layerkey1.index, layerkey3.index)).or_insert(0.0) += *weight * factor;
        });
    bigram_keys.extend(m);
}

fn layerkey_indices(bigrams: &Bigrams, layout: &Layout) -> (BigramIndices, f64) {
    let mut not_found_weight = 0.0;
    let mut bigram_keys = Vec::with_capacity(bigrams.grams.len());

    bigrams
        .grams
        .iter()
        //.filter(|((c1, c2), _weight)| !c1.is_whitespace() && !c2.is_whitespace())
        .for_each(|((c1, c2), weight)| {
            let layerkey1 = match layout.get_layerkey_index_for_char(c1) {
                Some(k) => k,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };
            let layerkey2 = match layout.get_layerkey_index_for_char(c2) {
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

    pub fn layerkey_indices(&self, layout: &Layout) -> (BigramIndices, f64, f64) {
        // println!("Before split: {:?}", self.bigrams.grams.get(&('l', 'r')));
        let (mut bigram_keys, not_found_weight) = layerkey_indices(&self.bigrams, layout);

        if self.split_modifiers.enabled {
            bigram_keys = self.split_bigram_modifiers(&bigram_keys, layout);
        }

        let found_weight = bigram_keys.iter().map(|(_, w)| w).sum();
        // bigram_keys
        //     .iter()
        //     .filter(|((c1, c2), _)| c1.char == 'l' && c2.char == 'r')
        //     .for_each(|((_, _), w)| {
        //         println!("After split: {}", w);
        //     });

        (bigram_keys, found_weight, not_found_weight)
    }

    pub fn layerkeys<'s>(
        bigrams: &[((LayerKeyIndex, LayerKeyIndex), f64)],
        layout: &'s Layout,
    ) -> Vec<((&'s LayerKey, &'s LayerKey), f64)> {
        bigrams
            .iter()
            .map(|((k1, k2), w)| ((layout.get_layerkey(k1), layout.get_layerkey(k2)), *w))
            .collect()
    }

    fn split_bigram_modifiers(
        &self,
        bigrams: &[((LayerKeyIndex, LayerKeyIndex), f64)],
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
            take_two_layerkey(base1, &mods1, *w, self.split_modifiers.same_key_mod_factor)
                .into_iter()
                .for_each(|(e, w)| {
                    bigram_keys.push((e, w));
                });

            take_two_layerkey(base2, &mods2, *w, self.split_modifiers.same_key_mod_factor)
                .into_iter()
                .for_each(|(e, w)| {
                    bigram_keys.push((e, w));
                });

            // log::debug!(
            //     "{:>3}{:<3} -> {}",
            //     k1.char.escape_debug().to_string(),
            //     k2.char.escape_debug().to_string(),
            //     v.iter()
            //         .map(|((t1, t2), w)| format!(
            //             "{}{} (weight: {:>12.3}) ",
            //             t1.char.escape_debug(),
            //             t2.char.escape_debug(),
            //             w
            //         ))
            //         .collect::<String>(),
            // );
        });

        bigram_keys
    }
}
