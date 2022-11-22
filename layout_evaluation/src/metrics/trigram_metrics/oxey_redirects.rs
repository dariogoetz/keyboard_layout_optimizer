use super::TrigramMetric;

use keyboard_layout::{
    key::{Finger, Hand},
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub exclude_thumbs: bool,
}

#[derive(Clone, Debug)]
pub struct OxeyRedirects {
    exclude_thumbs: bool,
}

impl OxeyRedirects {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
        }
    }
}

#[inline(always)]
fn inwards(k1: &LayerKey, k2: &LayerKey) -> bool {
    if k1.key.hand == Hand::Left {
        k1.key.matrix_position.0 < k2.key.matrix_position.0
    } else {
        k1.key.matrix_position.0 > k2.key.matrix_position.0
    }
}

impl TrigramMetric for OxeyRedirects {
    fn name(&self) -> &str {
        "Redirects"
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
        let h1 = k1.key.hand;
        let h2 = k2.key.hand;
        let h3 = k3.key.hand;

        if !(h1 == h2 && h2 == h3) {
            return Some(0.0);
        }

        let f1 = k1.key.finger;
        let f2 = k2.key.finger;
        let f3 = k3.key.finger;

        if self.exclude_thumbs
            && (f1 == Finger::Thumb || f2 == Finger::Thumb || f3 == Finger::Thumb)
        {
            return Some(0.0);
        }

        // at least one key shall be hit with the index finger (else it's a bad redirect)
        if !(f1 == Finger::Index || f2 == Finger::Index || f3 == Finger::Index) {
            return Some(0.0);
        }

        let inwards1 = inwards(k1, k2);
        let inwards2 = inwards(k2, k3);

        let outwards1 = inwards(k2, k1);
        let outwards2 = inwards(k3, k2);

        if (inwards1 && outwards2) || (outwards1 && inwards2) {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
