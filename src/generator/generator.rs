use std::collections::HashMap;

use crate::ast::AST;
use crate::Operator;
use crate::Type;
use crate::Value;

use inkwell::*;
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
    named_values: HashMap<String, PointerValue<'a>>,
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

    fn create_entry_block_alloca(&mut self, function: &FunctionValue, var_name: &str) -> PointerValue<'a> {
        let mut bb = function.get_first_basic_block().unwrap();
        let fi = bb.get_first_instruction();
        if fi.is_some() {
            self.builder.position_at(bb, &fi.unwrap());
            self.builder.build_alloca(self.context.i64_type(), var_name)
        } else {
            self.builder.position_at_end(bb);
            self.builder.build_alloca(self.context.i64_type(), var_name)
        }
    }

    fn gen_var_code(&mut self, name: &str, e: AST) -> IntValue<'a> {
        let basic_block_option = self.builder.get_insert_block();
        if basic_block_option.is_some() {
            let bastic_block = basic_block_option.unwrap();
            let function = bastic_block.get_parent().unwrap();
            let variable = self.create_entry_block_alloca(&function, name);
            let body = self.gen_code(e);
            let val = body.as_any_value_enum().into_int_value();
            self.builder.build_store(variable, val);
            self.named_values.insert(name.to_string(), variable);
            self.builder.build_load(variable, name).into_int_value()
        } else {
            panic!("Variable Location Error");
        }
    }

    fn gen_val_code(&mut self, data: i64) -> IntValue<'a> {
        self.context.i64_type().const_int(data as u64, true)
    }

    fn gen_identifier_code(&mut self, name: &str) -> IntValue<'a> {
        let pv = self.named_values.get(name).unwrap();
        self.builder.build_load(*pv, name).into_int_value()
    }

    fn gen_pointer_code(&mut self, ast: AST) -> PointerValue<'a> {
        if let AST::Identifier(x) = ast {
            *self.named_values.get(&x).unwrap()
        } else {
            panic!("Exptected Identifier")
        }
    }

    fn gen_binary_code(&mut self, op: Operator, l: AST, r: AST) -> IntValue<'a> {
        if let Operator::Assign = op.clone() {
            let lhs: PointerValue<'a> = self.gen_pointer_code(l);
            let rhs: IntValue<'a> = self.gen_code(r);
            self.builder.build_store(lhs, rhs);
            rhs
        } else {
            let lhs: IntValue<'a> = self.gen_code(l);
            let rhs: IntValue<'a> = self.gen_code(r);
    
            let val: IntValue<'a> = match op {
                Operator::Add => self.builder.build_int_add(lhs, rhs, "addtmp"),
                Operator::Sub => self.builder.build_int_sub(lhs, rhs, "subtmp"),
                Operator::Mul => self.builder.build_int_mul(lhs, rhs, "multmp"),
                Operator::Div => self.builder.build_int_signed_div(lhs, rhs, "divtmp"),
                Operator::Equal => self.builder.build_int_compare(IntPredicate::EQ, lhs, rhs, "comptmp"),
                _ => panic!(""),
            };
            val
        }        
    }

    fn gen_function_code(&mut self, name: &str, vars: Vec<AST>, body: AST) -> IntValue<'a> {

        let mut arr = Vec::new();
        for x in 0 .. vars.len() {
            arr.push(BasicTypeEnum::IntType(self.context.i64_type()));
        }

        let function_type = self.context.i64_type().fn_type(arr.as_slice(), true);

        let function = self.module.add_function(&name, function_type, Some(Linkage::Internal));

        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);


        let mut args = Vec::new();
        for x in vars {
            if let AST::Variable(id, _, _) = x{
                args.push(id);
            }
        }

        for (idx, param) in function.get_param_iter().enumerate() {
            param.set_name(&args[idx]);
        }

        self.named_values.clear();
        for (idx, param) in function.get_param_iter().enumerate() {
            let arg_name = args[idx].as_str();
            let alloca = self.create_entry_block_alloca(&function, arg_name);

            self.builder.build_store(alloca, param);

            self.named_values.insert(args[idx].clone(), alloca);
        }

        let ret_val = self.gen_code(body);
        self.builder.build_return(Some(&ret_val));
        ret_val
    }

    fn gen_call_code(&mut self, name: &str, vars: Vec<AST>) -> IntValue<'a> {
        let function = self.module.get_function(&name).unwrap();

        if function.get_params().len() != vars.len() {

        }
        let mut arr = Vec::new();
        for x in vars {
            arr.push(BasicValueEnum::IntValue(self.gen_code(x).as_any_value_enum().into_int_value()));
        }

        self.builder.build_call(function, arr.as_slice(), "calltmp").as_any_value_enum().into_int_value()
    }

    fn gen_if_code(&mut self, condition: AST, then: AST, el: AST) -> IntValue<'a> {
        let cond = self.gen_code(condition).as_any_value_enum().into_int_value();
        let cond = self.builder.build_int_compare(IntPredicate::NE, cond, self.context.bool_type().const_zero(), "ifcondition");

        let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.insert_basic_block_after(then_block, "else");
        let merge_block = self.context.insert_basic_block_after(else_block, "murge");

        self.builder.build_conditional_branch(cond, then_block, else_block);

        self.builder.position_at_end(then_block);
        let thenv = self.gen_code(then);
        self.builder.build_unconditional_branch(merge_block);

        let then_block = self.builder.get_insert_block().unwrap();

        let mut elsev = None;
        if !el.is_none() {
            self.builder.position_at_end(else_block);
            elsev = Some(self.gen_code(el));
        }
        self.builder.build_unconditional_branch(merge_block);

        let else_block = self.builder.get_insert_block().unwrap();
        
        self.builder.position_at_end(merge_block);
        let phi = self.builder.build_phi(self.context.i64_type(), "iftmp");

        if elsev.is_some() {
            let elsev = elsev.unwrap();
            phi.add_incoming(&[(&elsev.as_any_value_enum().into_int_value(), else_block)]);
        } else {
            phi.add_incoming(&[(&self.context.i64_type().const_zero(), else_block)]);
        }
        phi.add_incoming(&[(&thenv.as_any_value_enum().into_int_value(), then_block)]);

        phi.as_basic_value().into_int_value()
    }

    pub fn gen_code(&mut self, ast: AST) -> IntValue<'a> {
        match ast {
            AST::Binary(op, l, r) => self.gen_binary_code(op, *l, *r),
            AST::Identifier(name) => self.gen_identifier_code(&name),
            AST::Variable(name, _, value) => self.gen_var_code(&name, *value),
            AST::Value(Type::Int, Value::Int(x)) => self.gen_val_code(x),
            AST::Function(name, vars, body) => self.gen_function_code(&name, vars, *body),
            AST::Call(name, vars) => self.gen_call_code(&name, vars),
            AST::If(cond, then, el) => self.gen_if_code(*cond, *then, *el),
            AST::Statement(x) => {
                let mut a = self.gen_val_code(0);
                for i in x {
                    a = self.gen_code(i);
                }
                a
            }
            _ => panic!(""),
        }
    }

    pub fn get_module(&mut self) -> &mut Module<'a> {
        &mut self.module
    }
}
