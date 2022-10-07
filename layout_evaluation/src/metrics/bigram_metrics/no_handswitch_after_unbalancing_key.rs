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
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct NoHandSwitchAfterUnbalancingKey {}

impl NoHandSwitchAfterUnbalancingKey {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
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
        if k1.key.hand != k2.key.hand  // or handswitch occurred -> no cost
            || k1.key.finger == Finger::Thumb  // or one finger was a thumb -> no cost
            || k2.key.finger == Finger::Thumb
        // or other finger was a thumb -> no cost
        {
            return Some(0.0);
        }

        let dunbx = (k1.key.unbalancing.0 - k2.key.unbalancing.0).abs();
        let dunby = (k1.key.unbalancing.1 - k2.key.unbalancing.1).abs();

        let cost = dunbx + dunby;
        Some(weight * cost)
    }
}
