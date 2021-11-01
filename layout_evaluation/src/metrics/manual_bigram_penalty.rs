//! The bigram metric `ManualBigramPenalty` incurrs costs if the bigram is mapped
//! to one of a list of configurable "bad" key pairs (in terms of key locations).
//! In addition to the configurable key pairs, all key pairs from pinky to pinky
//! of the same hand are considered bad with a factor of one.

use super::BigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub matrix_positions: FxHashMap<((u8, u8), (u8, u8)), f64>,
}

#[derive(Clone, Debug)]
pub struct ManualBigramPenalty {
    matrix_positions: FxHashMap<((u8, u8), (u8, u8)), f64>,
}

impl ManualBigramPenalty {
    pub fn new(params: &Parameters) -> Self {
        let mut matrix_positions = params.matrix_positions.clone();

        matrix_positions.extend(
            params
                .matrix_positions
                .iter()
                .map(|(((x1, y1), (x2, y2)), w)| (((*x2, *y2), (*x1, *y1)), *w)),
        );

        Self { matrix_positions }
    }
}

impl BigramMetric for ManualBigramPenalty {
    fn name(&self) -> &str {
        "Manual Bigram Penalty"
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
        let x1 = k1.key.matrix_position.0 as u8;
        let y1 = k1.key.matrix_position.1 as u8;
        let x2 = k2.key.matrix_position.0 as u8;
        let y2 = k2.key.matrix_position.1 as u8;

        if let Some(val) = self.matrix_positions.get(&((x1, y1), (x2, y2))) {
            return Some(weight * *val);
        }

        // add manual penalty for all pinky finger repeats
        if k1.key.hand == k2.key.hand
            && k1.key.finger == Finger::Pinky
            && k2.key.finger == Finger::Pinky
        {
            return Some(weight);
        }

        return Some(0.0);
    }
}
