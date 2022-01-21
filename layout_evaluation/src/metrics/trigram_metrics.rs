//! The `metrics` module provides a trait for trigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};
use ordered_float::OrderedFloat;
use priority_queue::DoublePriorityQueue;

use std::env;

pub mod irregularity;
pub mod no_handswitch_in_trigram;
pub mod rolls;
pub mod secondary_bigrams;
pub mod trigram_finger_repeats;

/// TrigramMetric is a trait for metrics that iterates over weighted trigrams.
pub trait TrigramMetric: Send + Sync + TrigramMetricClone + std::fmt::Debug {
    /// Return the name of the metric.
    fn name(&self) -> &str;

    /// Compute the cost of one trigram (if that is possible, otherwise, return `None`).
    #[inline(always)]
    fn individual_cost(
        &self,
        _key1: &LayerKey,
        _key2: &LayerKey,
        _key3: &LayerKey,
        _weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        None
    }

    /// Compute the total cost for the metric.
    fn total_cost(
        &self,
        trigrams: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
        // total_weight is optional for performance reasons (it can be computed from trigrams)
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

        let total_weight = total_weight.unwrap_or_else(|| trigrams.iter().map(|(_, w)| w).sum());
        let cost_iter = trigrams.iter().filter_map(|(trigram, weight)| {
            let res = self.individual_cost(
                trigram.0,
                trigram.1,
                trigram.2,
                *weight,
                total_weight,
                layout,
            );

            res.map(|c| (trigram, c))
        });

        let (total_cost, msg) = if show_worst {
            let (total_cost, worst, worst_nonfixed) = cost_iter.fold(
                (0.0, DoublePriorityQueue::new(), DoublePriorityQueue::new()),
                |(mut total_cost, mut worst, mut worst_nonfixed), (trigram, cost)| {
                    total_cost += cost;

                    if !trigram.0.is_fixed && !trigram.1.is_fixed && !trigram.2.is_fixed {
                        worst_nonfixed.push(
                            (trigram.0.symbol, trigram.1.symbol, trigram.2.symbol),
                            OrderedFloat(cost.abs()),
                        );
                    }
                    worst.push(
                        (trigram.0.symbol, trigram.1.symbol, trigram.2.symbol),
                        OrderedFloat(cost.abs()),
                    );

                    if worst.len() > n_worst {
                        worst.pop_min();
                    }
                    if worst_nonfixed.len() > n_worst {
                        worst_nonfixed.pop_min();
                    }

                    (total_cost, worst, worst_nonfixed)
                },
            );

            let gen_msgs = |q: DoublePriorityQueue<(char, char, char), OrderedFloat<f64>>| {
                let worst_msgs: Vec<String> = q
                    .into_sorted_iter()
                    .rev()
                    .filter(|(_, cost)| cost.into_inner() > 0.0)
                    .map(|(gram, cost)| {
                        format!(
                            "{}{}{} ({:>5.2}%)",
                            gram.0.to_string().escape_debug(),
                            gram.1.to_string().escape_debug(),
                            gram.2.to_string().escape_debug(),
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
            let total_cost: f64 = cost_iter.map(|(_, c)| c).sum();

            (total_cost, None)
        };

        (total_cost, msg)
    }
}

impl Clone for Box<dyn TrigramMetric> {
    fn clone(&self) -> Box<dyn TrigramMetric> {
        self.clone_box()
    }
}

/// Helper trait for realizing clonability for `Box<dyn TrigramMetric>`.
pub trait TrigramMetricClone {
    fn clone_box(&self) -> Box<dyn TrigramMetric>;
}

impl<T> TrigramMetricClone for T
where
    T: 'static + TrigramMetric + Clone,
{
    fn clone_box(&self) -> Box<dyn TrigramMetric> {
        Box::new(self.clone())
    }
}
