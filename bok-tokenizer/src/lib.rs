pub mod token;
pub mod tokenizer;

pub use token::*;
pub use tokenizer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
