//! The `metrics` module provides a trait for trigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};
use priority_queue::DoublePriorityQueue;
use float_ord::FloatOrd;

pub mod irregularity;
pub mod no_handswitch_in_trigram;
pub mod secondary_bigrams;
pub mod trigram_finger_repeats;

const SHOW_WORST: bool = true;
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

        let (total_cost, msg) = if SHOW_WORST {
            let (total_cost, cost_with_mod, worst) = cost_iter.fold(
                (0.0, 0.0, DoublePriorityQueue::new()),
                |(mut total_cost, mut cost_with_mod, mut worst), (trigram, cost)| {
                    total_cost += cost;

                    if trigram.0.is_modifier || trigram.1.is_modifier || trigram.2.is_modifier {
                        cost_with_mod += cost;
                    };

                    worst.push(
                        (trigram.0.symbol, trigram.1.symbol, trigram.2.symbol),
                        FloatOrd(cost),
                    );
                    if worst.len() > N_WORST {
                        worst.pop_min();
                    }

                    (total_cost, cost_with_mod, worst)
                },
            );

            let mut msgs = Vec::new();

            let worst_msgs: Vec<String> = worst
                .into_sorted_iter()
                .rev()
                .filter(|(_, cost)| cost.0 > 0.0)
                .map(|(trigram, cost)| {
                    format!(
                        "{}{}{} ({:>5.2}%)",
                        trigram.0.to_string().escape_debug(),
                        trigram.1.to_string().escape_debug(),
                        trigram.2.to_string().escape_debug(),
                        100.0 * cost.0 / total_cost,
                    )
                })
                .collect();
            if !worst_msgs.is_empty() {
                msgs.push(format!("Worst trigrams: {}", worst_msgs.join(", ")))
            }

            if total_cost > 0.0 {
                msgs.push(format!(
                    "{:>5.2}% of cost involved a modifier",
                    100.0 * cost_with_mod / total_cost,
                ));
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
