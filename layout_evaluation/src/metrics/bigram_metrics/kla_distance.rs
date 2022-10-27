use super::BigramMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap, Position},
    layout::{LayerKey, LayerKeyIndex, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub keyup_distance: f64,
    pub keydown_distance: f64,
    pub dscoring: AHashMap<Hand, AHashMap<Finger, f64>>,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLADistance {
    keyup_distance: f64,
    keydown_distance: f64,
    dscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLADistance {
    pub fn new(params: &Parameters) -> Self {
        Self {
            keyup_distance: params.keyup_distance,
            keydown_distance: params.keydown_distance,
            dscoring: HandFingerMap::with_hashmap(&params.dscoring, 1.0),
            hscoring: HandMap::with_hashmap(&params.hscoring, 1.0),
        }
    }
}

impl BigramMetric for KLADistance {
    fn name(&self) -> &str {
        "Distance"
    }

    fn total_cost(
        &self,
        bigrams: &[((&LayerKey, &LayerKey), f64)],
        _total_weight: Option<f64>,
        layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut finger_values: HandFingerMap<f64> = HandFingerMap::with_default(0.0);

        // distance to either the previous key (if it was the same finger) or the finger's home-row key
        let dist_to_prev = |curr_key: &LayerKey, prev_positions: &HandFingerMap<Position>| {
            prev_positions
                .get(&curr_key.key.hand, &curr_key.key.finger)
                .distance(&curr_key.key.position)
        };

        bigrams.iter().for_each(|((prev_key, curr_key), weight)| {
            let prev_mods: AHashSet<LayerKeyIndex> =
                prev_key.modifiers.layerkeys().iter().cloned().collect();
            let curr_mods: AHashSet<LayerKeyIndex> =
                curr_key.modifiers.layerkeys().iter().cloned().collect();

            let mut prev_positions = layout.keyboard.home_row_positions.clone();
            prev_positions.set(
                &prev_key.key.hand,
                &prev_key.key.finger,
                prev_key.key.position,
            );
            prev_mods
                .iter()
                .map(|k| layout.get_layerkey(k))
                .for_each(|k| prev_positions.set(&k.key.hand, &k.key.finger, k.key.position));

            let released_mods = prev_mods
                .difference(&curr_mods)
                .map(|k| layout.get_layerkey(k));
            let pressed_mods = curr_mods
                .difference(&prev_mods)
                .map(|k| layout.get_layerkey(k));

            println!(
                "{}{} (weight: {})",
                prev_key.symbol.escape_debug(),
                curr_key.symbol.escape_debug(),
                weight
            );

            let cost = (dist_to_prev(&curr_key, &prev_positions)
                + self.keyup_distance
                + self.keydown_distance)
                * weight;
            *finger_values.get_mut(&curr_key.key.hand, &curr_key.key.finger) += cost;
            println!(
                "  key {}: {}",
                curr_key.symbol.escape_debug(),
                cost / weight
            );

            pressed_mods.for_each(|k| {
                let cost = (dist_to_prev(k, &prev_positions) + self.keydown_distance) * weight;
                println!(
                    "  pressed mod {}: {}",
                    k.symbol.escape_debug(),
                    cost / weight
                );
                *finger_values.get_mut(&k.key.hand, &k.key.finger) += cost;
            });

            released_mods.for_each(|k| {
                let cost = (dist_to_prev(k, &prev_positions) + self.keyup_distance) * weight;
                println!(
                    "  released mod {}: {}",
                    k.symbol.escape_debug(),
                    cost / weight
                );
                *finger_values.get_mut(&k.key.hand, &k.key.finger) += cost;
            });
            println!("");
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
                let fscore = self.dscoring.get(&hand, &finger);
                let hscore = self.hscoring.get(&hand);
                log::info!("{:?} {:?}: {}", hand, finger, l);
                l * fscore * hscore
            })
            .sum::<f64>();

        (cost, Some(message))
    }
}
