//! The `evaluation` module provides an [`Evaluator`] struct that can evaluate
//! layouts with respect to a list of metrics and ngram data.
//!
//! It can hold multiple metrics operating on the layout itself, unigrams, bigrams,
//! or trigrams. These are required to implement the corresponding trait from the `metrics` module.
//!
//! The ngram mapper is responsible for mapping char-based ngrams (as read from input data)
//! to singles, pairs, and triplets of [`LayerKey`]s that can then be analysed by the individual metrics.

use crate::results::{
    EvaluationResult, MetricResult, MetricResults, MetricType, NormalizationType,
};
use crate::{metrics::*, ngram_mapper::NgramMapper};

use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

/// A wrapper around individuals metric's parameters (`T`) specifying
/// additional generic attributes. This mostly facilitates configuration of
/// metrics in a config file.
#[derive(Clone, Deserialize, Debug)]
pub struct WeightedParams<T> {
    /// Wether the metric is to be evaluated.
    pub enabled: bool,
    /// The weight to use when aggregating all metrics.
    pub weight: f64,
    /// The normalization strategy to use.
    pub normalization: NormalizationType,
    /// The metric's individual parameters.
    pub params: T,
}

/// Compiles configuration parameters for all "default" metrics available.
/// This is usually read from a config file.
#[derive(Clone, Deserialize, Debug)]
pub struct MetricParameters {
    pub shortcut_keys: WeightedParams<layout_metrics::shortcut_keys::Parameters>,
    pub similar_letters: WeightedParams<layout_metrics::similar_letters::Parameters>,
    pub similar_letter_groups: WeightedParams<layout_metrics::similar_letter_groups::Parameters>,

    pub finger_balance: WeightedParams<unigram_metrics::finger_balance::Parameters>,
    pub hand_disbalance: WeightedParams<unigram_metrics::hand_disbalance::Parameters>,
    pub row_loads: WeightedParams<unigram_metrics::row_loads::Parameters>,
    pub key_costs: WeightedParams<unigram_metrics::key_costs::Parameters>,

    pub symmetric_handswitches: WeightedParams<bigram_metrics::symmetric_handswitches::Parameters>,
    pub finger_repeats: WeightedParams<bigram_metrics::finger_repeats::Parameters>,
    pub manual_bigram_penalty: WeightedParams<bigram_metrics::manual_bigram_penalty::Parameters>,
    pub movement_pattern: WeightedParams<bigram_metrics::movement_pattern::Parameters>,
    pub no_handswitch_after_unbalancing_key:
        WeightedParams<bigram_metrics::no_handswitch_after_unbalancing_key::Parameters>,
    pub unbalancing_after_neighboring:
        WeightedParams<bigram_metrics::unbalancing_after_neighboring::Parameters>,

    pub irregularity: WeightedParams<trigram_metrics::irregularity::Parameters>,
    pub no_handswitch_in_trigram:
        WeightedParams<trigram_metrics::no_handswitch_in_trigram::Parameters>,
    pub secondary_bigrams: WeightedParams<trigram_metrics::secondary_bigrams::Parameters>,
    pub trigram_finger_repeats: WeightedParams<trigram_metrics::trigram_finger_repeats::Parameters>,
    pub trigram_rolls: WeightedParams<trigram_metrics::rolls::Parameters>,
}

/// The [`Evaluator`] object is responsible for evaluating multiple metrics with respect to given ngram data.
/// The metrics are handled as dynamically dispatched trait objects for the metric traits in the `metrics` module.
#[derive(Clone, Debug)]
pub struct Evaluator {
    layout_metrics: Vec<(
        f64,
        NormalizationType,
        Box<dyn layout_metrics::LayoutMetric>,
    )>,
    unigram_metrics: Vec<(
        f64,
        NormalizationType,
        Box<dyn unigram_metrics::UnigramMetric>,
    )>,
    bigram_metrics: Vec<(
        f64,
        NormalizationType,
        Box<dyn bigram_metrics::BigramMetric>,
    )>,
    trigram_metrics: Vec<(
        f64,
        NormalizationType,
        Box<dyn trigram_metrics::TrigramMetric>,
    )>,
    ngram_mapper: Box<dyn NgramMapper>,
}

