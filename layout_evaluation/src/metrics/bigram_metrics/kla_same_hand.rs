use super::BigramMetric;

use ahash::AHashMap;
use keyboard_layout::{
    key::{Finger, Hand, HandMap},
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub ignore_modifiers: bool,
    pub ignore_thumbs: bool,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLASameHand {
    ignore_modifiers: bool,
    ignore_thumbs: bool,
    hscoring: HandMap<f64>,
}

impl KLASameHand {
    pub fn new(params: &Parameters) -> Self {
        Self {
            ignore_modifiers: params.ignore_modifiers,
            ignore_thumbs: params.ignore_thumbs,
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
            let mut prev_hands_used: HandMap<bool> = HandMap::with_default(false);
            if !(self.ignore_thumbs && prev_key.key.finger == Finger::Thumb) {
                prev_hands_used.set(&prev_key.key.hand, true);
            }
            if !self.ignore_modifiers {
                prev_key
                    .modifiers
                    .layerkey_indices()
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| prev_hands_used.set(&k.key.hand, true));
            }

            let mut curr_hands_used: HandMap<bool> = HandMap::with_default(false);
            if !(self.ignore_thumbs && curr_key.key.finger == Finger::Thumb) {
                curr_hands_used.set(&curr_key.key.hand, true);
            }
            if !self.ignore_modifiers {
                curr_key
                    .modifiers
                    .layerkey_indices()
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| curr_hands_used.set(&k.key.hand, true));
            }

            // check consecutive hands
            prev_hands_used
                .iter()
                .zip(curr_hands_used.iter())
                .zip(HandMap::<f64>::keys())
                .for_each(|((prev_used, curr_used), hand)| {
                    if *prev_used && *curr_used {
                        *hand_values.get_mut(&hand) += *weight;
                    }
                });
        });

        let message = format!(
            "Per hand (unweighted): {:>4.1} - {:<4.1}",
            hand_values.get(&Hand::Left),
            hand_values.get(&Hand::Right),
        );

        hand_values
            .iter_mut()
            .zip(HandMap::<f64>::keys().iter())
            .for_each(|(c, hand)| {
                let hscore = self.hscoring.get(hand);
                *c *= hscore
            });

        let cost = hand_values.iter().sum();

        (cost, Some(message))
    }
}
