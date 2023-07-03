/// The `common` module provides utility functions for resolving modifiers in ngrams.
use keyboard_layout::layout::LayerKeyIndex;

use ahash::AHashMap;
use std::{cmp::Eq, hash::Hash, slice};

/// Iterator over unigrams of the base-layer key and each modifier.
#[derive(Clone, Debug)]
pub struct TakeOneLayerKey<'a> {
    base_key: LayerKeyIndex,
    modifiers: &'a [LayerKeyIndex],
    weight: f64,
    iter: Option<slice::Iter<'a, LayerKeyIndex>>,
}

impl<'a> Iterator for TakeOneLayerKey<'a> {
    type Item = (LayerKeyIndex, f64);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            None => {
                self.iter = Some(self.modifiers.iter());
                Some((self.base_key, self.weight))
            }
            Some(iter) => iter.next().map(|m| (*m, self.weight)),
        }
    }
}

impl<'a> TakeOneLayerKey<'a> {
    pub fn new(base_key: LayerKeyIndex, modifiers: &'a [LayerKeyIndex], weight: f64) -> Self {
        Self {
            base_key,
            modifiers,
            weight,
            iter: None,
        }
    }
}

// // use length 3 for up to 2 modifiers
// // use length 4 for up to 3 modifiers (may cost arount 10%-20% performance)
// // or use smallvec/tinyvec that can overflow to the heap
// pub fn take_one_layerkey(
//     base_key: LayerKeyIndex,
//     modifiers: &[LayerKeyIndex],
//     weight: f64,
// ) -> ArrayVec<[(LayerKeyIndex, f64); 3]> {
//     let mut res = ArrayVec::<[(LayerKeyIndex, f64); 3]>::new();
//     res.push((base_key, weight));

//     modifiers.iter().for_each(|m| {
//         res.push((*m, weight));
//     });

//     res
// }

/// Iterator over bigrams of combinations of the base-layer key with each modifier
/// and the modifiers themselves. The number of resulting bigrams depends on the number of modifiers.
#[derive(Clone, Debug)]
pub struct TakeTwoLayerKey<'a> {
    base_key: LayerKeyIndex,
    modifiers: &'a [LayerKeyIndex],
    weight: f64,
    same_key_mod_factor: f64,
    iter_inner: Option<slice::Iter<'a, LayerKeyIndex>>,
    state_inner: Option<LayerKeyIndex>,
    inner_variant: u8,
    iter_outer: Option<slice::Iter<'a, LayerKeyIndex>>,
    state_outer: Option<LayerKeyIndex>,
}

impl<'a> Iterator for TakeTwoLayerKey<'a> {
    type Item = ((LayerKeyIndex, LayerKeyIndex), f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_outer.is_none() {
            // initialization
            self.iter_outer = Some(self.modifiers.iter());
        }

        if self.state_inner.is_none() {
            // initialization
            // or inner loop ended -> advance outer loop and reinit inner one
            self.state_outer = self.iter_outer.as_mut().unwrap().next().cloned();

            // the inner iterator is a clone of the outer (plus one skip, which happened in the line above)
            self.iter_inner = Some(self.iter_outer.as_ref().cloned().unwrap());
            self.state_inner = self.iter_inner.as_mut().unwrap().next().cloned();

            // on first entry of inner loop, first return base key variant
            if let Some(state_outer) = self.state_outer {
                return Some(((state_outer, self.base_key), self.weight));
            }
        }

        self.state_outer?; // if outer loop ended -> no more elements -> return None

        match self.inner_variant {
            0 => {
                self.inner_variant = 1;

                // first variant is m1-m2
                Some((
                    (self.state_outer.unwrap(), self.state_inner.unwrap()),
                    self.weight * self.same_key_mod_factor,
                ))
            }
            1 => {
                self.inner_variant = 0;

                // second variant is m2-m1
                let res = Some((
                    (self.state_inner.unwrap(), self.state_outer.unwrap()),
                    self.weight * self.same_key_mod_factor,
                ));
                // there is no third variant -> advance inner loop
                self.state_inner = self.iter_inner.as_mut().unwrap().next().cloned();

                res
            }
            _ => {
                unimplemented!();
            }
        }
    }
}

impl<'a> TakeTwoLayerKey<'a> {
    pub fn new(
        base_key: LayerKeyIndex,
        modifiers: &'a [LayerKeyIndex],
        weight: f64,
        same_key_mod_factor: f64,
    ) -> Self {
        Self {
            base_key,
            modifiers,
            weight,
            same_key_mod_factor,
            iter_inner: None,
            state_inner: None,
            inner_variant: 0,
            iter_outer: None,
            state_outer: None,
        }
    }
}

