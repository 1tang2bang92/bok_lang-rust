pub mod parser;
pub mod ast;

pub use parser::*;
pub use ast::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
