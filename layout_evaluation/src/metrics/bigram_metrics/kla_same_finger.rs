use super::BigramMetric;

use ahash::AHashMap;
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap},
    layout::{LayerKey, Layout},
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
            // collect used fingers and keys for previous symbol
            let mut prev_keys_per_finger: HandFingerMap<Option<&LayerKey>> =
                HandFingerMap::with_default(None);
            prev_keys_per_finger.set(&prev_key.key.hand, &prev_key.key.finger, Some(prev_key));
            if !self.ignore_modifiers {
                prev_key
                    .modifiers
                    .layerkeys()
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| prev_keys_per_finger.set(&k.key.hand, &k.key.finger, Some(k)));
            }

            // collect used fingers and keys for current symbol
            let mut curr_keys_per_finger: HandFingerMap<Option<&LayerKey>> =
                HandFingerMap::with_default(None);
            curr_keys_per_finger.set(&curr_key.key.hand, &curr_key.key.finger, Some(curr_key));
            if !self.ignore_modifiers {
                curr_key
                    .modifiers
                    .layerkeys()
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| curr_keys_per_finger.set(&k.key.hand, &k.key.finger, Some(k)));
            }

            // check for same finger activations
            prev_keys_per_finger
                .iter()
                .zip(curr_keys_per_finger.iter())
                .for_each(|(prev_used, curr_used)| {
                    if let (
                        Some(prev_used_key), // finger was used for previous symbol
                        Some(curr_used_key), // and finger was used for current symbol
                    ) = (prev_used, curr_used)
                    {
                        // if both keys are identical and are mods it is a hold -> no cost
                        if !(prev_used_key == curr_used_key && curr_used_key.is_modifier.is_some())
                        {
                            *finger_values
                                .get_mut(&curr_used_key.key.hand, &curr_used_key.key.finger) +=
                                *weight;
                        }
                    }
                });
        });

        let message = format!(
            "Per finger (unweighted): {:4.1} {:4.1} {:4.1} {:4.1} | {:>4.1} - {:<4.1} | {:4.1} {:4.1} {:4.1} {:4.1}",
            finger_values.get(&Hand::Left, &Finger::Pinky),
            finger_values.get(&Hand::Left, &Finger::Ring),
            finger_values.get(&Hand::Left, &Finger::Middle),
            finger_values.get(&Hand::Left, &Finger::Index),
            finger_values.get(&Hand::Left, &Finger::Thumb),
            finger_values.get(&Hand::Right, &Finger::Thumb),
            finger_values.get(&Hand::Right, &Finger::Index),
            finger_values.get(&Hand::Right, &Finger::Middle),
            finger_values.get(&Hand::Right, &Finger::Ring),
            finger_values.get(&Hand::Right, &Finger::Pinky),
        );

        finger_values
            .iter_mut()
            .zip(HandFingerMap::<f64>::keys().iter())
            .for_each(|(c, (hand, finger))| {
                let fscore = self.fscoring.get(hand, finger);
                let hscore = self.hscoring.get(hand);
                *c *= fscore * hscore
            });

        let cost = finger_values.iter().sum();

        (cost, Some(message))
    }
}
