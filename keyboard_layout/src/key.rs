//! This module provides structs for representing physical properties of keys in a keyboard

use ahash::AHashMap;
use serde::Deserialize;
use std::fmt;
use std::slice;

/// Row and columnar location on the keyboard
#[derive(Clone, Copy, Default, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct MatrixPosition(
    /// Index of column
    pub u8,
    /// Index of row
    pub u8,
);

/// 2D position on the keyboard
#[derive(Clone, Copy, Deserialize, PartialEq, Debug)]
pub struct Position(
    /// Horizontal positioning
    pub f64,
    /// Vertical positioning
    pub f64,
);

impl Default for Position {
    fn default() -> Self {
        Position(0.0, 0.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[repr(u8)]
pub enum Finger {
    Thumb,   // 0
    Pointer, // 1
    Middle,  // 2
    Ring,    // 3
    Pinky,   // 4
}

impl Default for Finger {
    fn default() -> Self {
        Finger::Thumb
    }
}

impl Finger {
    /// Counting distance between fingers (neighboring fingers have a distance of one)
    pub fn distance(&self, other: &Finger) -> u8 {
        (*self as u8).abs_diff(*other as u8)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[repr(u8)]
pub enum Hand {
    Left,  // 0
    Right, // 1
}

impl Default for Hand {
    fn default() -> Self {
        Hand::Left
    }
}

impl Hand {
    pub fn other(&self) -> Self {
        match self {
            Hand::Left => Hand::Right,
            Hand::Right => Hand::Left,
        }
    }
}

/// A map that associates each hand with a value
#[derive(Clone, Debug)]
pub struct HandMap<T: Copy>([T; 2]);

impl<T: Copy> HandMap<T> {
    pub fn with_default(default: T) -> Self {
        Self([default; 2])
    }

    pub fn with_hashmap(map: &AHashMap<Hand, T>, default: T) -> Self {
        let mut data = [default; 2];
        for (hand, elem) in map {
            data[*hand as usize] = *elem;
        }
        Self(data)
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn get(&self, hand: &Hand) -> &T {
        &self.0[*hand as usize]
    }

    pub fn get_mut(&mut self, hand: &Hand) -> &mut T {
        &mut self.0[*hand as usize]
    }

    pub fn set(&mut self, hand: &Hand, val: T) {
        self.0[*hand as usize] = val;
    }
}

impl<T: Copy + Default> Default for HandMap<T> {
    fn default() -> Self {
        Self([T::default(); 2])
    }
}

/// A map that associates each finger with a value
#[derive(Clone, Debug)]
pub struct FingerMap<T: Copy>([T; 5]);

impl<T: Copy> FingerMap<T> {
    pub fn with_default(default: T) -> Self {
        Self([default; 5])
    }

    pub fn with_hashmap(map: &AHashMap<Finger, T>, default: T) -> Self {
        let mut data = [default; 5];
        for (finger, elem) in map {
            data[*finger as usize] = *elem;
        }
        Self(data)
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn get(&self, finger: &Finger) -> &T {
        &self.0[*finger as usize]
    }

    pub fn set(&mut self, finger: &Finger, val: T) {
        self.0[*finger as usize] = val
    }
}

/// A map that associates each finger of each hand with a value
#[derive(Copy, Clone, Debug)]
pub struct HandFingerMap<T: Copy>([T; 10]);

impl<T: Copy> HandFingerMap<T> {
    pub fn with_default(default: T) -> Self {
        Self([default; 10])
    }

    pub fn with_hashmap(map: &AHashMap<Hand, AHashMap<Finger, T>>, default: T) -> Self {
        let mut data = [default; 10];
        for (hand, hand_map) in map {
            for (finger, elem) in hand_map {
                data[(*hand as usize) * 5 + (*finger as usize)] = *elem;
            }
        }
        Self(data)
    }

    #[inline(always)]
    fn index(hand: &Hand, finger: &Finger) -> usize {
        (*hand as usize) * 5 + (*finger as usize)
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.0.iter_mut()
    }

    pub fn set(&mut self, hand: &Hand, finger: &Finger, val: T) {
        self.0[Self::index(hand, finger)] = val;
    }

    pub fn get(&self, hand: &Hand, finger: &Finger) -> &T {
        &self.0[Self::index(hand, finger)]
    }

    pub fn get_mut(&mut self, hand: &Hand, finger: &Finger) -> &mut T {
        &mut self.0[Self::index(hand, finger)]
    }

    pub fn each_mut<F>(&mut self, f: F)
    where
        F: Fn(&Hand, &Finger, &mut T),
    {
        for hand in &[Hand::Left, Hand::Right] {
            for finger in &[
                Finger::Thumb,
                Finger::Pointer,
                Finger::Middle,
                Finger::Ring,
                Finger::Pinky,
            ] {
                f(hand, finger, &mut self.0[Self::index(hand, finger)]);
            }
        }
    }
}

impl<T: Copy + fmt::Display> fmt::Display for HandFingerMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = format!(
            "{:.2} {:.2} {:.2} {:.2} {:.2} - {:.2} {:.2} {:.2} {:.2} {:.2}",
            self.get(&Hand::Left, &Finger::Pinky),
            self.get(&Hand::Left, &Finger::Ring),
            self.get(&Hand::Left, &Finger::Middle),
            self.get(&Hand::Left, &Finger::Pointer),
            self.get(&Hand::Left, &Finger::Thumb),
            self.get(&Hand::Right, &Finger::Thumb),
            self.get(&Hand::Right, &Finger::Pointer),
            self.get(&Hand::Right, &Finger::Middle),
            self.get(&Hand::Right, &Finger::Ring),
            self.get(&Hand::Right, &Finger::Pinky),
        );

        write!(f, "{}", message)
    }
}

/// The [`Key`] struct represents a physical key on the keyboard. It provides various information about the location
/// of the key it represents and how it is (supposed to be) used, e.g. which hand and finger shall press it, how
/// "uncomfortable" it is to reach it (in terms of a cost valua), or if it forces the hand off the home row.
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Key {
    /// Hand of the finger used to press the key
    pub hand: Hand,

    /// Finger to press the key
    pub finger: Finger,

    /// Row and column position of the key
    pub matrix_position: MatrixPosition,

    /// 2D position of the key
    pub position: Position,

    /// Symmetriy index: Two different keys with identical symmetry index are considered symmetrical
    pub symmetry_index: u8,

    /// Cost value specifying how uncomfortable it is to reach/press the key
    pub cost: f64,

    /// How strongly does the hand need to move away from the home row (start position)
    pub unbalancing: f64,
}
