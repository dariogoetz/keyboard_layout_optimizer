//! The bigram metric `MovementPattern` puts cost on each bigram that is mapped to
//! (almost) neighboring fingers. Which finger combinations come with which costs is
//! configurable.

use super::BigramMetric;

use keyboard_layout::key::{Finger, Hand, HandFingerMap};
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct FingerSwitchCost {
    pub from: (Hand, Finger),
    pub to: (Hand, Finger),
    pub cost: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub finger_switch_costs: Vec<FingerSwitchCost>,
}

#[derive(Clone, Debug)]
pub struct MovementPattern {
    finger_switch_costs: HandFingerMap<HandFingerMap<f64>>,
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
        Some(
            weight
                * *self
                    .finger_switch_costs
                    .get(&k1.key.hand, &k1.key.finger)
                    .get(&k2.key.hand, &k2.key.finger),
        )
    }
}
