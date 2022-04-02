//! This module provides an implementation of the [`NgramMapper`] trait.

use super::bigram_mapper::{
    self, IncreaseCommonBigramsConfig, OnDemandBigramMapper, SecondaryBigramsFromTrigramsConfig,
};
use super::trigram_mapper::OnDemandTrigramMapper;
use super::unigram_mapper::OnDemandUnigramMapper;
use super::{MappedNgrams, NgramMapper};

use crate::ngrams::{Bigrams, Trigrams, Unigrams};

use keyboard_layout::layout::Layout;

use ahash::AHashMap;
use serde::Deserialize;
use std::hash::Hash;

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
    /// Parameters for adding secondary bigrams from trigrams.
    pub secondary_bigrams_from_trigrams: SecondaryBigramsFromTrigramsConfig,
    /// Parameters for the increase in weight of common bigrams (with already high frequency).
    pub increase_common_bigrams: IncreaseCommonBigramsConfig,
    /// Exclude ngrams that contain a line break, followed by a non-line-break character
    pub exclude_line_breaks: bool,
}

/// Implements the [`NgramMapper`] trait for generating ngrams in terms of [`LayerKey`]s for a given [`Layout`].
#[derive(Clone, Debug)]
pub struct OnDemandNgramMapper {
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
            unigram_mapper: OnDemandUnigramMapper::new(unigrams, config.split_modifiers.clone()),
            bigram_mapper: OnDemandBigramMapper::new(bigrams, config.split_modifiers.clone()),
            trigram_mapper: OnDemandTrigramMapper::new(trigrams, config.split_modifiers.clone()),
            config,
        }
    }

    /// Generate a [`OnDemandNgramMapper`] with a given corpus (text). Generates corresponding ngrams automatically.
    pub fn with_corpus(text: &str, config: NgramMapperConfig) -> Self {
        let unigrams = Unigrams::from_text(text).expect("Could not generate unigrams from text.");
        let bigrams = Bigrams::from_text(text).expect("Could not generate bigrams from text.");
        let trigrams = Trigrams::from_text(text).expect("Could not generate trigrams from text.");

        Self {
            unigram_mapper: OnDemandUnigramMapper::new(unigrams, config.split_modifiers.clone()),
            bigram_mapper: OnDemandBigramMapper::new(bigrams, config.split_modifiers.clone()),
            trigram_mapper: OnDemandTrigramMapper::new(trigrams, config.split_modifiers.clone()),
            config,
        }
    }
}

fn groupby_sum<T: Clone + Eq + Hash>(data: &[(T, f64)]) -> Vec<(T, f64)> {
    data.iter()
        .fold(AHashMap::default(), |mut m, (k, w)| {
            *m.entry(k.clone()).or_insert(0.0) += *w;
            m
        })
        .into_iter()
        .collect()
}

impl NgramMapper for OnDemandNgramMapper {
    fn mapped_ngrams<'s>(&self, layout: &'s Layout) -> MappedNgrams<'s> {
        // map char-based unigrams to LayerKeyIndex
        let (unigram_key_indices, unigrams_found, unigrams_not_found) =
            self.unigram_mapper.layerkey_indices(layout);
        // sum duplicates in unigram vecs (involves a hashmap -> use LayerKeyIndex instead of &LayerKey for performance)
        let unigram_key_indices = groupby_sum(&unigram_key_indices);
        // map LayerKeyIndex to &LayerKey
        let unigrams = OnDemandUnigramMapper::layerkeys(&unigram_key_indices, layout);

        // map trigrams before bigrams because secondary bigrams from trigrams map be added
        // map char-based trigrams to LayerKeyIndex
        let (trigram_key_indices, trigrams_found, trigrams_not_found) = self
            .trigram_mapper
            .layerkey_indices(layout, self.config.exclude_line_breaks);
        // sum duplicates in trigram vecs (involves a hashmap -> use LayerKeyIndex instead of &LayerKey for performance)
        let trigram_key_indices = groupby_sum(&trigram_key_indices);
        // map LayerKeyIndex to &LayerKey
        let trigrams = OnDemandTrigramMapper::layerkeys(&trigram_key_indices, layout);

        // if the same modifier appears consecutively, it is usually "hold" instead of repeatedly pressed
        // --> remove
        let trigrams = trigrams
            .into_iter()
            .filter(|((k1, k2, k3), _)| {
                !((k1 == k2 && k1.is_modifier) || (k2 == k3 && k2.is_modifier))
            })
            .collect();

        // map char-based bigrams to LayerKeyIndex
        let (mut bigram_key_indices, _bigrams_found, bigrams_not_found) = self
            .bigram_mapper
            .layerkey_indices(layout, self.config.exclude_line_breaks);

        // (if enabled) add bigrams consisting of first and third trigram symbols to vec of bigrams
        bigram_mapper::add_secondary_bigrams_from_trigrams(
            &mut bigram_key_indices,
            &trigram_key_indices,
            &self.config.secondary_bigrams_from_trigrams,
            layout,
        );

        // (if enabled) increase the weight of bigrams with high weight even higher
        bigram_key_indices = bigram_mapper::increase_common_bigrams(
            &bigram_key_indices,
            &self.config.increase_common_bigrams,
        );

        // sum duplicates in trigram vecs (involves a hashmap -> use LayerKeyIndex instead of &LayerKey for performance)
        let bigram_key_indices = groupby_sum(&bigram_key_indices);
        // recompute total found bigram weight (after adding secondary bigrams and increasing weights)
        let bigrams_found = bigram_key_indices.iter().map(|(_, w)| w).sum();
        // map LayerKeyIndex to &LayerKey
        let bigrams = OnDemandBigramMapper::layerkeys(&bigram_key_indices, layout);

        // if the same modifier appears consecutively, it is usually "hold" instead of repeatedly pressed
        let bigrams = bigrams
            .into_iter()
            .filter(|((k1, k2), _)| !(k1 == k2 && k1.is_modifier))
            .collect();

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
