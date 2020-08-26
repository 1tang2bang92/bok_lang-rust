#![allow(non_snake_case)]
#![allow(unused)]

pub mod tokenizer;
pub mod parser;
pub mod utils;

pub use tokenizer::*;
pub use parser::*;
pub use utils::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let t = tokenizer.tokenize("if a == 10 { let b = 20 } else if a == 20 { let b = 30 } else { let b = 40 }");
    println!("{:?}", t.clone());
    let mut parser = Parser::new();
    let p = parser.parse(t);
    println!("{:?}", p);
}
