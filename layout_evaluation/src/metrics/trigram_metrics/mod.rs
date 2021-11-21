//! The `metrics` module provides a trait for trigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};

pub mod irregularity;
pub mod secondary_bigrams;
pub mod no_handswitch_in_trigram;

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
        let mut worst: Option<((&LayerKey, &LayerKey, &LayerKey), f64)> = None;
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
                    match worst {
                        Some((_, worst_cost)) => {
                            if res > worst_cost {
                                worst = Some((trigram.clone(), res));
                            }
                        },
                        None => {
                            if res > 0.0 {
                                worst = Some((trigram.clone(), res));
                            }
                        },
                    };
                };

                res
            })
            .sum();

        let msg = worst.map(|(trigram, cost)| {
            format!(
                "Worst trigram: {}{}{} makes {:>5.2}% of total cost;  {:>5.2}% of cost involved a modifier",
                trigram.0.symbol.to_string().escape_debug(),
                trigram.1.symbol.to_string().escape_debug(),
                trigram.2.symbol.to_string().escape_debug(),
                100.0 * cost / total_cost,
                100.0 * cost_with_mod / total_cost,
            )
        });

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
