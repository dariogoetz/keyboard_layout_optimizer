use super::BigramMetric;

use keyboard_layout::{key::Finger, layout::{LayerKey, Layout}};

use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub positions: FxHashMap<((u8, u8), (u8, u8)), f64>,
}

#[derive(Clone, Debug)]
pub struct ManualBigramPenalty {
    positions: FxHashMap<((u8, u8), (u8, u8)), f64>,
}

impl ManualBigramPenalty {
    pub fn new(params: &Parameters) -> Self {
        let mut positions = params.positions.clone();

        positions.extend(params.positions
            .iter()
            .map(|(((x1, y1), (x2, y2)), w)| {
                (((*x2, *y2), (*x1, *y1)), *w)
            }));

        Self {
            positions,
        }
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
        let x1 = k1.key.position.0 as u8;
        let y1 = k1.key.position.1 as u8;
        let x2 = k2.key.position.0 as u8;
        let y2 = k2.key.position.1 as u8;

        if let Some(val) = self.positions.get(&((x1, y1), (x2, y2))) {
            return Some(weight * *val)
        }

        // add manual penalty for all pinky finger repeats
        if k1.key.hand == k2.key.hand && k1.key.finger == Finger::Pinky && k2.key.finger == Finger::Pinky {
            return Some(weight)
        }

        return Some(0.0)
    }
}
