//! The `metrics` module provides a trait for unigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};

pub mod finger_balance;
pub mod hand_disbalance;
pub mod key_costs;

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
        let mut worst: Option<(&LayerKey, f64)> = None;
        let total_weight = total_weight.unwrap_or_else(|| unigrams.iter().map(|(_, w)| w).sum());
        let total_cost = unigrams
            .iter()
            .filter_map(|(unigram, weight)| {
                let res = self.individual_cost(*unigram, *weight, total_weight, layout);
                if let Some(res) = res {
                    match worst {
                        Some((_, worst_cost)) => {
                            if res > worst_cost {
                                worst = Some((unigram.clone(), res));
                            }
                        },
                        None => {
                            if res > 0.0 {
                                worst = Some((unigram.clone(), res));
                            }
                        },
                    };
                };

                res
            })
            .sum();

        let msg = worst.map(|(unigram, cost)| {
            format!(
                "Worst unigram: {}, Cost: {:.2}% of total cost",
                unigram.symbol.to_string().escape_debug(),
                100.0 * cost / total_cost,
            )
        });

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
