//! This module provides a struct representing a keyboard.

use crate::key::{Finger, Hand, HandFingerMap, Key, MatrixPosition, Position};

use ahash::{AHashMap, AHashSet};
use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Invalid keyboard: Not the same number of keys in each keyboard list.")]
    WrongKeyNumber,
    #[error("Invalid keyboard: Duplicate `matrix_positions`.")]
    DuplicateMatrixPositions,
    #[error("Invalid keyboard: Duplicate `positions`.")]
    DuplicatePositions,
}

/// The index of a [`Key`] in the `keys` vec of a [`Keyboard`]
pub type KeyIndex = u8;

/// A struct representing a keyboard as a list of keys
#[derive(Clone, Debug)]
pub struct Keyboard {
    /// The keys of the keyboard
    pub keys: Vec<Key>,
    pub finger_resting_positions: HandFingerMap<Position>,
    plot_template: String,
    plot_template_short: String,
}

/// A collection of all relevant properties for the keys on a keyboard (configuration).
///
/// Corresponds to (parts of) a YAML configuration file.
#[derive(Deserialize, Debug)]
pub struct KeyboardYAML {
    matrix_positions: Vec<Vec<MatrixPosition>>,
    positions: Vec<Vec<Position>>,
    hands: Vec<Vec<Hand>>,
    fingers: Vec<Vec<Finger>>,
    key_costs: Vec<Vec<f64>>,
    symmetries: Vec<Vec<u8>>,
    unbalancing_positions: Vec<Vec<Position>>,
    finger_resting_positions: AHashMap<Hand, AHashMap<Finger, Position>>,
    plot_template: String,
    plot_template_short: String,
}

/// Takes a slice of some iterable and checks whether that iterable contains
/// duplicates of any of its elements.
fn contains_duplicates<T: PartialEq>(v: &[T]) -> bool {
    // Cycle through all elements
    v.iter().enumerate().any(|(first_idx, checked_pos)| {
        // Get index of last element that is equal to `checked_pos`
        let last_idx = v.iter().rposition(|pos| pos == checked_pos).unwrap();
        // See if that last element is different from the first one,
        // which would be a duplicate.
        first_idx != last_idx
    })
}

impl KeyboardYAML {
    /// Checks the [`KeyboardYAML`] for common errors.
    pub fn validate(&self) -> Result<()> {
        let flat_matrix_positions = self.matrix_positions.concat();
        let flat_positions = self.positions.concat();

        // Make sure that all settings that should have the same number of elements
        // do in fact have the same number of elements.
        let mut lengths = AHashSet::default();
        lengths.insert(flat_matrix_positions.len());
        lengths.insert(flat_positions.len());
        lengths.insert(self.hands.concat().len());
        lengths.insert(self.fingers.concat().len());
        lengths.insert(self.key_costs.concat().len());
        lengths.insert(self.symmetries.concat().len());
        lengths.insert(self.unbalancing_positions.concat().len());
        if lengths.len() > 1 {
            return Err(KeyboardError::WrongKeyNumber.into());
        }

        // Make sure there are no duplicates in `matrix_positions`.
        if contains_duplicates(&flat_matrix_positions) {
            return Err(KeyboardError::DuplicateMatrixPositions.into());
        }

        // Make sure there are no duplicates in `positions`.
        if contains_duplicates(&flat_positions) {
            return Err(KeyboardError::DuplicatePositions.into());
        }

        Ok(())
    }
}

impl Keyboard {
    /// Generate a [`Keyboard`] from a [`KeyboardYAML`] object
    pub fn from_yaml_object(k: KeyboardYAML) -> Self {
        let keys = k
            .hands
            .into_iter()
            .flatten()
            .zip(k.fingers.into_iter().flatten())
            .zip(k.matrix_positions.into_iter().flatten())
            .zip(k.positions.into_iter().flatten())
            .zip(k.symmetries.into_iter().flatten())
            .zip(k.key_costs.into_iter().flatten())
            .zip(k.unbalancing_positions.into_iter().flatten())
            .map(
                |(
                    (((((hand, finger), matrix_position), position), symmetry_index), cost),
                    unbalancing,
                )| Key {
                    hand,
                    finger,
                    matrix_position,
                    position,
                    symmetry_index,
                    cost,
                    unbalancing,
                },
            )
            .collect();

        Keyboard {
            keys,
            finger_resting_positions: HandFingerMap::with_hashmap(
                &k.finger_resting_positions,
                Position::default(),
            ),
            plot_template: k.plot_template,
            plot_template_short: k.plot_template_short,
        }
    }

    /// Generate a [`Keyboard`] from a YAML file
    pub fn from_yaml_file(filename: &str) -> Result<Self> {
        let f = File::open(filename)?;
        let k: KeyboardYAML = serde_yaml::from_reader(f)?;
        Ok(Keyboard::from_yaml_object(k))
    }

    /// Generate a [`Keyboard`] from a YAML string
    pub fn from_yaml_str(data: &str) -> Result<Self> {
        let k: KeyboardYAML = serde_yaml::from_str(data)?;
        Ok(Keyboard::from_yaml_object(k))
    }

    /// Plot a graphical representation of the keyboard with given key labels
    pub fn plot(&self, key_labels: &[String]) -> String {
        let mut reg = handlebars::Handlebars::new();
        reg.register_escape_fn(handlebars::no_escape);
        let labels: AHashMap<usize, String> = key_labels.iter().cloned().enumerate().collect();
        reg.render_template(&self.plot_template, &labels).unwrap()
    }

    /// Plot a compact graphical representation of the keyboard with given key labels without borders (compatible with ArneBab's input strings)
    pub fn plot_compact(&self, key_labels: &[String]) -> String {
        let mut reg = handlebars::Handlebars::new();
        reg.register_escape_fn(handlebars::no_escape);
        let labels: AHashMap<usize, String> = key_labels.iter().cloned().enumerate().collect();
        reg.render_template(&self.plot_template_short, &labels)
            .unwrap()
    }

    pub fn estimated_finger_loads(&self, exclude_thumbs: bool) -> HandFingerMap<f64> {
        let mut intended_loads: HandFingerMap<f64> = HandFingerMap::with_default(0.0);

        self.keys
            .iter()
            .filter(|k| !exclude_thumbs || k.finger != Finger::Thumb)
            .for_each(|k| {
                let il = intended_loads.get_mut(&k.hand, &k.finger);
                *il += 1.0 / (1.0 + k.cost);
            });

        let sum: f64 = intended_loads.iter().sum();
        intended_loads.iter_mut().for_each(|il| *il /= sum);

        intended_loads
    }

    pub fn estimated_row_loads(&self) -> AHashMap<u8, f64> {
        let mut intended_loads: AHashMap<u8, f64> = AHashMap::default();

        self.keys.iter().for_each(|k| {
            let il = intended_loads.entry(k.matrix_position.1).or_insert(0.0);
            *il += 1.0 / (1.0 + k.cost);
        });

        let sum: f64 = intended_loads.values().sum();
        intended_loads.values_mut().for_each(|il| *il /= sum);

        intended_loads
    }
}
