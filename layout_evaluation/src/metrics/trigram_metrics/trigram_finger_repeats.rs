//! The trigram metric `TrigramFingerRepeats` counts the weights of trigrams
//! that do not involve the same key for all symbols (thumbs and consecutive
//! identical keys are excluded).
//!
//! *Note:* This metric is not present in ArneBab's version.

use super::TrigramMetric;

use keyboard_layout::key::Finger;
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    // Factor applied for each lateral movement in the bigrams
    pub factor_lateral_movement: f64,
}

#[derive(Clone, Debug)]
pub struct TrigramFingerRepeats {
    factor_lateral_movement: f64,
}

impl TrigramFingerRepeats {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_lateral_movement: params.factor_lateral_movement,
        }
    }
}

impl TrigramMetric for TrigramFingerRepeats {
    fn name(&self) -> &str {
        "Trigram Finger Repeats"
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

        let finger1 = k1.key.finger;
        let finger2 = k2.key.finger;
        let finger3 = k3.key.finger;

        // exclude key repititions
        if k1 == k2 || k2 == k3 {
            return Some(0.0);
        }

        // exclude thumbs
        if finger1 == Finger::Thumb || finger2 == Finger::Thumb || finger3 == Finger::Thumb {
            return Some(0.0);
        }

        // only consider same finger on same hand
        if hand1 != hand2 || hand2 != hand3 {
            return Some(0.0);
        }

        if finger1 != finger2 || finger2 != finger3 {
            return Some(0.0);
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;
        let pos3 = k3.key.matrix_position;

        let mut cost = weight;

        if pos1.0 != pos2.0 {
            cost *= self.factor_lateral_movement;
        }

        if pos2.0 != pos3.0 {
            cost *= self.factor_lateral_movement;
        }

        Some(cost)
    }
}
