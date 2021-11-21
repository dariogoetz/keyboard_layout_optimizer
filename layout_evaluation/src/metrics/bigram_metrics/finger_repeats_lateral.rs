//! The bigram metric `FingerRepeatsLateral` incurrs a cost for bigram that uses the same finger
//! for different keys (thumb excluded) if a lateral (horizontal) movement takes place.
//!
//! *Note:* This metric is not present in ArneBab's version.

use super::BigramMetric;

use keyboard_layout::key::Finger;
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct FingerRepeatsLateral {}

impl FingerRepeatsLateral {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl BigramMetric for FingerRepeatsLateral {
    fn name(&self) -> &str {
        "Finger Repeats Lateral"
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
        if k1 == k2
            || k1.key.hand != k2.key.hand
            || k1.key.finger != k2.key.finger
            || k1.key.finger == Finger::Thumb
            || k1.key.matrix_position.0 == k2.key.matrix_position.0
        {
            return Some(0.0);
        }

        Some(weight)
    }
}
