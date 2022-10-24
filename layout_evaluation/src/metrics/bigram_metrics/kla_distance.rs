use super::BigramMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap},
    layout::{LayerKey, LayerKeyIndex, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub keyup_distance: f64,
    pub keydown_distance: f64,
    pub dscoring: AHashMap<Hand, AHashMap<Finger, f64>>,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLADistance {
    keyup_distance: f64,
    keydown_distance: f64,
    dscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLADistance {
    pub fn new(params: &Parameters) -> Self {
        Self {
            keyup_distance: params.keyup_distance,
            keydown_distance: params.keydown_distance,
            dscoring: HandFingerMap::with_hashmap(&params.dscoring, 1.0),
            hscoring: HandMap::with_hashmap(&params.hscoring, 1.0),
        }
    }
}

impl BigramMetric for KLADistance {
    fn name(&self) -> &str {
        "Distance"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        prev_key: &LayerKey,
        curr_key: &LayerKey,
        weight: f64,
        _total_weight: f64,
        layout: &Layout,
    ) -> Option<f64> {
        let prev_mods: AHashSet<LayerKeyIndex> =
            prev_key.modifiers.layerkeys().iter().cloned().collect();
        let curr_mods: AHashSet<LayerKeyIndex> =
            curr_key.modifiers.layerkeys().iter().cloned().collect();

        let released_mods = prev_mods
            .difference(&curr_mods)
            .map(|k| layout.get_layerkey(k));
        let pressed_mods = curr_mods
            .difference(&prev_mods)
            .map(|k| layout.get_layerkey(k));

        // a key's distance to the corresponding home-row key of the same finger
        let home_row_dist = |k: &LayerKey| {
            let hp = layout
                .keyboard
                .home_row_positions
                .get(&k.key.hand, &k.key.finger);

            hp.distance(&k.key.position)
        };

        let pressed_mods_distance = pressed_mods
            .map(|k| {
                let dscore = self.dscoring.get(&k.key.hand, &k.key.finger);
                let hscore = self.hscoring.get(&k.key.hand);
                (home_row_dist(k) + self.keydown_distance) * dscore * hscore
            })
            .sum::<f64>();

        let released_mods_distance = released_mods
            .map(|k| {
                let dscore = self.dscoring.get(&k.key.hand, &k.key.finger);
                let hscore = self.hscoring.get(&k.key.hand);
                (home_row_dist(k) + self.keyup_distance) * dscore * hscore
            })
            .sum::<f64>();

        let dscore = self.dscoring.get(&curr_key.key.hand, &curr_key.key.finger);
        let hscore = self.hscoring.get(&curr_key.key.hand);
        let key_distance = (home_row_dist(&curr_key) + self.keyup_distance + self.keydown_distance)
            * dscore
            * hscore;

        Some(weight * (key_distance + released_mods_distance + pressed_mods_distance))
    }
}
