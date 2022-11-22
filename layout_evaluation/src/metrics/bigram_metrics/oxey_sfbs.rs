use super::BigramMetric;

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
pub struct OxeySfbs {
    exclude_thumbs: bool,
}

impl OxeySfbs {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
        }
    }
}

impl BigramMetric for OxeySfbs {
    fn name(&self) -> &str {
        "Sfbs"
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

        if self.exclude_thumbs && (f1 == Finger::Thumb || f2 == Finger::Thumb) {
            return Some(0.0);
        }

        if f1 == f2 {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
