//! The `ngrams` module provides structs for reading (and to some extent modifying)
//! ngram (unigram, bigram, trigram) data that serve as the underlying data for layout
//! evaluations.

use anyhow::Result;
use rustc_hash::FxHashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Holds a hashmap of unigrams (single chars) with corresponding frequency (here often called "weight").
#[derive(Clone, Debug)]
pub struct Unigrams {
    pub grams: FxHashMap<char, f64>,
    pub total_weight: f64,
}

fn process_special_characters(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\\\", "\\")
}

fn process_special_characters_inverse(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\n", "\\n")
}

impl Unigrams {
    /// Collect unigrams from given text.
    pub fn from_str(text: &str) -> Result<Self> {
        let mut grams = FxHashMap::default();
        let mut total_weight = 0.0;
        text.chars()
            //.filter(|c| !c.is_whitespace())
            .for_each(|c| {
                *grams.entry(c).or_insert(0.0) += 1.0;
                total_weight += 1.0;
            });

        Ok(Self {
            grams,
            total_weight,
        })
    }

    /// Read unigrams and weights from a string containing lines with unigrams and their weights.
    pub fn from_frequencies_str(data: &str) -> Result<Self> {
        let mut grams = FxHashMap::default();
        let mut total_weight = 0.0;
        for line in data.lines() {
            let mut parts = line.trim_start().splitn(2, ' ');
            let weight: f64 = parts.next().unwrap().parse().unwrap();
            let unigram = parts.next().unwrap();
            let unigram = process_special_characters(unigram);
            let chars: Vec<char> = unigram.chars().collect();
            if chars.len() != 1 {
                log::error!("Len of unigram {} is unequad one: {:?}", unigram, chars);
            }
            let c = *chars.get(0).unwrap_or(&' ');
            total_weight += weight;
            *grams.entry(c).or_insert(0.0) += weight;
        }

        Ok(Unigrams {
            grams,
            total_weight,
        })
    }

    /// Read unigrams and weights from a file containing lines with unigrams and their weights.
    pub fn from_file(filename: &str) -> Result<Self> {
        let data = std::fs::read_to_string(filename)?;
        Unigrams::from_frequencies_str(&data)
    }

    /// Return a reduced set of the unigrams containing only the most common unigrams up to a
    /// given combined fraction.
    pub fn tops(&self, fraction: f64) -> Self {
        let target_weight = fraction * self.total_weight;
        let mut total_weight = 0.0;
        let mut sorted_grams: Vec<(char, f64)> = self.grams.clone().into_iter().collect();
        sorted_grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());
        let grams: FxHashMap<char, f64> = sorted_grams
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
        Self {
            grams,
            total_weight,
        }
    }

    /// Save frequencies to file
    pub fn save_frequencies<T: AsRef<Path>>(&self, filename: T) -> Result<(), String> {
        let p = filename.as_ref();
        create_dir_all(&p.parent().unwrap())
            .map_err(|e| format!("Unable to create directory '{}': {}", p.to_str().unwrap(), e))?;

        let mut grams: Vec<(char, f64)> = self.grams.iter().map(|(c, w)| (*c, *w) ).collect();
        grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

        let mut file = File::create(&filename)
            .map_err(|e| format!("Unable to create file '{}': {}", p.to_str().unwrap(), e))?;
        grams.iter().for_each(|(c, w)| {
            let processed = process_special_characters_inverse(&c.to_string());
            writeln!(&mut file, "{} {}", w, processed).unwrap();
        });

        Ok(())
    }
}

/// Holds a hashmap of bigrams (two chars) with corresponding frequency (here often called "weight").
#[derive(Clone, Debug)]
pub struct Bigrams {
    pub grams: FxHashMap<(char, char), f64>,
    pub total_weight: f64,
}

impl Bigrams {
    /// Collect bigrams from given text.
    pub fn from_str(text: &str) -> Result<Self> {
        let mut grams = FxHashMap::default();
        let mut total_weight = 0.0;
        text.chars()
            .zip(text.chars().skip(1))
            //.filter(|(c1, c2)| !c1.is_whitespace() && !c2.is_whitespace())
            .for_each(|c| {
                *grams.entry(c).or_insert(0.0) += 1.0;
                total_weight += 1.0;
            });

        Ok(Self {
            grams,
            total_weight,
        })
    }

    /// Read bigrams and weights from a string containing lines with bigrams and their weights.
    pub fn from_frequencies_str(data: &str) -> Result<Self> {
        let mut grams = FxHashMap::default();
        let mut total_weight = 0.0;
        for line in data.lines() {
            let mut parts = line.trim_start().splitn(2, ' ');
            let weight: f64 = parts.next().unwrap().parse().unwrap();
            let bigram = parts.next().unwrap();
            let bigram = process_special_characters(bigram);
            let c: Vec<char> = bigram.chars().collect();
            if c.len() != 2 {
                log::info!("Len of bigram {} is unequal two: {:?}", bigram, c);
            }
            total_weight += weight;
            *grams.entry((c[0], c[1])).or_insert(0.0) += weight;
        }

        Ok(Bigrams {
            grams,
            total_weight,
        })
    }

