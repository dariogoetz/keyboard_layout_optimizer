use super::BigramMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap},
    layout::{LayerKey, LayerKeyIndex, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub ignore_modifiers: bool,
    pub fscoring: AHashMap<Hand, AHashMap<Finger, f64>>,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLASameFinger {
    ignore_modifiers: bool,
    fscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLASameFinger {
    pub fn new(params: &Parameters) -> Self {
        Self {
            ignore_modifiers: params.ignore_modifiers,
            fscoring: HandFingerMap::with_hashmap(&params.fscoring, 1.0),
            hscoring: HandMap::with_hashmap(&params.hscoring, 1.0),
        }
    }
}

impl BigramMetric for KLASameFinger {
    fn name(&self) -> &str {
        "Same Finger"
    }

    fn total_cost(
        &self,
        bigrams: &[((&LayerKey, &LayerKey), f64)],
        _total_weight: Option<f64>,
        layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut finger_values: HandFingerMap<f64> = HandFingerMap::with_default(0.0);

        bigrams.iter().for_each(|((prev_key, curr_key), weight)| {
            let prev_mods: AHashSet<LayerKeyIndex> =
                prev_key.modifiers.layerkeys().iter().cloned().collect();
            let curr_mods: AHashSet<LayerKeyIndex> =
                curr_key.modifiers.layerkeys().iter().cloned().collect();

            let mut prev_fingers_used: HandFingerMap<Option<(&LayerKey, bool)>> =
                HandFingerMap::with_default(None);
            prev_fingers_used.set(
                &prev_key.key.hand,
                &prev_key.key.finger,
                Some((prev_key, false)),
            );
            if !self.ignore_modifiers {
                prev_mods
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| {
                        prev_fingers_used.set(&k.key.hand, &k.key.finger, Some((k, true)))
                    });
            }

            let mut curr_fingers_used: HandFingerMap<Option<(&LayerKey, bool)>> =
                HandFingerMap::with_default(None);
            curr_fingers_used.set(
                &curr_key.key.hand,
                &curr_key.key.finger,
                Some((curr_key, false)),
            );
            if !self.ignore_modifiers {
                curr_mods
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| {
                        curr_fingers_used.set(&k.key.hand, &k.key.finger, Some((k, true)))
                    });
            }

            // check for same finger activations
            prev_fingers_used
                .iter()
                .zip(curr_fingers_used.iter())
                .zip(curr_fingers_used.keys())
                .for_each(|((prev_used, curr_used), (hand, finger))| {
                    if let (
                        Some((prev_used_key, prev_used_is_mod)), // prev finger was used
                        Some((curr_used_key, curr_used_is_mod)), // curr finger is used
                    ) = (prev_used, curr_used)
                    {
                        if prev_used_key != curr_used_key // used for a different key (same key and modifier would be a hold)
                            || !prev_used_is_mod // or one of prev...
                            || !curr_used_is_mod
                        // or current key is a modifier
                        {
                            *finger_values.get_mut(&hand, &finger) += *weight;
                        }
                    }
                });
        });

        let total_weight: f64 = finger_values.iter().sum();

        let message = format!(
            "Finger values %: {:4.1} {:4.1} {:4.1} {:4.1} | {:>4.1} - {:<4.1} | {:4.1} {:4.1} {:4.1} {:4.1}",
            100.0 * finger_values.get(&Hand::Left, &Finger::Pinky) / total_weight,
            100.0 * finger_values.get(&Hand::Left, &Finger::Ring) / total_weight,
            100.0 * finger_values.get(&Hand::Left, &Finger::Middle) / total_weight,
            100.0 * finger_values.get(&Hand::Left, &Finger::Index) / total_weight,
            100.0 * finger_values.get(&Hand::Left, &Finger::Thumb) / total_weight,
            100.0 * finger_values.get(&Hand::Right, &Finger::Thumb) / total_weight,
            100.0 * finger_values.get(&Hand::Right, &Finger::Index) / total_weight,
            100.0 * finger_values.get(&Hand::Right, &Finger::Middle) / total_weight,
            100.0 * finger_values.get(&Hand::Right, &Finger::Ring) / total_weight,
            100.0 * finger_values.get(&Hand::Right, &Finger::Pinky) / total_weight,
        );

        let cost = finger_values
            .iter()
            .zip(finger_values.keys().iter())
            .map(|(c, (hand, finger))| {
                let fscore = self.fscoring.get(&hand, &finger);
                let hscore = self.hscoring.get(&hand);
                c * fscore * hscore
            })
            .sum::<f64>();

        (cost, Some(message))
    }
}
