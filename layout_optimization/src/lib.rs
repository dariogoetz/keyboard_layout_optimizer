pub mod common;
pub mod optimization;
pub mod optimization_abc;
pub mod optimization_sa;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
