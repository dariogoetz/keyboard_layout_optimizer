use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Clone, Copy, Deserialize, PartialEq, Debug)]
pub struct Position(pub isize, pub isize);

impl Position {
    pub fn distance(&self, other: &Position) -> f64 {
        (0.5 * (self.0 as f64 - other.0 as f64).powi(2) + (self.1 as f64 - other.1 as f64).powi(2))
            .sqrt()
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum Finger {
    Pinky = 4,
    Ring = 3,
    Middle = 2,
    Pointer = 1,
    Thumb = 0,
}

impl Default for Finger {
    fn default() -> Self {
        Finger::Thumb
    }
}

impl Finger {
    pub fn distance(&self, other: &Finger) -> usize {
        (*self as isize - *other as isize).abs() as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum Hand {
    Left = 0,
    Right = 1,
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

#[derive(Clone, Debug)]
pub struct HandMap<T: Copy>([T; 2]);

impl<T: Copy> HandMap<T> {
    pub fn with_default(default: T) -> Self {
        Self([default; 2])
    }

    pub fn with_hashmap(map: &FxHashMap<Hand, T>, default: T) -> Self {
        let mut data = [default; 2];
        for (hand, elem) in map {
            data[*hand as usize] = *elem;
        }
        Self(data)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
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

#[derive(Clone, Debug)]
pub struct FingerMap<T: Copy>([T; 5]);

impl<T: Copy> FingerMap<T> {
    pub fn with_default(default: T) -> Self {
        Self([default; 5])
    }

    pub fn with_hashmap(map: &FxHashMap<Finger, T>, default: T) -> Self {
        let mut data = [default; 5];
        for (finger, elem) in map {
            data[*finger as usize] = *elem;
        }
        Self(data)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn get(&self, finger: &Finger) -> &T {
        &self.0[*finger as usize]
    }

    pub fn set(&mut self, finger: &Finger, val: T) {
        self.0[*finger as usize] = val
    }
}

#[derive(Copy, Clone, Debug)]
pub struct HandFingerMap<T: Copy>([T; 10]);

impl<T: Copy> HandFingerMap<T> {
    pub fn with_default(default: T) -> Self {
        Self([default; 10])
    }

    pub fn with_hashmap(map: &FxHashMap<Hand, FxHashMap<Finger, T>>, default: T) -> Self {
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

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
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

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Key {
    pub index: usize,
    pub hand: Hand,
    pub finger: Finger,
    pub position: Position,
    pub symmetry_key: usize,
    pub cost: f64,
    pub unbalancing: f64,
}
