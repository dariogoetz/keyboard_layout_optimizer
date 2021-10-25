use keyboard_layout::layout::LayerKeyIndex;

pub fn take_one_layerkey(
    base_key: LayerKeyIndex,
    modifiers: &[LayerKeyIndex],
    weight: f64,
) -> Vec<(LayerKeyIndex, f64)> {
    let mut res: Vec<(LayerKeyIndex, f64)> = vec![(base_key, weight)];

    modifiers.iter().for_each(|m| {
        res.push((*m, weight));
    });

    res
}

pub fn take_two_layerkey(
    base_key: LayerKeyIndex,
    modifiers: &[LayerKeyIndex],
    weight: f64,
    same_key_mod_adjustment: f64,
) -> Vec<((LayerKeyIndex, LayerKeyIndex), f64)> {
    let mut res: Vec<((LayerKeyIndex, LayerKeyIndex), f64)> = Vec::new();

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

pub fn take_three_layerkey(
    base_key: LayerKeyIndex,
    modifiers: &[LayerKeyIndex],
    weight: f64,
    same_key_mod_adjustment: f64,
) -> Vec<((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64)> {
    let mut res: Vec<((LayerKeyIndex, LayerKeyIndex, LayerKeyIndex), f64)> = Vec::new();

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
