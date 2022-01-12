use std::path::Path;
use structopt::StructOpt;

use layout_evaluation::ngrams::{Bigrams, Trigrams, Unigrams};

#[derive(StructOpt, Debug)]
#[structopt(name = "Ngram frequency generator")]
struct Options {
    /// Read text from this file
    filename: String,

    /// Name for resulting ngram frequencies (a directory at that path will be generated)
    out: String,
}

fn main() {
    dotenv::dotenv().ok();
    let options = Options::from_args();
    env_logger::init();

    let text = std::fs::read_to_string(&options.filename).expect(&format!(
        "Could not read corpus file from {}.",
        options.filename
    ));

    let d = Path::new(&options.out);

    let unigrams = Unigrams::from_str(&text).expect("Could not generate unigrams from text.");
    let p = d.join("1-grams.txt");
    unigrams.save_frequencies(&p).unwrap();

    let bigrams = Bigrams::from_str(&text).expect("Could not generate bigrams from text.");
    let p = d.join("2-grams.txt");
    bigrams.save_frequencies(&p).unwrap();

    let trigrams = Trigrams::from_str(&text).expect("Could not generate trigrams from text.");
    let p = d.join("3-grams.txt");
    trigrams.save_frequencies(&p).unwrap();
}
