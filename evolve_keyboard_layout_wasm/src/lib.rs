mod utils;

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use keyboard_layout::{
    keyboard::{Keyboard, KeyboardYAML},
    layout_generator::{BaseLayoutYAML, NeoLayoutGenerator},
};

use layout_evaluation::{
    evaluation::{Evaluator, MetricParameters},
    results::EvaluationResult,
    ngram_mapper::on_demand_ngram_mapper::{NgramMapperConfig, OnDemandNgramMapper},
    ngrams::{Bigrams, Trigrams, Unigrams},
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Clone, Deserialize, Debug)]
pub struct NGramConfig {
    pub unigrams: String,
    pub bigrams: String,
    pub trigrams: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct EvaluationParameters {
    pub ngrams: NGramConfig,
    pub metrics: MetricParameters,
    pub ngram_mapper: NgramMapperConfig,
}

#[derive(Deserialize, Debug)]
pub struct LayoutConfig {
    pub keyboard: KeyboardYAML,
    pub base_layout: BaseLayoutYAML,
}

#[derive(Debug, Clone, Serialize)]
struct LayoutEvaluation {
    total_cost: f64,
    details: EvaluationResult,
    printed: Option<String>,
    plot: Option<String>,
}

impl From<EvaluationResult> for LayoutEvaluation {
    fn from(res: EvaluationResult) -> Self {
        Self {
            total_cost: res.total_cost(),
            details: res.clone(),
            printed: None,
            plot: None,
        }
    }
}

#[wasm_bindgen]
pub struct LayoutPlotter {
    layout_generator: NeoLayoutGenerator,
}

#[wasm_bindgen]
impl LayoutPlotter {
    pub fn new(
        layout_cfg_str: &str,
    ) -> Result<LayoutPlotter, JsValue> {

        utils::set_panic_hook();

        let layout_cfg: LayoutConfig = serde_yaml::from_str(layout_cfg_str)
            .map_err(|e| format!("Could not read layout config: {:?}", e))?;

        let keyboard = Arc::new(
            Keyboard::from_yaml_object(layout_cfg.keyboard)
        );

        let layout_generator = NeoLayoutGenerator::from_object(layout_cfg.base_layout, keyboard.clone());

        Ok(LayoutPlotter {
            layout_generator,
        })
    }

    pub fn plot(&self, layout_str: &str, layer: usize) -> Result<String, JsValue> {
        let layout = self.layout_generator.generate_unchecked(layout_str)
            .map_err(|e| format!("Could not plot the layout: {:?}", e))?;
        Ok(layout.plot_layer(layer))
    }
}

#[wasm_bindgen]
pub struct NgramProvider {
    ngram_provider: OnDemandNgramMapper,
}

#[wasm_bindgen]
impl NgramProvider {
    pub fn with_frequencies(eval_params_str: &str, unigrams_str: &str, bigrams_str: &str, trigrams_str: &str) -> Result<NgramProvider, JsValue> {

        let unigrams = Unigrams::from_frequencies_str(unigrams_str)
            .map_err(|e| format!("Could not load unigrams: {:?}", e))?;

        let bigrams = Bigrams::from_frequencies_str(bigrams_str)
            .map_err(|e| format!("Could not load bigrams: {:?}", e))?;

        let trigrams = Trigrams::from_frequencies_str(trigrams_str)
            .map_err(|e| format!("Could not load trigrams: {:?}", e))?;

        let eval_params: EvaluationParameters = serde_yaml::from_str(eval_params_str)
            .map_err(|e| format!("Could not read evaluation parameters: {:?}", e))?;

        let ngram_mapper_config = eval_params.ngram_mapper.clone();

        let ngram_provider =
            OnDemandNgramMapper::with_ngrams(unigrams, bigrams, trigrams, ngram_mapper_config);

        Ok(NgramProvider {
            ngram_provider,
        })
    }

    pub fn with_text(eval_params_str: &str, text: &str) -> Result<NgramProvider, JsValue> {
        let eval_params: EvaluationParameters = serde_yaml::from_str(eval_params_str)
            .map_err(|e| format!("Could not read evaluation parameters: {:?}", e))?;

        let ngram_mapper_config = eval_params.ngram_mapper.clone();

        let ngram_provider =
            OnDemandNgramMapper::with_corpus(&text, ngram_mapper_config);

        Ok(NgramProvider {
            ngram_provider,
        })
    }
}

#[wasm_bindgen]
pub struct LayoutEvaluator {
    layout_generator: NeoLayoutGenerator,
    evaluator: Evaluator,
}

#[wasm_bindgen]
impl LayoutEvaluator {
    pub fn new(
        layout_cfg_str: &str,
        eval_params_str: &str,
        ngram_provider: &NgramProvider,
    ) -> Result<LayoutEvaluator, JsValue> {

        utils::set_panic_hook();

        let layout_cfg: LayoutConfig = serde_yaml::from_str(layout_cfg_str)
            .map_err(|e| format!("Could not read layout config: {:?}", e))?;

        let keyboard = Arc::new(
            Keyboard::from_yaml_object(layout_cfg.keyboard)
        );

        let layout_generator = NeoLayoutGenerator::from_object(layout_cfg.base_layout, keyboard.clone());

        let eval_params: EvaluationParameters = serde_yaml::from_str(eval_params_str)
            .map_err(|e| format!("Could not read evaluation parameters: {:?}", e))?;

        let evaluator =
            Evaluator::default(Box::new(ngram_provider.ngram_provider.clone())).default_metrics(&eval_params.metrics);

        Ok(LayoutEvaluator {
            layout_generator,
            evaluator,
        })
    }

    pub fn evaluate(&self, layout_str: &str) -> Result<JsValue, JsValue> {
        let layout = self.layout_generator.generate(layout_str)
            .map_err(|e| format!("Could not generate layout: {:?}", e))?;
        let res = self.evaluator.evaluate_layout(&layout);
        let printed = Some(format!("{}", res));
        let plot = Some(layout.plot());

        let mut res: LayoutEvaluation = res.into();
        res.printed = printed;
        res.plot = plot;
        Ok(JsValue::from_serde(&res).unwrap())
    }

    pub fn plot(&self, layout_str: &str, layer: usize) -> Result<String, JsValue> {
        let layout = self.layout_generator.generate(layout_str)
            .map_err(|e| format!("Could not plot the layout: {:?}", e))?;
        Ok(layout.plot_layer(layer))
    }
}
