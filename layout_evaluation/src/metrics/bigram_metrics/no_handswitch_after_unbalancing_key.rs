//! The bigram metric [`NoHandSwitchAfterUnbalancingKey`] assigns a cost to each bigram
//! that starts with an unbalancing key and ends on the same hand (no thumbs). The cost increases
//! with the square of the vertical distance. If the second key is unbalancing as well
//! and horizontally far away (more than three keys), the cost is increased even further.

use super::BigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub unbalancing_after_unbalancing: f64,
}

#[derive(Clone, Debug)]
pub struct NoHandSwitchAfterUnbalancingKey {
    unbalancing_after_unbalancing: f64,
}

impl NoHandSwitchAfterUnbalancingKey {
    pub fn new(params: &Parameters) -> Self {
        Self {
            unbalancing_after_unbalancing: params.unbalancing_after_unbalancing,
        }
    }
}

impl BigramMetric for NoHandSwitchAfterUnbalancingKey {
    fn name(&self) -> &str {
        "No Handswitch After Unbalancing Key"
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

        if unb1 <= 0.0  // first key is not unbalancing -> no cost
            || k1.key.hand != k2.key.hand  // or handswitch occurred -> no cost
            || k1.key.finger == Finger::Thumb  // or one finger was a thumb -> no cost
            || k2.key.finger == Finger::Thumb
        // or other finger was a thumb -> no cost
        {
            return Some(0.0);
        }

        let mut cost = unb1;

        // if the other key is unbalancing too and on the other side of the hand, put extra cost on it depending on their distance
        let unb2 = k2.key.unbalancing;
        let dx = k1.key.matrix_position.0.abs_diff(k2.key.matrix_position.0) as f64;
        let dy = k1.key.matrix_position.1.abs_diff(k2.key.matrix_position.1) as f64;
        if unb2 > 0.0 && dx > 3.0 {
            // second key is also unbalancing -> extra cost
            cost += unb1 * unb2 * self.unbalancing_after_unbalancing * (dx + dy - 3.0);
        };
        cost *= 1.0 + dy * dy;

        Some(weight * cost)
    }
}
