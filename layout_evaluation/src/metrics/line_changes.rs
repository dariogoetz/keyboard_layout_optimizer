//! The bigram metric `LineChanges` is a measure for the vertical distance
//! to travel for a bigram (excluding thumbs). The vertical distance is normalized by the "horizontal" distance
//! of the fingers. More precisely, the number of rows to travel is squared and divided by the
//! finger distance. Additional adjustments are applied if the movement is upwards/downwards from/to
//! short/long fingers and if the involved keys are unbalancing (as configured for the keyboard).
//! The resulting value is squared (after being multiplied to the bigram's weight).

use super::BigramMetric;

use keyboard_layout::key::{Finger, Hand, HandFingerMap};
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    short_fingers: Vec<(Hand, Finger)>,
    long_fingers: Vec<(Hand, Finger)>,
    short_up_to_long_or_long_down_to_short_reduction: f64,
    short_down_to_long_or_long_up_to_short_increase: f64,
    count_row_changes_between_hands: bool,
}

#[derive(Clone, Debug)]
pub struct LineChanges {
    finger_is_short: HandFingerMap<bool>,
    finger_is_long: HandFingerMap<bool>,
    short_up_to_long_or_long_down_to_short_reduction: f64,
    short_down_to_long_or_long_up_to_short_increase: f64,
    count_row_changes_between_hands: bool,
}

impl LineChanges {
    pub fn new(params: &Parameters) -> Self {
        let mut finger_is_short = HandFingerMap::with_default(false);
        params
            .short_fingers
            .iter()
            .for_each(|(h, f)| finger_is_short.set(h, f, true));

        let mut finger_is_long = HandFingerMap::with_default(false);
        params
            .long_fingers
            .iter()
            .for_each(|(h, f)| finger_is_long.set(h, f, true));

        Self {
            finger_is_short,
            finger_is_long,
            short_up_to_long_or_long_down_to_short_reduction: params.short_up_to_long_or_long_down_to_short_reduction,
            short_down_to_long_or_long_up_to_short_increase: params.short_down_to_long_or_long_up_to_short_increase,
            count_row_changes_between_hands: params.count_row_changes_between_hands,
        }
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

        if k1.key.finger == Finger::Thumb || k2.key.finger == Finger::Thumb {
            return Some(0.0);
        }
        if !(self.count_row_changes_between_hands || k1.key.hand == k2.key.hand) {
            return Some(0.0);
        }

        let f1 = k1.key.finger;
        let f2 = k2.key.finger;
        let h1 = k1.key.hand;
        let h2 = k2.key.hand;
        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;
        let unb1 = k1.key.unbalancing;
        let unb2 = k2.key.unbalancing;

        let mut num_rows = (pos1.1 - pos2.1).abs() as f64;
        let upwards: bool = pos2.1 < pos1.1;
        let downwards: bool = pos2.1 > pos1.1;

        if (upwards && *self.finger_is_short.get(&h1, &f1) && *self.finger_is_long.get(&h2, &f2))
            || (downwards
                && *self.finger_is_long.get(&h1, &f1)
                && *self.finger_is_short.get(&h2, &f2))
        {
            num_rows -= self.short_up_to_long_or_long_down_to_short_reduction;
        }

        if (downwards && *self.finger_is_short.get(&h1, &f1) && *self.finger_is_long.get(&h2, &f2))
            || (upwards
                && *self.finger_is_long.get(&h1, &f1)
                && *self.finger_is_short.get(&h2, &f2))
        {
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
