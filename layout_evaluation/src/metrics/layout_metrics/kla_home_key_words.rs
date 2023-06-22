use super::LayoutMetric;

use std::iter::FromIterator;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::{key::MatrixPosition, layout::Layout};

use serde::Deserialize;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub words_filename: String,
    pub min_word_length: usize,
    pub home_row_positions: Vec<MatrixPosition>,
}

#[derive(Clone, Debug)]
pub struct KLAHomeKeyWords {
    words: AHashMap<String, (AHashSet<char>, usize, f64)>, // set of chars, number of unique chars, weight
    home_row_positions: AHashSet<MatrixPosition>,
}

#[derive(Debug, Deserialize)]
struct WordRecord {
    _row: usize,
    word: String,
    weight: f64,
}

impl KLAHomeKeyWords {
    pub fn new(params: &Parameters) -> Self {
        let file = File::open(&params.words_filename)
            .unwrap_or_else(|_| panic!("Could not open words file {}", params.words_filename));
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(file);

        let mut words = AHashMap::default();
        reader.deserialize().for_each(|r| {
            let r: WordRecord = r.expect("Could not read record!");

            if r.word.len() >= params.min_word_length {
                let word = r.word.to_lowercase();
                // only store unique characters
                let s = AHashSet::from_iter(word.to_lowercase().chars());
                let l = s.len();
                let entry = words.entry(word).or_insert((s, l, 0.0));
                entry.2 += r.weight;
            }
        });
        Self {
            words,
            home_row_positions: AHashSet::from_iter(params.home_row_positions.iter().cloned()),
        }
    }
}

impl LayoutMetric for KLAHomeKeyWords {
    fn name(&self) -> &str {
        "Home Key Words"
    }

    fn total_cost(&self, layout: &Layout) -> (f64, Option<String>) {
        let mut found_weight = 0.0;
        let mut found_words = 0;

        let home_row_chars: AHashSet<char> = layout
            .layerkeys
            .iter()
            .filter_map(|k| {
                if k.layer == 0 && self.home_row_positions.contains(&k.key.matrix_position) {
                    Some(k.symbol)
                } else {
                    None
                }
            })
            .collect();

        self.words.iter().for_each(|(_word, (chars, len, weight))| {
            if home_row_chars.is_superset(chars) {
                found_weight += *len as f64 * *weight;
                found_words += 1;
            }
        });

        let total_words: f64 = self.words.len() as f64;
        let total_weight: f64 = self
            .words
            .iter()
            .map(|(_word, (_chars, _len, weight))| *weight)
            .sum();

        let message = format!(
            "{} out of {} (distinct lowercase) words",
            found_words, total_words
        );

        let cost = -(found_weight / total_weight);

        (cost, Some(message))
    }
}
