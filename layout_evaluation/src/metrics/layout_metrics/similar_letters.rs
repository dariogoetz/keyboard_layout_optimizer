//! The layout metric `SimilarLetters` checks configurable pairs of keys
//! for sensible placement. (e.g. "aä", "bp", or "mn")
//! The keys' positioning is rated the following way:
//! - 0% cost if they are next to each other (not diagonal, though)
//! - 50% cost if they are in the same column but not touching (e.g. bottom row to top row)
//! - 50% cost if they have symmetric positions
//! - 100% cost if none of the criteria apply

use super::LayoutMetric;

use keyboard_layout::layout::Layout;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub similar_letters: Vec<(char, char)>,
}

#[derive(Clone, Debug)]
pub struct SimilarLetters {
    similar_letters: Vec<(char, char)>,
}

impl SimilarLetters {
    pub fn new(params: &Parameters) -> Self {
        Self {
            similar_letters: params.similar_letters.to_vec(),
        }
    }
}

impl LayoutMetric for SimilarLetters {
    fn name(&self) -> &str {
        "Similar Letters"
    }

    fn total_cost(&self, layout: &Layout) -> (f64, Option<String>) {
        let mut cost = 0.0;

        for (c1, c2) in &self.similar_letters {
            let cost_to_add;
            let key1 = &layout.get_layerkey_for_symbol(c1).unwrap().key;
            let key2 = &layout.get_layerkey_for_symbol(c2).unwrap().key;

            let neighbor_horizontally = key1.matrix_position.1 == key2.matrix_position.1
                && ((key1.matrix_position.0 - key2.matrix_position.0).abs() == 1);
            let neighbor_vertically = key1.matrix_position.0 == key2.matrix_position.0
                && (key1.matrix_position.1 - key2.matrix_position.1).abs() == 1;

            if neighbor_horizontally || neighbor_vertically {
                cost_to_add = 0.0;
            } else if key1.matrix_position.0 == key2.matrix_position.0 {
                cost_to_add = 0.5;
            } else if key1.symmetry_index == key2.symmetry_index {
                cost_to_add = 0.5;
            } else {
                cost_to_add = 1.0;
            }
            cost += cost_to_add;

            log::trace!(
                "{} {:?} - {} {:?} - Cost: {}",
                c1,
                key1.matrix_position,
                c2,
                key2.matrix_position,
                cost_to_add
            );
        }

        (cost, None)
    }
}
