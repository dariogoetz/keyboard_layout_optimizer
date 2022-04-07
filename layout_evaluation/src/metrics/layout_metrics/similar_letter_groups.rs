//! Used to be called "`AsymmetricKeys`"" in earlier versions of this optimizer.
//!
//! The layout metric [`SimilarLetterGroups`] matches the relative locations of configurable pairs of
//! groups of keys against each other, e.g. "aou" and "äöü". If each key has the same relative
//! location to its counterpart as the others the costs are zero.
//! Otherwise, a cost is given for each inconsistency in
//! - hand directon (left to right or right to left)
//! - finger distance with direction
//! - column distance
//! - vertical direction (top to bottom or bottom to top)

use super::LayoutMetric;

use keyboard_layout::{key::Hand, layout::Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub letter_group_pairs: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub struct SimilarLetterGroups {
    letter_group_pairs: Vec<(String, String)>,
}

impl SimilarLetterGroups {
    pub fn new(params: &Parameters) -> Self {
        Self {
            letter_group_pairs: params.letter_group_pairs.to_vec(),
        }
    }
}

/// Compares how many values of [data] are not equal to other values of [data].
/// More differences result in a higher cost.
fn costs<T: PartialEq>(data: &[T]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut cost = 0.0;
    let mut n = 0.0;
    for (i, d1) in data.iter().enumerate() {
        for d2 in data.iter().skip(i + 1) {
            n += 1.0;
            if d1 != d2 {
                cost += 1.0;
            }
        }
    }

    ((cost / n) as f64).ln_1p()
}

impl LayoutMetric for SimilarLetterGroups {
    fn name(&self) -> &str {
        "Similar Letter-Groups"
    }

    fn total_cost(&self, layout: &Layout) -> (f64, Option<String>) {
        let mut cost = 0.0;

        for (s1, s2) in &self.letter_group_pairs {
            let letters_per_group = s1.len();
            let mut hand_directions: Vec<i8> = Vec::with_capacity(letters_per_group);
            let mut finger_directions: Vec<i8> = Vec::with_capacity(letters_per_group);
            let mut column_distances: Vec<i8> = Vec::with_capacity(letters_per_group);
            let mut v_directions: Vec<i8> = Vec::with_capacity(letters_per_group);

            for (c1, c2) in s1.chars().zip(s2.chars()) {
                let key1 = &layout.get_layerkey_for_symbol(&c1).unwrap().key;
                let key2 = &layout.get_layerkey_for_symbol(&c2).unwrap().key;

                let hand_direction = match (&key1.hand, &key2.hand) {
                    (&Hand::Left, &Hand::Right) => 1,
                    (&Hand::Right, &Hand::Left) => -1,
                    _ => 0,
                };
                hand_directions.push(hand_direction);

                // take key1 - key2 for comparability with ArneBab
                let finger_direction = key1.finger as i8 - key2.finger as i8;
                finger_directions.push(finger_direction);

                let column_distance = key2.matrix_position.0 as i8 - key1.matrix_position.0 as i8;
                column_distances.push(column_distance);

                let v_dist = key2.matrix_position.1 as i8 - key1.matrix_position.1 as i8;
                let v_direction = match v_dist {
                    0 => 0,
                    d if d < 0 => -1,
                    _ => 1,
                };
                v_directions.push(v_direction);
            }

            cost += costs(&hand_directions)
                + costs(&finger_directions)
                + costs(&column_distances)
                + costs(&v_directions);

            if cost > 0.0 {
                log::trace!(
                    "{} - {}, Hand direction: {:.2}, Finger direction: {:.2}, Column distance: {:.2}, Vertical direction: {:.2}",
                    s1, s2,
                    costs(&hand_directions),
                    costs(&finger_directions),
                    costs(&column_distances),
                    costs(&v_directions),
                )
            }
        }

        (cost, None)
    }
}
