use rustc_hash::FxHashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Cache<T: Clone> {
    cache: Arc<Mutex<FxHashMap<String, T>>>,
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
