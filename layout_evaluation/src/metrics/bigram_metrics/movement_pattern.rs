//! The bigram metric [`MovementPattern`] puts cost on each bigram that is mapped to
//! (almost) neighboring fingers. Which finger combinations come with which costs is
//! configurable.

use super::BigramMetric;

use ahash::AHashMap;
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap},
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct FingerSwitchCost {
    pub from: (Hand, Finger),
    pub to: (Hand, Finger),
    pub cost: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Cost associated with bigrams from a finger to another one
    finger_switch_factor: Vec<FingerSwitchCost>,
    finger_lengths: AHashMap<Hand, AHashMap<Finger, f64>>,
    short_down_to_long_or_long_up_to_short_factor: f64,
    same_row_offset: f64,
    unbalancing_factor: f64,
    lateral_stretch_factor: f64,
}

#[derive(Clone, Debug)]
pub struct MovementPattern {
    finger_switch_factor: HandFingerMap<HandFingerMap<f64>>,
    finger_lengths: HandFingerMap<f64>,
    short_down_to_long_or_long_up_to_short_factor: f64,
    same_row_offset: f64,
    unbalancing_factor: f64,
    lateral_stretch_factor: f64,
}

impl MovementPattern {
    pub fn new(params: &Parameters) -> Self {
        let mut finger_switch_factor =
            HandFingerMap::with_default(HandFingerMap::with_default(0.0));
        params.finger_switch_factor.iter().for_each(|fsc| {
            let m = finger_switch_factor.get_mut(&fsc.from.0, &fsc.from.1);
            m.set(&fsc.to.0, &fsc.to.1, fsc.cost);
        });
        let finger_lengths = HandFingerMap::with_hashmap(&params.finger_lengths, 1.0);

        Self {
            finger_switch_factor,
            finger_lengths,
            short_down_to_long_or_long_up_to_short_factor: params
                .short_down_to_long_or_long_up_to_short_factor,
            same_row_offset: params.same_row_offset,
            unbalancing_factor: params.unbalancing_factor,
            lateral_stretch_factor: params.lateral_stretch_factor,
        }
    }
}

impl BigramMetric for MovementPattern {
    fn name(&self) -> &str {
        "Movement Pattern"
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
        let f1 = k1.key.finger;
        let f2 = k2.key.finger;
        let h1 = k1.key.hand;
        let h2 = k2.key.hand;

        if f1 == Finger::Thumb || f2 == Finger::Thumb || h1 != h2 || f1 == f2 {
            return Some(0.0);
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;

        let upwards: bool = pos2.1 < pos1.1;
        let downwards: bool = pos2.1 > pos1.1;

        let finger_length_diff =
            self.finger_lengths.get(&h1, &f1) - self.finger_lengths.get(&h2, &f2);
        let first_is_longer = finger_length_diff > 0.0;
        let first_is_shorter = finger_length_diff < 0.0;

        let num_rows = pos1.1.abs_diff(pos2.1) as f64;

        let finger_switch_factor = self.finger_switch_factor.get(&h1, &f1).get(&h2, &f2);
        let direction_factor = if (downwards && first_is_shorter) || (upwards && first_is_longer) {
            1.0 + finger_length_diff.abs() * self.short_down_to_long_or_long_up_to_short_factor
        } else {
            1.0
        };

        let unbalancing_factor = 1.0
            + (self.unbalancing_factor
                * ((k1.key.unbalancing.0 - k2.key.unbalancing.0).abs()
                    + (k1.key.unbalancing.1 - k2.key.unbalancing.1).abs()));

        let lateral_stretch_factor = 1.0
            + (f1.distance(&f2))
                .abs_diff(k1.key.matrix_position.0.abs_diff(k2.key.matrix_position.0))
                as f64
                * self.lateral_stretch_factor;

        let cost = (self.same_row_offset + num_rows * num_rows)
            * finger_switch_factor
            * direction_factor
            * unbalancing_factor
            * lateral_stretch_factor;

        Some(weight * cost)
    }
}
