//! This module provides an implementation of the [`NgramMapper`] trait.

use super::bigram_mapper::OnDemandBigramMapper;
use super::trigram_mapper::OnDemandTrigramMapper;
use super::unigram_mapper::OnDemandUnigramMapper;
use super::{MappedNgrams, NgramMapper};

use crate::ngrams::{Bigrams, IncreaseCommonNgramsConfig, Trigrams, Unigrams};

use keyboard_layout::layout::Layout;

use serde::Deserialize;

/// Configuration parameters for the modifier splitting process.
#[derive(Clone, Deserialize, Debug)]
pub struct SplitModifiersConfig {
    /// Whether to split ngrams with higher-layer symbols into multiple base-layer ones.
    pub enabled: bool,
    /// Weight factor for generated ngrams that involve two modifiers for the same key.
    pub same_key_mod_factor: f64,
}

/// Configuration parameters for the [`OnDemandNgramMapper`].
#[derive(Clone, Deserialize, Debug)]
pub struct NgramMapperConfig {
    /// Parameters for the modifiers splitting process.
    pub split_modifiers: SplitModifiersConfig,
    /// Parameters for the increase in weight of common ngrams (with already high frequency).
    pub increase_common_ngrams: IncreaseCommonNgramsConfig,
    /// Exclude ngrams that contain a line break, followed by a non-line-break character
    pub exclude_line_breaks: bool,
}

/// Implements the [`NgramMapper`] trait for generating ngrams in terms of [`LayerKey`]s for a given [`Layout`].
#[derive(Clone, Debug)]
pub struct OnDemandNgramMapper {
    unigrams: Unigrams,
    bigrams: Bigrams,
    trigrams: Trigrams,
    unigram_mapper: OnDemandUnigramMapper,
    bigram_mapper: OnDemandBigramMapper,
    trigram_mapper: OnDemandTrigramMapper,
    config: NgramMapperConfig,
}

impl OnDemandNgramMapper {
    /// Generate a [`OnDemandNgramMapper`] with given char-based ngrams.
    pub fn with_ngrams(
        unigrams: Unigrams,
        bigrams: Bigrams,
        trigrams: Trigrams,
        config: NgramMapperConfig,
    ) -> Self {
        Self {
            unigrams,
            bigrams,
            trigrams,
            unigram_mapper: OnDemandUnigramMapper::new(config.split_modifiers.clone()),
            bigram_mapper: OnDemandBigramMapper::new(config.split_modifiers.clone()),
            trigram_mapper: OnDemandTrigramMapper::new(config.split_modifiers.clone()),
            config,
        }
    }

    /// Generate a [`OnDemandNgramMapper`] with a given corpus (text). Generates corresponding ngrams automatically.
    pub fn with_corpus(text: &str, config: NgramMapperConfig) -> Self {
        let unigrams = Unigrams::from_text(text).expect("Could not generate unigrams from text.");
        let bigrams = Bigrams::from_text(text).expect("Could not generate bigrams from text.");
        let trigrams = Trigrams::from_text(text).expect("Could not generate trigrams from text.");

        Self {
            unigrams,
            bigrams,
            trigrams,
            unigram_mapper: OnDemandUnigramMapper::new(config.split_modifiers.clone()),
            bigram_mapper: OnDemandBigramMapper::new(config.split_modifiers.clone()),
            trigram_mapper: OnDemandTrigramMapper::new(config.split_modifiers.clone()),
            config,
        }
    }
}

// TODO: implement function (generic over ngrams) that increases common ngrams before splitting
// TODO: implement a variant with simple quadratic behavior

impl NgramMapper for OnDemandNgramMapper {
    fn map_ngrams<'s>(&self, layout: &'s Layout) -> MappedNgrams<'s> {
        let unigrams = self
            .unigrams
            .increase_common(&self.config.increase_common_ngrams);

        // map char-based unigrams to LayerKeyIndex
        let (unigram_key_indices, unigrams_found, unigrams_not_found) =
            self.unigram_mapper.layerkey_indices(&unigrams, layout);
        // map LayerKeyIndex to &LayerKey
        let unigrams = OnDemandUnigramMapper::get_layerkeys(&unigram_key_indices, layout);

        let bigrams = self
            .bigrams
            .increase_common(&self.config.increase_common_ngrams);
        // map char-based bigrams to LayerKeyIndex
        let (bigram_key_indices, _bigrams_found, bigrams_not_found) = self
            .bigram_mapper
            .layerkey_indices(&bigrams, layout, self.config.exclude_line_breaks);

        let bigrams_found = bigram_key_indices.values().sum();
        // map LayerKeyIndex to &LayerKey
        let bigrams = OnDemandBigramMapper::get_filtered_layerkeys(&bigram_key_indices, layout);

        let trigrams = self
            .trigrams
            .increase_common(&self.config.increase_common_ngrams);
        // map char-based trigrams to LayerKeyIndex
        let (trigram_key_indices, trigrams_found, trigrams_not_found) = self
            .trigram_mapper
            .layerkey_indices(&trigrams, layout, self.config.exclude_line_breaks);
        // map LayerKeyIndex to &LayerKey
        let trigrams = OnDemandTrigramMapper::get_filtered_layerkeys(&trigram_key_indices, layout);

        // sorting costs about 10% performance per evaluation and only gains some niceties in debugging
        // unigrams.sort_by(|(_, w1), (_, w2)| w1.partial_cmp(&w2).unwrap());
        // bigrams.sort_by(|(_, w1), (_, w2)| w1.partial_cmp(&w2).unwrap());
        // trigrams.sort_by(|(_, w1), (_, w2)| w1.partial_cmp(&w2).unwrap());

        MappedNgrams {
            unigrams,
            unigrams_found,
            unigrams_not_found,
            bigrams,
            bigrams_found,
            bigrams_not_found,
            trigrams,
            trigrams_found,
            trigrams_not_found,
        }
    }
}
