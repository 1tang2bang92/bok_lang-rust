#![allow(non_snake_case)]
#![allow(unused)]

mod parser;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let t = tokenizer.tokenize("print(game)");
    let mut parser = Parser::new();
    let p = parser.parse(t.clone());
    println!("{:?}", t);
    println!("{:?}", p);
}
