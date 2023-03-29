use core::slice;

use super::BigramMetric;

use ahash::AHashMap;
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap, Position},
    layout::{LayerKey, Layout},
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

#[derive(Copy, Clone, Debug)]
enum KeyUsage<'a> {
    Idle(Position),
    Used(&'a LayerKey),
}

#[derive(Copy, Clone, Debug)]
struct FingerStates<'a>(HandFingerMap<KeyUsage<'a>>);

impl<'a> FingerStates<'a> {
    fn with_positions(positions: &HandFingerMap<Position>) -> Self {
        let mut data = HandFingerMap::with_default(KeyUsage::Idle(Position(0.0, 0.0)));
        positions
            .iter()
            .zip(HandFingerMap::<Position>::keys())
            .for_each(|(p, (hand, finger))| data.set(&hand, &finger, KeyUsage::Idle(*p)));

        Self(data)
    }

    #[inline(always)]
    fn register_key(&mut self, k: &'a LayerKey) {
        self.0.set(&k.key.hand, &k.key.finger, KeyUsage::Used(k));
    }

    #[inline(always)]
    pub fn iter(&self) -> slice::Iter<'_, KeyUsage> {
        self.0.iter()
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

        let home_row_positions = FingerStates::with_positions(&layout.keyboard.home_row_positions);

        bigrams.iter().for_each(|((prev_key, curr_key), weight)| {
            // collect used fingers and keys for previous symbol
            let mut prev_used_keys = home_row_positions;
            prev_used_keys.register_key(prev_key);
            if !self.ignore_modifiers {
                prev_key.modifiers.layerkeys().iter().for_each(|k| {
                    prev_used_keys.register_key(layout.get_layerkey(k));
                });
            }

            // collect used fingers and keys for currend symbol
            let mut curr_used_keys = home_row_positions;
            curr_used_keys.register_key(curr_key);
            if !self.ignore_modifiers {
                curr_key.modifiers.layerkeys().iter().for_each(|k| {
                    curr_used_keys.register_key(layout.get_layerkey(k));
                });
            }

            prev_used_keys
                .iter()
                .zip(curr_used_keys.iter())
                .for_each(|(prev_used, curr_used)| {
                    match (prev_used, curr_used) {
                        // finger remains idle
                        (KeyUsage::Idle(_), KeyUsage::Idle(_)) => (),

                        // move previously idle finger to key press it
                        (KeyUsage::Idle(prev_pos), KeyUsage::Used(curr_key)) => {
                            let dist = prev_pos.distance(&curr_key.key.position)
                                + self.keydown_distance
                                + self.keyup_distance;
                            *finger_values.get_mut(&curr_key.key.hand, &curr_key.key.finger) +=
                                dist * weight;
                        }

                        // return finger from previous key press to home row
                        (KeyUsage::Used(prev_key), KeyUsage::Idle(curr_pos)) => {
                            let dist = prev_key.key.position.distance(curr_pos);
                            *finger_values.get_mut(&prev_key.key.hand, &prev_key.key.finger) +=
                                dist * weight;
                        }

                        // move finger from previous keypress to key and press it (same finger activation)
                        (KeyUsage::Used(prev_key), KeyUsage::Used(curr_key)) => {
                            // if both keys are identical and are mods it is a hold -> no cost
                            if !(prev_key == curr_key && curr_key.is_modifier.is_some()) {
                                let dist = curr_key.key.position.distance(&prev_key.key.position)
                                    + self.keydown_distance
                                    + self.keyup_distance;
                                *finger_values.get_mut(&curr_key.key.hand, &curr_key.key.finger) +=
                                    dist * weight;
                            }
                        }
                    };
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
                let fscore = self.dscoring.get(hand, finger);
                let hscore = self.hscoring.get(hand);
                *c *= fscore * hscore
            });

        let cost = finger_values.iter().sum();

        (cost, Some(message))
    }
}