impl Evaluator {
    /// Generate an "empty" [`Evaluator`] object without any metric.
    pub fn default(ngram_mapper: Box<dyn NgramMapper>) -> Self {
        Evaluator {
            layout_metrics: Vec::new(),
            unigram_metrics: Vec::new(),
            bigram_metrics: Vec::new(),
            trigram_metrics: Vec::new(),
            ngram_mapper,
        }
    }

    /// Add all "default" metrics to the evaluator.
    pub fn default_metrics(mut self, params: &MetricParameters) -> Self {
        // layout metrics
        self.layout_metric(
            Box::new(layout_metrics::shortcut_keys::ShortcutKeys::new(
                &params.shortcut_keys.params,
            )),
            params.shortcut_keys.weight,
            params.shortcut_keys.normalization.clone(),
            params.shortcut_keys.enabled,
        );
        self.layout_metric(
            Box::new(layout_metrics::similar_letters::SimilarLetters::new(
                &params.similar_letters.params,
            )),
            params.similar_letters.weight,
            params.similar_letters.normalization.clone(),
            params.similar_letters.enabled,
        );
        self.layout_metric(
            Box::new(
                layout_metrics::similar_letter_groups::SimilarLetterGroups::new(
                    &params.similar_letter_groups.params,
                ),
            ),
            params.similar_letter_groups.weight,
            params.similar_letter_groups.normalization.clone(),
            params.similar_letter_groups.enabled,
        );

        // unigram metrics
        self.unigram_metric(
            Box::new(unigram_metrics::finger_balance::FingerBalance::new(
                &params.finger_balance.params,
            )),
            params.finger_balance.weight,
            params.finger_balance.normalization.clone(),
            params.finger_balance.enabled,
        );
        self.unigram_metric(
            Box::new(unigram_metrics::hand_disbalance::HandDisbalance::new(
                &params.hand_disbalance.params,
            )),
            params.hand_disbalance.weight,
            params.hand_disbalance.normalization.clone(),
            params.hand_disbalance.enabled,
        );
        self.unigram_metric(
            Box::new(unigram_metrics::row_loads::RowLoads::new(
                &params.row_loads.params,
            )),
            params.row_loads.weight,
            params.row_loads.normalization.clone(),
            params.row_loads.enabled,
        );
        self.unigram_metric(
            Box::new(unigram_metrics::key_costs::KeyCost::new(
                &params.key_costs.params,
            )),
            params.key_costs.weight,
            params.key_costs.normalization.clone(),
            params.key_costs.enabled,
        );

        // bigram metrics
        self.bigram_metric(
            Box::new(bigram_metrics::finger_repeats::FingerRepeats::new(
                &params.finger_repeats.params,
            )),
            params.finger_repeats.weight,
            params.finger_repeats.normalization.clone(),
            params.finger_repeats.enabled,
        );
        self.bigram_metric(
            Box::new(
                bigram_metrics::manual_bigram_penalty::ManualBigramPenalty::new(
                    &params.manual_bigram_penalty.params,
                ),
            ),
            params.manual_bigram_penalty.weight,
            params.manual_bigram_penalty.normalization.clone(),
            params.manual_bigram_penalty.enabled,
        );
        self.bigram_metric(
            Box::new(bigram_metrics::movement_pattern::MovementPattern::new(
                &params.movement_pattern.params,
            )),
            params.movement_pattern.weight,
            params.movement_pattern.normalization.clone(),
            params.movement_pattern.enabled,
        );
        self.bigram_metric(
            Box::new(
                bigram_metrics::no_handswitch_after_unbalancing_key::NoHandSwitchAfterUnbalancingKey::new(
                    &params.no_handswitch_after_unbalancing_key.params,
                ),
            ),
            params.no_handswitch_after_unbalancing_key.weight,
            params
                .no_handswitch_after_unbalancing_key
                .normalization
                .clone(),
            params.no_handswitch_after_unbalancing_key.enabled,
        );
        self.bigram_metric(
            Box::new(
                bigram_metrics::unbalancing_after_neighboring::UnbalancingAfterNeighboring::new(
                    &params.unbalancing_after_neighboring.params,
                ),
            ),
            params.unbalancing_after_neighboring.weight,
            params.unbalancing_after_neighboring.normalization.clone(),
            params.unbalancing_after_neighboring.enabled,
        );
        self.bigram_metric(
            Box::new(
                bigram_metrics::symmetric_handswitches::SymmetricHandswitches::new(
                    &params.symmetric_handswitches.params,
                ),
            ),
            params.symmetric_handswitches.weight,
            params.symmetric_handswitches.normalization.clone(),
            params.symmetric_handswitches.enabled,
        );

        // trigram_metrics
        self.trigram_metric(
            Box::new(trigram_metrics::irregularity::Irregularity::new(
                self.bigram_metrics.clone(),
                &params.irregularity.params,
            )),
            params.irregularity.weight,
            params.irregularity.normalization.clone(),
            params.irregularity.enabled,
        );
        self.trigram_metric(
            Box::new(
                trigram_metrics::no_handswitch_in_trigram::NoHandswitchInTrigram::new(
                    &params.no_handswitch_in_trigram.params,
                ),
            ),
            params.no_handswitch_in_trigram.weight,
            params.no_handswitch_in_trigram.normalization.clone(),
            params.no_handswitch_in_trigram.enabled,
        );
        self.trigram_metric(
            Box::new(trigram_metrics::secondary_bigrams::SecondaryBigrams::new(
                self.bigram_metrics.clone(),
                &params.secondary_bigrams.params,
            )),
            params.secondary_bigrams.weight,
            params.secondary_bigrams.normalization.clone(),
            params.secondary_bigrams.enabled,
        );
        self.trigram_metric(
            Box::new(
                trigram_metrics::trigram_finger_repeats::TrigramFingerRepeats::new(
                    &params.trigram_finger_repeats.params,
                ),
            ),
            params.trigram_finger_repeats.weight,
            params.trigram_finger_repeats.normalization.clone(),
            params.trigram_finger_repeats.enabled,
        );
        self.trigram_metric(
            Box::new(trigram_metrics::rolls::TrigramRolls::new(
                &params.trigram_rolls.params,
            )),
            params.trigram_rolls.weight,
            params.trigram_rolls.normalization.clone(),
            params.trigram_rolls.enabled,
        );

        self
    }

