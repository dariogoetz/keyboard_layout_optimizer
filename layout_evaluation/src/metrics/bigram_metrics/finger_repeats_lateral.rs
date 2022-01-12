//! The bigram metric `FingerRepeatsLateral` incurrs a cost for bigram that uses the same finger
//! for different keys (thumb excluded) if a lateral (horizontal) movement takes place.
//!
//! *Note:* This metric is not present in ArneBab's version.

use super::BigramMetric;

use keyboard_layout::key::Finger;
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// If some of the involved keys are unbalancing, add the unbalancing weight with this factor
    pub unbalancing_factor: f64,
}

#[derive(Clone, Debug)]
pub struct FingerRepeatsLateral {
    unbalancing_factor: f64,
}

impl FingerRepeatsLateral {
    pub fn new(params: &Parameters) -> Self {
        Self {
            unbalancing_factor: params.unbalancing_factor,
        }
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

        Some(
            (1.0 + self.unbalancing_factor * k1.key.unbalancing)
                * (1.0 + self.unbalancing_factor * k2.key.unbalancing)
                * weight,
        )
    }
}
