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
pub struct OxeyOutwardRolls {
    exclude_thumbs: bool,
}

impl OxeyOutwardRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
        }
    }
}

impl TrigramMetric for OxeyOutwardRolls {
    fn name(&self) -> &str {
        "Outward Rolls"
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

        let first_roll = h1 == h2 && h2 != h3;
        let second_roll = h1 != h2 && h2 == h3;

        if !(first_roll || second_roll) {
            return Some(0.0);
        }

        let (kr1, kr2) = if first_roll { (k1, k2) } else { (k2, k3) };

        let outwards: bool = if kr1.key.hand == Hand::Left {
            kr1.key.matrix_position.0 > kr2.key.matrix_position.0
        } else {
            kr1.key.matrix_position.0 < kr2.key.matrix_position.0
        };

        if !outwards {
            return Some(0.0);
        }

        Some(weight)
    }
}
