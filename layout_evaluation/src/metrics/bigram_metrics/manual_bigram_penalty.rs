//! The bigram metric [`ManualBigramPenalty`] incurrs costs if the bigram is mapped
//! to one of a list of configurable "bad" key pairs (in terms of key locations).

use super::BigramMetric;

use keyboard_layout::layout::{LayerKey, Layout};

use ahash::AHashMap;
use serde::Deserialize;

/// A tuple, structured the following way: (Column, Row)
type MatrixPosition = (u8, u8);

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub add_mirrored: bool,
    pub matrix_positions: AHashMap<(MatrixPosition, MatrixPosition), f64>,
}

#[derive(Clone, Debug)]
pub struct ManualBigramPenalty {
    matrix_positions: AHashMap<(MatrixPosition, MatrixPosition), f64>,
}

impl ManualBigramPenalty {
    pub fn new(params: &Parameters) -> Self {
        let mut matrix_positions = params.matrix_positions.clone();

        // add the reversed bigrams as well, if configured
        if params.add_mirrored {
            matrix_positions.extend(
                params
                    .matrix_positions
                    .iter()
                    .map(|(((x1, y1), (x2, y2)), w)| (((*x2, *y2), (*x1, *y1)), *w)),
            );
        }

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
        let x1 = k1.key.matrix_position.0;
        let y1 = k1.key.matrix_position.1;
        let x2 = k2.key.matrix_position.0;
        let y2 = k2.key.matrix_position.1;

        if let Some(val) = self.matrix_positions.get(&((x1, y1), (x2, y2))) {
            return Some(weight * *val);
        }

        Some(0.0)
    }
}
