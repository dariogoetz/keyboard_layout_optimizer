use crate::results::{MetricResult, MetricResults, MetricType, NormalizationType};
use crate::{metrics::*, ngram_mapper::NgramMapper};

use keyboard_layout::layout::{LayerKey, Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct WeightedParams<T> {
    pub enabled: bool,
    pub weight: f64,
    pub normalization: NormalizationType,
    pub params: T,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MetricParameters {
    pub shortcut_keys: WeightedParams<shortcut_keys::Parameters>,
    pub asymmetric_keys: WeightedParams<asymmetric_keys::Parameters>,
    pub key_costs: WeightedParams<key_costs::Parameters>,
    pub hand_disbalance: WeightedParams<hand_disbalance::Parameters>,
    pub finger_repeats: WeightedParams<finger_repeats::Parameters>,
    pub finger_repeats_top_bottom: WeightedParams<finger_repeats_top_bottom::Parameters>,
    pub movement_pattern: WeightedParams<movement_pattern::Parameters>,
    pub no_handswitch_after_unbalancing_key:
        WeightedParams<no_handswitch_after_unbalancing_key::Parameters>,
    pub unbalancing_after_neighboring: WeightedParams<unbalancing_after_neighboring::Parameters>,
    pub finger_balance: WeightedParams<finger_balance::Parameters>,
    pub line_changes: WeightedParams<line_changes::Parameters>,
    pub asymmetric_bigrams: WeightedParams<asymmetric_bigrams::Parameters>,
    pub manual_bigram_penalty: WeightedParams<manual_bigram_penalty::Parameters>,
    pub irregularity: WeightedParams<irregularity::Parameters>,
    pub no_handswitch_in_trigram: WeightedParams<no_handswitch_in_trigram::Parameters>,
}

#[derive(Clone, Debug)]
pub struct Evaluator {
    layout_metrics: Vec<(f64, NormalizationType, Box<dyn LayoutMetric>)>,
    unigram_metrics: Vec<(f64, NormalizationType, Box<dyn UnigramMetric>)>,
    bigram_metrics: Vec<(f64, NormalizationType, Box<dyn BigramMetric>)>,
    trigram_metrics: Vec<(f64, NormalizationType, Box<dyn TrigramMetric>)>,
    ngram_mapper: Box<dyn NgramMapper>,
}

impl Evaluator {
    pub fn default(ngram_mapper: Box<dyn NgramMapper>) -> Self {
        Evaluator {
            layout_metrics: Vec::new(),
            unigram_metrics: Vec::new(),
            bigram_metrics: Vec::new(),
            trigram_metrics: Vec::new(),
            ngram_mapper,
        }
    }

    pub fn default_metrics(&mut self, params: &MetricParameters) -> Self {
        // layout metrics
        self.layout_metric(
            Box::new(shortcut_keys::ShortcutKeys::new(
                &params.shortcut_keys.params,
            )),
            params.shortcut_keys.weight,
            params.shortcut_keys.normalization.clone(),
            params.shortcut_keys.enabled,
        );
        self.layout_metric(
            Box::new(asymmetric_keys::AsymmetricKeys::new(
                &params.asymmetric_keys.params,
            )),
            params.asymmetric_keys.weight,
            params.asymmetric_keys.normalization.clone(),
            params.asymmetric_keys.enabled,
        );

        // unigram metrics
        self.unigram_metric(
            Box::new(key_costs::KeyCost::new(&params.key_costs.params)),
            params.key_costs.weight,
            params.key_costs.normalization.clone(),
            params.key_costs.enabled,
        );
        self.unigram_metric(
            Box::new(hand_disbalance::HandDisbalance::new(
                &params.hand_disbalance.params,
            )),
            params.hand_disbalance.weight,
            params.hand_disbalance.normalization.clone(),
            params.hand_disbalance.enabled,
        );
        self.unigram_metric(
            Box::new(finger_balance::FingerBalance::new(
                &params.finger_balance.params,
            )),
            params.finger_balance.weight,
            params.finger_balance.normalization.clone(),
            params.finger_balance.enabled,
        );

        // bigram metrics
        self.bigram_metric(
            Box::new(finger_repeats::FingerRepeats::new(
                &params.finger_repeats.params,
            )),
            params.finger_repeats.weight,
            params.finger_repeats.normalization.clone(),
            params.finger_repeats.enabled,
        );
        self.bigram_metric(
            Box::new(finger_repeats_top_bottom::FingerRepeatsTopBottom::new(
                &params.finger_repeats_top_bottom.params,
            )),
            params.finger_repeats_top_bottom.weight,
            params.finger_repeats_top_bottom.normalization.clone(),
            params.finger_repeats_top_bottom.enabled,
        );
        self.bigram_metric(
            Box::new(movement_pattern::MovementPattern::new(
                &params.movement_pattern.params,
            )),
            params.movement_pattern.weight,
            params.movement_pattern.normalization.clone(),
            params.movement_pattern.enabled,
        );
        self.bigram_metric(
            Box::new(
                no_handswitch_after_unbalancing_key::NoHandSwitchAfterUnbalancingKey::new(
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
                unbalancing_after_neighboring::UnbalancingAfterNeighboring::new(
                    &params.unbalancing_after_neighboring.params,
                ),
            ),
            params.unbalancing_after_neighboring.weight,
            params.unbalancing_after_neighboring.normalization.clone(),
            params.unbalancing_after_neighboring.enabled,
        );
        self.bigram_metric(
            Box::new(line_changes::LineChanges::new(&params.line_changes.params)),
            params.line_changes.weight,
            params.line_changes.normalization.clone(),
            params.line_changes.enabled,
        );
        self.bigram_metric(
            Box::new(asymmetric_bigrams::AsymmetricBigrams::new(
                &params.asymmetric_bigrams.params,
            )),
            params.asymmetric_bigrams.weight,
            params.asymmetric_bigrams.normalization.clone(),
            params.asymmetric_bigrams.enabled,
        );
        self.bigram_metric(
            Box::new(manual_bigram_penalty::ManualBigramPenalty::new(
                &params.manual_bigram_penalty.params,
            )),
            params.manual_bigram_penalty.weight,
            params.manual_bigram_penalty.normalization.clone(),
            params.manual_bigram_penalty.enabled,
        );

        // trigram_metrics
        self.trigram_metric(
            Box::new(irregularity::Irregularity::new(
                self.bigram_metrics.clone(),
                &params.irregularity.params,
            )),
            params.irregularity.weight,
            params.irregularity.normalization.clone(),
            params.irregularity.enabled,
        );

        self.trigram_metric(
            Box::new(no_handswitch_in_trigram::NoHandswitchInTrigram::new(
                &params.no_handswitch_in_trigram.params,
            )),
            params.no_handswitch_in_trigram.weight,
            params.no_handswitch_in_trigram.normalization.clone(),
            params.no_handswitch_in_trigram.enabled,
        );

        self.to_owned()
    }

    pub fn layout_metric(
        &mut self,
        metric: Box<dyn LayoutMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) -> Self {
        if enabled {
            self.layout_metrics.push((weight, normalization, metric));
        }
        self.to_owned()
    }

    pub fn unigram_metric(
        &mut self,
        metric: Box<dyn UnigramMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) -> Self {
        if enabled {
            self.unigram_metrics.push((weight, normalization, metric));
        }
        self.to_owned()
    }

    pub fn bigram_metric(
        &mut self,
        metric: Box<dyn BigramMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) -> Self {
        if enabled {
            self.bigram_metrics.push((weight, normalization, metric));
        }
        self.to_owned()
    }

    pub fn trigram_metric(
        &mut self,
        metric: Box<dyn TrigramMetric>,
        weight: f64,
        normalization: NormalizationType,
        enabled: bool,
    ) -> Self {
        if enabled {
            self.trigram_metrics.push((weight, normalization, metric));
        }
        self.to_owned()
    }

    fn evaluate_layout_metrics(&self, layout: &Layout) -> Vec<MetricResult> {
        if self.layout_metrics.is_empty() {
            return Vec::new();
        }

        let mut metric_costs: Vec<MetricResult> = Vec::new();
        for (weight, normalization, metric) in self.layout_metrics.iter() {
            let (cost, message) = metric.total_cost(layout);
            metric_costs.push(MetricResult {
                name: metric.name().to_string(),
                cost,
                weight: *weight,
                normalization: normalization.clone(),
                message,
            });
        }

        metric_costs
    }

    fn evaluate_unigram_metrics(
        &self,
        layout: &Layout,
        keys: &[(&LayerKey, f64)],
    ) -> Vec<MetricResult> {
        if self.unigram_metrics.is_empty() {
            return Vec::new();
        }

        let total_weight = keys.iter().map(|(_, w)| w).sum();
        let mut metric_costs: Vec<MetricResult> = Vec::new();
        for (weight, normalization, metric) in self.unigram_metrics.iter() {
            let (cost, message) = metric.total_cost(keys, Some(total_weight), layout);
            metric_costs.push(MetricResult {
                name: metric.name().to_string(),
                cost,
                weight: *weight,
                normalization: normalization.clone(),
                message,
            });
        }

        metric_costs
    }

    fn evaluate_bigram_metrics(
        &self,
        layout: &Layout,
        keys: &[((&LayerKey, &LayerKey), f64)],
    ) -> Vec<MetricResult> {
        if self.bigram_metrics.is_empty() {
            return Vec::new();
        }

        let total_weight = keys.iter().map(|(_, w)| w).sum();
        let mut metric_costs: Vec<MetricResult> = Vec::new();
        for (weight, normalization, metric) in self.bigram_metrics.iter() {
            let (cost, message) = metric.total_cost(keys, Some(total_weight), layout);
            metric_costs.push(MetricResult {
                name: metric.name().to_string(),
                cost,
                weight: *weight,
                normalization: normalization.clone(),
                message,
            });
        }

        metric_costs
    }

    fn evaluate_trigram_metrics<'s>(
        &self,
        layout: &'s Layout,
        keys: &[((&LayerKey, &LayerKey, &LayerKey), f64)],
    ) -> Vec<MetricResult> {
        if self.trigram_metrics.is_empty() {
            return Vec::new();
        }

        let total_weight = keys.iter().map(|(_, w)| w).sum();
        let mut metric_costs: Vec<MetricResult> = Vec::new();
        for (weight, normalization, metric) in self.trigram_metrics.iter() {
            let (cost, message) = metric.total_cost(keys, Some(total_weight), layout);
            metric_costs.push(MetricResult {
                name: metric.name().to_string(),
                cost,
                weight: *weight,
                normalization: normalization.clone(),
                message,
            });
        }

        metric_costs
    }

    pub fn evaluate_layout(&self, layout: &Layout) -> Vec<MetricResults> {
        let mapped_ngrams = self.ngram_mapper.mapped_ngrams(layout);

        // Layout metrics
        let metric_costs = self.evaluate_layout_metrics(layout);
        let layout_costs = MetricResults {
            metric_type: MetricType::Layout,
            found_weight: 1.0,
            not_found_weight: 0.0,
            metric_costs,
        };

        // Unigram metrics
        let metric_costs = self.evaluate_unigram_metrics(layout, &mapped_ngrams.unigrams);
        let unigram_costs = MetricResults {
            metric_type: MetricType::Unigram,
            found_weight: mapped_ngrams.unigrams_found,
            not_found_weight: mapped_ngrams.unigrams_not_found,
            metric_costs,
        };

        // Bigram metrics
        let metric_costs = self.evaluate_bigram_metrics(layout, &mapped_ngrams.bigrams);
        let bigram_costs = MetricResults {
            metric_type: MetricType::Bigram,
            found_weight: mapped_ngrams.bigrams_found,
            not_found_weight: mapped_ngrams.bigrams_not_found,
            metric_costs,
        };

        // Trigram metrics
        let metric_costs = self.evaluate_trigram_metrics(layout, &mapped_ngrams.trigrams);
        let trigram_costs = MetricResults {
            metric_type: MetricType::Trigram,
            found_weight: mapped_ngrams.trigrams_found,
            not_found_weight: mapped_ngrams.trigrams_not_found,
            metric_costs,
        };

        vec![layout_costs, unigram_costs, bigram_costs, trigram_costs]
    }
}
