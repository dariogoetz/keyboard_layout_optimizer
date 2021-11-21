//! The `metrics` module provides a trait for bigram metrics.
use keyboard_layout::layout::{LayerKey, Layout};

pub mod asymmetric_bigrams;
pub mod finger_repeats;
pub mod finger_repeats_lateral;
pub mod finger_repeats_top_bottom;
pub mod line_changes;
pub mod manual_bigram_penalty;
pub mod movement_pattern;
pub mod no_handswitch_after_unbalancing_key;
pub mod unbalancing_after_neighboring;

/// BigramMetric is a trait for metrics that iterates over weighted bigrams.
pub trait BigramMetric: Send + Sync + BigramMetricClone + std::fmt::Debug {
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
        let mut worst: Option<((&LayerKey, &LayerKey), f64)> = None;
        let mut cost_with_mod = 0.0;
        let total_weight = total_weight.unwrap_or_else(|| bigrams.iter().map(|(_, w)| w).sum());
        let total_cost = bigrams
            .iter()
            .filter_map(|(bigram, weight)| {
                let res = self.individual_cost(bigram.0, bigram.1, *weight, total_weight, layout);
                if let Some(res) = res {
                    if bigram.0.is_modifier || bigram.1.is_modifier {
                        cost_with_mod += res;
                    };
                    match worst {
                        Some((_, worst_cost)) => {
                            if res > worst_cost {
                                worst = Some((bigram.clone(), res));
                            }
                        },
                        None => {
                            if res > 0.0 {
                                worst = Some((bigram.clone(), res));
                            }
                        },
                    };
                };

                res
            })
            .sum();

        let msg = worst.map(|(bigram, cost)| {
            format!(
                "Worst bigram: {}{} makes {:>5.2}% of total cost;  {:>5.2}% of cost involved a modifier",
                bigram.0.symbol.to_string().escape_debug(),
                bigram.1.symbol.to_string().escape_debug(),
                100.0 * cost / total_cost,
                100.0 * cost_with_mod / total_cost,
            )
        });

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
