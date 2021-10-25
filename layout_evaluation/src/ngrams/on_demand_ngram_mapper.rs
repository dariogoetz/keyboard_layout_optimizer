use super::bigram_mapper::{self, SecondaryBigramsFromTrigramsConfig, IncreaseCommonBigramsConfig, OnDemandBigramMapper};
use super::trigram_mapper::OnDemandTrigramMapper;
use super::unigram_mapper::OnDemandUnigramMapper;
use super::{ngrams, MappedNgrams, NgramMapper};

use keyboard_layout::layout::Layout;

use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct SplitModifiersConfig {
    pub enabled: bool,
    pub same_key_mod_factor: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NgramMapperConfig {
    pub split_modifiers: SplitModifiersConfig,
    pub secondary_bigrams_from_trigrams: SecondaryBigramsFromTrigramsConfig,
    pub increase_common_bigrams: IncreaseCommonBigramsConfig,
}

#[derive(Clone, Debug)]
pub struct OnDemandNgramMapper {
    unigram_mapper: OnDemandUnigramMapper,
    bigram_mapper: OnDemandBigramMapper,
    trigram_mapper: OnDemandTrigramMapper,
    config: NgramMapperConfig,
}

impl OnDemandNgramMapper {
    pub fn with_ngrams(
        unigrams: &ngrams::Unigrams,
        bigrams: &ngrams::Bigrams,
        trigrams: &ngrams::Trigrams,
        config: NgramMapperConfig,
    ) -> Self {
        Self {
            unigram_mapper: OnDemandUnigramMapper::new(unigrams, config.split_modifiers.clone()),
            bigram_mapper: OnDemandBigramMapper::new(bigrams, config.split_modifiers.clone()),
            trigram_mapper: OnDemandTrigramMapper::new(trigrams, config.split_modifiers.clone()),
            config,
        }
    }

    pub fn with_corpus(
        text: &str,
        config: NgramMapperConfig,
    ) -> Self {
        let unigrams =
            ngrams::Unigrams::from_str(text).expect("Could not generate unigrams from text.");
        let bigrams =
            ngrams::Bigrams::from_str(text).expect("Could not generate bigrams from text.");
        let trigrams =
            ngrams::Trigrams::from_str(text).expect("Could not generate trigrams from text.");

        Self {
            unigram_mapper: OnDemandUnigramMapper::new(&unigrams, config.split_modifiers.clone()),
            bigram_mapper: OnDemandBigramMapper::new(&bigrams, config.split_modifiers.clone()),
            trigram_mapper: OnDemandTrigramMapper::new(&trigrams, config.split_modifiers.clone()),
            config,
        }
    }
}

fn groupby_sum<T: Clone + Eq + std::hash::Hash>(data: &[(T, f64)]) -> Vec<(T, f64)> {
    data.iter()
        .fold(FxHashMap::default(), |mut m, (k, w)| {
            *m.entry(k.clone()).or_insert(0.0) += *w;
            m
        })
        .into_iter()
        .collect()
}

impl NgramMapper for OnDemandNgramMapper {
    fn mapped_ngrams<'s>(&self, layout: &'s Layout) -> MappedNgrams<'s> {
        let (unigram_key_indices, unigrams_found, unigrams_not_found) =
            self.unigram_mapper.layerkey_indices(layout);
        let unigram_key_indices = groupby_sum(&unigram_key_indices);
        let unigrams = OnDemandUnigramMapper::layerkeys(&unigram_key_indices, &layout);

        let (trigram_key_indices, trigrams_found, trigrams_not_found) =
            self.trigram_mapper.layerkey_indices(layout);
        let trigram_key_indices = groupby_sum(&trigram_key_indices);
        let trigrams = OnDemandTrigramMapper::layerkeys(&trigram_key_indices, &layout);

        let (mut bigram_key_indices, _bigrams_found, bigrams_not_found) =
            self.bigram_mapper.layerkey_indices(layout);

        bigram_mapper::add_secondary_bigrams_from_trigrams(
            &mut bigram_key_indices,
            &trigram_key_indices,
            &self.config.secondary_bigrams_from_trigrams,
            layout,
        );

        bigram_key_indices =
            bigram_mapper::increase_common_bigrams(&bigram_key_indices, &self.config.increase_common_bigrams);

        // ensure that each bigram has the correct weight (no duplicates)
        let bigram_key_indices = groupby_sum(&bigram_key_indices);
        let bigrams_found = bigram_key_indices.iter().map(|(_, w)| w).sum();
        let bigrams = OnDemandBigramMapper::layerkeys(&bigram_key_indices, &layout);

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
