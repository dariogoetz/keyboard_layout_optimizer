mod utils;

use argmin::core::{observers::Observe, Error, State, KV};
use genevo::prelude::*;
use instant::Instant;
use serde::Serialize;
use std::{str::FromStr, sync::Arc};
use wasm_bindgen::prelude::*;

use keyboard_layout::{
    config::LayoutConfig, keyboard::Keyboard, layout::Layout, layout_generator::LayoutGenerator,
    neo_layout_generator::NeoLayoutGenerator,
};

use layout_evaluation::{
    cache::Cache,
    config::EvaluationParameters,
    evaluation::Evaluator,
    ngram_mapper::on_demand_ngram_mapper::OnDemandNgramMapper,
    ngrams::{Bigrams, Trigrams, Unigrams},
    results::EvaluationResult,
};

use layout_optimization_common::LayoutPermutator;
use layout_optimization_genetic::optimization as genevo_optimization;
use layout_optimization_sa::optimization::{
    self as sa_optimization, CustomObserver as SaCustomObserver, SaIterState,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Clone, Serialize)]
struct LayoutEvaluation {
    total_cost: f64,
    details: EvaluationResult,
    printed: Option<String>,
    plot: Option<String>,
    layout: Option<String>,
}

impl From<EvaluationResult> for LayoutEvaluation {
    fn from(res: EvaluationResult) -> Self {
        Self {
            total_cost: res.total_cost(),
            details: res,
            printed: None,
            plot: None,
            layout: None,
        }
    }
}

#[wasm_bindgen]
pub struct LayoutPlotter {
    layout_generator: NeoLayoutGenerator,
}

#[wasm_bindgen]
impl LayoutPlotter {
    pub fn new(layout_cfg_str: &str) -> Result<LayoutPlotter, JsValue> {
        utils::set_panic_hook();

        let layout_cfg = LayoutConfig::from_str(layout_cfg_str)
            .map_err(|e| format!("Could not read layout config: {:?}", e))?;

        let keyboard = Arc::new(Keyboard::from_yaml_object(layout_cfg.keyboard));

        let layout_generator = NeoLayoutGenerator::from_object(layout_cfg.base_layout, keyboard);

        Ok(LayoutPlotter { layout_generator })
    }

