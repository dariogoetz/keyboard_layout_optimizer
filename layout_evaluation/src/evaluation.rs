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
use crate::{
    metrics::{bigram_metrics::*, layout_metrics::*, trigram_metrics::*, unigram_metrics::*},
    ngram_mapper::NgramMapper,
};

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
    pub shortcut_keys: Option<WeightedParams<shortcut_keys::Parameters>>,
    pub similar_letters: Option<WeightedParams<similar_letters::Parameters>>,
    pub similar_letter_groups: Option<WeightedParams<similar_letter_groups::Parameters>>,

    pub finger_balance: Option<WeightedParams<finger_balance::Parameters>>,
    pub hand_disbalance: Option<WeightedParams<hand_disbalance::Parameters>>,
    pub row_loads: Option<WeightedParams<row_loads::Parameters>>,
    pub key_costs: Option<WeightedParams<key_costs::Parameters>>,
    pub modifier_usage: Option<WeightedParams<modifier_usage::Parameters>>,

    pub symmetric_handswitches: Option<WeightedParams<symmetric_handswitches::Parameters>>,
    pub finger_repeats: Option<WeightedParams<finger_repeats::Parameters>>,
    pub manual_bigram_penalty: Option<WeightedParams<manual_bigram_penalty::Parameters>>,
    pub movement_pattern: Option<WeightedParams<movement_pattern::Parameters>>,
    pub no_handswitch_after_unbalancing_key:
        Option<WeightedParams<no_handswitch_after_unbalancing_key::Parameters>>,

    pub irregularity: Option<WeightedParams<irregularity::Parameters>>,
    pub no_handswitch_in_trigram: Option<WeightedParams<no_handswitch_in_trigram::Parameters>>,
    pub secondary_bigrams: Option<WeightedParams<secondary_bigrams::Parameters>>,
    pub trigram_finger_repeats: Option<WeightedParams<trigram_finger_repeats::Parameters>>,
    pub trigram_rolls: Option<WeightedParams<trigram_rolls::Parameters>>,

    pub kla_home_key_words: Option<WeightedParams<kla_home_key_words::Parameters>>,
    pub kla_same_finger_words: Option<WeightedParams<kla_same_finger_words::Parameters>>,
    pub kla_distance: Option<WeightedParams<kla_distance::Parameters>>,
    pub kla_finger_usage: Option<WeightedParams<kla_finger_usage::Parameters>>,
    pub kla_same_finger: Option<WeightedParams<kla_same_finger::Parameters>>,
    pub kla_same_hand: Option<WeightedParams<kla_same_hand::Parameters>>,

    pub oxey_combined_trigram: Option<WeightedParams<oxey_combined_trigram::Parameters>>,

    pub oxey_sfbs: Option<WeightedParams<oxey_sfbs::Parameters>>,
    pub oxey_lsbs: Option<WeightedParams<oxey_lsbs::Parameters>>,
    pub oxey_dsfbs: Option<WeightedParams<oxey_dsfbs::Parameters>>,
    pub oxey_inward_rolls: Option<WeightedParams<oxey_inward_rolls::Parameters>>,
    pub oxey_outward_rolls: Option<WeightedParams<oxey_outward_rolls::Parameters>>,
    pub oxey_onehands: Option<WeightedParams<oxey_onehands::Parameters>>,
    pub oxey_alternates: Option<WeightedParams<oxey_alternates::Parameters>>,
    pub oxey_alternates_sfs: Option<WeightedParams<oxey_alternates_sfs::Parameters>>,
    pub oxey_redirects: Option<WeightedParams<oxey_redirects::Parameters>>,
    pub oxey_bad_redirects: Option<WeightedParams<oxey_bad_redirects::Parameters>>,
}

/// The [`Evaluator`] object is responsible for evaluating multiple metrics with respect to given ngram data.
/// The metrics are handled as dynamically dispatched trait objects for the metric traits in the `metrics` module.
#[derive(Clone, Debug)]
pub struct Evaluator {
    layout_metrics: Vec<(f64, NormalizationType, Box<dyn LayoutMetric>)>,
    unigram_metrics: Vec<(f64, NormalizationType, Box<dyn UnigramMetric>)>,
    bigram_metrics: Vec<(f64, NormalizationType, Box<dyn BigramMetric>)>,
    trigram_metrics: Vec<(f64, NormalizationType, Box<dyn TrigramMetric>)>,
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
        macro_rules! add_metric {
            ($metric_type:ident, $metric_name:ident, $metric_struct:ident) => {
                if let Some(p) = &params.$metric_name {
                    if p.enabled {
                        self.$metric_type(
                            Box::new($metric_name::$metric_struct::new(&p.params)),
                            p.weight,
                            p.normalization.clone(),
                        );
                    }
                }
            };
            ($metric_type:ident, $metric_name:ident, $metric_struct:ident, "add_bigram_metrics") => {
                if let Some(p) = &params.$metric_name {
                    if p.enabled {
                        self.$metric_type(
                            Box::new($metric_name::$metric_struct::new(
                                self.bigram_metrics.clone(),
                                &p.params,
                            )),
                            p.weight,
                            p.normalization.clone(),
                        );
                    }
                }
            };
        }

