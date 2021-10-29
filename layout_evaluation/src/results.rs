//! The `results` module contains structs representing the results of metric evaluations.

use serde::Deserialize;

/// The `NormalizationType` specifies how the total cost of a metric evaluation shall be normalized.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum NormalizationType {
    /// Divide the metric result's cost value by a fixed value.
    Fixed(f64),
    /// Divide the metric result's cost value by the sum of the ngram weights that could be mapped by the layout and a given fixed value.
    WeightFound(f64),
    /// Divide the metric result's cost value by the sum of all ngram weights and a given fixed value.
    WeightAll(f64),
}

/// Specify which data a metric operates on.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    Layout,
    Unigram,
    Bigram,
    Trigram,
}

/// Describes the result of an individual metric evaluation.
#[derive(Debug, Clone)]
pub struct MetricResult {
    /// Name of the metric.
    pub name: String,
    /// Resulting total cost value (not normalized).
    pub cost: f64,
    /// An optional message that may contain additional details.
    pub message: Option<String>,
    /// The weight that shall be used when aggregating all metrics.
    pub weight: f64,
    /// The normalization type to apply.
    pub normalization: NormalizationType,
}

/// Describes a list of metric evaluation results of the same `MetricType`.
#[derive(Debug, Clone)]
pub struct MetricResults {
    /// Type of the metric, i.e. which data the metrics operated on.
    pub metric_type: MetricType,
    /// The total amount of weight (ngram frequencies) from ngrams that could be mapped by the layout.
    pub found_weight: f64,
    /// The total amount of weight (ngram frequencies) from ngrams that contained symbols that coult not be mapped by the layout.
    pub not_found_weight: f64,
    /// A list of the individual metric results.
    pub metric_costs: Vec<MetricResult>,
}

impl MetricResults {
    /// Print a summary of the metric results to stdout.
    pub fn print(&self) {
        println!("{:?} metrics:", self.metric_type);

        if self.metric_type != MetricType::Layout {
            println!(
                "  Not found: {:.4}% of {:.4}",
                100.0 * self.not_found_weight / (self.not_found_weight + self.found_weight),
                self.not_found_weight + self.found_weight
            );
        }
        for metric_cost in self.metric_costs.iter() {
            println!(
                "  {:>9.4} (weighted: {:>9.4}) {:<35} | {}",
                self.compute_metric_cost(metric_cost, true, false),
                self.compute_metric_cost(metric_cost, true, true),
                metric_cost.name,
                metric_cost.message.as_ref().unwrap_or(&"".to_string()),
            );
        }
    }

    /// Normalize a metric's cost value with given normalization strategy.
    fn normalize_value(&self, val: f64, normalization_type: &NormalizationType) -> f64 {
        match normalization_type {
            NormalizationType::Fixed(t) => val / t,
            NormalizationType::WeightFound(t) => val / (t * self.found_weight),
            NormalizationType::WeightAll(t) => {
                val / (t * self.found_weight + self.not_found_weight)
            }
        }
    }

    /// Helper function for weighting and normalizing individual metric's results.
    fn compute_metric_cost(
        &self,
        metric_cost: &MetricResult,
        normalize: bool,
        weight: bool,
    ) -> f64 {
        let cost = match weight {
            true => metric_cost.weight * metric_cost.cost,
            false => metric_cost.cost,
        };

        match normalize {
            true => self.normalize_value(cost, &metric_cost.normalization),
            false => cost,
        }
    }

    /// Helper function for aggregating all individual metrics' results to a total value.
    fn aggregate_metric_costs(&self, normalize: bool, weight: bool) -> f64 {
        self.metric_costs.iter().fold(0.0, |acc, metric_cost| {
            acc + self.compute_metric_cost(metric_cost, normalize, weight)
        })
    }

    /// Compute the weighted and normalized total cost of all metrics.
    pub fn total_cost(&self) -> f64 {
        self.aggregate_metric_costs(true, true)
    }

    /// Compute the weighted but not normalized total cost of all metrics.
    pub fn unnormalized_total_cost(&self) -> f64 {
        self.aggregate_metric_costs(false, true)
    }
}
