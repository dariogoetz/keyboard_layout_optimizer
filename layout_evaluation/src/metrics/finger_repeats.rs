use super::BigramMetric;

use keyboard_layout::key::Finger;
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub index_finger_factor: f64,
    pub critical_fraction: f64,
    pub factor: f64,
    pub total_weight_threshold: f64,
}

#[derive(Clone, Debug)]
pub struct FingerRepeats {
    index_finger_factor: f64,
    critical_fraction: f64,
    factor: f64,
    total_weight_threshold: f64,
}

impl FingerRepeats {
    pub fn new(params: &Parameters) -> Self {
        Self {
            index_finger_factor: params.index_finger_factor,
            critical_fraction: params.critical_fraction,
            factor: params.factor,
            total_weight_threshold: params.total_weight_threshold,
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
        total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        let critical_point = self.critical_fraction * total_weight;
        if k1 == k2 || k1.key.hand != k2.key.hand || k1.key.finger != k2.key.finger {
            return Some(0.0);
        }
        let mut cost = weight;

        // NOTE: In ArneBab's solution, increasing common repeats is done in a previous, separate step (in "finger_repeats_from_file")

        // reduce weight of index finger repeats
        if k1.key.finger == Finger::Pointer {
            cost *= self.index_finger_factor;
        }

        // increase weight of common repeats
        if cost > critical_point && total_weight > self.total_weight_threshold {
            cost += (cost - critical_point) * (self.factor - 1.0);
        }

        Some(cost)
    }
}
