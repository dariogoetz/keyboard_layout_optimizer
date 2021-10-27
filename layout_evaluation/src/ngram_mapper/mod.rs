use keyboard_layout::layout::{LayerKey, Layout};

mod common;

pub mod bigram_mapper;
pub mod trigram_mapper;
pub mod unigram_mapper;

pub mod on_demand_ngram_mapper;

use keyboard_layout::layout::LayerKeyIndex;

pub type UnigramIndices = Vec<(LayerKeyIndex, f64)>;
pub type BigramIndices = Vec<((LayerKeyIndex, LayerKeyIndex), f64)>;
pub type TrigramIndices = Vec<((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64)>;

pub struct MappedNgrams<'s> {
    unigrams: Vec<(&'s LayerKey, f64)>,
    pub unigrams_not_found: f64,
    pub unigrams_found: f64,
    pub bigrams: Vec<((&'s LayerKey, &'s LayerKey), f64)>,
    pub bigrams_not_found: f64,
    pub bigrams_found: f64,
    pub trigrams: Vec<((&'s LayerKey, &'s LayerKey, &'s LayerKey), f64)>,
    pub trigrams_not_found: f64,
    pub trigrams_found: f64,
}

pub trait NgramMapper: Send + Sync + NgramMapperClone + std::fmt::Debug {
    fn mapped_ngrams<'s>(&self, layout: &'s Layout) -> MappedNgrams<'s>;
}

// in order to implement clone for Box<dyn LayoutMetric>, the following trick is necessary
// see https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
// alternative: use `dyn_clone` crate

impl Clone for Box<dyn NgramMapper> {
    fn clone(&self) -> Box<dyn NgramMapper> {
        self.clone_box()
    }
}

pub trait NgramMapperClone {
    fn clone_box(&self) -> Box<dyn NgramMapper>;
}

impl<T> NgramMapperClone for T
where
    T: 'static + NgramMapper + Clone,
{
    fn clone_box(&self) -> Box<dyn NgramMapper> {
        Box::new(self.clone())
    }
}
