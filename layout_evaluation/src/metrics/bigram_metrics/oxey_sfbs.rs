use super::BigramMetric;

use ahash::AHashSet;
use keyboard_layout::{
    key::Finger,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    exclude_thumbs: bool,
    exclude_modifiers: bool,
    exclude_chars: Vec<char>,
}

#[derive(Clone, Debug)]
pub struct OxeySfbs {
    exclude_thumbs: bool,
    exclude_modifiers: bool,
    exclude_chars: AHashSet<char>,
}

impl OxeySfbs {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
            exclude_modifiers: params.exclude_modifiers,
            exclude_chars: params.exclude_chars.iter().cloned().collect(),
        }
    }
}

impl BigramMetric for OxeySfbs {
    fn name(&self) -> &str {
        "Sfbs"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        if self.exclude_modifiers && (k1.is_modifier.is_some() || k2.is_modifier.is_some()) {
            return Some(0.0);
        }

        if !self.exclude_chars.is_empty()
            && (self.exclude_chars.contains(&k1.symbol) || self.exclude_chars.contains(&k2.symbol))
        {
            return Some(0.0);
        }

        if k1 == k2 {
            return Some(0.0);
        }
        let h1 = k1.key.hand;
        let h2 = k2.key.hand;

        if h1 != h2 {
            return Some(0.0);
        }

        let f1 = k1.key.finger;
        let f2 = k2.key.finger;

        if self.exclude_thumbs && (f1 == Finger::Thumb || f2 == Finger::Thumb) {
            return Some(0.0);
        }

        if f1 == f2 {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
