//! The trigram metric `SecondaryBigrams` splits each trigram into two bigrams
//! and evaluates each bigram with all configured bigram metrics that can assign costs to
//! individual bigrams (`individual_cost` does not return `None`). The two bigram costs are multiplied and finally, the
//! square root of their sum is the resulting irregularity cost.
//!
//! *Note:* ArneBab's irregularity does not include all bigram metrics (asymmetric bigrams is missing).

use super::TrigramMetric;
use crate::metrics::bigram_metrics::BigramMetric;

use crate::results::NormalizationType;

use keyboard_layout::layout::{LayerKey, Layout};

use rustc_hash::FxHashSet;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Factor to apply to a trigram's weight before assigning it to the secondary bigram if the trigram involves no handswitch.
    pub factor_no_handswitch: f64,
    /// Factor to apply to a trigram's weight before assigning it to the secondary bigram if the trigram involves a handswitch.
    pub factor_handswitch: f64,
    /// Exclude secondary bigrams for trigrams containing at least one of the given symbols
    pub exclude_containing:  FxHashSet<char>,
}

#[derive(Clone, Debug)]
pub struct SecondaryBigrams {
    bigram_metrics: Vec<(f64, NormalizationType, Box<dyn BigramMetric>)>,
    factor_no_handswitch: f64,
    factor_handswitch: f64,
    exclude_containing:  FxHashSet<char>,
}

impl SecondaryBigrams {
    pub fn new(
        bigram_metrics: Vec<(f64, NormalizationType, Box<dyn BigramMetric>)>,
        params: &Parameters,
    ) -> Self {
        Self {
            bigram_metrics,
            factor_no_handswitch: params.factor_no_handswitch,
            factor_handswitch: params.factor_handswitch,
            exclude_containing: params.exclude_containing.clone(),
        }
    }
}

impl TrigramMetric for SecondaryBigrams {
    fn name(&self) -> &str {
        "Secondary Bigrams"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        total_weight: f64,
        layout: &Layout,
    ) -> Option<f64> {
        if self.exclude_containing.contains(&k1.symbol)
            || self.exclude_containing.contains(&k2.symbol)
            || self.exclude_containing.contains(&k3.symbol) {
                return Some(0.0)
        };

        let factor = if k1.key.hand == k2.key.hand && k2.key.hand == k3.key.hand {
            self.factor_no_handswitch
        } else {
            self.factor_handswitch
        };

        let cost: f64 = self
            .bigram_metrics
            .iter()
            .map(|(metric_weight, _, metric)| {
                factor * metric_weight
                    * metric
                        .individual_cost(k1, k3, weight, total_weight, layout)
                        .unwrap_or(0.0)
            })
            .sum();

        Some(cost)
    }
}
