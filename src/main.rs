#![allow(non_snake_case)]
#![allow(unused)]

mod parser;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let v = tokenizer.tokenize("모르겟다{123}");
    println!("{:?}", v);
}
