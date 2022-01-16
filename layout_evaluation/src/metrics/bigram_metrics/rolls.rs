use std::collections::HashSet;

use super::BigramMetric;

use keyboard_layout::{
    key::Hand,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Factor to apply to a trigram's weight if the roll is going inwards
    pub factor_inward: f64,
    /// Factor to apply to a trigram's weight if the roll is going outwards
    pub factor_outward: f64,
    /// Rows to exclude for finger rolls
    pub exclude_rows: HashSet<isize>,
}

#[derive(Clone, Debug)]
pub struct BigramRolls {
    factor_inward: f64,
    factor_outward: f64,
    exclude_rows: HashSet<isize>,
}

impl BigramRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
            exclude_rows: params.exclude_rows.clone(),
        }
    }
}

impl BigramMetric for BigramRolls {
    fn name(&self) -> &str {
        "Bigram Rolls"
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
        if k1.key.hand != k2.key.hand {
            return Some(0.0);
        };

        // finger repeats are not considered rolls
        if k1.key.finger == k2.key.finger {
            return Some(0.0);
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;

        // exclude rolls with keys in exclude_rows
        if self.exclude_rows.contains(&pos1.1) || self.exclude_rows.contains(&pos2.1) {
            return Some(0.0);
        }

        // only consider rolls on same row
        if pos1.1 != pos2.1 {
            return Some(0.0);
        }

        let inward = (k1.key.hand == Hand::Left && pos1.0 < pos2.0)
            || (k1.key.hand == Hand::Right && pos1.0 > pos2.0);

        let outward = (k1.key.hand == Hand::Left && pos1.0 > pos2.0)
            || (k1.key.hand == Hand::Right && pos1.0 < pos2.0);

        // both bigrams need to have the same direction
        let mut cost = if inward {
            -self.factor_inward
        } else if outward {
            -self.factor_outward
        } else {
            return Some(0.0);
        };

        cost /= (1.0 + k1.key.unbalancing) * (1.0 + k2.key.unbalancing);

        // log::info!("bigram roll: {}{} -> {:4.3}",
        //     k1.symbol.to_string().escape_debug(),
        //     k2.symbol.to_string().escape_debug(),
        //     cost.abs() * weight
        // );

        Some(cost * weight)
    }
}
