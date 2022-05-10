//! This module provides an implementation of the [`NgramMapper`] trait.

use super::bigram_mapper::OnDemandBigramMapper;
use super::trigram_mapper::OnDemandTrigramMapper;
use super::unigram_mapper::OnDemandUnigramMapper;
use super::{MappedBigrams, MappedTrigrams, MappedUnigrams, NgramMapper};

use crate::ngrams::{Bigrams, Trigrams, Unigrams};

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
}

impl NgramMapper for OnDemandNgramMapper {
    fn map_unigrams<'s>(&self, layout: &'s Layout) -> MappedUnigrams<'s> {
        // map char-based unigrams to LayerKeyIndex
        let (key_indices, weight_not_found) =
            self.unigram_mapper.layerkey_indices(&self.unigrams, layout);
        let weight_found = self.unigrams.total_weight() - weight_not_found;
        // map LayerKeyIndex to &LayerKey
        let grams = OnDemandUnigramMapper::get_layerkeys(&key_indices, layout);

        MappedUnigrams {
            grams,
            weight_not_found,
            weight_found,
        }
    }

    fn map_bigrams<'s>(&self, layout: &'s Layout) -> MappedBigrams<'s> {
        // map char-based bigrams to LayerKeyIndex
        let (key_indices, weight_not_found) = self.bigram_mapper.layerkey_indices(
            &self.bigrams,
            layout,
            self.config.exclude_line_breaks,
        );
        let weight_found = self.bigrams.total_weight() - weight_not_found;
        // map LayerKeyIndex to &LayerKey
        let grams = OnDemandBigramMapper::get_filtered_layerkeys(&key_indices, layout);

        MappedBigrams {
            grams,
            weight_not_found,
            weight_found,
        }
    }

    fn map_trigrams<'s>(&self, layout: &'s Layout) -> MappedTrigrams<'s> {
        // map char-based trigrams to LayerKeyIndex
        let (key_indices, weight_not_found) = self.trigram_mapper.layerkey_indices(
            &self.trigrams,
            layout,
            self.config.exclude_line_breaks,
        );
        let weight_found = self.trigrams.total_weight() - weight_not_found;
        // map LayerKeyIndex to &LayerKey
        let grams = OnDemandTrigramMapper::get_filtered_layerkeys(&key_indices, layout);

        MappedTrigrams {
            grams,
            weight_not_found,
            weight_found,
        }
    }
}
