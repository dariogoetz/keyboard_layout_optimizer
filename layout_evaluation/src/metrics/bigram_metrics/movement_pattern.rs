//! The bigram metric [`MovementPattern`] puts cost on each bigram that is mapped to
//! (almost) neighboring fingers. Which finger combinations come with which costs is
//! configurable.

use super::BigramMetric;

use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap},
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct FingerSwitchCost {
    pub from: (Hand, Finger),
    pub to: (Hand, Finger),
    pub cost: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Cost associated with bigrams from a finger to another one
    pub finger_switch_costs: Vec<FingerSwitchCost>,
    /// Reduce penalties for bigrams on the same row by this factor
    pub same_row_reduction_factor: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct MovementPattern {
    finger_switch_costs: HandFingerMap<HandFingerMap<f64>>,
    same_row_reduction_factor: Vec<f64>,
}

impl MovementPattern {
    pub fn new(params: &Parameters) -> Self {
        let mut finger_switch_costs = HandFingerMap::with_default(HandFingerMap::with_default(0.0));
        params.finger_switch_costs.iter().for_each(|fsc| {
            let m = finger_switch_costs.get_mut(&fsc.from.0, &fsc.from.1);
            m.set(&fsc.to.0, &fsc.to.1, fsc.cost);
        });

        Self {
            finger_switch_costs,
            same_row_reduction_factor: params.same_row_reduction_factor.to_vec(),
        }
    }
}

impl BigramMetric for MovementPattern {
    fn name(&self) -> &str {
        "Movement Pattern"
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
        let mut cost = weight
            * *self
                .finger_switch_costs
                .get(&k1.key.hand, &k1.key.finger)
                .get(&k2.key.hand, &k2.key.finger);

        // if both keys are on the same row, they might be reduced in cost
        if k1.key.matrix_position.1 == k2.key.matrix_position.1 {
            let reduction_factor = self
                .same_row_reduction_factor
                .get(k1.key.matrix_position.1 as usize)
                .unwrap_or(&0.0);
            cost *= 1.0 - reduction_factor;
        }

        Some(cost)
    }
}