    pub fn plot(&self, layout_str: &str, layer: usize) -> Result<String, JsValue> {
        let layout_str: String = layout_str.chars().filter(|c| !c.is_whitespace()).collect();
        let layout = self
            .layout_generator
            .generate_unchecked(&layout_str)
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
    pub fn with_frequencies(
        eval_params_str: &str,
        unigrams_str: &str,
        bigrams_str: &str,
        trigrams_str: &str,
    ) -> Result<NgramProvider, JsValue> {
        let mut unigrams = Unigrams::from_frequencies_str(unigrams_str)
            .map_err(|e| format!("Could not load unigrams: {:?}", e))?;
        let mut bigrams = Bigrams::from_frequencies_str(bigrams_str)
            .map_err(|e| format!("Could not load bigrams: {:?}", e))?;
        let mut trigrams = Trigrams::from_frequencies_str(trigrams_str)
            .map_err(|e| format!("Could not load trigrams: {:?}", e))?;

        let eval_params: EvaluationParameters = serde_yaml::from_str(eval_params_str)
            .map_err(|e| format!("Could not read evaluation parameters: {:?}", e))?;

        let ngrams_config = eval_params.ngrams;
        if ngrams_config.increase_common_ngrams.enabled {
            unigrams = unigrams.increase_common(&ngrams_config.increase_common_ngrams);
            bigrams = bigrams.increase_common(&ngrams_config.increase_common_ngrams);
            trigrams = trigrams.increase_common(&ngrams_config.increase_common_ngrams);
        }

        let ngram_provider =
            OnDemandNgramMapper::with_ngrams(unigrams, bigrams, trigrams, eval_params.ngram_mapper);

        Ok(NgramProvider { ngram_provider })
    }

    pub fn with_text(eval_params_str: &str, text: &str) -> Result<NgramProvider, JsValue> {
        let mut unigrams = Unigrams::from_text(text)
            .map_err(|e| format!("Could not generate unigrams from text: {:?}", e))?;
        let mut bigrams = Bigrams::from_text(text)
            .map_err(|e| format!("Could not generate bigrams from text: {:?}", e))?;
        let mut trigrams = Trigrams::from_text(text)
            .map_err(|e| format!("Could not generate trigrams from text: {:?}", e))?;

        let eval_params: EvaluationParameters = serde_yaml::from_str(eval_params_str)
            .map_err(|e| format!("Could not read evaluation parameters: {:?}", e))?;

        let ngrams_config = eval_params.ngrams;
        if ngrams_config.increase_common_ngrams.enabled {
            unigrams = unigrams.increase_common(&ngrams_config.increase_common_ngrams);
            bigrams = bigrams.increase_common(&ngrams_config.increase_common_ngrams);
            trigrams = trigrams.increase_common(&ngrams_config.increase_common_ngrams);
        }

        let ngram_provider =
            OnDemandNgramMapper::with_ngrams(unigrams, bigrams, trigrams, eval_params.ngram_mapper);

        Ok(NgramProvider { ngram_provider })
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

        let layout_cfg = LayoutConfig::from_str(layout_cfg_str)
            .map_err(|e| format!("Could not read layout config: {:?}", e))?;

        let keyboard = Arc::new(Keyboard::from_yaml_object(layout_cfg.keyboard));

        let layout_generator = NeoLayoutGenerator::from_object(layout_cfg.base_layout, keyboard);

        let eval_params: EvaluationParameters = serde_yaml::from_str(eval_params_str)
            .map_err(|e| format!("Could not read evaluation parameters: {:?}", e))?;

        let evaluator = Evaluator::default(Box::new(ngram_provider.ngram_provider.clone()))
            .default_metrics(&eval_params.metrics);

        Ok(LayoutEvaluator {
            layout_generator,
            evaluator,
        })
    }

    pub fn evaluate(&self, layout_str: &str) -> Result<JsValue, JsValue> {
        let layout_str: String = layout_str.chars().filter(|c| !c.is_whitespace()).collect();
        let layout = self
            .layout_generator
            .generate(&layout_str)
            .map_err(|e| format!("Could not generate layout: {:?}", e))?;
        let res = self.evaluator.evaluate_layout(&layout);
        let printed = Some(format!("{}", res));
        let plot = Some(layout.plot());
        let layout_str = Some(layout_str);

        let mut res: LayoutEvaluation = res.into();
        res.printed = printed;
        res.plot = plot;
        res.layout = layout_str;
        Ok(JsValue::from_serde(&res).unwrap())
    }

    pub fn plot(&self, layout_str: &str, layer: usize) -> Result<String, JsValue> {
        let layout_str: String = layout_str.chars().filter(|c| !c.is_whitespace()).collect();
        let layout = self
            .layout_generator
            .generate(&layout_str)
            .map_err(|e| format!("Could not plot the layout: {:?}", e))?;
        Ok(layout.plot_layer(layer))
    }

    pub fn permutable_keys(&self) -> JsValue {
        let permutable_keys = self.layout_generator.permutable_keys();
        JsValue::from_serde(&permutable_keys).unwrap()
    }
}

#[wasm_bindgen]
pub struct LayoutOptimizer {
    evaluator: Evaluator,
    simulator: genevo_optimization::MySimulator,
    permutator: LayoutPermutator,
    layout_generator: Box<dyn LayoutGenerator>,
    all_time_best: Option<(usize, Vec<usize>)>,
    parameters: genevo_optimization::Parameters,
}

#[wasm_bindgen]
impl LayoutOptimizer {
    pub fn new(
        layout_str: &str,
        optimization_params_str: &str,
        layout_evaluator: &LayoutEvaluator,
        fixed_characters: &str,
        start_with_layout: bool,
    ) -> Result<LayoutOptimizer, JsValue> {
        utils::set_panic_hook();

        let layout_str: String = layout_str.chars().filter(|c| !c.is_whitespace()).collect();

        let parameters: genevo_optimization::Parameters =
            serde_yaml::from_str(optimization_params_str)
                .map_err(|e| format!("Could not read optimization params: {:?}", e))?;

        let layout_generator: Box<dyn LayoutGenerator> =
            Box::new(layout_evaluator.layout_generator.clone());

        let (simulator, permutator) = genevo_optimization::init_optimization(
            &parameters,
            &layout_evaluator.evaluator,
            &layout_str,
            &layout_generator,
            fixed_characters,
            start_with_layout,
            true,
        );

        Ok(LayoutOptimizer {
            evaluator: layout_evaluator.evaluator.clone(),
            simulator,
            permutator,
            layout_generator,
            all_time_best: None,
            parameters,
        })
    }

    pub fn parameters(&self) -> JsValue {
        JsValue::from_serde(&self.parameters).unwrap()
    }

