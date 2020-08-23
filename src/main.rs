#![allow(non_snake_case)]
#![allow(unused)]

mod parser;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let v = tokenizer.tokenize("if (a + b == 10) { print(\"123\") }");
    println!("{:?}", v);
    let mut tokenizer = Tokenizer::new();
    let v = tokenizer.tokenize("fn print(a: String) { c.printf(\"%s\", a); }");
    println!("{:?}", v);
}
