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
pub struct OxeyAlternates {
    exclude_thumbs: bool,
}

impl OxeyAlternates {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
        }
    }
}

impl TrigramMetric for OxeyAlternates {
    fn name(&self) -> &str {
        "Alternates"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        if self.exclude_thumbs
            && (k1.key.finger == Finger::Thumb
                || k2.key.finger == Finger::Thumb
                || k3.key.finger == Finger::Thumb)
        {
            return Some(0.0);
        }

        let h1 = k1.key.hand;
        let h2 = k2.key.hand;
        let h3 = k3.key.hand;

        if h1 != h2 && h2 != h3 && !(h1 == h3 && k1.key.finger == k3.key.finger) {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}