//! The `metrics` module provides a trait for bigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};

use ordered_float::OrderedFloat;
use priority_queue::DoublePriorityQueue;
use std::{env, fmt};

pub mod finger_repeats;
pub mod kla_distance;
pub mod kla_finger_usage;
pub mod kla_same_finger;
pub mod manual_bigram_penalty;
pub mod movement_pattern;
pub mod no_handswitch_after_unbalancing_key;
pub mod symmetric_handswitches;

/// BigramMetric is a trait for metrics that iterates over weighted bigrams.
pub trait BigramMetric: Send + Sync + BigramMetricClone + fmt::Debug {
    /// Return the name of the metric.
    fn name(&self) -> &str;

    /// Compute the cost of one bigram (if that is possible, otherwise, return `None`).
    #[inline(always)]
    fn individual_cost(
        &self,
        _key1: &LayerKey,
        _key2: &LayerKey,
        _weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        None
    }

    /// Compute the total cost for the metric.
    fn total_cost(
        &self,
        bigrams: &[((&LayerKey, &LayerKey), f64)],
        // total_weight is optional for performance reasons (it can be computed from bigrams).
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

        let total_weight = total_weight.unwrap_or_else(|| bigrams.iter().map(|(_, w)| w).sum());
        let cost_iter = bigrams
            .iter()
            .enumerate()
            .filter_map(|(i, (bigram, weight))| {
                let cost_option =
                    self.individual_cost(bigram.0, bigram.1, *weight, total_weight, layout);

                cost_option.map(|cost| (i, bigram, cost))
            });

        let (total_cost, msg) = if show_worst {
            let (total_cost, worst, worst_nonfixed) = cost_iter.fold(
                (0.0, DoublePriorityQueue::new(), DoublePriorityQueue::new()),
                |(mut total_cost, mut worst, mut worst_nonfixed), (i, bigram, cost)| {
                    total_cost += cost;

                    worst.push(i, OrderedFloat(cost.abs()));
                    if !bigram.0.is_fixed && !bigram.1.is_fixed {
                        worst_nonfixed.push(i, OrderedFloat(cost.abs()));
                    }

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
                        let (gram, _) = bigrams[i];
                        format!(
                            "{}{} ({:>5.2}%)",
                            gram.0,
                            gram.1,
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

        (total_cost, msg)
    }
}

impl Clone for Box<dyn BigramMetric> {
    fn clone(&self) -> Box<dyn BigramMetric> {
        self.clone_box()
    }
}

/// Helper trait for realizing clonability for `Box<dyn BigramMetric>`.
pub trait BigramMetricClone {
    fn clone_box(&self) -> Box<dyn BigramMetric>;
}

impl<T> BigramMetricClone for T
where
    T: 'static + BigramMetric + Clone,
{
    fn clone_box(&self) -> Box<dyn BigramMetric> {
        Box::new(self.clone())
    }
}
