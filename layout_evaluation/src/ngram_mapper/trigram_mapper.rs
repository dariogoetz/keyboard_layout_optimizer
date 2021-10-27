use super::TrigramIndices;
use super::{common::*, on_demand_ngram_mapper::SplitModifiersConfig};

use crate::ngrams::Trigrams;

use keyboard_layout::layout::{LayerKey, LayerKeyIndex, Layout};

fn mapped_trigrams<'s>(trigrams: &Trigrams, layout: &'s Layout) -> (TrigramIndices, f64) {
    let mut not_found_weight = 0.0;
    let mut trigram_keys = Vec::with_capacity(trigrams.grams.len());

    trigrams
        .grams
        .iter()
        //.filter(|((c1, c2, c3), _weight)| {
        //    !c1.is_whitespace() && !c2.is_whitespace() && !c3.is_whitespace()
        //})
        .for_each(|((c1, c2, c3), weight)| {
            let key1 = match layout.get_layerkey_index_for_symbol(c1) {
                Some(k) => k,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            let key2 = match layout.get_layerkey_index_for_symbol(c2) {
                Some(k) => k,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            let key3 = match layout.get_layerkey_index_for_symbol(c3) {
                Some(k) => k,
                None => {
                    not_found_weight += *weight;
                    return;
                }
            };

            trigram_keys.push(((key1, key2, key3), *weight));
        });

    (trigram_keys, not_found_weight)
}

#[derive(Clone, Debug)]
pub struct OnDemandTrigramMapper {
    trigrams: Trigrams,
    split_modifiers: SplitModifiersConfig,
}

impl OnDemandTrigramMapper {
    pub fn new(trigrams: &Trigrams, split_modifiers: SplitModifiersConfig) -> Self {
        Self {
            trigrams: trigrams.clone(),
            split_modifiers,
        }
    }

    pub fn layerkey_indices(&self, layout: &Layout) -> (TrigramIndices, f64, f64) {
        let (mut trigram_keys, not_found_weight) = mapped_trigrams(&self.trigrams, layout);

        if self.split_modifiers.enabled {
            trigram_keys = self.split_trigram_modifiers(&trigram_keys, layout);
        }

        let found_weight = trigram_keys.iter().map(|(_, w)| w).sum();

        (trigram_keys, found_weight, not_found_weight)
    }

    pub fn layerkeys<'s>(
        trigrams: &[((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64)],
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

    fn split_trigram_modifiers<'s>(
        &self,
        trigrams: &[((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64)],
        layout: &'s Layout,
    ) -> TrigramIndices {
        let mut trigram_keys = Vec::with_capacity(2 * trigrams.len());
        trigrams.iter().for_each(|((k1, k2, k3), w)| {
            let (base1, mods1) = layout.resolve_modifiers(k1);
            let (base2, mods2) = layout.resolve_modifiers(k2);
            let (base3, mods3) = layout.resolve_modifiers(k3);

            take_one_layerkey(base1, &mods1, *w)
                .iter()
                .for_each(|(e1, _)| {
                    take_one_layerkey(base2, &mods2, *w)
                        .iter()
                        .for_each(|(e2, _)| {
                            take_one_layerkey(base3, &mods3, *w)
                                .iter()
                                .for_each(|(e3, _)| {
                                    if (*e1 != *e2) && (*e2 != *e3) {
                                        // log::trace!(
                                        //     "one each:                    {}{}{}",
                                        //     e1.symbol.escape_debug(),
                                        //     e2.symbol.escape_debug(),
                                        //     e3.symbol.escape_debug(),
                                        // );
                                        trigram_keys.push(((*e1, *e2, *e3), *w));
                                    }
                                });
                        });
                });

            take_two_layerkey(base1, &mods1, *w, self.split_modifiers.same_key_mod_factor)
                .iter()
                .for_each(|((e1, e2), w1)| {
                    take_one_layerkey(base2, &mods2, *w)
                        .iter()
                        .for_each(|(e3, _)| {
                            if (*e1 != *e2) && (*e2 != *e3) {
                                // log::trace!(
                                //     "two of first, one of second: {}{}{}",
                                //     e1.symbol.escape_debug(),
                                //     e2.symbol.escape_debug(),
                                //     e3.symbol.escape_debug(),
                                // );
                                trigram_keys.push(((*e1, *e2, *e3), *w1));
                            }
                        });
                });

            take_one_layerkey(base1, &mods1, *w)
                .iter()
                .for_each(|(e1, _)| {
                    take_two_layerkey(base2, &mods2, *w, self.split_modifiers.same_key_mod_factor)
                        .iter()
                        .for_each(|((e2, e3), w1)| {
                            if (*e1 != *e2) && (*e2 != *e3) {
                                // log::trace!(
                                //     "one of first, two of second: {}{}{}",
                                //     e1.symbol.escape_debug(),
                                //     e2.symbol.escape_debug(),
                                //     e3.symbol.escape_debug(),
                                // );
                                trigram_keys.push(((*e1, *e2, *e3), *w1));
                            }
                        });
                });

            take_two_layerkey(base2, &mods2, *w, self.split_modifiers.same_key_mod_factor)
                .iter()
                .for_each(|((e1, e2), w1)| {
                    take_one_layerkey(base3, &mods3, *w)
                        .iter()
                        .for_each(|(e3, _)| {
                            if (*e1 != *e2) && (*e2 != *e3) {
                                // log::trace!(
                                //     "two of second, one of third: {}{}{}",
                                //     e1.symbol.escape_debug(),
                                //     e2.symbol.escape_debug(),
                                //     e3.symbol.escape_debug(),
                                // );
                                trigram_keys.push(((*e1, *e2, *e3), *w1));
                            }
                        });
                });

            take_one_layerkey(base2, &mods2, *w)
                .iter()
                .for_each(|(e1, _)| {
                    take_two_layerkey(base3, &mods3, *w, self.split_modifiers.same_key_mod_factor)
                        .iter()
                        .for_each(|((e2, e3), w1)| {
                            if (*e1 != *e2) && (*e2 != *e3) {
                                // log::trace!(
                                //     "one of second, two of third: {}{}{}",
                                //     e1.symbol.escape_debug(),
                                //     e2.symbol.escape_debug(),
                                //     e3.symbol.escape_debug(),
                                // );
                                trigram_keys.push(((*e1, *e2, *e3), *w1));
                            }
                        });
                });

            take_three_layerkey(base1, &mods1, *w, self.split_modifiers.same_key_mod_factor)
                .into_iter()
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of first:              {}{}{}",
                    //     e.0.symbol.escape_debug(),
                    //     e.1.symbol.escape_debug(),
                    //     e.2.symbol.escape_debug(),
                    // );
                    trigram_keys.push((e, w));
                });

            take_three_layerkey(base2, &mods2, *w, self.split_modifiers.same_key_mod_factor)
                .into_iter()
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of second:             {}{}{}",
                    //     e.0.symbol.escape_debug(),
                    //     e.1.symbol.escape_debug(),
                    //     e.2.symbol.escape_debug(),
                    // );
                    trigram_keys.push((e, w));
                });

            take_three_layerkey(base3, &mods3, *w, self.split_modifiers.same_key_mod_factor)
                .into_iter()
                .for_each(|(e, w)| {
                    // log::trace!(
                    //     "three of third:              {}{}{}",
                    //     e.0.symbol.escape_debug(),
                    //     e.1.symbol.escape_debug(),
                    //     e.2.symbol.escape_debug(),
                    // );
                    trigram_keys.push((e, w));
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

        trigram_keys
    }
}
