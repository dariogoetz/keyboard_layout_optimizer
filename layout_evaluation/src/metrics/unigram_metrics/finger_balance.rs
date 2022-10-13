//! The unigram metric [`FingerBalance`] compares the aggregated unigram frequencies
//! per finger with configurable intended finger loads. The metric costs come from
//! discrepancies which are computed based on a standard deviation computation.
//!
//! *Note:* In contrast to ArneBab's version of the metric, thumb keys are excluded
//! from the discrepancy computation.

use super::UnigramMetric;

use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap},
    layout::{LayerKey, Layout},
};

use ahash::AHashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub intended_loads: AHashMap<(Hand, Finger), f64>,
}

#[derive(Clone, Debug)]
pub struct FingerBalance {
    intended_loads: AHashMap<(Hand, Finger), f64>,
}

impl FingerBalance {
    pub fn new(params: &Parameters) -> Self {
        // normalize intended loads
        let total_intended = params
            .intended_loads
            .iter()
            .filter(|((_h, f), _l)| *f != Finger::Thumb)
            .fold(0.0, |acc, (_, l)| acc + l);
        let mut intended_loads = params.intended_loads.clone();
        intended_loads.values_mut().for_each(|l| {
            *l /= total_intended;
        });
        Self { intended_loads }
    }
}

impl UnigramMetric for FingerBalance {
    fn name(&self) -> &str {
        "Finger Balance"
    }

    fn total_cost(
        &self,
        unigrams: &[(&LayerKey, f64)],
        _total_weight: Option<f64>,
        _layout: &Layout,
    ) -> (f64, Option<String>) {
        let mut finger_loads: HandFingerMap<f64> = HandFingerMap::with_default(0.0);

        // NOTE: ArneBab includes the thumb in the computation (in contrast to here). I believe that this is not helpful,
        // as it contains a large discrepancy (only one thumb is used for the spacebar) and the spacebar
        // is a fixed key anyways
        let mut total_weight = 0.0;
        unigrams
            .iter()
            .filter(|(key, _weight)| key.key.finger != Finger::Thumb)
            .for_each(|(key, weight)| {
                *finger_loads.get_mut(&key.key.hand, &key.key.finger) += *weight;
                total_weight += *weight;
            });

        // A version more similar to ArneBab's solution using the standard deviation
        let fractions: Vec<f64> = self
            .intended_loads
            .iter()
            .filter(|((_hand, finger), _intended_load)| *finger != Finger::Thumb)
            .map(|((hand, finger), intended_load)| {
                let load = finger_loads.get(hand, finger) / total_weight;
                log::trace!(
                    "Finger: {:>13}, Intended: {:>5.2}, Load: {:>5.2}, Fraction: {:>.4}",
                    format!("{:?} {:?}", hand, finger),
                    100.0 * intended_load,
                    100.0 * load,
                    load / intended_load,
                );
                load / intended_load
            })
            .collect();

        let mean: f64 = fractions.iter().sum::<f64>() / fractions.len() as f64;
        let var = fractions
            .iter()
            .map(|f| (f - mean) * (f - mean))
            .sum::<f64>()
            / (fractions.len() - 1) as f64;

        let message = format!(
            "Finger loads % (no thumb): {:.1} {:.1} {:.1} {:.1} - {:.1} {:.1} {:.1} {:.1}",
            100.0 * finger_loads.get(&Hand::Left, &Finger::Pinky) / total_weight,
            100.0 * finger_loads.get(&Hand::Left, &Finger::Ring) / total_weight,
            100.0 * finger_loads.get(&Hand::Left, &Finger::Middle) / total_weight,
            100.0 * finger_loads.get(&Hand::Left, &Finger::Index) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Index) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Middle) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Ring) / total_weight,
            100.0 * finger_loads.get(&Hand::Right, &Finger::Pinky) / total_weight,
        );

        (var.sqrt(), Some(message))

        // A version using the total variation distance instead of standard deviation
        // This is a more uniform approach, i.e. half distance on two fingers equals full distance on one finger
        // total variation distance (between 0 and 1)
        // let diff = self
        //     .intended_loads
        //     .iter()
        //     .filter(|((_hand, finger), _intended_load)| *finger != Finger::Thumb)
        //     .map(|((hand, finger), intended_load)| {
        //         let load = finger_loads.get(hand, finger) / total_weight;
        //         log::trace!(
        //             "Finger: {:>13}, Intended: {:>5.2}, Load: {:>5.2}",
        //             format!("{:?} {:?}", hand, finger),
        //             100.0 * intended_load,
        //             100.0 * load,
        //         );
        //         (load - intended_load).abs()
        //     })
        //     .sum::<f64>();

        // (0.5 * diff, None)
    }
}