    pub fn step(&mut self) -> Result<JsValue, JsValue> {
        let result = self.simulator.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let best_solution = step.result.best_solution;
                if let Some(king) = &self.all_time_best {
                    if best_solution.solution.fitness > king.0 {
                        self.all_time_best = Some((
                            best_solution.solution.fitness,
                            best_solution.solution.genome,
                        ));
                    }
                } else {
                    self.all_time_best = Some((
                        best_solution.solution.fitness,
                        best_solution.solution.genome,
                    ));
                }

                let layout_str = self
                    .permutator
                    .generate_string(&self.all_time_best.as_ref().unwrap().1);
                let layout = self.layout_generator.generate(&layout_str).unwrap();
                let res = self.evaluator.evaluate_layout(&layout);
                let printed = Some(format!("{}", res));
                let plot = Some(layout.plot());

                let mut res: LayoutEvaluation = res.into();
                res.printed = printed;
                res.plot = plot;
                res.layout = Some(layout_str);

                Ok(JsValue::from_serde(&Some(res)).unwrap())
            }
            Ok(SimResult::Final(_, _, _, _)) => {
                Ok(JsValue::from_serde(&None::<Option<EvaluationResult>>).unwrap())
                // break
            }
            Err(error) => {
                Err(format!("Error in optimization: {:?}", error).into())
                // break
            }
        }
    }
}

/// An observer that outputs important information in a more human-readable format than `Argmin`'s original implementation.
struct SaObserver {
    permutator: LayoutPermutator,
    last_update_call: Instant,
    update_callback: js_sys::Function,
    new_best_callback: js_sys::Function,
}

impl Observe<SaIterState> for SaObserver {
    fn observe_iter(&mut self, state: &SaIterState, kv: &KV) -> Result<(), Error> {
        if (state.iter > 0) && (self.last_update_call.elapsed().as_millis() > 500) {
            self.last_update_call = Instant::now();
            let this = JsValue::null();
            let iter_js = JsValue::from(state.iter);
            let mut t_string = String::from("Not found.");
            for (key, value) in &kv.kv {
                if *key == "t" {
                    let t_num: f32 = value.to_string().parse().unwrap();
                    let t_long_str = format!("{:.3}", t_num);
                    t_string = format!("{:.5}", t_long_str);
                }
            }
            let t_js = JsValue::from(t_string);
            let _ = self.update_callback.call2(&this, &iter_js, &t_js);
        }
        if state.is_best() && (state.param != state.prev_best_param) {
            let this = JsValue::null();
            let layout_js = JsValue::from(
                self.permutator
                    .generate_string(state.param.as_ref().unwrap()),
            );
            let cost_js = JsValue::from(state.cost);
            let _ = self.new_best_callback.call2(&this, &layout_js, &cost_js);
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub fn sa_optimize(
    layout_str: &str,
    optimization_params_str: &str,
    layout_evaluator: &LayoutEvaluator,
    fixed_characters: &str,
    start_with_layout: bool,
    update_callback: js_sys::Function,
    new_best_callback: js_sys::Function,
) {
    let mut parameters: sa_optimization::Parameters = serde_yaml::from_str(optimization_params_str)
        .map_err(|e| format!("Could not read optimization params: {:?}", e))
        .unwrap();
    // Make sure the initial temperature is greater than zero.
    parameters.correct_init_temp();

    // Display the maximum amount of iterations on the website.
    let this = JsValue::null();
    let zero = JsValue::from(0);
    let init_temp = JsValue::from(match parameters.init_temp {
        Some(t) => {
            let t_long_str = format!("{:.3}", t);
            format!("{:.5}", t_long_str)
        }
        None => "Calculating...".to_string(),
    });
    let _ = update_callback.call2(&this, &zero, &init_temp);

    let observer = SaObserver {
        permutator: LayoutPermutator::new(layout_str, fixed_characters),
        last_update_call: Instant::now(),
        update_callback: update_callback.clone(),
        new_best_callback,
    };
    let layout_generator: Box<dyn LayoutGenerator> =
        Box::new(layout_evaluator.layout_generator.clone());

    let layout_str: String = layout_str.chars().filter(|c| !c.is_whitespace()).collect();

    let _: (String, Layout) = sa_optimization::optimize(
        /* Thread_name: */ "Web optimization",
        &parameters,
        &layout_str,
        fixed_characters,
        &layout_generator,
        start_with_layout,
        &layout_evaluator.evaluator,
        /* log_everything: */ false,
        Some(Cache::new()),
        Some(SaCustomObserver(Box::new(observer))),
    );
    let minus_one = JsValue::from(-1);
    let _ = update_callback.call1(&this, &minus_one);
}
