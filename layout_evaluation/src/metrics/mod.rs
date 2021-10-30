//! The `metrics` module provides traits for layout, unigram, bigram, and trigram metrics.

use keyboard_layout::layout::{LayerKey, Layout};

pub mod hand_disbalance;
pub mod key_costs;
pub mod shortcut_keys;

pub mod asymmetric_bigrams;
pub mod asymmetric_keys;
pub mod finger_balance;
pub mod finger_repeats;
pub mod finger_repeats_top_bottom;
pub mod line_changes;
pub mod manual_bigram_penalty;
pub mod movement_pattern;
pub mod no_handswitch_after_unbalancing_key;
pub mod unbalancing_after_neighboring;

pub mod irregularity;
pub mod no_handswitch_in_trigram;

/// LayoutMetric is a trait for metrics that depends only on the layout.
pub trait LayoutMetric: Send + Sync + LayoutMetricClone + std::fmt::Debug {
    /// Return the name of the metric
    fn name(&self) -> &str;
    /// Compute the total cost for the metric
    fn total_cost(&self, layout: &Layout) -> (f64, Option<String>);
}

// in order to implement clone for Box<dyn LayoutMetric>, the following trick is necessary
// see https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
// alternative: use `dyn_clone` crate

impl Clone for Box<dyn LayoutMetric> {
    fn clone(&self) -> Box<dyn LayoutMetric> {
        self.clone_box()
    }
}

/// Helper trait for realizing clonability for `Box<dyn LayoutMetric>`.
pub trait LayoutMetricClone {
    fn clone_box(&self) -> Box<dyn LayoutMetric>;
}

impl<T> LayoutMetricClone for T
where
    T: 'static + LayoutMetric + Clone,
{
    fn clone_box(&self) -> Box<dyn LayoutMetric> {
        Box::new(self.clone())
    }
}

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
        let total_weight = total_weight.unwrap_or_else(|| bigrams.iter().map(|(_, w)| w).sum());
        let total_cost = bigrams
            .iter()
            .filter_map(|(bigram, weight)| {
                let res = self.individual_cost(bigram.0, bigram.1, *weight, total_weight, layout);
                if let Some(res) = res {
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
                "Worst bigram: {}{}, Cost: {:.2}% of total cost",
                bigram.0.symbol.to_string().escape_debug(),
                bigram.1.symbol.to_string().escape_debug(),
                100.0 * cost / total_cost,
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
                "Worst trigram: {}{}{}, Cost: {:.2}% of total cost",
                trigram.0.symbol.to_string().escape_debug(),
                trigram.1.symbol.to_string().escape_debug(),
                trigram.2.symbol.to_string().escape_debug(),
                100.0 * cost / total_cost,
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
