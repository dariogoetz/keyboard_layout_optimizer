//! The bigram metric `UnbalancingAfterNeighoring` assigns a cost to bigrams that
//! are mapped to at least one unbalancing key (and no thumb). The unbalancing strength value(s) is (are)
//! divided by the square of the finger distance for the cost value.

use super::BigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

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
        _layout: &Layout,
    ) -> Option<f64> {
        let unb1 = k1.key.unbalancing;
        let unb2 = k2.key.unbalancing;

        if (unb1 <= 0.0 && unb2 <= 0.0)
            || k1.key.hand != k2.key.hand
            || k1.key.finger == k2.key.finger  // && k1.key.hand == k2.key.hand
            || k1.key.finger == Finger::Thumb  // in ArneBab's version, this is included in the "finger_distance" function
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
