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
use inkwell::module::*;
use inkwell::AddressSpace;

pub struct Generator<'a> {
    context: &'a Context,
    builder: Builder<'a>,
    module: Module<'a>,
    named_values: HashMap<String, Box<dyn AnyValue<'a> + 'a>>,
    tmp_values: Vec<Box<dyn AnyValue<'a> + 'a>>,
}

impl<'a> Generator<'a> {
    pub fn new(context: &'a Context, builder: Builder<'a>, module: Module<'a>) -> Self {
        Self {
            context: &context,
            builder: builder,
            module: module,
            named_values: HashMap::new(),
            tmp_values: Vec::new(),
        }
    }

    fn create_entry_block_alloca(&mut self, function: &FunctionValue, var_name: &String) -> PointerValue<'a> {
        let mut bb = function.get_first_basic_block().unwrap();
        let fi = bb.get_first_instruction().unwrap();
        self.builder.position_at(bb, &fi);
        self.builder.build_alloca(self.context.i64_type(), var_name)
    }

    fn gen_var_code(&mut self, name: String, e: AST) -> &Box<dyn AnyValue<'a> + 'a> {
        let basic_block_option = self.builder.get_insert_block();
        if basic_block_option.is_some() {
            let bastic_block = basic_block_option.unwrap();
            let function = bastic_block.get_parent().unwrap();
            let variable = self.create_entry_block_alloca(&function, &name);
            self.builder.build_store(variable, self.context.i64_type().const_int(0, true));
            self.named_values.insert(name.clone(), Box::new(variable));
        self.named_values.get(&name).unwrap()
        } else {
            let variable = self.module.add_global(self.context.i64_type(), Some(AddressSpace::Global), &name);
            let variable = variable.as_pointer_value();
            let body = self.gen_code(e);
            let val = body.as_any_value_enum().into_int_value().get_sign_extended_constant().unwrap();
            self.builder.build_store(variable, self.context.i64_type().const_int(val as u64, true));
            self.named_values.insert(name.clone(), Box::new(variable));
            self.named_values.get(&name).unwrap()
        }
    }

    fn gen_val_code(&mut self, data: i64) -> &Box<dyn AnyValue<'a> + 'a> {
        let val: IntValue<'a> = self.context.i64_type().const_int(data as u64, true);
        self.tmp_values.push(Box::new(val));
        self.tmp_values.last().unwrap()
    }

    fn gen_identifier_code(&mut self, s: String) -> &Box<dyn AnyValue<'a> + 'a> {
        let v = self.named_values.get(&s);
        let v = v.unwrap().as_any_value_enum().into_pointer_value();
        self.tmp_values.push(Box::new(self.builder.build_load(v, &s)));
        self.tmp_values.last().unwrap()
    }

    fn gen_pointer_code(&mut self, ast: AST) -> PointerValue<'a> {
        if let AST::Identifier(x) = ast {
            let v = self.named_values.get(&x);
            let v = v.unwrap().as_any_value_enum().into_pointer_value();
            v
        } else {
            panic!("Exptected Identifier")
        }
    }

    fn gen_binary_code(&mut self, op: Operator, l: AST, r: AST) -> &Box<dyn AnyValue<'a> + 'a> {
        if let Operator::Assign = op.clone() {
            let lhs: PointerValue<'a> = self.gen_pointer_code(l);
            let rhs: IntValue<'a> = self.gen_code(r).as_any_value_enum().into_int_value();
            self.builder.build_store(lhs, rhs);
            self.tmp_values.push(Box::new(rhs));
            self.tmp_values.last().unwrap()
        } else {
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
    }

    pub fn gen_code(&mut self, ast: AST) -> &Box<dyn AnyValue<'a> + 'a> {
        match ast {
            AST::Binary(op, l, r) => self.gen_binary_code(op, *l, *r),
            AST::Identifier(x) => self.gen_identifier_code(x),
            AST::Variable(name, _, value) => self.gen_var_code(name, *value),
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
