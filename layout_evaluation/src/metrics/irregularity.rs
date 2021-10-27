use super::{BigramMetric, TrigramMetric};

use crate::results::NormalizationType;

use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct Irregularity {
    bigram_metrics: Vec<(f64, NormalizationType, Box<dyn BigramMetric>)>,
}

impl Irregularity {
    pub fn new(
        bigram_metrics: Vec<(f64, NormalizationType, Box<dyn BigramMetric>)>,
        _params: &Parameters,
    ) -> Self {
        Self { bigram_metrics }
    }
}

impl TrigramMetric for Irregularity {
    fn name(&self) -> &str {
        "Irregularity"
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
        let costs: (f64, f64) = self
            .bigram_metrics
            .iter()
            .map(|(metric_weight, _, metric)| {
                let cost1 = metric_weight
                    * metric
                        .individual_cost(k1, k2, weight, total_weight, layout)
                        .unwrap_or(0.0);
                let cost2 = metric_weight
                    * metric
                        .individual_cost(k2, k3, weight, total_weight, layout)
                        .unwrap_or(0.0);
                (cost1, cost2)
            })
            .fold((0.0, 0.0), |(acc1, acc2), (c1, c2)| (acc1 + c1, acc2 + c2));

        Some(costs.0 * costs.1)
    }

    fn total_cost(
        &self,
        trigrams: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
        total_weight: Option<f64>,
        layout: &Layout,
    ) -> (f64, Option<String>) {
        // NOTE: ArneBab's solution does not involve all bigram metrics (the asymmetric bigrams metric is missing)

        let total_weight = total_weight.unwrap_or_else(|| trigrams.iter().map(|(_, w)| w).sum());
        let total_cost: f64 = trigrams
            .iter()
            .filter_map(|(trigram, weight)| {
                self.individual_cost(
                    trigram.0,
                    trigram.1,
                    trigram.2,
                    *weight,
                    total_weight,
                    layout,
                )
            })
            .sum();

        (total_cost.sqrt(), None)
    }
}
