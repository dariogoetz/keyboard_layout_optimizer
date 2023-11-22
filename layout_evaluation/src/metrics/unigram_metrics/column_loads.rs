use super::UnigramMetric;

use itertools::Itertools;
use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use ahash::AHashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    intended_loads: AHashMap<usize, f64>,
}

#[derive(Clone, Debug)]
pub struct ColumnLoads {
    intended_loads: AHashMap<usize, f64>,
}

impl ColumnLoads {
    pub fn new(params: &Parameters) -> Self {
        // normalize intended loads
        let total_intended: f64 = params.intended_loads.values().sum();
        let mut intended_loads = params.intended_loads.clone();
        intended_loads.values_mut().for_each(|l| {
            *l /= total_intended;
        });
        Self { intended_loads }
    }
}

impl UnigramMetric for ColumnLoads {
    fn name(&self) -> &str {
        "Column Loads"
    }

    fn total_cost(
        &self,
        unigrams: &[(&LayerKey, f64)],
        _total_weight: Option<f64>,
        _layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut column_loads: AHashMap<usize, f64> = AHashMap::default();

        unigrams
            .iter()
            .filter(|(key, _weight)| key.key.finger != Finger::Thumb)
            .for_each(|(layerkey, weight)| {
                let col = layerkey.key.matrix_position.0 as usize;
                *column_loads.entry(col).or_insert(0.0) += *weight;
            });
        let total_weight: f64 = column_loads.values().sum();

        // A version more similar to ArneBab's solution using the standard deviation
        let fractions: Vec<f64> = self
            .intended_loads
            .iter()
            .map(|(col, intended_load)| {
                let load = column_loads.get(col).unwrap_or(&0.0) / total_weight;
                load / intended_load
            })
            .collect();

        let mean: f64 = fractions.iter().sum::<f64>() / fractions.len() as f64;
        let var = fractions
            .iter()
            .map(|f| (f - mean) * (f - mean))
            .sum::<f64>()
            / (fractions.len() - 1) as f64;

        let mut messages = Vec::new();
        column_loads
            .into_iter()
            .sorted_by_key(|(col, _)| *col)
            .for_each(|(col, load)| {
                let intended_load = self.intended_loads.get(&col).unwrap_or(&0.0);
                let msg = format!(
                    "C{}: {:>.1}% ({:>.1}%)",
                    col,
                    100.0 * load / total_weight,
                    100.0 * intended_load
                );
                messages.push(msg);
            });

        let message = messages.join("; ");

        (var.sqrt(), Some(message))

        // A version using the total variation distance instead of standard deviation
        // This is a more uniform approach, i.e. half distance on two fingers equals full distance on one finger
        // total variation distance (between 0 and 1)
        // let diff = self
        //     .intended_loads
        //     .iter()
        //     .map(|((hand, finger), intended_load)| {
        //         let load = column_load.get(col).unwrap_or(&0.0) / total_weight;
        //         (load - intended_load).abs()
        //     })
        //     .sum::<f64>();

        // (0.5 * diff, None)
    }
}
