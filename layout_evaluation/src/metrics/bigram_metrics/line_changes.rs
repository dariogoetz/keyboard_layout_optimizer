//! The bigram metric `LineChanges` is a measure for the vertical distance
//! to travel for a bigram (excluding thumbs). The vertical distance is normalized by the "horizontal" distance
//! of the fingers. More precisely, the number of rows to travel is squared and divided by the
//! finger distance. Additional adjustments are applied if the movement is upwards/downwards from/to
//! shorter/longer fingers and if the involved keys are unbalancing (as configured for the keyboard).
//! The resulting value is squared (after being multiplied to the bigram's weight).
//!
//! In contrast to ArneBab's metric, finger length is compared relatively to each other, not
//! absolutely.

use super::BigramMetric;

use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap},
    layout::{LayerKey, Layout},
};

use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    finger_lengths: FxHashMap<Hand, FxHashMap<Finger, f64>>,
    short_up_to_long_or_long_down_to_short_reduction: f64,
    short_down_to_long_or_long_up_to_short_increase: f64,
    count_row_changes_between_hands: bool,
}

#[derive(Clone, Debug)]
pub struct LineChanges {
    finger_lengths: HandFingerMap<f64>,
    short_up_to_long_or_long_down_to_short_reduction: f64,
    short_down_to_long_or_long_up_to_short_increase: f64,
    count_row_changes_between_hands: bool,
}

impl LineChanges {
    pub fn new(params: &Parameters) -> Self {
        let finger_lengths = HandFingerMap::with_hashmap(&params.finger_lengths, 1.0);

        Self {
            finger_lengths,
            short_up_to_long_or_long_down_to_short_reduction: params
                .short_up_to_long_or_long_down_to_short_reduction,
            short_down_to_long_or_long_up_to_short_increase: params
                .short_down_to_long_or_long_up_to_short_increase,
            count_row_changes_between_hands: params.count_row_changes_between_hands,
        }
    }
}

impl LineChanges {
    #[inline(always)]
    fn finger_is_longer(&self, h1: &Hand, f1: &Finger, h2: &Hand, f2: &Finger) -> bool {
        let len1 = self.finger_lengths.get(h1, f1);
        let len2 = self.finger_lengths.get(h2, f2);

        len1 > len2
    }

    #[inline(always)]
    fn finger_is_shorter(&self, h1: &Hand, f1: &Finger, h2: &Hand, f2: &Finger) -> bool {
        let len1 = self.finger_lengths.get(h1, f1);
        let len2 = self.finger_lengths.get(h2, f2);

        len1 < len2
    }
}

impl BigramMetric for LineChanges {
    fn name(&self) -> &str {
        "Line Changes"
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
        // NOTE: ArneBab's solution only excludes the spacebar. Here, all thumb keys are excluded, in particular one M4 modifier.

        let f1 = k1.key.finger;
        let f2 = k2.key.finger;
        let h1 = k1.key.hand;
        let h2 = k2.key.hand;

        let first_is_longer = self.finger_is_longer(&h1, &f1, &h2, &f2);
        let first_is_shorter = self.finger_is_shorter(&h1, &f1, &h2, &f2);

        if f1 == Finger::Thumb || f2 == Finger::Thumb {
            return Some(0.0);
        }
        if !(self.count_row_changes_between_hands || h1 == h2) {
            return Some(0.0);
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;
        let unb1 = k1.key.unbalancing;
        let unb2 = k2.key.unbalancing;

        let mut num_rows = (pos1.1 as i8 - pos2.1 as i8).abs() as f64;
        let upwards: bool = pos2.1 < pos1.1;
        let downwards: bool = pos2.1 > pos1.1;

        if (upwards && first_is_shorter) || (downwards && first_is_longer) {
            num_rows -= self.short_up_to_long_or_long_down_to_short_reduction;
        }

        if (downwards && first_is_shorter) || (upwards && first_is_longer) {
            num_rows += self.short_down_to_long_or_long_up_to_short_increase;
        }

        // NOTE: In ArneBab's solution, there may be keys that do not belong to a finger.
        // For these, the (potentially warped) positions are used. Here, all keys have a finger
        // Also, the correction for row 3 does not apply here for that reason

        let finger_distance = f1.distance(&f2) as f64;

        let sqrt_cost =
            num_rows * num_rows / finger_distance.max(0.5) * (1.0 + unb1) * (1.0 + unb2);

        Some(weight * sqrt_cost * sqrt_cost)
    }
}
