//! The `ngrams` module provides structs for reading (and to some extent modifying)
//! ngram (unigram, bigram, trigram) data that serve as the underlying data for layout
//! evaluations.

use crate::ngram_mapper::common::NgramMap;

use ahash::AHashMap;
use anyhow::Result;
use serde::Deserialize;
use std::{
    fs::{self, create_dir_all, File},
    io::{BufWriter, Write},
    path::Path,
};

/// Configuration parameters for ngram processing
#[derive(Debug, Clone, Deserialize)]
pub struct NgramsConfig {
    /// Parameters for the increase in weight of common ngrams (with already high frequency).
    pub increase_common_ngrams: IncreaseCommonNgramsConfig,
}

/// Configuration parameters for process of increasing the weight of common ngrams.
#[derive(Debug, Clone, Deserialize)]
pub struct IncreaseCommonNgramsConfig {
    /// Whether to increase the weight of common ngrams even further.
    pub enabled: bool,
    /// The critical fraction above which a ngram's weight will be increased.
    pub critical_fraction: f64,
    /// The slope with which the ngram's weight will be increased.
    /// The increment is performed linearly starting from the critical fraction,
    /// i.e. a ngram with weight equal the critical fraction is actually not affected.
    pub factor: f64,
    /// A minimum total weight (of all ngrams) that needs to be achieved. Otherwise no
    /// increment takes place.
    pub total_weight_threshold: f64,
}

impl Default for IncreaseCommonNgramsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            critical_fraction: 0.001,
            factor: 2.0,
            total_weight_threshold: 20.0,
        }
    }
}

pub fn increase_common_ngrams<T>(
    symbol_weights: &mut AHashMap<T, f64>,
    config: &IncreaseCommonNgramsConfig,
) {
    if !config.enabled {
        return;
    }

    let total_weight: f64 = symbol_weights.values().sum();
    let critical_point = config.critical_fraction * total_weight;

    symbol_weights.values_mut().for_each(|weight| {
        if *weight > critical_point && total_weight > config.total_weight_threshold {
            *weight += (*weight - critical_point) * (config.factor - 1.0);
        }
    });
}

/// Holds a hashmap of unigrams (single chars) with corresponding frequency (here often called "weight").
#[derive(Clone, Debug)]
pub struct Unigrams {
    pub grams: AHashMap<char, f64>,
}

fn process_special_characters(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\\\", "\\")
}

fn process_special_characters_inverse(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n")
}

impl Unigrams {
    /// Collect unigrams from given text.
    pub fn from_text(text: &str) -> Result<Self> {
        let mut grams = AHashMap::default();
        let chars = text.chars().filter(|c| *c != '\r');
        chars
            //.filter(|c| !c.is_whitespace())
            .for_each(|c| {
                grams.insert_or_add_weight(c, 1.0);
            });

        Ok(Self { grams })
    }

    /// Read unigrams and weights from a string containing lines with unigrams and their weights.
    pub fn from_frequencies_str(data: &str) -> Result<Self> {
        let mut grams = AHashMap::default();
        for line in data.lines() {
            let mut parts = line.trim_start().splitn(2, ' ');
            let weight: f64 = parts.next().unwrap().parse().unwrap();
            let unigram = parts.next().unwrap();
            let unigram = process_special_characters(unigram);
            let chars: Vec<char> = unigram.chars().collect();
            if chars.len() != 1 {
                log::error!("Len of unigram {} is unequad one: {:?}", unigram, chars);
            }
            let c = *chars.first().unwrap_or(&' ');
            grams.insert_or_add_weight(c, weight);
        }

        Ok(Unigrams { grams })
    }

    /// Read unigrams and weights from a file containing lines with unigrams and their weights.
    pub fn from_file(filename: &str) -> Result<Self> {
        let data = fs::read_to_string(filename)?;
        Unigrams::from_frequencies_str(&data)
    }

    /// Total weight of all combined unigrams
    pub fn total_weight(&self) -> f64 {
        self.grams.values().sum()
    }

    /// Return a reduced set of the unigrams containing only the most common unigrams up to a
    /// given combined fraction.
    pub fn tops(&self, fraction: f64) -> Self {
        let target_weight = fraction * self.total_weight();
        let mut total_weight = 0.0;
        let mut sorted_grams: Vec<(char, f64)> = self.grams.clone().into_iter().collect();
        sorted_grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());
        let grams: AHashMap<char, f64> = sorted_grams
            .iter()
            .take_while(|(_c, w)| {
                let res = total_weight < target_weight;
                total_weight += *w;

                res
            })
            .cloned()
            .collect();

