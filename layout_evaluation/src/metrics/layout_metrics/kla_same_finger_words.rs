use std::iter::FromIterator;

use super::LayoutMetric;

use ahash::{AHashMap, AHashSet};
use keyboard_layout::layout::Layout;

use serde::Deserialize;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    pub words_filename: String,
    pub min_word_length: usize,
}

#[derive(Clone, Debug)]
pub struct KLASameFingerWords {
    words: AHashMap<String, (usize, f64)>, // (chars, number of unique chars, weight)
}

#[derive(Debug, Deserialize)]
struct WordRecord {
    _row: usize,
    word: String,
    weight: f64,
}

impl KLASameFingerWords {
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
                let l = AHashSet::<char>::from_iter(word.chars()).len(); // use number of unique
                let entry = words.entry(word).or_insert((l, 0.0));
                entry.1 += r.weight;
            }
        });
        Self { words }
    }
}

impl LayoutMetric for KLASameFingerWords {
    fn name(&self) -> &str {
        "Same Finger Words"
    }

    fn total_cost(&self, layout: &Layout) -> (f64, Option<String>) {
        let mut found_char_weight = 0.0;
        let mut found_words = 0;

        self.words.iter().for_each(|(word, (len, weight))| {
            // map chars to LayerKeys
            let layerkeys: Vec<_> = word
                .chars()
                .map(|c| layout.get_layerkey_for_symbol(&c))
                .collect();

            // check if all bigrams are not a finger-repeat (on different keys)
            if layerkeys
                .iter()
                .zip(layerkeys.iter().skip(1))
                .all(|(k1, k2)| {
                    if let (Some(k1), Some(k2)) = (k1, k2) {
                        k1.key == k2.key
                            || k1.key.hand != k2.key.hand
                            || k1.key.finger != k2.key.finger
                    } else {
                        false
                    }
                })
            {
                found_char_weight += *len as f64 * *weight;
                found_words += 1;
            }
        });

        let total_words: f64 = self.words.len() as f64;
        let total_weight: f64 = self
            .words
            .iter()
            .map(|(_word, (_len, weight))| *weight)
            .sum();

        let message = format!(
            "{} out of {} (distinct lowercase) words",
            found_words, total_words
        );

        let cost = -(found_char_weight / total_weight);

        (cost, Some(message))
    }
}
