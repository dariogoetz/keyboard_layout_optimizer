use super::BigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct OxeyLsbs {}

impl OxeyLsbs {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl BigramMetric for OxeyLsbs {
    fn name(&self) -> &str {
        "Lsbs"
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
        if k1 == k2 {
            return Some(0.0);
        }
        let h1 = k1.key.hand;
        let h2 = k2.key.hand;

        if h1 != h2 {
            return Some(0.0);
        }

        let f1 = k1.key.finger;
        let f2 = k2.key.finger;

        if f1 == Finger::Thumb || f2 == Finger::Thumb {
            return Some(0.0);
        }

        if f1.distance(&f2) == 1 && k1.key.matrix_position.0.abs_diff(k2.key.matrix_position.0) > 1
        {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