// // use length 4 for up to 2 modifiers
// // use length 9 for up to 3 modifiers
// pub fn take_two_layerkey(
//     base_key: LayerKeyIndex,
//     modifiers: &[LayerKeyIndex],
//     weight: f64,
//     same_key_mod_adjustment: f64,
// ) -> ArrayVec<[((LayerKeyIndex, LayerKeyIndex), f64); 4]> {
//     let mut res = ArrayVec::<[((LayerKeyIndex, LayerKeyIndex), f64); 4]>::new();

//     modifiers.iter().enumerate().for_each(|(i, m1)| {
//         res.push(((*m1, base_key), weight));

//         modifiers.iter().skip(i + 1).for_each(|m2| {
//             if m1 != m2 {
//                 res.push(((*m1, *m2), same_key_mod_adjustment * weight));
//                 res.push(((*m2, *m1), same_key_mod_adjustment * weight));
//             }
//         });
//     });

//     res
// }

/// Iterator over trigrams of combinations of the base-layer key and with two modifiers.
/// If there is no or only one modifier, the result is empty.
#[derive(Clone, Debug)]
pub struct TakeThreeLayerKey<'a> {
    base_key: LayerKeyIndex,
    modifiers: &'a [LayerKeyIndex],
    weight: f64,
    same_key_mod_factor: f64,
    iter_inner: Option<slice::Iter<'a, LayerKeyIndex>>,
    state_inner: Option<LayerKeyIndex>,
    inner_variant: u8,
    iter_middle: Option<slice::Iter<'a, LayerKeyIndex>>,
    state_middle: Option<LayerKeyIndex>,
    middle_variant: u8,
    iter_outer: Option<slice::Iter<'a, LayerKeyIndex>>,
    state_outer: Option<LayerKeyIndex>,
}

