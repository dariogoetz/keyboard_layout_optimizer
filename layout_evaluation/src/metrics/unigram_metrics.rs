//! The `metrics` module provides a trait for unigram metrics.
use std::usize;

use keyboard_layout::layout::{LayerKey, Layout};
use priority_queue::DoublePriorityQueue;

pub mod finger_balance;
pub mod hand_disbalance;
pub mod key_costs;

const N_WORST: usize = 3;

/// UnigramMetric is a trait for metrics that iterate over weighted unigrams.
pub trait UnigramMetric: Send + Sync + UnigramMetricClone + std::fmt::Debug {
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
        let mut worst = DoublePriorityQueue::new();

        let total_weight = total_weight.unwrap_or_else(|| unigrams.iter().map(|(_, w)| w).sum());
        let total_cost = unigrams
            .iter()
            .filter_map(|(unigram, weight)| {
                let res = self.individual_cost(*unigram, *weight, total_weight, layout);
                if let Some(res) = res {
                    worst.push(unigram.symbol, (1_000_000.0 * res) as usize);
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
            .map(|(unigram, cost)| {
                format!(
                    "{} ({:>5.2}%)",
                    unigram.to_string().escape_debug(),
                    100.0 * (cost as f64 / 1_000_000.0) / total_cost,
                )
            })
            .collect();

        let msg = Some(format!("Worst unigrams: {}", msgs.join(", ")));

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
