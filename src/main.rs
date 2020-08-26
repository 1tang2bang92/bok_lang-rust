#![allow(non_snake_case)]
#![allow(unused)]

mod parser;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let t = tokenizer.tokenize("{1 + 2 2 + 3}");
    println!("{:?}", t.clone());
    let mut parser = Parser::new();
    let p = parser.parse(t);
    println!("{:?}", p);
}
