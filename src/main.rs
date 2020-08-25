#![allow(non_snake_case)]
#![allow(unused)]

mod parser;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let v = tokenizer.tokenize("if money > 10000 { happy = 10 } else { happy = -10 }");
    println!("{:?}", v);
}
