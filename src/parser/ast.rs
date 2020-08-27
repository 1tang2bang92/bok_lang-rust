use crate::token::*;

#[derive(Clone, Debug)]
pub enum AST {
    Binary(Operator, Box<AST>, Box<AST>),
    Unary(Operator, Box<AST>),
    Value(Type),
    Identifier(String),
    FN(String, Vec<AST>, Box<AST>),
    Call(String, Vec<AST>),
    Let(Box<AST>, Box<AST>),
    If(Box<AST>, Box<AST>, Box<AST>),
    Loop(Box<AST>),
    ReservedWord(ReservedWord),
    Statement(Vec<AST>),
    None,
}