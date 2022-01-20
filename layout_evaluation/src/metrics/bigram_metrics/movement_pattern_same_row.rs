use std::collections::HashSet;

use super::BigramMetric;

use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap},
    layout::{LayerKey, Layout},
};

use std::convert::TryInto;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct FingerSwitchCost {
    pub from: (Hand, Finger),
    pub to: (Hand, Finger),
    pub cost: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Rows to exclude for finger rolls
    pub exclude_rows: HashSet<isize>,
    /// If to exclude bigrams containing keys with a positive "unbalancing" value
    pub exclude_unbalancing: bool,
    // If to exclude bigrams containing a lateral finger movement
    pub exclude_lateral_finger_movement: bool,
    /// Finger-specific costs
    pub finger_switch_costs: Vec<FingerSwitchCost>,
}

#[derive(Clone, Debug)]
pub struct MovementPatternSameRow {
    exclude_rows: HashSet<isize>,
    exclude_unbalancing: bool,
    exclude_lateral_finger_movement: bool,
    finger_switch_costs: HandFingerMap<HandFingerMap<f64>>,
}

impl MovementPatternSameRow {
    pub fn new(params: &Parameters) -> Self {
        let mut finger_switch_costs = HandFingerMap::with_default(HandFingerMap::with_default(0.0));
        params.finger_switch_costs.iter().for_each(|fsc| {
            let m = finger_switch_costs.get_mut(&fsc.from.0, &fsc.from.1);
            m.set(&fsc.to.0, &fsc.to.1, fsc.cost);
        });
        Self {
            exclude_rows: params.exclude_rows.clone(),
            exclude_unbalancing: params.exclude_unbalancing,
            exclude_lateral_finger_movement: params.exclude_lateral_finger_movement,
            finger_switch_costs,
        }
    }
}

impl BigramMetric for MovementPatternSameRow {
    fn name(&self) -> &str {
        "Movement Pattern (same row)"
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

        // no roll on lateral finger movements
        if k1.key.finger.distance(&k2.key.finger) < (pos1.0 - pos2.0).abs().try_into().unwrap() {
            return Some(0.0)
        }

        // exclude unbalancing keys, if required
        if self.exclude_unbalancing && (k1.key.unbalancing > 0.0 || k2.key.unbalancing > 0.0) {
            return Some(0.0)
        }

        // apply finger-specific costs
        let cost = *self.finger_switch_costs.get(&k1.key.hand, &k1.key.finger).get(&k2.key.hand, &k2.key.finger);

        Some(-cost * weight)
    }
}
