//! The `metrics` module provides a trait for trigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};
use priority_queue::DoublePriorityQueue;

pub mod irregularity;
pub mod no_handswitch_in_trigram;
pub mod secondary_bigrams;
pub mod trigram_finger_repeats;

const N_WORST: usize = 3;

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
        let mut worst = DoublePriorityQueue::new();
        let mut cost_with_mod = 0.0;
        let total_weight = total_weight.unwrap_or_else(|| trigrams.iter().map(|(_, w)| w).sum());
        let total_cost = trigrams
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
