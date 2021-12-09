//! The trigram metric `Irregularity` splits each trigram into two bigrams
//! and evaluates each bigram with all configured bigram metrics that can assign costs to
//! individual bigrams (`individual_cost` does not return `None`). The two bigram costs are multiplied and finally, the
//! square root of their sum is the resulting irregularity cost.
//!
//! *Note:* ArneBab's irregularity does not include all bigram metrics (asymmetric bigrams is missing).

use super::TrigramMetric;
use crate::metrics::bigram_metrics::BigramMetric;

use crate::results::NormalizationType;

use keyboard_layout::layout::{LayerKey, Layout};

use priority_queue::DoublePriorityQueue;
use serde::Deserialize;

const N_WORST: usize = 3;

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

        Some((1.0 + costs.0) * (1.0 + costs.1))
    }

    fn total_cost(
        &self,
        trigrams: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
        total_weight: Option<f64>,
        layout: &Layout,
    ) -> (f64, Option<String>) {
        // NOTE: ArneBab's solution does not involve all bigram metrics (the asymmetric bigrams metric is missing)

        let mut worst = DoublePriorityQueue::new();
        let mut cost_with_mod = 0.0;
        let total_weight = total_weight.unwrap_or_else(|| trigrams.iter().map(|(_, w)| w).sum());
        let total_cost: f64 = trigrams
            .iter()
            .filter_map(|(trigram, weight)| {
                let res = self.individual_cost(
                    trigram.0,
                    trigram.1,
                    trigram.2,
                    *weight,
                    total_weight,
                    layout,
                );

                if let Some(res) = res {
                    if trigram.0.is_modifier || trigram.1.is_modifier || trigram.2.is_modifier {
                        cost_with_mod += res;
                    };
                    worst.push(
                        (trigram.0.symbol, trigram.1.symbol, trigram.2.symbol),
                        (1_000_000.0 * res) as usize,
                    );
                    if worst.len() > N_WORST {
                        worst.pop_min();
                    }
                };

                res
            })
            .sum();

        let msgs: Vec<String> = worst
            .into_sorted_iter()
            .rev()
            .map(|(trigram, cost)| {
                format!(
                    "{}{}{} ({:>5.2}%)",
                    trigram.0.to_string().escape_debug(),
                    trigram.1.to_string().escape_debug(),
                    trigram.2.to_string().escape_debug(),
                    100.0 * (cost as f64 / 1_000_000.0) / total_cost,
                )
            })
            .collect();

        let msg = Some(format!(
            "Worst trigrams: {};  {:>5.2}% of cost involved a modifier",
            msgs.join(", "),
            100.0 * cost_with_mod / total_cost,
        ));

        (total_cost.sqrt(), msg)
    }
}
