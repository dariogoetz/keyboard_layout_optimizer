use super::TrigramMetric;

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
    pub exclude_rows: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct TrigramRolls {
    factor_inward: f64,
    factor_outward: f64,
    exclude_rows: Vec<u8>,
}

impl TrigramRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
            exclude_rows: params.exclude_rows.clone(),
        }
    }
}

impl TrigramMetric for TrigramRolls {
    fn name(&self) -> &str {
        "Trigram Rolls"
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
        if k1.key.hand != k2.key.hand || k2.key.hand != k3.key.hand {
            return Some(0.0);
        };

        // finger repeats are not considered rolls
        if k1.key.finger == k2.key.finger || k2.key.finger == k3.key.finger {
            return Some(0.0);
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;
        let pos3 = k3.key.matrix_position;

        // exclude rolls with keys in exclude_rows
        if self.exclude_rows.contains(&pos1.1)
            || self.exclude_rows.contains(&pos2.1)
            || self.exclude_rows.contains(&pos3.1)
        {
            return Some(0.0);
        }

        // only consider rolls on same row
        if pos1.1 != pos2.1 || pos2.1 != pos3.1 {
            return Some(0.0);
        }

        // only allow rolls with keys that are directly next to each others
        let inward1 = (k1.key.hand == Hand::Left && pos1.0 == pos2.0 - 1)
            || (k1.key.hand == Hand::Right && pos1.0 == pos2.0 + 1);

        let inward2 = (k2.key.hand == Hand::Left && pos2.0 == pos3.0 - 1)
            || (k2.key.hand == Hand::Right && pos2.0 == pos3.0 + 1);

        let outward1 = (k1.key.hand == Hand::Left && pos1.0 == pos2.0 + 1)
            || (k1.key.hand == Hand::Right && pos1.0 == pos2.0 - 1);

        let outward2 = (k2.key.hand == Hand::Left && pos2.0 == pos3.0 + 1)
            || (k2.key.hand == Hand::Right && pos2.0 == pos3.0 - 1);

        // both bigrams need to have the same direction
        let mut cost = if inward1 && inward2 {
            -self.factor_inward
        } else if outward1 && outward2 {
            -self.factor_outward
        } else {
            return Some(0.0);
        };

        cost /=
            (1.0 + k1.key.unbalancing) * (1.0 + k2.key.unbalancing) * (1.0 + k3.key.unbalancing);

        // log::info!("trigram roll: {}{}{} -> {:4.3}",
        //     k1.symbol.to_string().escape_debug(),
        //     k2.symbol.to_string().escape_debug(),
        //     k3.symbol.to_string().escape_debug(),
        //     cost.abs() * weight
        // );

        Some(cost * weight)
    }
}