        log::info!(
            "Unigrams: Reducing from originally {} to the top {} ngrams.",
            self.grams.len(),
            grams.len()
        );
        Self { grams }
    }

    // Return a reduced set of unigrams filtering out those containing a given character
    pub fn exclude_char(&self, exclude: &char) -> Self {
        let grams: AHashMap<char, f64> = self
            .grams
            .iter()
            .filter_map(|(c, w)| if *c == *exclude { None } else { Some((*c, *w)) })
            .collect();
        Self { grams }
    }

    /// Save frequencies to file
    pub fn save_frequencies<T: AsRef<Path>>(&self, filename: T) -> Result<(), String> {
        let p = filename.as_ref();
        create_dir_all(p.parent().unwrap()).map_err(|e| {
            format!(
                "Unable to create directory '{}': {}",
                p.to_str().unwrap(),
                e
            )
        })?;

        let mut grams: Vec<(char, f64)> = self.grams.iter().map(|(c, w)| (*c, *w)).collect();
        grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

        let file = File::create(&filename)
            .map_err(|e| format!("Unable to create file '{}': {}", p.to_str().unwrap(), e))?;
        let mut buf_writer = BufWriter::new(file);
        grams.iter().for_each(|(c, w)| {
            let processed = process_special_characters_inverse(&c.to_string());
            writeln!(&mut buf_writer, "{} {}", w, processed).unwrap();
        });

        Ok(())
    }

    pub fn increase_common(&self, params: &IncreaseCommonNgramsConfig) -> Self {
        let mut grams = self.grams.clone();
        increase_common_ngrams(&mut grams, params);
        Self { grams }
    }
}

/// Holds a hashmap of bigrams (two chars) with corresponding frequency (here often called "weight").
#[derive(Clone, Debug)]
pub struct Bigrams {
    pub grams: AHashMap<(char, char), f64>,
}

impl Bigrams {
    /// Collect bigrams from given text.
    pub fn from_text(text: &str) -> Result<Self> {
        let mut grams = AHashMap::default();
        let chars = text.chars().filter(|c| *c != '\r');
        chars
            .clone()
            .zip(chars.clone().skip(1))
            //.filter(|(c1, c2)| !c1.is_whitespace() && !c2.is_whitespace())
            .for_each(|c| {
                grams.insert_or_add_weight(c, 1.0);
            });

        Ok(Self { grams })
    }

    /// Read bigrams and weights from a string containing lines with bigrams and their weights.
    pub fn from_frequencies_str(data: &str) -> Result<Self> {
        let mut grams = AHashMap::default();
        for line in data.lines() {
            let mut parts = line.trim_start().splitn(2, ' ');
            let weight: f64 = parts.next().unwrap().parse().unwrap();
            let bigram = parts.next().unwrap();
            let bigram = process_special_characters(bigram);
            let c: Vec<char> = bigram.chars().collect();
            if c.len() != 2 {
                log::info!("Len of bigram {} is unequal two: {:?}", bigram, c);
            }
            grams.insert_or_add_weight((c[0], c[1]), weight);
        }

        Ok(Bigrams { grams })
    }

    /// Read bigrams and weights from a file containing lines with bigrams and their weights.
    pub fn from_file(filename: &str) -> Result<Self> {
        let data = fs::read_to_string(filename)?;
        Bigrams::from_frequencies_str(&data)
    }

    /// Total weight of all combined bigrams
    pub fn total_weight(&self) -> f64 {
        self.grams.values().sum()
    }

    /// Return a reduced set of the bigrams containing only the most common bigrams up to a
    /// given combined fraction.
    pub fn tops(&self, fraction: f64) -> Self {
        let target_weight = fraction * self.total_weight();
        let mut total_weight = 0.0;
        let mut sorted_grams: Vec<((char, char), f64)> = self.grams.clone().into_iter().collect();
        sorted_grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());
        let grams: AHashMap<(char, char), f64> = sorted_grams
            .iter()
            .take_while(|(_c, w)| {
                let res = total_weight < target_weight;
                total_weight += *w;

                res
            })
            .cloned()
            .collect();

        log::info!(
            "Bigrams: Reducing from originally {} to the top {} ngrams.",
            self.grams.len(),
            grams.len()
        );
        Self { grams }
    }

    // Return a reduced set of bigrams filtering out those containing a given character
    pub fn exclude_char(&self, exclude: &char) -> Self {
        let grams: AHashMap<(char, char), f64> = self
            .grams
            .iter()
            .filter_map(|((c1, c2), w)| {
                if *c1 == *exclude || *c2 == *exclude {
                    None
                } else {
                    Some(((*c1, *c2), *w))
                }
            })
            .collect();
        Self { grams }
    }

    /// Save frequencies to file
    pub fn save_frequencies<T: AsRef<Path>>(&self, filename: T) -> Result<(), String> {
        let p = filename.as_ref();
        create_dir_all(p.parent().unwrap()).map_err(|e| {
            format!(
                "Unable to create directory '{}': {}",
                p.to_str().unwrap(),
                e
            )
        })?;

        let mut grams: Vec<((char, char), f64)> =
            self.grams.iter().map(|(c, w)| (*c, *w)).collect();
        grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

        let file = File::create(&filename)
            .map_err(|e| format!("Unable to create file '{}': {}", p.to_str().unwrap(), e))?;
        let mut buf_writer = BufWriter::new(file);
        grams.iter().for_each(|((c1, c2), w)| {
            let processed1 = process_special_characters_inverse(&c1.to_string());
            let processed2 = process_special_characters_inverse(&c2.to_string());
            writeln!(&mut buf_writer, "{} {}{}", w, processed1, processed2).unwrap();
        });

        Ok(())
    }

    pub fn increase_common(&self, params: &IncreaseCommonNgramsConfig) -> Self {
        let mut grams = self.grams.clone();
        increase_common_ngrams(&mut grams, params);
        Self { grams }
    }
}

