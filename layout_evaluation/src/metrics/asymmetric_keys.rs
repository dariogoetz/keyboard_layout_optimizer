use super::LayoutMetric;

use keyboard_layout::{key::Hand, layout::Layout};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub similar_letters: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub struct AsymmetricKeys {
    similar_letters: Vec<(String, String)>,
}

impl AsymmetricKeys {
    pub fn new(params: &Parameters) -> Self {
        Self {
            similar_letters: params.similar_letters.to_vec(),
        }
    }
}

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

impl LayoutMetric for AsymmetricKeys {
    fn name(&self) -> &str {
        "Asymmetric Keys"
    }

    fn total_cost(&self, layout: &Layout) -> (f64, Option<String>) {
        let mut cost = 0.0;

        for (s1, s2) in &self.similar_letters {
            let chars1: Vec<char> = s1.chars().collect();
            let chars2: Vec<char> = s2.chars().collect();

            let mut hand_directions = Vec::new();
            let mut finger_directions = Vec::new();
            let mut column_distances = Vec::new();
            let mut v_directions = Vec::new();

            for (c1, c2) in chars1.iter().zip(chars2.iter()) {
                let key1 = layout.get_layerkey_for_char(c1).unwrap().key;
                let key2 = layout.get_layerkey_for_char(c2).unwrap().key;

                let hand_direction = match (&key1.hand, &key2.hand) {
                    (&Hand::Left, &Hand::Right) => 1,
                    (&Hand::Right, &Hand::Left) => -1,
                    _ => 0,
                };
                hand_directions.push(hand_direction);

                // take key1 - key2 for comparability with ArneBab
                let finger_direction = key1.finger as isize - key2.finger as isize;
                finger_directions.push(finger_direction);

                let column_distance = key2.position.0 - key1.position.0;
                column_distances.push(column_distance);

                let v_dist = key2.position.1 - key1.position.1;
                let v_direction = if v_dist == 0 {
                    0
                } else if v_dist < 0 {
                    -1
                } else {
                    1
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
