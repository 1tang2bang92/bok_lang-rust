use crate::token::*;

#[derive(Clone, Debug)]
pub enum AST {
    Bit(Operator, Box<AST>, Box<AST>),
    Sum(Operator, Box<AST>, Box<AST>),
    Product(Operator, Box<AST>, Box<AST>),
    Assign(Operator, Box<AST>, Box<AST>),
    Compare(Operator, Box<AST>, Box<AST>),
    Value(Type),
    Identifier(String),
    FN(Box<AST>, Box<AST>, Box<AST>),
    Call(Box<AST>, Box<AST>),
    Let(Box<AST>, Box<AST>),
    If(Box<AST>, Box<AST>, Box<AST>),
    Loop(Box<AST>),
    ReservedWord(ReservedWord),
    Statement(Vec<AST>),
    None,
}