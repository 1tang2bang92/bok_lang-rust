#![allow(non_snake_case)]
#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;

use bok_tokenizer::*;
use bok_parser::*;

fn main() {
    let mut f = File::open("test.bs").expect("File Open Error");

    let mut fs = String::new();
    f.read_to_string(&mut fs).expect("File Read Error");

    let mut tokenizer = Tokenizer::new();
    //let t = tokenizer.tokenize("if a == 10 { let b = 20 } else if a == 20 { let b = 30 } else { let b = 40 }");
    let t = tokenizer.tokenize(fs.as_ref());
    println!("{:?}", t.clone());
    let mut parser = Parser::new();
    let p = parser.parse(t);
    println!("{:?}", p);
}
