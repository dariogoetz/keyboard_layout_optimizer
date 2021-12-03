use keyboard_layout::layout::Layout;
use keyboard_layout::layout_generator::NeoLayoutGenerator;
use rand::{seq::SliceRandom, thread_rng};
use std::sync::{Arc, Mutex};
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct PermutationLayoutGenerator {
    perm_keys: Vec<char>,
    perm_indices: Vec<usize>,
    fixed_keys: Vec<char>,
    fixed_indices: Vec<usize>,
    pub layout_generator: NeoLayoutGenerator,
}

impl PermutationLayoutGenerator {
    pub fn new(layout: &str, fixed: &str, layout_generator: &NeoLayoutGenerator) -> Self {
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
            layout_generator: layout_generator.clone(),
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
    pub fn switch_n_keys(&self, permutation: &[usize], nr_switches: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = permutation.to_vec();

        // Shuffle some (self.n_switches) permutable chars
        indices.partial_shuffle(&mut thread_rng(), nr_switches);
        indices
    }

    pub fn generate_layout(&self, permutation: &[usize]) -> Layout {
        let s = self.generate_string(permutation);
        self.layout_generator.generate(&s).unwrap()
    }

    pub fn get_permutable_indices(&self) -> Vec<usize> {
        self.perm_indices.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Cache<T: Clone> {
    cache: Arc<Mutex<FxHashMap<String, T>>>
}

impl<T: Clone> Cache<T> {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(FxHashMap::default())),
        }
    }

    pub fn get_or_insert_with<F: Fn() -> T>(&self, elem: &str, f: F) -> T {
        let cache_val;
        {
            let cache = self.cache.lock().unwrap();
            cache_val = cache.get(elem).cloned();
        }
        cache_val.unwrap_or_else(|| {
            let res = f();
            {
                let mut cache = self.cache.lock().unwrap();
                cache.insert(elem.to_owned(), res.clone());
            }
            res
        })
    }
}