        // layout metrics
        add_metric!(layout_metric, shortcut_keys, ShortcutKeys);
        add_metric!(layout_metric, similar_letters, SimilarLetters);
        add_metric!(layout_metric, similar_letter_groups, SimilarLetterGroups);

        // unigram metrics
        add_metric!(unigram_metric, finger_balance, FingerBalance);
        add_metric!(unigram_metric, hand_disbalance, HandDisbalance);
        add_metric!(unigram_metric, row_loads, RowLoads);
        add_metric!(unigram_metric, modifier_usage, ModifierUsage);
        add_metric!(unigram_metric, key_costs, KeyCost);

        // bigram metrics
        add_metric!(bigram_metric, finger_repeats, FingerRepeats);
        add_metric!(bigram_metric, manual_bigram_penalty, ManualBigramPenalty);
        add_metric!(bigram_metric, movement_pattern, MovementPattern);
        add_metric!(
            bigram_metric,
            no_handswitch_after_unbalancing_key,
            NoHandSwitchAfterUnbalancingKey
        );
        add_metric!(bigram_metric, symmetric_handswitches, SymmetricHandswitches);

        // trigram_metrics
        add_metric!(
            trigram_metric,
            no_handswitch_in_trigram,
            NoHandswitchInTrigram
        );
        add_metric!(trigram_metric, trigram_finger_repeats, TrigramFingerRepeats);
        add_metric!(trigram_metric, trigram_rolls, TrigramRolls);
        add_metric!(
            trigram_metric,
            irregularity,
            Irregularity,
            "add_bigram_metrics"
        );
        add_metric!(
            trigram_metric,
            secondary_bigrams,
            SecondaryBigrams,
            "add_bigram_metrics"
        );

        add_metric!(layout_metric, kla_same_finger_words, KLASameFingerWords);
        add_metric!(layout_metric, kla_home_key_words, KLAHomeKeyWords);

        add_metric!(bigram_metric, kla_distance, KLADistance);
        add_metric!(bigram_metric, kla_finger_usage, KLAFingerUsage);
        add_metric!(bigram_metric, kla_same_finger, KLASameFinger);
        add_metric!(bigram_metric, kla_same_hand, KLASameHand);

        add_metric!(trigram_metric, oxey_combined_trigram, OxeyCombinedTrigram);

        add_metric!(bigram_metric, oxey_sfbs, OxeySfbs);
        add_metric!(bigram_metric, oxey_lsbs, OxeyLsbs);
        add_metric!(trigram_metric, oxey_dsfbs, OxeyDsfbs);
        add_metric!(trigram_metric, oxey_inward_rolls, OxeyInwardRolls);
        add_metric!(trigram_metric, oxey_outward_rolls, OxeyOutwardRolls);
        add_metric!(trigram_metric, oxey_onehands, OxeyOnehands);
        add_metric!(trigram_metric, oxey_alternates, OxeyAlternates);
        add_metric!(trigram_metric, oxey_alternates_sfs, OxeyAlternatesSfs);
        add_metric!(trigram_metric, oxey_redirects, OxeyRedirects);
        add_metric!(trigram_metric, oxey_bad_redirects, OxeyBadRedirects);

        self
    }

    /// Add a metric that operates only on the layout itself ("layout metric").
    pub fn layout_metric(
        &mut self,
        metric: Box<dyn LayoutMetric>,
        weight: f64,
        normalization: NormalizationType,
    ) {
        self.layout_metrics.push((weight, normalization, metric));
    }

    /// Add a metric that operates on the unigram data ("unigram metric").
    pub fn unigram_metric(
        &mut self,
        metric: Box<dyn UnigramMetric>,
        weight: f64,
        normalization: NormalizationType,
    ) {
        self.unigram_metrics.push((weight, normalization, metric));
    }

    /// Add a metric that operates on the bigram data ("bigram metric").
    pub fn bigram_metric(
        &mut self,
        metric: Box<dyn BigramMetric>,
        weight: f64,
        normalization: NormalizationType,
    ) {
        self.bigram_metrics.push((weight, normalization, metric));
    }

    /// Add a metric that operates on the trigram data ("trigram metric").
    pub fn trigram_metric(
        &mut self,
        metric: Box<dyn TrigramMetric>,
        weight: f64,
        normalization: NormalizationType,
    ) {
        self.trigram_metrics.push((weight, normalization, metric));
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
        if !self.layout_metrics.is_empty() {
            let metric_costs = self.evaluate_layout_metrics(layout);
            let mut layout_costs = MetricResults::new(MetricType::Layout, 1.0, 0.0);
            metric_costs
                .into_iter()
                .for_each(|mc| layout_costs.add_result(mc));
            results.push(layout_costs);
        }

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
