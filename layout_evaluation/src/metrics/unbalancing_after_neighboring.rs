use super::BigramMetric;

use keyboard_layout::key::Finger;
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct UnbalancingAfterNeighboring {}

impl UnbalancingAfterNeighboring {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl BigramMetric for UnbalancingAfterNeighboring {
    fn name(&self) -> &str {
        "Unbalancing After Neighboring"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        weight: f64,
        _total_weight: f64,
        layout: &Layout,
    ) -> Option<f64> {
        let unb1 = layout.keyboard.unbalancing_positions[k1.key.index];
        let unb2 = layout.keyboard.unbalancing_positions[k2.key.index];

        if (unb1 <= 0.0 && unb2 <= 0.0)
            || k1.key.hand != k2.key.hand
            || k1.key.finger == k2.key.finger  // && k1.key.hand == k2.key.hand
            || k1.key.finger == Finger::Thumb
            || k2.key.finger == Finger::Thumb
        {
            // no unbalancing keys
            // or different hands
            // or same finger twice
            // or a thumb is involved
            // --> no cost
            return Some(0.0);
        }

        let d = k1.key.finger.distance(&k2.key.finger) as f64;
        Some(weight * (unb1 + unb2) / (d * d))
    }
}
