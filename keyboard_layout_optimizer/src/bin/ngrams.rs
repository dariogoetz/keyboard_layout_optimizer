use clap::Parser;
use std::{fs, path::Path};

use layout_evaluation::ngrams::{Bigrams, Trigrams, Unigrams};

#[derive(Parser, Debug)]
#[clap(name = "Ngram frequency generator")]
/// Generate ngram-frequency files from a given text file.
struct Options {
    /// Read text from this file
    filename: String,

    /// Name for resulting ngram frequencies (a directory at that path will be generated)
    out: String,
}

fn main() {
    dotenv::dotenv().ok();
    let options = Options::parse();
    env_logger::init();

    let text = fs::read_to_string(&options.filename)
        .unwrap_or_else(|_| panic!("Could not read corpus file from {}.", options.filename));

    let d = Path::new(&options.out);

    let unigrams = Unigrams::from_text(&text).expect("Could not generate unigrams from text.");
    let p = d.join("1-grams.txt");
    unigrams.save_frequencies(p).unwrap();

    let bigrams = Bigrams::from_text(&text).expect("Could not generate bigrams from text.");
    let p = d.join("2-grams.txt");
    bigrams.save_frequencies(p).unwrap();

    let trigrams = Trigrams::from_text(&text).expect("Could not generate trigrams from text.");
    let p = d.join("3-grams.txt");
    trigrams.save_frequencies(p).unwrap();
}