    /// Add a metric that operates only on the layout itself ("layout metric").
    pub fn layout_metric(
        &mut self,
        metric: Box<dyn layout_metrics::LayoutMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) {
        if enabled {
            self.layout_metrics.push((weight, normalization, metric));
        }
    }

    /// Add a metric that operates on the unigram data ("unigram metric").
    pub fn unigram_metric(
        &mut self,
        metric: Box<dyn unigram_metrics::UnigramMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) {
        if enabled {
            self.unigram_metrics.push((weight, normalization, metric));
        }
    }

    /// Add a metric that operates on the bigram data ("bigram metric").
    pub fn bigram_metric(
        &mut self,
        metric: Box<dyn bigram_metrics::BigramMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) {
        if enabled {
            self.bigram_metrics.push((weight, normalization, metric));
        }
    }

    /// Add a metric that operates on the trigram data ("trigram metric").
    pub fn trigram_metric(
        &mut self,
        metric: Box<dyn trigram_metrics::TrigramMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) {
        if enabled {
            self.trigram_metrics.push((weight, normalization, metric));
        }
    }

    /// Evaluate all layout metrics for a layout.
    fn evaluate_layout_metrics(&self, layout: &Layout) -> Vec<MetricResult> {
        if self.layout_metrics.is_empty() {
            return Vec::new();
        }

        let metric_costs: Vec<MetricResult> = self
            .layout_metrics
            .iter()
            .map(|(weight, normalization, metric)| {
                let (cost, message) = metric.total_cost(layout);
                MetricResult {
                    name: metric.name().to_string(),
                    cost,
                    weight: *weight,
                    normalization: normalization.clone(),
                    message,
                }
            })
            .collect();

        metric_costs
    }

    /// Evaluate all unigram metrics for a layout.
    fn evaluate_unigram_metrics(
        &self,
        layout: &Layout,
        keys: &[(&LayerKey, f64)],
    ) -> Vec<MetricResult> {
        if self.unigram_metrics.is_empty() {
            return Vec::new();
        }

        let total_weight = keys.iter().map(|(_, w)| w).sum();
        let metric_costs: Vec<MetricResult> = self
            .unigram_metrics
            .iter()
            .map(|(weight, normalization, metric)| {
                let (cost, message) = metric.total_cost(keys, Some(total_weight), layout);
                MetricResult {
                    name: metric.name().to_string(),
                    cost,
                    weight: *weight,
                    normalization: normalization.clone(),
                    message,
                }
            })
            .collect();

        metric_costs
    }

