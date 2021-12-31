use structopt::StructOpt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use layout_evaluation::ngrams::{Bigrams, Trigrams, Unigrams};

#[derive(StructOpt, Debug)]
#[structopt(name = "Ngram frequency generator")]
struct Options {
    /// Read text from this file
    filename: String,

    /// Directory name for resulting ngram frequency files
    out: String,
}

fn process_special_characters(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\n", "\\n")}

fn main() {
    dotenv::dotenv().ok();
    let options = Options::from_args();
    env_logger::init();

    let text = std::fs::read_to_string(&options.filename)
        .expect(&format!("Could not read corpus file from {}.", options.filename));

    let d = Path::new(&options.out);
    create_dir_all(&d).expect(&format!("Unable to create directory '{}'", d.to_str().unwrap()));

    let unigrams = Unigrams::from_str(&text).expect("Could not generate unigrams from text.");
    let mut unigrams: Vec<(char, f64)> = unigrams.grams.into_iter().collect();
    unigrams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

    let p = d.join("1-grams.txt");
    let mut file = File::create(&p).expect(&format!("Unable to create file '{}'", p.to_str().unwrap()));
    unigrams.iter().for_each(|(c, w)| {
        let processed = process_special_characters(&c.to_string());
        writeln!(&mut file, "{} {}", w, processed).unwrap();
    });


    let bigrams = Bigrams::from_str(&text).expect("Could not generate bigrams from text.");
    let mut bigrams: Vec<((char, char), f64)> = bigrams.grams.into_iter().collect();
    bigrams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

    let p = d.join("2-grams.txt");
    let mut file = File::create(&p).expect(&format!("Unable to create file '{}'", p.to_str().unwrap()));
    bigrams.iter().for_each(|((c1, c2), w)| {
        let processed1 = process_special_characters(&c1.to_string());
        let processed2 = process_special_characters(&c2.to_string());
        writeln!(&mut file, "{} {}{}", w, processed1, processed2).unwrap();
    });


    let trigrams = Trigrams::from_str(&text).expect("Could not generate trigrams from text.");
    let mut trigrams: Vec<((char, char, char), f64)> = trigrams.grams.into_iter().collect();
    trigrams.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

    let p = d.join("3-grams.txt");
    let mut file = File::create(&p).expect(&format!("Unable to create file '{}'", p.to_str().unwrap()));
    trigrams.iter().for_each(|((c1, c2, c3), w)| {
        let processed1 = process_special_characters(&c1.to_string());
        let processed2 = process_special_characters(&c2.to_string());
        let processed3 = process_special_characters(&c3.to_string());
        writeln!(&mut file, "{} {}{}{}", w, processed1, processed2, processed3).unwrap();
    });




}
