use super::BigramMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap},
    layout::{LayerKey, LayerKeyIndex, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub fscoring: AHashMap<Hand, AHashMap<Finger, f64>>,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLASameFinger {
    fscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLASameFinger {
    pub fn new(params: &Parameters) -> Self {
        Self {
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

        let is_same_finger = |k1: &LayerKey, k2: &LayerKey| {
            k1.key.hand == k2.key.hand && k1.key.finger == k2.key.finger
        };

        bigrams.iter().for_each(|((prev_key, curr_key), weight)| {
            let prev_mods: AHashSet<LayerKeyIndex> =
                prev_key.modifiers.layerkeys().iter().cloned().collect();
            let curr_mods: AHashSet<LayerKeyIndex> =
                curr_key.modifiers.layerkeys().iter().cloned().collect();

            // current key vs. prev key
            if is_same_finger(curr_key, prev_key) {
                *finger_values.get_mut(&curr_key.key.hand, &curr_key.key.finger) += *weight;
            }
            // current key vs. previous mods
            prev_mods
                .iter()
                .map(|k| layout.get_layerkey(k))
                .for_each(|prev_mod| {
                    if is_same_finger(curr_key, prev_mod) {
                        *finger_values.get_mut(&curr_key.key.hand, &curr_key.key.finger) += *weight;
                    }
                });

            curr_mods
                .iter()
                .map(|k| layout.get_layerkey(k))
                .for_each(|curr_mod| {
                    // current mod vs. previous key
                    if is_same_finger(curr_mod, prev_key) {
                        *finger_values.get_mut(&curr_mod.key.hand, &curr_mod.key.finger) += *weight;
                    }
                    // current mods vs. previous mods
                    prev_mods
                        .iter()
                        .map(|k| layout.get_layerkey(k))
                        .for_each(|prev_mod| {
                            if is_same_finger(curr_mod, prev_mod) {
                                *finger_values.get_mut(&curr_mod.key.hand, &curr_mod.key.finger) +=
                                    *weight;
                            }
                        });
                });
        });

        let total_weight: f64 = finger_values.iter().sum();

        let message = format!(
            "Finger values %: {:3.1} {:3.1} {:3.1} {:3.1} | {:3.1} - {:3.1} | {:3.1} {:3.1} {:3.1} {:3.1}",
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
            .map(|(l, (hand, finger))| {
                let fscore = self.fscoring.get(&hand, &finger);
                let hscore = self.hscoring.get(&hand);
                log::info!("{:?} {:?}: {}", hand, finger, l * fscore * hscore);
                l * fscore * hscore
            })
            .sum::<f64>();

        (cost, Some(message))
    }
}
