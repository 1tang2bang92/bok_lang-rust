use std::collections::HashMap;

use crate::ast::AST;
use crate::Operator;

use inkwell::builder::*;
use inkwell::context::*;
use inkwell::values::*;

pub struct Generator<'a> {
    context: &'a Context,
    builder: Builder<'a>,
    named_values: HashMap<String, Box<dyn AnyValue<'a>>>,
}

impl<'a> Generator<'a> {
    pub fn new(context: &'a Context, builder: Builder) -> Self {
        let named_values = HashMap::new();
        Self {
            context: &context,
            builder: context.create_builder(),
            named_values,
        }
    }

    fn gen_identifier_code(&mut self, s: String) -> &'a Box<dyn AnyValue> {
        let v = self.named_values.get(&s);
        return v.unwrap();
    }

    fn gen_binary_code(&mut self, AST::Binary(op, l, r): AST) {
        let lhs = self.gen_code(*l);
        let rhs = self.gen_code(*r);

        match op {
            Operator::Add => {}
            Operator::Sub => {}
            Operator::Mul => {}
            Operator::Div => {}
            _ => {}
        }
    }

    pub fn gen_code(&mut self, ast: AST) {}
}