    /// Read bigrams and weights from a file containing lines with bigrams and their weights.
    pub fn from_file(filename: &str) -> Result<Self> {
        let data = std::fs::read_to_string(filename)?;
        Bigrams::from_frequencies_str(&data)
    }

    /// Return a reduced set of the bigrams containing only the most common bigrams up to a
    /// given combined fraction.
    pub fn tops(&self, fraction: f64) -> Self {
        let target_weight = fraction * self.total_weight;
        let mut total_weight = 0.0;
        let mut sorted_grams: Vec<((char, char), f64)> = self.grams.clone().into_iter().collect();
        sorted_grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());
        let grams: FxHashMap<(char, char), f64> = sorted_grams
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
        Self {
            grams,
            total_weight,
        }
    }

    /// Save frequencies to file
    pub fn save_frequencies<T: AsRef<Path>>(&self, filename: T) -> Result<(), String> {
        let p = filename.as_ref();
        create_dir_all(&p.parent().unwrap())
            .map_err(|e| format!("Unable to create directory '{}': {}", p.to_str().unwrap(), e))?;

        let mut grams: Vec<((char, char), f64)> = self.grams.iter().map(|(c, w)| (*c, *w) ).collect();
        grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

        let mut file = File::create(&filename)
            .map_err(|e| format!("Unable to create file '{}': {}", p.to_str().unwrap(), e))?;
        grams.iter().for_each(|((c1, c2), w)| {
            let processed1 = process_special_characters_inverse(&c1.to_string());
            let processed2 = process_special_characters_inverse(&c2.to_string());
            writeln!(&mut file, "{} {}{}", w, processed1, processed2).unwrap();
        });

        Ok(())
    }
}

/// Holds a hashmap of trigrams (three chars) with corresponding frequency (here often called "weight").
#[derive(Clone, Debug)]
pub struct Trigrams {
    pub grams: FxHashMap<(char, char, char), f64>,
    pub total_weight: f64,
}

impl Trigrams {
    /// Collect trigrams from given text.
    pub fn from_str(text: &str) -> Result<Self> {
        let mut grams = FxHashMap::default();
        let mut total_weight = 0.0;
        text.chars()
            .zip(text.chars().skip(1))
            .zip(text.chars().skip(2))
            //.filter(|((c1, c2), c3)| {
            //    !c1.is_whitespace() && !c2.is_whitespace() && !c3.is_whitespace()
            //})
            .for_each(|((c1, c2), c3)| {
                *grams.entry((c1, c2, c3)).or_insert(0.0) += 1.0;
                total_weight += 1.0;
            });

        Ok(Self {
            grams,
            total_weight,
        })
    }

    /// Read trigrams and weights from a string containing lines with trigrams and their weights.
    pub fn from_frequencies_str(data: &str) -> Result<Self> {
        let mut grams = FxHashMap::default();
        let mut total_weight = 0.0;
        for line in data.lines() {
            let mut parts = line.trim_start().splitn(2, ' ');
            let weight: f64 = parts.next().unwrap().parse().unwrap();
            let trigram = parts.next().unwrap();
            let trigram = process_special_characters(trigram);
            let c: Vec<char> = trigram.chars().collect();
            if c.len() != 3 {
                log::info!("Len of trigram {} is unequal three: {:?}", trigram, c);
            }
            total_weight += weight;
            *grams.entry((c[0], c[1], c[2])).or_insert(0.0) += weight;
        }

        Ok(Trigrams {
            grams,
            total_weight,
        })
    }

    /// Read trigrams and weights from a file containing lines with trigrams and their weights.
    pub fn from_file(filename: &str) -> Result<Self> {
        let data = std::fs::read_to_string(filename)?;
        Trigrams::from_frequencies_str(&data)
    }

    /// Return a reduced set of the trigrams containing only the most common trigrams up to a
    /// given combined fraction.
    pub fn tops(&self, fraction: f64) -> Self {
        let target_weight = fraction * self.total_weight;
        let mut total_weight = 0.0;
        let mut sorted_grams: Vec<((char, char, char), f64)> =
            self.grams.clone().into_iter().collect();
        sorted_grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());
        let grams: FxHashMap<(char, char, char), f64> = sorted_grams
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
        Self {
            grams,
            total_weight,
        }
    }

    /// Save frequencies to file
    pub fn save_frequencies<T: AsRef<Path>>(&self, filename: T) -> Result<(), String> {
        let p = filename.as_ref();
        create_dir_all(&p.parent().unwrap())
            .map_err(|e| format!("Unable to create directory '{}': {}", p.to_str().unwrap(), e))?;

        let mut grams: Vec<((char, char, char), f64)> = self.grams.iter().map(|(c, w)| (*c, *w) ).collect();
        grams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

        let mut file = File::create(&filename)
            .map_err(|e| format!("Unable to create file '{}': {}", p.to_str().unwrap(), e))?;
        grams.iter().for_each(|((c1, c2, c3), w)| {
            let processed1 = process_special_characters_inverse(&c1.to_string());
            let processed2 = process_special_characters_inverse(&c2.to_string());
            let processed3 = process_special_characters_inverse(&c3.to_string());
            writeln!(&mut file, "{} {}{}{}", w, processed1, processed2, processed3).unwrap();
        });

        Ok(())
    }
}