/// Holds a hashmap of trigrams (three chars) with corresponding frequency (here often called "weight").
#[derive(Clone, Debug)]
pub struct Trigrams {
    pub grams: AHashMap<(char, char, char), f64>,
}

impl Trigrams {
    /// Collect trigrams from given text.
    pub fn from_text(text: &str) -> Result<Self> {
        let mut grams = AHashMap::default();
        let chars = text.chars().filter(|c| *c != '\r');
        chars
            .clone()
            .zip(chars.clone().skip(1))
            .zip(chars.clone().skip(2))
            //.filter(|((c1, c2), c3)| {
            //    !c1.is_whitespace() && !c2.is_whitespace() && !c3.is_whitespace()
            //})
            .for_each(|((c1, c2), c3)| {
                grams.insert_or_add_weight((c1, c2, c3), 1.0);
            });

        Ok(Self { grams })
    }

    /// Read trigrams and weights from a string containing lines with trigrams and their weights.
    pub fn from_frequencies_str(data: &str) -> Result<Self> {
        let mut grams = AHashMap::default();
        for line in data.lines() {
            let mut parts = line.trim_start().splitn(2, ' ');
            let weight: f64 = parts.next().unwrap().parse().unwrap();
            let trigram = parts.next().unwrap();
            let trigram = process_special_characters(trigram);
            let c: Vec<char> = trigram.chars().collect();
            if c.len() != 3 {
                log::info!("Len of trigram {} is unequal three: {:?}", trigram, c);
            }
            grams.insert_or_add_weight((c[0], c[1], c[2]), weight);
        }

        Ok(Trigrams { grams })
    }

    /// Read trigrams and weights from a file containing lines with trigrams and their weights.
    pub fn from_file(filename: &str) -> Result<Self> {
        let data = fs::read_to_string(filename)?;
        Trigrams::from_frequencies_str(&data)
    }

    /// Total weight of all combined trigrams
    pub fn total_weight(&self) -> f64 {
        self.grams.values().sum()
    }

    /// Return a reduced set of the trigrams containing only the most common trigrams up to a
    /// given combined fraction.
    pub fn tops(&self, fraction: f64) -> Self {
        let target_weight = fraction * self.total_weight();
        let mut total_weight = 0.0;
        let mut sorted_grams: Vec<((char, char, char), f64)> =
            self.grams.clone().into_iter().collect();
        sorted_grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());
        let grams: AHashMap<(char, char, char), f64> = sorted_grams
            .iter()
            .take_while(|(_c, w)| {
                let res = total_weight < target_weight;
                total_weight += *w;

                res
            })
            .cloned()
            .collect();

        log::info!(
            "Trigrams: Reducing from originally {} to the top {} ngrams.",
            self.grams.len(),
            grams.len()
        );
        Self { grams }
    }

    // Return a reduced set of trigrams filtering out those containing a given character
    pub fn exclude_char(&self, exclude: &char) -> Self {
        let grams: AHashMap<(char, char, char), f64> = self
            .grams
            .iter()
            .filter_map(|((c1, c2, c3), w)| {
                if *c1 == *exclude || *c2 == *exclude || *c3 == *exclude {
                    None
                } else {
                    Some(((*c1, *c2, *c3), *w))
                }
            })
            .collect();
        Self { grams }
    }

    /// Save frequencies to file
    pub fn save_frequencies<T: AsRef<Path>>(&self, filename: T) -> Result<(), String> {
        let p = filename.as_ref();
        create_dir_all(p.parent().unwrap()).map_err(|e| {
            format!(
                "Unable to create directory '{}': {}",
                p.to_str().unwrap(),
                e
            )
        })?;

        let mut grams: Vec<((char, char, char), f64)> =
            self.grams.iter().map(|(c, w)| (*c, *w)).collect();
        grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

        let file = File::create(&filename)
            .map_err(|e| format!("Unable to create file '{}': {}", p.to_str().unwrap(), e))?;
        let mut buf_writer = BufWriter::new(file);
        grams.iter().for_each(|((c1, c2, c3), w)| {
            let processed1 = process_special_characters_inverse(&c1.to_string());
            let processed2 = process_special_characters_inverse(&c2.to_string());
            let processed3 = process_special_characters_inverse(&c3.to_string());
            writeln!(
                &mut buf_writer,
                "{} {}{}{}",
                w, processed1, processed2, processed3
            )
            .unwrap();
        });

        Ok(())
    }

    pub fn increase_common(&self, params: &IncreaseCommonNgramsConfig) -> Self {
        let mut grams = self.grams.clone();
        increase_common_ngrams(&mut grams, params);
        Self { grams }
    }
}
