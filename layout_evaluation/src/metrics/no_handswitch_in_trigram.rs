use super::TrigramMetric;

use keyboard_layout::layout::{LayerKey, Layout};
use keyboard_layout::key::Finger;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub factor_with_direction_change: f64,
    pub factor_without_direction_change: f64,
}

#[derive(Clone, Debug)]
pub struct NoHandswitchInTrigram {
    factor_with_direction_change: f64,
    factor_without_direction_change: f64,
}

impl NoHandswitchInTrigram {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_with_direction_change: params.factor_with_direction_change,
            factor_without_direction_change: params.factor_without_direction_change,
        }
    }
}

impl TrigramMetric for NoHandswitchInTrigram {
    fn name(&self) -> &str {
        "No handswitch in trigram"
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
        let hand1 = k1.key.hand;
        let hand2 = k2.key.hand;
        let hand3 = k3.key.hand;

        // NOTE: In ArneBab's solution, a precomputed table is used, that only contains letters, period and comma
        // Here, we use "non-fixed" keys, which should (but need not, depending on configuration), amount to the same

        // exclude modifiers (see ArneBab's explanation in comments for layout_cost.py:_trigram_key_tables)
        if k1.is_fixed || k2.is_fixed || k3.is_fixed {
            return Some(0.0)
        }

        // should be already done with "not fixed"
        if k1.key.finger == Finger::Thumb || k2.key.finger == Finger::Thumb || k3.key.finger == Finger::Thumb {
            return Some(0.0)
        }

        if hand1 != hand2 || hand2 != hand3 {
            return Some(0.0)
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;
        let pos3 = k3.key.matrix_position;

        let factor = if (pos1.0 > pos2.0 && pos2.0 < pos3.0) || (pos1.0 < pos2.0 && pos2.0 > pos3.0)
        {
            self.factor_with_direction_change
        } else {
            self.factor_without_direction_change
        };

        Some(weight * factor)
    }
}
