#![allow(non_snake_case)]
#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;

use inkwell::builder::*;
use inkwell::context::*;

pub mod generator;
pub mod parser;
pub mod tokenizer;
pub mod utils;

pub use generator::*;
pub use parser::*;
pub use tokenizer::*;
pub use utils::*;

fn main() {
    let context = Context::create();

    let mut generator = Generator::new(&context, context.create_builder());

    let mut f = File::open("test.bs").expect("File Open Error");

    let mut fs = String::new();
    f.read_to_string(&mut fs).expect("File Read Error");

    let mut tokenizer = Tokenizer::new();
    //let t = tokenizer.tokenize("if a == 10 { let b = 20 } else if a == 20 { let b = 30 } else { let b = 40 }");
    let t = tokenizer.tokenize(fs.as_ref());
    println!("{:?}", t.clone());
    let mut parser = Parser::new();
    let p = parser.parse(t);
    println!("{:?}", p.clone());

    println!("{:?}",generator.gen_code(p).as_any_value_enum().into_int_value().get_sign_extended_constant());
}
