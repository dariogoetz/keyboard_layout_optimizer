use super::TrigramMetric;

use ahash::AHashSet;
use keyboard_layout::{
    key::{Finger, Hand},
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    exclude_thumbs: bool,
    exclude_modifiers: bool,
    exclude_chars: Vec<char>,
    same_hand_double_finger_repeat: f64,
    same_hand_single_finger_repeat: f64,
    same_hand_roll: f64,
    same_hand_redirect: f64,
    same_hand_bad_redirect: f64,
    roll_same_finger: f64,
    roll_inwards: f64,
    roll_outwards: f64,
    roll_other: f64,
    alternate_other_finger: f64,
    alternate_same_key: f64,
    alternate_finger_repeat: f64,
}

#[derive(Clone, Debug)]
pub struct OxeyCombinedTrigram {
    exclude_thumbs: bool,
    exclude_modifiers: bool,
    exclude_chars: AHashSet<char>,
    same_hand_double_finger_repeat: f64,
    same_hand_single_finger_repeat: f64,
    same_hand_roll: f64,
    same_hand_redirect: f64,
    same_hand_bad_redirect: f64,
    roll_same_finger: f64,
    roll_inwards: f64,
    roll_outwards: f64,
    roll_other: f64,
    alternate_other_finger: f64,
    alternate_same_key: f64,
    alternate_finger_repeat: f64,
}

#[derive(Debug, Default)]
struct TrigramTypeCounts {
    same_hand_double_finger_repeat: f64,
    same_hand_single_finger_repeat: f64,
    same_hand_roll: f64,
    same_hand_redirect: f64,
    same_hand_bad_redirect: f64,
    roll_same_finger: f64,
    roll_inwards: f64,
    roll_outwards: f64,
    roll_other: f64,
    alternate_other_finger: f64,
    alternate_same_key: f64,
    alternate_finger_repeat: f64,
}

#[inline(always)]
fn is_inwards_movement(k1: &LayerKey, k2: &LayerKey) -> bool {
    if k1.key.hand == Hand::Left {
        k1.key.matrix_position.0 < k2.key.matrix_position.0
    } else {
        k1.key.matrix_position.0 > k2.key.matrix_position.0
    }
}

impl OxeyCombinedTrigram {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
            exclude_modifiers: params.exclude_modifiers,
            exclude_chars: params.exclude_chars.iter().cloned().collect(),
            same_hand_double_finger_repeat: params.same_hand_double_finger_repeat,
            same_hand_single_finger_repeat: params.same_hand_single_finger_repeat,
            same_hand_roll: params.same_hand_roll,
            same_hand_redirect: params.same_hand_redirect,
            same_hand_bad_redirect: params.same_hand_bad_redirect,
            roll_same_finger: params.roll_same_finger,
            roll_inwards: params.roll_inwards,
            roll_outwards: params.roll_outwards,
            roll_other: params.roll_other,
            alternate_other_finger: params.alternate_other_finger,
            alternate_same_key: params.alternate_same_key,
            alternate_finger_repeat: params.alternate_finger_repeat,
        }
    }

    fn same_hand(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        counts: &mut TrigramTypeCounts,
    ) {
        let f1 = k1.key.finger;
        let f2 = k2.key.finger;
        let f3 = k3.key.finger;

        // finger repeats
        if f1 == f2 && f2 == f3 {
            counts.same_hand_double_finger_repeat += weight;
        } else if f1 == f2 || f2 == f3 {
            counts.same_hand_single_finger_repeat += weight;
        } else {
            // rolls
            let inwards1 = is_inwards_movement(k1, k2);
            let inwards2 = is_inwards_movement(k2, k3);

            let outwards1 = is_inwards_movement(k2, k1);
            let outwards2 = is_inwards_movement(k3, k2);

            if (inwards1 && inwards2) || (outwards1 && outwards2) {
                counts.same_hand_roll += weight;
            } else if f1 == Finger::Index || f2 == Finger::Index || f3 == Finger::Index {
                counts.same_hand_redirect += weight;
            } else {
                counts.same_hand_bad_redirect += weight;
            }
        }
    }

    fn roll(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        counts: &mut TrigramTypeCounts,
    ) {
        let (kr1, kr2) = if k1.key.hand == k2.key.hand {
            (k1, k2)
        } else {
            (k2, k3)
        };

        if kr1.key.finger == kr2.key.finger {
            counts.roll_same_finger += weight;
        } else {
            let inwards = is_inwards_movement(kr1, kr2);
            let outwards = is_inwards_movement(kr2, kr1);
            if inwards {
                counts.roll_inwards += weight;
            } else if outwards {
                counts.roll_outwards += weight;
            } else {
                counts.roll_other += weight;
            }
        }
    }

    fn alternate(
        &self,
        k1: &LayerKey,
        _k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        counts: &mut TrigramTypeCounts,
    ) {
        if k1.key.finger == k3.key.finger {
            counts.alternate_finger_repeat += weight;
        } else if k1.key == k3.key {
            counts.alternate_same_key += weight;
        } else {
            counts.alternate_other_finger += weight;
        }
    }
}

