#![allow(non_snake_case)]
#![allow(unused)]

mod parser;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize("1");
    println!("Hello, world!");
}
