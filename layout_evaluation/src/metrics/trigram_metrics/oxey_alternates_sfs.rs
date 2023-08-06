use super::TrigramMetric;

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
pub struct OxeyAlternatesSfs {
    exclude_thumbs: bool,
    exclude_modifiers: bool,
    exclude_chars: AHashSet<char>,
}

impl OxeyAlternatesSfs {
    pub fn new(params: &Parameters) -> Self {
        Self {
            exclude_thumbs: params.exclude_thumbs,
            exclude_modifiers: params.exclude_modifiers,
            exclude_chars: params.exclude_chars.iter().cloned().collect(),
        }
    }
}

impl TrigramMetric for OxeyAlternatesSfs {
    fn name(&self) -> &str {
        "Alternates (sfs)"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        if self.exclude_thumbs
            && (k1.key.finger == Finger::Thumb
                || k2.key.finger == Finger::Thumb
                || k3.key.finger == Finger::Thumb)
        {
            return Some(0.0);
        }

        if self.exclude_modifiers
            && (k1.is_modifier.is_some() || k2.is_modifier.is_some() || k3.is_modifier.is_some())
        {
            return Some(0.0);
        }

        if !self.exclude_chars.is_empty()
            && (self.exclude_chars.contains(&k1.symbol)
                || self.exclude_chars.contains(&k2.symbol)
                || self.exclude_chars.contains(&k3.symbol))
        {
            return Some(0.0);
        }

        let h1 = k1.key.hand;
        let h2 = k2.key.hand;
        let h3 = k3.key.hand;

        if h1 != h2 && h2 != h3 && (h1 == h3 && k1.key.finger == k3.key.finger) {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
