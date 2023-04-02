//! The unigram metric [`ModifierUsage`] sums each modifier unigram's weight

use super::UnigramMetric;

use keyboard_layout::layout::{LayerKey, LayerModifierType, LayerModifiers, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub hold_cost: f64,
    pub one_shot_cost: f64,
}

#[derive(Clone, Debug)]
pub struct ModifierUsage {
    pub hold_cost: f64,
    pub one_shot_cost: f64,
}

impl ModifierUsage {
    pub fn new(params: &Parameters) -> Self {
        Self {
            hold_cost: params.hold_cost,
            one_shot_cost: params.one_shot_cost,
        }
    }
}

impl UnigramMetric for ModifierUsage {
    fn name(&self) -> &str {
        "Hold Modifier Usage"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        key: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        // costs if this key is a modifier
        let key_cost = match key.is_modifier {
            LayerModifierType::Hold => self.hold_cost,
            LayerModifierType::OneShot => self.one_shot_cost,
            _ => 0.0,
        };

        // costs if this key relies on modifiers (that were not split in ngram splitting)
        let modifier_costs = match &key.modifiers {
            LayerModifiers::Hold(v) => self.hold_cost * v.len() as f64,
            LayerModifiers::OneShot(v) => self.one_shot_cost * v.len() as f64,
        };

        Some(weight * (key_cost + modifier_costs))
    }
}
