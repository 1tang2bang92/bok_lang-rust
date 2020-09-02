use crate::token::*;
use inkwell::values::*;

#[derive(Clone, Debug)]
pub enum AST {
    Binary(Operator, Box<AST>, Box<AST>),
    Unary(Operator, Box<AST>),
    Value(Type, Value),
    Identifier(String),
    Function(String, Vec<AST>, Box<AST>),
    Call(String, Vec<AST>),
    Variable(String, Type, Box<AST>),
    If(Box<AST>, Box<AST>, Box<AST>),
    Loop(Box<AST>),
    Statement(Vec<AST>),
    Break(Box<AST>),
    None,
}

impl AST {
    pub fn is_none(&self) -> bool {
        match self {
            AST::None => true,
            _ => false,
        }
    }
}
