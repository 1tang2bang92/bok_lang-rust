#![allow(non_snake_case)]
#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;

use inkwell::*;
use inkwell::builder::*;
use inkwell::context::*;
use inkwell::targets::*;

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

    let mut generator = Generator::new(&context, context.create_builder(), context.create_module("Entry"));

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
    generator.gen_code(p);
    //println!("{:?}",generator.gen_code(p).as_any_value_enum().into_pointer_value().print_to_string());

    let mut module = generator.get_module();
    /*let triple = TargetMachine::get_default_triple();
    let triple = TargetTriple::create("x86_64-pc-win32");
    module.set_triple(&triple);

    let target = Target::from_triple(&triple).unwrap();
    
    let cpu = "generic";
    let features = "";
    let level = OptimizationLevel::Default;
    let reloc_mode = RelocMode::Default;
    let code_model = CodeModel::Default;
    let targetmachine = target.create_target_machine(&triple, cpu, features, level, reloc_mode, code_model).unwrap();*/
    println!("{:?}", module.print_to_string());
}
