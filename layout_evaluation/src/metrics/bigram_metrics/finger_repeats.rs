//! The bigram metric [`FingerRepeats`] incurrs a cost for bigram that uses the same finger
//! for different keys (thumb excluded). If the finger is the pointer, the cost may be multiplied
//! with a configurable factor (usually lessening the cost).
//!
//! *Note:* In contrast to ArneBab's version of the metric, thumbs are excluded.

use super::BigramMetric;

use ahash::AHashMap;
use keyboard_layout::{
    key::{Finger, FingerMap, Hand},
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub finger_factors: AHashMap<Finger, f64>,
    pub stretch_factor: f64,
    pub curl_factor: f64,
    pub lateral_factor: f64,
    pub in_line_factor: f64,
}

#[derive(Clone, Debug)]
pub struct FingerRepeats {
    finger_factors: FingerMap<f64>,
    stretch_factor: f64,
    curl_factor: f64,
    lateral_factor: f64,
    in_line_factor: f64,
}

impl FingerRepeats {
    pub fn new(params: &Parameters) -> Self {
        Self {
            finger_factors: FingerMap::with_hashmap(&params.finger_factors, 1.0),
            stretch_factor: params.stretch_factor,
            curl_factor: params.curl_factor,
            lateral_factor: params.lateral_factor,
            in_line_factor: params.in_line_factor,
        }
    }
}

impl BigramMetric for FingerRepeats {
    fn name(&self) -> &str {
        "Finger Repeats"
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
        if (k1 == k2 && k1.is_modifier)
            || k1.key.hand != k2.key.hand
            || k1.key.finger != k2.key.finger
        {
            return Some(0.0);
        }

        let pos1 = k1.key.matrix_position;
        let pos2 = k2.key.matrix_position;
        let is_thumb: bool = k1.key.finger == Finger::Thumb;

        let upwards: bool = pos2.1 < pos1.1;
        let downwards: bool = pos2.1 > pos1.1;
        let inwards: bool = if k1.key.hand == Hand::Left {
            pos1.0 < pos2.0
        } else {
            pos1.0 > pos2.0
        };
        let outwards: bool = if k1.key.hand == Hand::Left {
            pos1.0 > pos2.0
        } else {
            pos1.0 < pos2.0
        };

        let dist_in_line = if is_thumb {
            pos1.0.abs_diff(pos2.0) as f64
        } else {
            pos1.1.abs_diff(pos2.1) as f64
        };
        let dist_lateral = if is_thumb {
            pos1.1.abs_diff(pos2.1) as f64
        } else {
            pos1.0.abs_diff(pos2.0) as f64
        };

        let direction_factor = if inwards || (!is_thumb && upwards) {
            self.stretch_factor
        } else if outwards || (!is_thumb && downwards) {
            self.curl_factor
        } else {
            1.0
        };

        let finger_factor = self.finger_factors.get(&k1.key.finger);
        let in_line_dist_factor = 1.0 + self.in_line_factor * dist_in_line;
        let lateral_dist_factor = 1.0 + self.lateral_factor * dist_lateral;

        let cost = finger_factor * lateral_dist_factor * direction_factor * in_line_dist_factor;

        Some(weight * cost)
    }
}
