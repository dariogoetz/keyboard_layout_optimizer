use super::TrigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub exclude_thumbs: bool,
}

#[derive(Clone, Debug)]
pub struct OxeyDsfbs {
    exclude_thumbs: bool,
}

impl OxeyDsfbs {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
        }
    }
}

impl TrigramMetric for OxeyDsfbs {
    fn name(&self) -> &str {
        "Dsfbs"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        _k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        // no same-key sfbs
        if k1 == k3 {
            return Some(0.0);
        }

        let h1 = k1.key.hand;
        let h3 = k3.key.hand;

        if h1 != h3 {
            return Some(0.0);
        }

        let f1 = k1.key.finger;
        let f3 = k3.key.finger;

        if self.exclude_thumbs && (f1 == Finger::Thumb || f3 == Finger::Thumb) {
            return Some(0.0);
        }

        if f1 == f3 {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
