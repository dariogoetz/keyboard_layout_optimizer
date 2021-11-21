//! The unigram metric `HandDisbalance` compares the aggregated unigram frequencies
//! for both hands (excluding thumbs). The resulting cost is the distance of each hand's load to 0.5.

use super::UnigramMetric;

use keyboard_layout::key::{Finger, Hand, HandMap};
use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct HandDisbalance {}

impl HandDisbalance {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl UnigramMetric for HandDisbalance {
    fn name(&self) -> &str {
        "Hand Disbalance"
    }

    fn total_cost(
        &self,
        unigrams: &[(&LayerKey, f64)],
        _total_weight: Option<f64>,
        _layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut hand_loads: HandMap<f64> = HandMap::default();
        let mut total_weight = 0.0;
        unigrams
            .iter()
            .filter(|(key, _weight)| key.key.finger != Finger::Thumb)
            .for_each(|(key, weight)| {
                *hand_loads.get_mut(&key.key.hand) += *weight;
                total_weight += *weight;
            });

        let left_fraction = hand_loads.get(&Hand::Left) / total_weight;
        let right_fraction = hand_loads.get(&Hand::Right) / total_weight;

        let message = format!(
            "Hand loads % (no thumb): {:.2} - {:.2}",
            100.0 * left_fraction,
            100.0 * right_fraction
        );

        (0.5 * (left_fraction - right_fraction).abs(), Some(message))
    }
}
