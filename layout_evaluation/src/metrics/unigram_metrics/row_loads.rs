//! The unigram metric `RowLoads` is an informational metric (with no cost)
//! that evaluates which fraction of the unigrams is typed on each row (excluding
//! fixed and thumb keys).

use super::UnigramMetric;

use keyboard_layout::{
    layout::{LayerKey, Layout},
    key::Finger,
};

use serde::Deserialize;
use rustc_hash::FxHashMap;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct RowLoads {}

impl RowLoads {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl UnigramMetric for RowLoads {
    fn name(&self) -> &str {
        "Row Loads"
    }

    fn total_cost(
        &self,
        unigrams: &[(&LayerKey, f64)],
        _total_weight: Option<f64>,
        _layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut row_load: FxHashMap<isize, f64> = FxHashMap::default();
        let mut total_weight = 0.0;
        unigrams
            .iter()
            .filter(|(key, _weight)| !key.is_fixed && key.key.finger != Finger::Thumb)
            .for_each(|(key, weight)| {
                *row_load.entry(key.key.matrix_position.1).or_insert(0.0) += *weight;
                total_weight += *weight;
            });

        let mut messages = Vec::new();

        row_load.into_iter().for_each(|(row, load)|{
            let msg = format!("Row {}: {:>.3}%", row, 100.0 * load / total_weight);
            messages.push(msg);
        });

        let message = messages.join("; ");

        (0.0, Some(message))
    }
}
