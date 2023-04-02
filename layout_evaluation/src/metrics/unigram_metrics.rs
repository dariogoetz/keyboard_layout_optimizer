//! The `metrics` module provides a trait for unigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};
use ordered_float::OrderedFloat;
use priority_queue::DoublePriorityQueue;

use std::{env, fmt};

pub mod finger_balance;
pub mod hand_disbalance;
pub mod key_costs;
pub mod row_loads;
pub mod modifier_usage;

/// UnigramMetric is a trait for metrics that iterate over weighted unigrams.
pub trait UnigramMetric: Send + Sync + UnigramMetricClone + fmt::Debug {
    /// Return the name of the metric
    fn name(&self) -> &str;

    /// Compute the cost of one unigram (if that is possible, otherwise, return `None`).
    #[inline(always)]
    fn individual_cost(
        &self,
        _key1: &LayerKey,
        _weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        None
    }

    /// Compute the total cost for the metric.
    fn total_cost(
        &self,
        unigrams: &[(&LayerKey, f64)],
        // total_weight is optional for performance reasons (it can be computed from unigrams)
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

        let total_weight = total_weight.unwrap_or_else(|| unigrams.iter().map(|(_, w)| w).sum());
        let cost_iter = unigrams
            .iter()
            .enumerate()
            .filter_map(|(i, (unigram, weight))| {
                let cost_option = self.individual_cost(*unigram, *weight, total_weight, layout);

                cost_option.map(|cost| (i, unigram, cost))
            });

        let (total_cost, msg) = if show_worst {
            let (total_cost, worst) = cost_iter.fold(
                (0.0, DoublePriorityQueue::new()),
                |(mut total_cost, mut worst), (i, _, cost)| {
                    total_cost += cost;
                    worst.push(i, OrderedFloat(cost.abs()));
                    if worst.len() > n_worst {
                        worst.pop_min();
                    }

                    (total_cost, worst)
                },
            );

            let mut msgs = Vec::new();

            let worst_msgs: Vec<String> = worst
                .into_sorted_iter()
                .rev()
                .filter(|(_, cost)| cost.into_inner() > 0.0)
                .map(|(i, cost)| {
                    let (gram, _) = unigrams[i];
                    format!(
                        "{} ({:>5.2}%)",
                        gram,
                        100.0 * cost.into_inner() / total_cost,
                    )
                })
                .collect();

            if !worst_msgs.is_empty() {
                msgs.push(format!("Worst unigrams: {}", worst_msgs.join(", ")))
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

impl Clone for Box<dyn UnigramMetric> {
    fn clone(&self) -> Box<dyn UnigramMetric> {
        self.clone_box()
    }
}

/// Helper trait for realizing clonability for `Box<dyn UnigramMetric>`.
pub trait UnigramMetricClone {
    fn clone_box(&self) -> Box<dyn UnigramMetric>;
}

impl<T> UnigramMetricClone for T
where
    T: 'static + UnigramMetric + Clone,
{
    fn clone_box(&self) -> Box<dyn UnigramMetric> {
        Box::new(self.clone())
    }
}
