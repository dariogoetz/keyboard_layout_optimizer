use colored::Colorize;
use rustc_hash::FxHashMap;
use std::fmt::Display;
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

impl<T: Clone + Display + PartialOrd> Cache<T> {
    pub fn highlighted_fmt(&self, current_layout_str: Option<&str>) -> String {
        let mut results: Vec<(String, T)>;
        {
            let cache = self.cache.lock().unwrap();
            results = cache.iter().map(|(s, c)| (s.clone(), c.clone())).collect();
        }
        results.sort_by(|(_, c1), (_, c2)| c1.partial_cmp(c2).unwrap());

        let mut output_string =
            "Layouts ordered from best (lowest cost) to worst (highest cost) ↓".to_string();
        for (i, (l, cost)) in results.into_iter().enumerate() {
            let result_line = format!("{} ({:.1})", l, cost);
            if current_layout_str.is_some() && current_layout_str.unwrap() == l {
                output_string.push_str(&format!(
                    "\n{:>4} {} (current)",
                    format!("{}.", i + 1),
                    result_line.bold()
                ));
            } else {
                output_string.push_str(&format!("\n{:>4} {}", format!("{}.", i + 1), result_line));
            }
        }

        output_string
    }
}

impl<T: Clone + Display + PartialOrd> std::fmt::Display for Cache<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.highlighted_fmt(None))
    }
}