impl TrigramMetric for OxeyCombinedTrigram {
    fn name(&self) -> &str {
        "Combined"
    }

    fn total_cost(
        &self,
        trigrams: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
        // total_weight is optional for performance reasons (it can be computed from trigrams)
        _total_weight: Option<f64>,
        _layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut counts = TrigramTypeCounts::default();

        trigrams.iter().for_each(|((k1, k2, k3), weight)| {
            let h1 = k1.key.hand;
            let h2 = k2.key.hand;
            let h3 = k3.key.hand;

            if self.exclude_thumbs
                && (k1.key.finger == Finger::Thumb
                    || k2.key.finger == Finger::Thumb
                    || k3.key.finger == Finger::Thumb)
            {
                return;
            }

            if self.exclude_modifiers
                && (k1.is_modifier.is_some()
                    || k2.is_modifier.is_some()
                    || k3.is_modifier.is_some())
            {
                return;
            }

            if !self.exclude_chars.is_empty()
                && (self.exclude_chars.contains(&k1.symbol)
                    || self.exclude_chars.contains(&k2.symbol)
                    || self.exclude_chars.contains(&k3.symbol))
            {
                return;
            }

            if h1 == h2 && h2 == h3 {
                self.same_hand(k1, k2, k3, *weight, &mut counts);
            } else if h1 == h2 || h2 == h3 {
                self.roll(k1, k2, k3, *weight, &mut counts);
            } else {
                self.alternate(k1, k2, k3, *weight, &mut counts);
            }
        });

        let message = format!(
            "[SameHand: Onehand: {:.1} 2-Rep: {:.1} 1-Rep: {:.1} Redirect: {:.1} BadRedirect: {:.1}] [Roll: Inward: {:.1} Outward: {:.1} SameFinger: {:.1} Other: {:.1}] [Alternate: Normal: {:.1} SameKey: {:.1} SameFinger: {:.1}]",
            100.0 * counts.same_hand_roll,
            100.0 * counts.same_hand_double_finger_repeat,
            100.0 * counts.same_hand_single_finger_repeat,
            100.0 * counts.same_hand_redirect,
            100.0 * counts.same_hand_bad_redirect,
            100.0 * counts.roll_inwards,
            100.0 * counts.roll_outwards,
            100.0 * counts.roll_same_finger,
            100.0 * counts.roll_other,
            100.0 * counts.alternate_other_finger,
            100.0 * counts.alternate_same_key,
            100.0 * counts.alternate_finger_repeat,
        );

        let cost_same_hand = counts.same_hand_double_finger_repeat
            * self.same_hand_double_finger_repeat
            + counts.same_hand_single_finger_repeat * self.same_hand_single_finger_repeat
            + counts.same_hand_roll * self.same_hand_roll
            + counts.same_hand_redirect * self.same_hand_redirect
            + counts.same_hand_bad_redirect * self.same_hand_bad_redirect;
        let cost_roll = counts.roll_same_finger * self.roll_same_finger
            + counts.roll_inwards * self.roll_inwards
            + counts.roll_outwards * self.roll_outwards
            + counts.roll_other * self.roll_other;
        let cost_alternate = counts.alternate_finger_repeat * self.alternate_finger_repeat
            + counts.alternate_same_key * self.alternate_same_key
            + counts.alternate_other_finger * self.alternate_other_finger;

        (cost_same_hand + cost_roll + cost_alternate, Some(message))
    }
}
