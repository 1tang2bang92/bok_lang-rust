#![allow(non_snake_case)]
#![allow(unused)]

use std::fs::File;
use std::path::Path;
use std::{ffi::CString, io::prelude::*};

use inkwell::builder::*;
use inkwell::context::*;
use inkwell::passes::*;
use inkwell::targets::*;
use inkwell::*;

pub mod generator;
pub mod parser;
pub mod tokenizer;
pub mod utils;

pub use generator::*;
pub use parser::*;
pub use tokenizer::*;
pub use utils::*;

fn main() {
    //complie option
    let mut display_lexer_output = false;
    let mut display_parser_output = false;
    let mut display_compiler_output = false;

    //match complite option on terminal
    for arg in std::env::args() {
        match arg.as_str() {
            "--dl" => display_lexer_output = true,
            "--dp" => display_parser_output = true,
            "--dc" => display_compiler_output = true,
            _ => (),
        }
    }

    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("Entry");

    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let mut generator = Generator::new(&context, builder, module, fpm);

    let mut f = File::open("test.bs").expect("File Open Error");

    let mut fs = String::new();
    f.read_to_string(&mut fs).expect("File Read Error");

    //lex
    let mut tokenizer = Tokenizer::new();
    let t = tokenizer.tokenize(fs.as_ref());
    if display_lexer_output {
        println!("{:?}", t.clone());
    }
    //let t = tokenizer.tokenize("if a == 10 { let b = 20 } else if a == 20 { let b = 30 } else { let b = 40 }");

    //parsing
    let mut parser = Parser::new();
    let p = parser.parse(t);

    if display_parser_output {
        println!("{:?}", p.clone());
    }

    //generate ir code
    generator.gen_code(p);
    //println!("{:?}",generator.gen_code(p).as_any_value_enum().into_pointer_value().print_to_string());

    if display_compiler_output {
        let mut module = generator.get_module();
        module.print_to_file("output.ir");
        module.print_to_stderr();
    }

    let triple = TargetMachine::get_default_triple();
    generator.get_module().set_triple(&triple);
    //let triple = TargetTriple::create("x86_64-unknown-linux-gnu");
    //println!("{:?}", &triple);

    let config = InitializationConfig {
        asm_parser: true,
        asm_printer: true,
        base: true,
        disassembler: true,
        info: true,
        machine_code: true,
    };
    Target::initialize_native(&config);

    let target = Target::from_triple(&triple).unwrap();
    let cpu = "generic";
    let features = "";
    let level = OptimizationLevel::Default;
    let reloc_mode = RelocMode::Default;
    let code_model = CodeModel::Default;
    let targetmachine = target
        .create_target_machine(&triple, cpu, features, level, reloc_mode, code_model)
        .unwrap();

    let engin = generator.get_module().create_jit_execution_engine(OptimizationLevel::Default).unwrap();
    let target_data = engin.get_target_data();
    generator.get_module().set_data_layout(&target_data.get_data_layout());
    targetmachine.add_analysis_passes(generator.get_passmanager());
    targetmachine.write_to_file(generator.get_module(), FileType::Object, Path::new("output.o"));
}
