#![allow(non_snake_case)]
#![allow(unused)]

mod parser;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let v = tokenizer.tokenize("if 모르겟다 > 100 { 시발 = \"true\" loop if 시발 >=20 { 멘붕() }");
    println!("{:?}", v);
}
