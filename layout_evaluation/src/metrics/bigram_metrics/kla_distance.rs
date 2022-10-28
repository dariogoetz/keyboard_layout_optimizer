use super::BigramMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap, Position},
    layout::{LayerKey, LayerKeyIndex, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub ignore_modifiers: bool,
    pub keyup_distance: f64,
    pub keydown_distance: f64,
    pub dscoring: AHashMap<Hand, AHashMap<Finger, f64>>,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLADistance {
    ignore_modifiers: bool,
    keyup_distance: f64,
    keydown_distance: f64,
    dscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLADistance {
    pub fn new(params: &Parameters) -> Self {
        Self {
            ignore_modifiers: params.ignore_modifiers,
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
            if !self.ignore_modifiers {
                prev_mods
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| prev_positions.set(&k.key.hand, &k.key.finger, k.key.position));
            }

            let mut curr_positions = layout.keyboard.home_row_positions.clone();
            curr_positions.set(
                &curr_key.key.hand,
                &curr_key.key.finger,
                curr_key.key.position,
            );
            if !self.ignore_modifiers {
                curr_mods
                    .iter()
                    .map(|k| layout.get_layerkey(k))
                    .for_each(|k| curr_positions.set(&k.key.hand, &k.key.finger, k.key.position));
            }

            // finger goes to current key and presses and releases it
            let dist_to_key = (dist_to_prev(&curr_key, &prev_positions)
                + self.keydown_distance
                + self.keyup_distance)
                * weight;
            *finger_values.get_mut(&curr_key.key.hand, &curr_key.key.finger) += dist_to_key;

            if curr_key.key.hand != prev_key.key.hand || curr_key.key.finger != prev_key.key.finger
            {
                // if the previous key was hit by a different finger,
                // that finger returns to the home row (or some mod)
                let dist_return = dist_to_prev(&prev_key, &curr_positions) * weight;
                *finger_values.get_mut(&prev_key.key.hand, &prev_key.key.finger) += dist_return;
            }

            if !self.ignore_modifiers {
                let released_mods = prev_mods
                    .difference(&curr_mods)
                    .map(|k| layout.get_layerkey(k));
                let pressed_mods = curr_mods
                    .difference(&prev_mods)
                    .map(|k| layout.get_layerkey(k));

                // fingers move to the modifiers (if they did not hit the previous key before)
                pressed_mods
                    // if the finger pressed the previous key, the movement has been accounted for above
                    .filter(|k| {
                        k.key.hand != prev_key.key.hand || k.key.finger != prev_key.key.finger
                    })
                    .for_each(|k| {
                        let dist = (dist_to_prev(k, &prev_positions)
                            + self.keydown_distance
                            + self.keyup_distance)
                            * weight;
                        *finger_values.get_mut(&k.key.hand, &k.key.finger) += dist;
                    });

                // fingers from previously pressed mods return to home row
                // (if they are not used to hit the current key)
                released_mods
                    // if the finger will press the current key, the movement has been accounted for above
                    .filter(|k| {
                        k.key.hand != curr_key.key.hand || k.key.finger != curr_key.key.finger
                    })
                    .for_each(|k| {
                        let dist_to_homerow = dist_to_prev(k, &curr_positions) * weight;
                        *finger_values.get_mut(&k.key.hand, &k.key.finger) += dist_to_homerow;
                    });
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

        let keys = finger_values.keys();
        finger_values
            .iter_mut()
            .zip(keys.iter())
            .for_each(|(c, (hand, finger))| {
                let fscore = self.dscoring.get(&hand, &finger);
                let hscore = self.hscoring.get(&hand);
                *c *= fscore * hscore
            });

        let cost = finger_values.iter().sum();

        (cost, Some(message))
    }
}
