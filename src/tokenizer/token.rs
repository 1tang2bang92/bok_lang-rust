#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    And,
    Or,
    Not,
    Equal,
    LT,
    GT,
    LTE,
    GTE,
    NE,
    Comma,
}

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Id(String),
    None,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    None,
}

#[derive(Clone, Debug)]
pub enum ReservedWord {
    If,
    Else,
    Loop,
    FN,
    Let,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Continue,
    Break,
    Collon,
    SemiCollon,
}

#[derive(Clone, Debug)]
pub enum Token {
    Operator(Operator),
    Value(Type, Value),
    ReservedWord(ReservedWord),
    Identifier(String),
    EOF,
    Error,
}
