//! The trigram metric [`Irregularity`] splits each trigram into two bigrams
//! and evaluates each bigram with all configured bigram metrics that can assign costs to
//! individual bigrams (`individual_cost` does not return `None`). The two bigram costs are multiplied and finally, the
//! square root of their sum is the resulting irregularity cost.
//!
//! *Note:* ArneBab's irregularity does not include all bigram metrics (asymmetric bigrams is missing).

use super::TrigramMetric;
use crate::metrics::bigram_metrics::BigramMetric;
use crate::results::NormalizationType;
use keyboard_layout::layout::{LayerKey, Layout};

use ordered_float::OrderedFloat;
use priority_queue::DoublePriorityQueue;
use serde::Deserialize;
use std::env;

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

        let cost = (1.0 + costs.0) * (1.0 + costs.1) - 1.0;
        Some(cost.max(0.0))
    }

    fn total_cost(
        &self,
        trigrams: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
        total_weight: Option<f64>,
        layout: &Layout,
    ) -> (f64, Option<String>) {
        let show_worst: bool = env::var("SHOW_WORST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(true);
        let n_worst: usize = env::var("N_WORST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);

        // NOTE: ArneBab's solution does not involve all bigram metrics (the asymmetric bigrams metric is missing)

        let total_weight = total_weight.unwrap_or_else(|| trigrams.iter().map(|(_, w)| w).sum());
        let cost_iter = trigrams
            .iter()
            .enumerate()
            .filter_map(|(i, (trigram, weight))| {
                let cost_option = self.individual_cost(
                    trigram.0,
                    trigram.1,
                    trigram.2,
                    *weight,
                    total_weight,
                    layout,
                );

                cost_option.map(|cost| (i, trigram, cost))
            });

        let (total_cost, msg) = if show_worst {
            let (total_cost, worst, worst_nonfixed) = cost_iter.fold(
                (0.0, DoublePriorityQueue::new(), DoublePriorityQueue::new()),
                |(mut total_cost, mut worst, mut worst_nonfixed), (i, trigram, cost)| {
                    total_cost += cost;

                    if !trigram.0.is_fixed && !trigram.1.is_fixed && !trigram.2.is_fixed {
                        worst_nonfixed.push(i, OrderedFloat(cost.abs()));
                    }
                    worst.push(i, OrderedFloat(cost.abs()));

                    if worst.len() > n_worst {
                        worst.pop_min();
                    }
                    if worst_nonfixed.len() > n_worst {
                        worst_nonfixed.pop_min();
                    }

                    (total_cost, worst, worst_nonfixed)
                },
            );

            let gen_msgs = |q: DoublePriorityQueue<usize, OrderedFloat<f64>>| {
                let worst_msgs: Vec<String> = q
                    .into_sorted_iter()
                    .rev()
                    .filter(|(_, cost)| cost.into_inner() > 0.0)
                    .map(|(i, cost)| {
                        let (gram, _) = trigrams[i];
                        format!(
                            "{}{}{} ({:>5.2}%)",
                            gram.0,
                            gram.1,
                            gram.2,
                            100.0 * cost.into_inner() / total_cost,
                        )
                    })
                    .collect();

                worst_msgs
            };

            let mut msgs = Vec::new();

            let worst_msgs = gen_msgs(worst);
            if !worst_msgs.is_empty() {
                msgs.push(format!("Worst: {}", worst_msgs.join(", ")))
            }

            let worst_nonfixed_msgs = gen_msgs(worst_nonfixed);
            if !worst_nonfixed_msgs.is_empty() {
                msgs.push(format!(
                    "Worst non-fixed: {}",
                    worst_nonfixed_msgs.join(", ")
                ))
            }

            let msg = Some(msgs.join(";  "));

            (total_cost, msg)
        } else {
            let total_cost: f64 = cost_iter.map(|(_, _, c)| c).sum();

            (total_cost, None)
        };

        (total_cost.sqrt(), msg)
    }
}
