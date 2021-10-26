pub mod evaluation;
pub mod results;
pub mod metrics;
pub mod ngrams;
pub mod ngram_mapper;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
