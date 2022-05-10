//! The bigram metric [`FingerRepeats`] incurrs a cost for bigram that uses the same finger
//! for different keys (thumb excluded). If the finger is the pointer, the cost may be multiplied
//! with a configurable factor (usually lessening the cost).
//!
//! *Note:* In contrast to ArneBab's version of the metric, thumbs are excluded.

use super::BigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// If the finger repetition is done by the index finger, the cost is multiplied with this factor.
    pub index_finger_factor: f64,
    /// If the finger repetition is done by the pinky finger, the cost is multiplied with this factor.
    pub pinky_finger_factor: f64,
    /// If some of the involved keys are unbalancing, add the unbalancing weight with this factor
    pub unbalancing_factor: f64,
}

#[derive(Clone, Debug)]
pub struct FingerRepeats {
    index_finger_factor: f64,
    pinky_finger_factor: f64,
    unbalancing_factor: f64,
}

impl FingerRepeats {
    pub fn new(params: &Parameters) -> Self {
        Self {
            index_finger_factor: params.index_finger_factor,
            pinky_finger_factor: params.pinky_finger_factor,
            unbalancing_factor: params.unbalancing_factor,
        }
    }
}

impl BigramMetric for FingerRepeats {
    fn name(&self) -> &str {
        "Finger Repeats"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        if k1 == k2 || k1.key.hand != k2.key.hand || k1.key.finger != k2.key.finger {
            return Some(0.0);
        }

        let mut cost = (1.0 + self.unbalancing_factor * k1.key.unbalancing)
            * (1.0 + self.unbalancing_factor * k2.key.unbalancing)
            * weight;

        // NOTE: In ArneBab's solution, increasing common repeats is done in a previous,
        // separate step (in "finger_repeats_from_file")

        // reduce weight of index finger repeats
        if k1.key.finger == Finger::Pointer {
            cost *= self.index_finger_factor;
        }
        // increase weight of pinky finger repeats
        if k1.key.finger == Finger::Pinky {
            cost *= self.pinky_finger_factor;
        }

        Some(cost)
    }
}
