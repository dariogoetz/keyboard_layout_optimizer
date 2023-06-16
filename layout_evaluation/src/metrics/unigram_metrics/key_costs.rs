//! The unigram metric [`KeyCost`] multiplies each unigram's weight with the key cost
//! of the corresponding key (as configured for the [`Keyboard`]) and the associated
//! layer cost (as configured for the [`Layout`]).

use super::UnigramMetric;

use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct KeyCost {}

impl KeyCost {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl UnigramMetric for KeyCost {
    fn name(&self) -> &str {
        "Key Costs"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        key: &LayerKey,
        weight: f64,
        total_weight: f64,
        layout: &Layout,
    ) -> Option<f64> {
        let modifier_cost: f64 = key
            .modifiers
            .layerkey_indices()
            .iter()
            .map(|i| layout.get_layerkey(i).key.cost)
            .sum();
        let cost = key.key.cost + modifier_cost;

        // log the top scorers (with weight > 1%)
        if weight > 0.01 * total_weight {
            log::trace!("Unigram: {:>3}, Finger: {:<13}, Weight: {:>12.2}, Cost per key: {:>8.4}, Cost: {:>14.4}",
                        key, format!("{:?} {:?}", key.key.hand, key.key.finger), weight, cost, weight * cost);
        }

        Some(weight * cost)
    }
}
