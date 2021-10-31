/// The `common` module provides utility functions for resolving modifiers in ngrams.

use keyboard_layout::layout::LayerKeyIndex;

use tinyvec::ArrayVec;

/// Generate an `ArrayVec` unigrams of the base-layer key and each modifier.
///
/// For performance reasons, an `ArrayVec` of fixed size is chosen with a capacity only allowing two modifiers per layer.
/// If more modifiers shall be supported, increase the capacity (for a smaller performance penalty) or use
/// an overflowing implementation such as `SmallVec` or `TinyVec`.

// use length 3 for up to 2 modifiers
// use length 4 for up to 3 modifiers (may cost arount 10%-20% performance)
// or use smallvec/tinyvec that can overflow to the heap
pub fn take_one_layerkey(
    base_key: LayerKeyIndex,
    modifiers: &[LayerKeyIndex],
    weight: f64,
) -> ArrayVec<[(LayerKeyIndex, f64); 3]> {
    let mut res = ArrayVec::<[(LayerKeyIndex, f64); 3]>::new();
    res.push((base_key, weight));

    modifiers.iter().for_each(|m| {
        res.push((*m, weight));
    });

    res
}

/// Generate an `ArrayVec` of bigrams of combinations of the base-layer key with each modifier
/// and the modifiers themselves. The number of resulting bigrams depends on the number of modifiers.
///
/// For performance reasons, an `ArrayVec` of fixed size is chosen with a capacity only allowing two modifiers per layer.
/// If more modifiers shall be supported, increase the capacity (for a smaller performance penalty) or use
/// an overflowing implementation such as `SmallVec` or `TinyVec`.

// use length 4 for up to 2 modifiers
// use length 9 for up to 3 modifiers
pub fn take_two_layerkey(
    base_key: LayerKeyIndex,
    modifiers: &[LayerKeyIndex],
    weight: f64,
    same_key_mod_adjustment: f64,
) -> ArrayVec<[((LayerKeyIndex, LayerKeyIndex), f64); 4]> {
    let mut res = ArrayVec::<[((LayerKeyIndex, LayerKeyIndex), f64); 4]>::new();

    modifiers.iter().enumerate().for_each(|(i, m1)| {
        res.push(((*m1, base_key), weight));

        modifiers.iter().skip(i + 1).for_each(|m2| {
            if m1 != m2 {
                res.push(((*m1, *m2), same_key_mod_adjustment * weight));
                res.push(((*m2, *m1), same_key_mod_adjustment * weight));
            }
        });
    });

    res
}

/// Generate an `ArrayVec` of trigrams of combinations of the base-layer key and with two modifiers.
/// If there is no or only one modifier, the result is empty. If there are two modifiers, the result
/// has length two. If there were three modifiers (disabled at the moment, would crash due to ArrayVec capacity),
/// the length would be 18.
///
/// For performance reasons, an `ArrayVec` of fixed size is chosen with a capacity only allowing two modifiers per layer.
/// If more modifiers shall be supported, increase the capacity (for a smaller performance penalty) or use
/// an overflowing implementation such as `SmallVec` or `TinyVec`.

// use length 2 for up to 2 modifiers
// use length 18 for up to 3 modifiers
pub fn take_three_layerkey(
    base_key: LayerKeyIndex,
    modifiers: &[LayerKeyIndex],
    weight: f64,
    same_key_mod_adjustment: f64,
) -> ArrayVec<[((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64); 2]> {
    let mut res = ArrayVec::<[((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64); 2]>::new();

    modifiers.iter().enumerate().for_each(|(i, m1)| {
        modifiers.iter().skip(i + 1).for_each(|m2| {
            res.push(((*m1, *m2, base_key), same_key_mod_adjustment * weight));
            res.push(((*m2, *m1, base_key), same_key_mod_adjustment * weight));

            // the following is only relevant for keys with 3+ modifiers (which normally does not occur)
            modifiers.iter().skip(i + 2).for_each(|m3| {
                res.extend(vec![
                    (
                        (*m1, *m2, *m3),
                        same_key_mod_adjustment * same_key_mod_adjustment * weight,
                    ),
                    (
                        (*m1, *m3, *m2),
                        same_key_mod_adjustment * same_key_mod_adjustment * weight,
                    ),
                    (
                        (*m2, *m1, *m3),
                        same_key_mod_adjustment * same_key_mod_adjustment * weight,
                    ),
                    (
                        (*m2, *m3, *m1),
                        same_key_mod_adjustment * same_key_mod_adjustment * weight,
                    ),
                    (
                        (*m3, *m1, *m2),
                        same_key_mod_adjustment * same_key_mod_adjustment * weight,
                    ),
                    (
                        (*m3, *m2, *m1),
                        same_key_mod_adjustment * same_key_mod_adjustment * weight,
                    ),
                ]);
            });
        });
    });

    res
}
