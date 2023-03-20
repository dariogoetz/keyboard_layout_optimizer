use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone, Debug)]
pub struct LayoutPermutator {
    perm_keys: Vec<char>,
    perm_indices: Vec<usize>,
    fixed_keys: Vec<char>,
    fixed_indices: Vec<usize>,
}

impl LayoutPermutator {
    pub fn new(layout: &str, fixed: &str) -> Self {
        let mut perm_keys = Vec::new();
        let mut perm_indices = Vec::new();
        let mut fixed_keys = Vec::new();
        let mut fixed_indices = Vec::new();

        for (i, c) in layout.chars().enumerate() {
            if fixed.contains(c) {
                fixed_keys.push(c);
                fixed_indices.push(i);
            } else {
                perm_keys.push(c);
                perm_indices.push(i);
            }
        }
        Self {
            perm_keys,
            perm_indices,
            fixed_keys,
            fixed_indices,
        }
    }

    pub fn generate_string(&self, permutation: &[usize]) -> String {
        let mut res: Vec<char> = vec!['-'; self.fixed_keys.len() + self.perm_keys.len()];

        self.fixed_indices
            .iter()
            .zip(self.fixed_keys.iter())
            .for_each(|(i, c)| res[*i] = *c);

        permutation
            .iter()
            .zip(self.perm_keys.iter())
            .for_each(|(i, c)| res[*i] = *c);

        res.iter().collect()
    }

    pub fn generate_random(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = self.perm_indices.to_vec();
        indices.shuffle(&mut thread_rng());

        indices
    }

    /// Takes in a Layout, switches [nr_switches] keys in that layout, then returns it.
    /// Layout, in this case, is a [Vec<usize>].
    pub fn perform_n_swaps(&self, permutation: &[usize], nr_switches: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = permutation.to_vec();
        let vec: Vec<usize> = (0..permutation.len()).collect();
        let rng = &mut thread_rng();

        // Perform nr_switches switches
        for _ in 0..nr_switches {
            let mut sw = vec.choose_multiple(rng, 2);
            let sw0 = sw.next().unwrap();
            let sw1 = sw.next().unwrap();
            indices.swap(*sw0, *sw1);
        }

        indices
    }

    pub fn switch_n_keys(&self, permutation: &[usize], n_keys: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = permutation.to_vec();
        let rng = &mut thread_rng();

        let vec: Vec<usize> = (0..permutation.len()).collect();
        let sw_from: Vec<&usize> = vec.choose_multiple(rng, n_keys).collect();
        let mut sw_to = sw_from.to_vec();
        sw_to.shuffle(rng);

        // Perform nr_switches switches
        for (from, to) in sw_from.into_iter().zip(sw_to.into_iter()) {
            indices[*to] = permutation[*from];
        }

        indices
    }

    pub fn get_permutable_indices(&self) -> Vec<usize> {
        self.perm_indices.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
