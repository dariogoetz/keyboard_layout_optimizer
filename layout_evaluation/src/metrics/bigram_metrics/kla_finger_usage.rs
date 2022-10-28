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
pub struct KLAFingerUsage {
    fscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLAFingerUsage {
    pub fn new(params: &Parameters) -> Self {
        Self {
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
        let mut finger_loads: HandFingerMap<f64> = HandFingerMap::with_default(0.0);

        bigrams.iter().for_each(|((prev_key, curr_key), weight)| {
            let prev_mods: AHashSet<LayerKeyIndex> =
                prev_key.modifiers.layerkeys().iter().cloned().collect();
            let curr_mods: AHashSet<LayerKeyIndex> =
                curr_key.modifiers.layerkeys().iter().cloned().collect();

            let pressed_mods = curr_mods
                .difference(&prev_mods)
                .map(|k| layout.get_layerkey(k));

            pressed_mods.for_each(|k| *finger_loads.get_mut(&k.key.hand, &k.key.finger) += *weight);

            *finger_loads.get_mut(&curr_key.key.hand, &curr_key.key.finger) += *weight;
        });

        let total_weight: f64 = finger_loads.iter().sum();

        let message = format!(
            "Finger loads %: {:4.1} {:4.1} {:4.1} {:4.1} | {:>4.1} - {:<4.1} | {:4.1} {:4.1} {:4.1} {:4.1}",
            100.0 * finger_loads.get(&Hand::Left, &Finger::Pinky) / total_weight,
            100.0 * finger_loads.get(&Hand::Left, &Finger::Ring) / total_weight,
            100.0 * finger_loads.get(&Hand::Left, &Finger::Middle) / total_weight,
            100.0 * finger_loads.get(&Hand::Left, &Finger::Index) / total_weight,
            100.0 * finger_loads.get(&Hand::Left, &Finger::Thumb) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Thumb) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Index) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Middle) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Ring) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Pinky) / total_weight,
        );

        let cost = finger_loads
            .iter()
            .zip(finger_loads.keys().iter())
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