impl<'a> Iterator for TakeThreeLayerKey<'a> {
    type Item = ((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_outer.is_none() {
            // initialization
            self.iter_outer = Some(self.modifiers.iter());
        }

        if self.state_middle.is_none() {
            // initialization
            // or middle iterator ended -> advance outer loop and reinit middle one
            self.state_outer = self.iter_outer.as_mut().unwrap().next().cloned();

            self.state_outer?; // if outer iterator ended -> no more elements -> return None

            // the middle iterator is a clone of the outer (plus one skip, which happened in the line above)
            self.iter_middle = Some(self.iter_outer.as_ref().cloned().unwrap());
            self.state_middle = self.iter_middle.as_mut().unwrap().next().cloned();

            self.state_middle?; // if middle loop ended right after being initialized -> nothing left to do  -> return None
        }

        if self.state_inner.is_none() {
            // on first two entries of middle loop, return base key variant
            match self.middle_variant {
                0 => {
                    // first variant is m1-basekey
                    self.middle_variant = 1;
                    return Some((
                        (
                            self.state_outer.unwrap(),
                            self.state_middle.unwrap(),
                            self.base_key,
                        ),
                        self.weight * self.same_key_mod_factor,
                    ));
                }
                1 => {
                    // second variant is m2-basekey
                    self.middle_variant = 0;
                    let res = Some((
                        (
                            self.state_middle.unwrap(),
                            self.state_outer.unwrap(),
                            self.base_key,
                        ),
                        self.weight * self.same_key_mod_factor,
                    ));

                    // prepare inner iterator
                    // the inner iterator is a clone of the middle (plus one skip)
                    self.iter_inner = Some(self.iter_middle.as_ref().cloned().unwrap());
                    self.state_inner = self.iter_inner.as_mut().unwrap().next().cloned();

                    if self.state_inner.is_none() {
                        // inner iterator was empty -> advance middle iterator
                        self.state_middle = self.iter_middle.as_mut().unwrap().next().cloned();
                    }

                    return res;
                }
                _ => (),
            }
        }

        match self.inner_variant {
            0 => {
                self.inner_variant = 1;

                // first variant is m1-m2-m3
                Some((
                    (
                        self.state_outer.unwrap(),
                        self.state_middle.unwrap(),
                        self.state_inner.unwrap(),
                    ),
                    self.weight * self.same_key_mod_factor * self.same_key_mod_factor,
                ))
            }
            1 => {
                self.inner_variant = 2;

                // second variant is m1-m3-m2
                Some((
                    (
                        self.state_outer.unwrap(),
                        self.state_inner.unwrap(),
                        self.state_middle.unwrap(),
                    ),
                    self.weight * self.same_key_mod_factor * self.same_key_mod_factor,
                ))
            }
            2 => {
                self.inner_variant = 3;

                // third variant is m2-m1-m3
                Some((
                    (
                        self.state_middle.unwrap(),
                        self.state_outer.unwrap(),
                        self.state_inner.unwrap(),
                    ),
                    self.weight * self.same_key_mod_factor * self.same_key_mod_factor,
                ))
            }
            3 => {
                self.inner_variant = 4;

                // fourth variant is m2-m3-m1
                Some((
                    (
                        self.state_middle.unwrap(),
                        self.state_inner.unwrap(),
                        self.state_outer.unwrap(),
                    ),
                    self.weight * self.same_key_mod_factor * self.same_key_mod_factor,
                ))
            }
            4 => {
                self.inner_variant = 5;

                // fifth variant is m3-m1-m2
                Some((
                    (
                        self.state_inner.unwrap(),
                        self.state_outer.unwrap(),
                        self.state_middle.unwrap(),
                    ),
                    self.weight * self.same_key_mod_factor * self.same_key_mod_factor,
                ))
            }
            5 => {
                self.inner_variant = 0;

                // sixth variant is m3-m2-m1
                let res = Some((
                    (
                        self.state_inner.unwrap(),
                        self.state_middle.unwrap(),
                        self.state_outer.unwrap(),
                    ),
                    self.weight * self.same_key_mod_factor * self.same_key_mod_factor,
                ));

                // there is no seventh variant -> advance inner loop
                self.state_inner = self.iter_inner.as_mut().unwrap().next().cloned();

                if self.state_inner.is_none() {
                    // inner iterator has finished -> advance middle loop
                    self.state_middle = self.iter_middle.as_mut().unwrap().next().cloned();
                };

                res
            }
            _ => {
                unimplemented!();
            }
        }
    }
}

impl<'a> TakeThreeLayerKey<'a> {
    pub fn new(
        base_key: LayerKeyIndex,
        modifiers: &'a [LayerKeyIndex],
        weight: f64,
        same_key_mod_factor: f64,
    ) -> Self {
        Self {
            base_key,
            modifiers,
            weight,
            same_key_mod_factor,
            iter_inner: None,
            state_inner: None,
            inner_variant: 0,
            iter_middle: None,
            state_middle: None,
            middle_variant: 0,
            iter_outer: None,
            state_outer: None,
        }
    }
}

// // use length 2 for up to 2 modifiers
// // use length 10 for up to 3 modifiers
// pub fn take_three_layerkey(
//     base_key: LayerKeyIndex,
//     modifiers: &[LayerKeyIndex],
//     weight: f64,
//     same_key_mod_adjustment: f64,
// ) -> ArrayVec<[((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64); 18]> {
//     let mut res = ArrayVec::<[((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64); 18]>::new();

//     modifiers.iter().enumerate().for_each(|(i, m1)| {
//         modifiers.iter().skip(i + 1).enumerate().for_each(|(j, m2)| {
//             res.push(((*m1, *m2, base_key), same_key_mod_adjustment * weight));
//             res.push(((*m2, *m1, base_key), same_key_mod_adjustment * weight));

//             // the following is only relevant for keys with 3+ modifiers (which normally does not occur)
//             modifiers.iter().skip(j + i + 2).for_each(|m3| {
//                 res.extend(vec![
//                     (
//                         (*m1, *m2, *m3),
//                         same_key_mod_adjustment * same_key_mod_adjustment * weight,
//                     ),
//                     (
//                         (*m1, *m3, *m2),
//                         same_key_mod_adjustment * same_key_mod_adjustment * weight,
//                     ),
//                     (
//                         (*m2, *m1, *m3),
//                         same_key_mod_adjustment * same_key_mod_adjustment * weight,
//                     ),
//                     (
//                         (*m2, *m3, *m1),
//                         same_key_mod_adjustment * same_key_mod_adjustment * weight,
//                     ),
//                     (
//                         (*m3, *m1, *m2),
//                         same_key_mod_adjustment * same_key_mod_adjustment * weight,
//                     ),
//                     (
//                         (*m3, *m2, *m1),
//                         same_key_mod_adjustment * same_key_mod_adjustment * weight,
//                     ),
//                 ]);
//             });
//         });
//     });

//     res
// }

pub trait NgramMap<Ngram: Eq + Hash> {
    /// Adds the ngram to the HashMap if it does not already exist.
    /// If it does exist, simply add its weight to the preexisting weight.
    fn insert_or_add_weight(&mut self, k: Ngram, w: f64);
}

impl<Ngram: Eq + Hash> NgramMap<Ngram> for AHashMap<Ngram, f64> {
    #[inline(always)]
    fn insert_or_add_weight(&mut self, k: Ngram, w: f64) {
        *self.entry(k).or_insert(0.0) += w;
    }
}