    /// Evaluate all bigram metrics for a layout.
    fn evaluate_bigram_metrics(
        &self,
        layout: &Layout,
        keys: &[((&LayerKey, &LayerKey), f64)],
    ) -> Vec<MetricResult> {
        if self.bigram_metrics.is_empty() {
            return Vec::new();
        }

        let total_weight = keys.iter().map(|(_, w)| w).sum();
        let metric_costs: Vec<MetricResult> = self
            .bigram_metrics
            .iter()
            .map(|(weight, normalization, metric)| {
                let (cost, message) = metric.total_cost(keys, Some(total_weight), layout);
                MetricResult {
                    name: metric.name().to_string(),
                    cost,
                    weight: *weight,
                    normalization: normalization.clone(),
                    message,
                }
            })
            .collect();

        metric_costs
    }

    /// Evaluate all trigram metrics for a layout.
    fn evaluate_trigram_metrics<'s>(
        &self,
        layout: &'s Layout,
        keys: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
    ) -> Vec<MetricResult> {
        if self.trigram_metrics.is_empty() {
            return Vec::new();
        }

        let total_weight = keys.iter().map(|(_, w)| w).sum();
        let metric_costs: Vec<MetricResult> = self
            .trigram_metrics
            .iter()
            .map(|(weight, normalization, metric)| {
                let (cost, message) = metric.total_cost(keys, Some(total_weight), layout);
                MetricResult {
                    name: metric.name().to_string(),
                    cost,
                    weight: *weight,
                    normalization: normalization.clone(),
                    message,
                }
            })
            .collect();

        metric_costs
    }

    /// Evaluate all metrics for a layout.
    pub fn evaluate_layout(&self, layout: &Layout) -> EvaluationResult {
        let mut results: Vec<MetricResults> = Vec::new();

        // Layout metrics
        let metric_costs = self.evaluate_layout_metrics(layout);
        let mut layout_costs = MetricResults::new(MetricType::Layout, 1.0, 0.0);
        metric_costs
            .into_iter()
            .for_each(|mc| layout_costs.add_result(mc));
        results.push(layout_costs);

        // Unigram metrics
        if !self.unigram_metrics.is_empty() {
            let mapped_unigrams = self.ngram_mapper.map_unigrams(layout);
            let metric_costs = self.evaluate_unigram_metrics(layout, &mapped_unigrams.grams);
            let mut unigram_costs = MetricResults::new(
                MetricType::Unigram,
                mapped_unigrams.weight_found,
                mapped_unigrams.weight_not_found,
            );
            metric_costs
                .into_iter()
                .for_each(|mc| unigram_costs.add_result(mc));

            results.push(unigram_costs);
        }

        // Bigram metrics
        if !self.bigram_metrics.is_empty() {
            let mapped_bigrams = self.ngram_mapper.map_bigrams(layout);
            let metric_costs = self.evaluate_bigram_metrics(layout, &mapped_bigrams.grams);
            let mut bigram_costs = MetricResults::new(
                MetricType::Bigram,
                mapped_bigrams.weight_found,
                mapped_bigrams.weight_not_found,
            );
            metric_costs
                .into_iter()
                .for_each(|mc| bigram_costs.add_result(mc));

            results.push(bigram_costs);
        }

        // Trigram metrics
        if !self.trigram_metrics.is_empty() {
            let mapped_trigrams = self.ngram_mapper.map_trigrams(layout);
            let metric_costs = self.evaluate_trigram_metrics(layout, &mapped_trigrams.grams);
            let mut trigram_costs = MetricResults::new(
                MetricType::Trigram,
                mapped_trigrams.weight_found,
                mapped_trigrams.weight_not_found,
            );
            metric_costs
                .into_iter()
                .for_each(|mc| trigram_costs.add_result(mc));

            results.push(trigram_costs);
        }

        EvaluationResult::new(layout.as_text(), results)
    }
}
