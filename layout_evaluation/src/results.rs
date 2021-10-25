use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum NormalizationType {
    Fixed(f64),
    WeightFound(f64),
    WeightAll(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    Layout,
    Unigram,
    Bigram,
    Trigram,
}

#[derive(Debug, Clone)]
pub struct MetricResult {
    pub name: String,
    pub cost: f64,
    pub message: Option<String>,
    pub weight: f64,
    pub normalization: NormalizationType,
}

#[derive(Debug, Clone)]
pub struct MetricResults {
    pub metric_type: MetricType,
    pub found_weight: f64,
    pub not_found_weight: f64,
    pub metric_costs: Vec<MetricResult>,
}

impl MetricResults {
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

    fn normalize_value(&self, val: f64, normalization_type: &NormalizationType) -> f64 {
        match normalization_type {
            NormalizationType::Fixed(t) => val / t,
            NormalizationType::WeightFound(t) => val / (t * self.found_weight),
            NormalizationType::WeightAll(t) => {
                val / (t * self.found_weight + self.not_found_weight)
            }
        }
    }

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

    fn aggregate_metric_costs(&self, normalize: bool, weight: bool) -> f64 {
        self.metric_costs.iter().fold(0.0, |acc, metric_cost| {
            acc + self.compute_metric_cost(metric_cost, normalize, weight)
        })
    }

    pub fn total_cost(&self) -> f64 {
        self.aggregate_metric_costs(true, true)
    }

    pub fn unnormalized_total_cost(&self) -> f64 {
        self.aggregate_metric_costs(false, true)
    }
}
