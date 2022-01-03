use rustc_hash::FxHashMap;
use std::path::Path;
use std::str::FromStr;
use structopt::StructOpt;

use layout_evaluation::ngrams::{Bigrams, Trigrams, Unigrams};

#[derive(Debug)]
struct WeightedComponent(f64, String);

impl FromStr for WeightedComponent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.splitn(2, ":").collect();

        let path = components[0].to_string();
        let weight = f64::from_str(components[1]).unwrap();

        Ok(WeightedComponent(weight, path))
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Ngram frequency merge")]
struct Options {
    /// Pairs of weight and ngram frequency directory in the form path:weight
    components: Vec<WeightedComponent>,

    /// Directory name for resulting ngram frequency files
    out: String,
}

fn add<T: Clone + Eq + std::hash::Hash>(
    weight: f64,
    res: &mut FxHashMap<T, f64>,
    ngrams: &FxHashMap<T, f64>,
) {
    ngrams.iter().fold(res, |acc, (c, w)| {
        let entry = acc.entry(c.clone()).or_default();
        *entry += weight * w;

        acc
    });
}

fn main() {
    dotenv::dotenv().ok();
    let options = Options::from_args();
    env_logger::init();

    let mut res_unigrams = FxHashMap::default();
    let mut res_bigrams = FxHashMap::default();
    let mut res_trigrams = FxHashMap::default();

    let mut target_unigrams_total: Option<f64> = None;
    let mut target_bigrams_total: Option<f64> = None;
    let mut target_trigrams_total: Option<f64> = None;

    for component in options.components {
        log::info!("Processing {}...", component.1);

        let p = Path::new(&component.1).join("1-grams.txt");
        let unigrams = Unigrams::from_file(&p.to_str().unwrap())
            .expect(&format!("Could not read 1-gramme file from '{:?}'.", &p));

        let unigrams_total = unigrams.total_weight();

        // first ngram file determines "absolute level"
        target_unigrams_total = target_unigrams_total.or(Some(unigrams_total));
        add(
            component.0 * target_unigrams_total.unwrap() / unigrams_total,
            &mut res_unigrams,
            &unigrams.grams,
        );

        let p = Path::new(&component.1).join("2-grams.txt");
        let bigrams = Bigrams::from_file(&p.to_str().unwrap())
            .expect(&format!("Could not read 2-gramme file from '{:?}'.", &p));

        let bigrams_total = bigrams.total_weight();

        // first ngram file determines "absolute level"
        target_bigrams_total = target_bigrams_total.or(Some(bigrams_total));
        add(
            component.0 * target_bigrams_total.unwrap() / bigrams_total,
            &mut res_bigrams,
            &bigrams.grams,
        );

        let p = Path::new(&component.1).join("3-grams.txt");
        let trigrams = Trigrams::from_file(&p.to_str().unwrap())
            .expect(&format!("Could not read 3-gramme file from '{:?}'.", &p));

        let trigrams_total = trigrams.total_weight();

        // first ngram file determines "absolute level"
        target_trigrams_total = target_trigrams_total.or(Some(trigrams_total));
        add(
            component.0 * target_trigrams_total.unwrap() / trigrams_total,
            &mut res_trigrams,
            &trigrams.grams,
        );
    }

    log::info!("Writing result to {}...", options.out);
    let out = Path::new(&options.out);
    Unigrams {
        grams: res_unigrams,
    }
    .save_frequencies(out.join("1-grams.txt"))
    .unwrap();
    Bigrams { grams: res_bigrams }
        .save_frequencies(out.join("2-grams.txt"))
        .unwrap();
    Trigrams {
        grams: res_trigrams,
    }
    .save_frequencies(out.join("3-grams.txt"))
    .unwrap();
}
