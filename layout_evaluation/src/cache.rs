use colored::Colorize;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use std::{fmt, sync::Arc};

#[derive(Clone, Debug)]
pub struct Cache<T: Clone> {
    cache: Arc<Mutex<FxHashMap<String, T>>>,
}

impl<T: Clone> Default for Cache<T> {
    fn default() -> Self {
        Cache::new()
    }
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
            let cache = self.cache.lock();
            cache_val = cache.get(elem).cloned();
        }
        cache_val.unwrap_or_else(|| {
            let res = f();
            {
                let mut cache = self.cache.lock();
                cache.insert(elem.to_owned(), res.clone());
            }
            res
        })
    }
}

impl<T: Clone + fmt::Display + PartialOrd> Cache<T> {
    pub fn highlighted_fmt(&self, current_layout_str: Option<&str>, max_entries: usize) -> String {
        let mut results: Vec<(String, T)>;
        {
            let cache = self.cache.lock();
            results = cache.iter().map(|(s, c)| (s.clone(), c.clone())).collect();
        }

        if results.is_empty() {
            String::new()
        } else {
            results.sort_by(|(_, c1), (_, c2)| c1.partial_cmp(c2).unwrap());
            let enumeration_length = max_entries.to_string().chars().count();
            let mut output_string =
                "Optimized layouts found during this run, ordered from best (lowest cost) to worst (highest cost):".to_string();

            for (i, (l, cost)) in results.into_iter().enumerate() {
                if i >= max_entries {
                    output_string.push_str(&format!(
                        "\n⋮⋮⋮\nOnly the best {} layouts are displayed.",
                        max_entries,
                    ));
                    break;
                }
                let result_line = format!("{} ({:.1})", l, cost);
                if current_layout_str.is_some() && current_layout_str.unwrap() == l {
                    output_string.push_str(&format!(
                        "\n{:>width$}. {} (current)",
                        i + 1,
                        result_line.bold(),
                        width = enumeration_length,
                    ));
                } else {
                    output_string.push_str(&format!(
                        "\n{:>width$}. {}",
                        i + 1,
                        result_line,
                        width = enumeration_length,
                    ));
                }
            }
            output_string
        }
    }
}

impl<T: Clone + fmt::Display + PartialOrd> fmt::Display for Cache<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.highlighted_fmt(None, 30))
    }
}
