use std::collections::HashMap;

use crate::ast::AST;
use crate::Operator;
use crate::Type;
use crate::Value;

use inkwell::builder::*;
use inkwell::context::*;
use inkwell::values::*;
use inkwell::types::*;
use inkwell::basic_block::*;

pub struct Generator<'a> {
    context: &'a Context,
    builder: Builder<'a>,
    named_values: HashMap<String, Box<dyn AnyValue<'a> + 'a>>,
    tmp_values: Vec<Box<dyn AnyValue<'a> + 'a>>,
    tmp_blocks: Vec<BasicBlock<'a>>,
    tmp_context_refs: Vec<ContextRef<'a>>, 
    tmp_builders: Vec<Builder<'a>>,
}

impl<'a> Generator<'a> {
    pub fn new(context: &'a Context, builder: Builder) -> Self {
        Self {
            context: &context,
            builder: context.create_builder(),
            named_values: HashMap::new(),
            tmp_values: Vec::new(),
            tmp_blocks: Vec::new(),
            tmp_context_refs: Vec::new(),
            tmp_builders: Vec::new(),
        }
    }    

    fn gen_val_code(&mut self, data: i64) -> &Box<dyn AnyValue<'a> + 'a> {
        let val: IntValue<'a> = self.context.i64_type().const_int(data as u64, true);
        self.tmp_values.push(Box::new(val));
        self.tmp_values.last().unwrap()
    }

    fn gen_identifier_code(&mut self, s: String) -> &Box<dyn AnyValue<'a> + 'a> {
        let v = self.named_values.get(&s);
        v.unwrap()
    }

    fn gen_binary_code(&mut self, op: Operator, l: AST, r: AST) -> &Box<dyn AnyValue<'a> + 'a> {
        let lhs: IntValue<'a> = self.gen_code(l).as_any_value_enum().into_int_value();
        let rhs: IntValue<'a> = self.gen_code(r).as_any_value_enum().into_int_value();

        let val: IntValue<'a> = match op {
            Operator::Add => self.builder.build_int_add(lhs, rhs, "addtmp"),
            Operator::Sub => self.builder.build_int_sub(lhs, rhs, "subtmp"),
            Operator::Mul => self.builder.build_int_mul(lhs, rhs, "multmp"),
            Operator::Div => self.builder.build_int_signed_div(lhs, rhs, "divtmp"),
            _ => panic!(""),
        };
        self.tmp_values.push(Box::new(val));
        self.tmp_values.last().unwrap()
    }

    pub fn gen_code(&mut self, ast: AST) -> &Box<dyn AnyValue<'a> + 'a> {
        match ast {
            AST::Binary(op, l, r) => self.gen_binary_code(op, *l, *r),
            AST::Identifier(x) => self.gen_identifier_code(x),
            //AST::Variable(name, _, value) => self.gen_var_code(name, *value),
            AST::Value(Type::Int, Value::Int(x)) => self.gen_val_code(x),
            AST::Statement(x) => {
                let mut a = unsafe {(self as *mut Self).as_mut().unwrap()}.gen_val_code(0);
                for i in x {
                    a = unsafe {(self as *mut Self).as_mut().unwrap()}.gen_code(i);
                }
                a
            }
            _ => panic!(""),
        }
    }

    pub fn get_ir(&mut self) {
    }
}
