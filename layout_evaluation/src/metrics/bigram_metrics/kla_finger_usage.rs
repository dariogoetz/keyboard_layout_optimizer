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
pub struct KLAFingerUsage {
    ignore_modifiers: bool,
    fscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLAFingerUsage {
    pub fn new(params: &Parameters) -> Self {
        Self {
            ignore_modifiers: params.ignore_modifiers,
            fscoring: HandFingerMap::with_hashmap(&params.fscoring, 1.0),
            hscoring: HandMap::with_hashmap(&params.hscoring, 1.0),
        }
    }
}

impl BigramMetric for KLAFingerUsage {
    fn name(&self) -> &str {
        "Finger Usage"
    }

    fn total_cost(
        &self,
        bigrams: &[((&LayerKey, &LayerKey), f64)],
        _total_weight: Option<f64>,
        layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut finger_values: HandFingerMap<f64> = HandFingerMap::with_default(0.0);

        bigrams.iter().for_each(|((prev_key, curr_key), weight)| {
            *finger_values.get_mut(&curr_key.key.hand, &curr_key.key.finger) += *weight;

            if !self.ignore_modifiers {
                let prev_mods: AHashSet<LayerKeyIndex> = prev_key
                    .modifiers
                    .layerkey_indices()
                    .iter()
                    .cloned()
                    .collect();
                let curr_mods: AHashSet<LayerKeyIndex> = curr_key
                    .modifiers
                    .layerkey_indices()
                    .iter()
                    .cloned()
                    .collect();

                let pressed_mods = curr_mods
                    .difference(&prev_mods)
                    .map(|k| layout.get_layerkey(k));

                pressed_mods
                    .for_each(|k| *finger_values.get_mut(&k.key.hand, &k.key.finger) += *weight);
            }
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
