use super::BigramMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{
    key::{Finger, Hand, HandFingerMap, HandMap},
    layout::{LayerKey, LayerKeyIndex, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub press_distance: f64,
    pub modifier_penalty: f64,
    pub dscoring: AHashMap<Hand, AHashMap<Finger, f64>>,
    pub hscoring: AHashMap<Hand, f64>,
}

#[derive(Clone, Debug)]
pub struct KLADistance {
    press_distance: f64,
    modifier_penalty: f64,
    dscoring: HandFingerMap<f64>,
    hscoring: HandMap<f64>,
}

impl KLADistance {
    pub fn new(params: &Parameters) -> Self {
        Self {
            press_distance: params.press_distance,
            modifier_penalty: params.modifier_penalty,
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

        let held_mods: Vec<&LayerKey> = prev_mods
            .intersection(&curr_mods)
            .map(|k| layout.get_layerkey(k))
            .collect();
        let released_mods: Vec<&LayerKey> = prev_mods
            .difference(&curr_mods)
            .map(|k| layout.get_layerkey(k))
            .collect();
        let pressed_mods: Vec<&LayerKey> = curr_mods
            .difference(&prev_mods)
            .map(|k| layout.get_layerkey(k))
            .collect();

        // log::info!("Held:    {:?}", held_mods);
        // log::info!("Release: {:?}", released_mods);
        // log::info!("Pressed: {:?}", pressed_mods);

        // a key's distance to the corresponding home-row key of the same finger
        let home_row_dist = |k: &&LayerKey| {
            let hp = layout
                .keyboard
                .home_row_positions
                .get(&k.key.hand, &k.key.finger);

            // why the factor 1.1?
            let dx = 1.1 * (k.key.position.0 - hp.0);
            let mut dy = k.key.position.1 - hp.1;
            // why only factor 1.1 if direction down?
            dy = if dy > 0.0 { 1.1 * dy } else { dy };

            (dx * dx + dy * dy).sqrt()
        };

        let held_mod_distance = held_mods
            .iter()
            .map(|k| {
                let dscore = self.dscoring.get(&k.key.hand, &k.key.finger);
                let hscore = self.hscoring.get(&k.key.hand);
                // held mods only have a modifier penalty
                self.modifier_penalty * dscore * hscore
            })
            .sum::<f64>();

        // moveFingerToKey
        let pressed_mods_distance = pressed_mods
            .iter()
            .map(|k| {
                let dscore = self.dscoring.get(&k.key.hand, &k.key.finger);
                let hscore = self.hscoring.get(&k.key.hand);
                // pressed mods have a moving penalty, a press penalty and a modifier penalty
                (home_row_dist(k) + self.press_distance + self.modifier_penalty) * dscore * hscore
            })
            .sum::<f64>();

        // returnFingersToHomeRow
        let released_mods_distance = released_mods
            .iter()
            .map(|k| {
                let dscore = self.dscoring.get(&k.key.hand, &k.key.finger);
                let hscore = self.hscoring.get(&k.key.hand);
                // released mods have a moving penalty and a modifier penalty but no press penalty
                (home_row_dist(k) + self.modifier_penalty) * dscore * hscore
            })
            .sum::<f64>();

        let dscore = self.dscoring.get(&curr_key.key.hand, &curr_key.key.finger);
        let hscore = self.hscoring.get(&curr_key.key.hand);
        let key_distance = (home_row_dist(&curr_key) + self.press_distance) * dscore * hscore;

        Some(
            weight
                * (key_distance
                    + released_mods_distance
                    + pressed_mods_distance
                    + held_mod_distance),
        )
    }
}
