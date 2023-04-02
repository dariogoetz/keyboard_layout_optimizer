//! The trigram metric [`NoHandSwithInTrigram`] counts the weights of trigrams
//! that do not involve a handswitch (thumbs are excluded). The cost may differ depending on whether
//! there is a direction change from the first to the second bigram within the
//! trigram.
//!
//! *Note:* In ArneBab's version of the metric, a precomputed table is used that only involves
//! letters, period, and comma. Here, this is modelled by only including keys that are configured
//! as "fixed" in the [`Keyboard`].

use super::TrigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    factor_with_direction_change: f64,
    factor_without_direction_change: f64,
    factor_contains_index: f64,
    factor_same_key: f64,
    factor_contains_finger_repeat: f64,
    factor_same_key_start_end: f64,
}

#[derive(Clone, Debug)]
pub struct NoHandswitchInTrigram {
    factor_with_direction_change: f64,
    factor_without_direction_change: f64,
    factor_contains_index: f64,
    factor_same_key: f64,
    factor_contains_finger_repeat: f64,
    factor_same_key_start_end: f64,
}

impl NoHandswitchInTrigram {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_with_direction_change: params.factor_with_direction_change,
            factor_without_direction_change: params.factor_without_direction_change,
            factor_contains_index: params.factor_contains_index,
            factor_same_key: params.factor_same_key,
            factor_contains_finger_repeat: params.factor_contains_finger_repeat,
            factor_same_key_start_end: params.factor_same_key_start_end,
        }
    }
}

impl TrigramMetric for NoHandswitchInTrigram {
    fn name(&self) -> &str {
        "No Handswitch in Trigram"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        let hand1 = k1.key.hand;
        let hand2 = k2.key.hand;
        let hand3 = k3.key.hand;

        // NOTE: In ArneBab's solution, a precomputed table is used, that only contains letters, period, and comma
        // Here, we use "non-fixed" keys, which should (but need not, depending on configuration), amount to the same

        // exclude modifiers (see ArneBab's explanation in comments for layout_cost.py:_trigram_key_tables)
        if k1.is_modifier.is_some() || k2.is_modifier.is_some() || k3.is_modifier.is_some() {
            return Some(0.0);
        }

        if k1.key.finger == Finger::Thumb
            || k2.key.finger == Finger::Thumb
            || k3.key.finger == Finger::Thumb
        {
            return Some(0.0);
        }

        if hand1 != hand2 || hand2 != hand3 {
            return Some(0.0);
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;
        let pos3 = k3.key.matrix_position;

        let contains_repeat = (k1.key.finger == k2.key.finger && k1.key.hand == k2.key.hand)
            || (k2.key.finger == k3.key.finger && k2.key.hand == k3.key.hand);
        let same_key = pos1 == pos2 && pos2 == pos3;
        let contains_index = if k1.key.finger == Finger::Index
            || k2.key.finger == Finger::Index
            || k3.key.finger == Finger::Index
        {
            self.factor_contains_index
        } else {
            1.0
        };

        let factor = if same_key {
            self.factor_same_key
        } else if contains_repeat {
            self.factor_contains_finger_repeat
        } else if pos1 == pos3 {
            self.factor_same_key_start_end
        } else if (pos1.0 > pos2.0 && pos2.0 < pos3.0) || (pos1.0 < pos2.0 && pos2.0 > pos3.0) {
            self.factor_with_direction_change
        } else {
            self.factor_without_direction_change
        };

        Some(weight * factor * contains_index)
    }
}
