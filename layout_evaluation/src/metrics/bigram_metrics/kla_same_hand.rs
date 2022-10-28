use super::BigramMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{
    key::{Finger, Hand, HandMap},
    layout::{LayerKey, LayerKeyIndex, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub ignore_modifiers: bool,
    pub ignore_thumb: bool,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLASameHand {
    ignore_modifiers: bool,
    ignore_thumb: bool,
    hscoring: HandMap<f64>,
}

impl KLASameHand {
    pub fn new(params: &Parameters) -> Self {
        Self {
            ignore_modifiers: params.ignore_modifiers,
            ignore_thumb: params.ignore_thumb,
            hscoring: HandMap::with_hashmap(&params.hscoring, 1.0),
        }
    }
}

impl BigramMetric for KLASameHand {
    fn name(&self) -> &str {
        "Same Hand"
    }

    fn total_cost(
        &self,
        bigrams: &[((&LayerKey, &LayerKey), f64)],
        _total_weight: Option<f64>,
        layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut hand_values: HandMap<f64> = HandMap::with_default(0.0);

        bigrams.iter().for_each(|((prev_key, curr_key), weight)| {
            let prev_mods: AHashSet<LayerKeyIndex> =
                prev_key.modifiers.layerkeys().iter().cloned().collect();
            let curr_mods: AHashSet<LayerKeyIndex> =
                curr_key.modifiers.layerkeys().iter().cloned().collect();

            let mut prev_hands_used: HandMap<bool> = HandMap::with_default(false);
            if !(self.ignore_thumb && prev_key.key.finger == Finger::Thumb) {
                prev_hands_used.set(&prev_key.key.hand, true);
            }
            if !self.ignore_modifiers {
                prev_mods
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| prev_hands_used.set(&k.key.hand, true));
            }

            let mut curr_hands_used: HandMap<bool> = HandMap::with_default(false);
            if !(self.ignore_thumb && curr_key.key.finger == Finger::Thumb) {
                curr_hands_used.set(&curr_key.key.hand, true);
            }
            if !self.ignore_modifiers {
                curr_mods
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| curr_hands_used.set(&k.key.hand, true));
            }

            // check consecutive hands
            prev_hands_used
                .iter()
                .zip(curr_hands_used.iter())
                .zip(curr_hands_used.keys())
                .for_each(|((prev_used, curr_used), hand)| {
                    if *prev_used && *curr_used {
                        *hand_values.get_mut(&hand) += *weight;
                    }
                });
        });

        let total_weight: f64 = hand_values.iter().sum();

        let message = format!(
            "Finger loads %: {:>4.1} - {:<4.1}",
            100.0 * hand_values.get(&Hand::Left) / total_weight,
            100.0 * hand_values.get(&Hand::Right) / total_weight,
        );

        let cost = hand_values
            .iter()
            .zip(hand_values.keys().iter())
            .map(|(c, hand)| c * self.hscoring.get(&hand))
            .sum::<f64>();

        (cost, Some(message))
    }
}
