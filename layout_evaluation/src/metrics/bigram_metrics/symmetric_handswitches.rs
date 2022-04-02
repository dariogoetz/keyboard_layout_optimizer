//! The bigram metric [`SymmetricHandswitches`] metric assigns a negative cost to each bigram
//! for which the two keys are symmetrical on each hand (thumbs are excluded).
//!
//! *Note*: In contrast to ArneBab's version, this gives negative costs to symmetric handswitches
//! instead of positive costs to all other bigrams. Also, thumbs are excluded.

use super::BigramMetric;

use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct SymmetricHandswitches {}

impl SymmetricHandswitches {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl BigramMetric for SymmetricHandswitches {
    fn name(&self) -> &str {
        "Symmetric Handswitches"
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
        if k1.key.hand != k2.key.hand
            && k1.key.symmetry_index == k2.key.symmetry_index
            && k1.key.finger != Finger::Thumb
            && k2.key.finger != Finger::Thumb
        {
            Some(-weight)
        } else {
            Some(0.0)
        }
    }
}
