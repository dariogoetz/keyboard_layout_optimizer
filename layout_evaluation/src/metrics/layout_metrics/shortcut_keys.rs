//! The layout metric [`ShortcutKeys`] penalizes configurable letters (usually "cvxz")
//! if they are not within the first 5 columns of the left side of the keyboard.
//! The cost to add for each such a letter is configurable.

use super::LayoutMetric;

use keyboard_layout::layout::Layout;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub shortcut_chars: String,
    pub cost: f64,
    pub within_n_leftmost_cols: u8,
}

#[derive(Clone, Debug)]
pub struct ShortcutKeys {
    shortcut_chars: Vec<char>,
    cost: f64,
    within_n_leftmost_cols: u8,
}

impl ShortcutKeys {
    pub fn new(params: &Parameters) -> Self {
        Self {
            shortcut_chars: params.shortcut_chars.chars().collect(),
            cost: params.cost,
            within_n_leftmost_cols: params.within_n_leftmost_cols,
        }
    }
}

impl LayoutMetric for ShortcutKeys {
    fn name(&self) -> &str {
        "Badly Positioned Shortcut Keys"
    }

    fn total_cost(&self, layout: &Layout) -> (f64, Option<String>) {
        let mut cost = 0.0;
        let mut bad_keys = Vec::new();
        self.shortcut_chars.iter().for_each(|c| {
            if let Some(k) = layout.get_layerkey_for_symbol(c) {
                // NOTE: In ArneBab's solution, the top rows do not "skip a column" as we do.
                // Therefore, a special case needs to be made for row 3, in contrast to here.
                if k.key.matrix_position.0 > self.within_n_leftmost_cols {
                    cost += self.cost;
                    bad_keys.push(*c);
                    log::trace!(
                        "Shorcut: {}, Finger: {:>13}, Matrix Position: {:.0} (is > {}), Cost: {:>2.2}",
                        c.escape_debug().to_string(),
                        format!("{:?} {:?}", k.key.hand, k.key.finger),
                        k.key.matrix_position.0,
                        self.within_n_leftmost_cols,
                        self.cost
                    );
                }
            }
        });

        let message = if !bad_keys.is_empty() {
            Some(format!(
                "Bad shortcuts: {}",
                bad_keys.iter().collect::<String>()
            ))
        } else {
            None
        };

        (cost, message)
    }
}
