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
}

#[derive(Clone, Debug)]
pub enum Type {
    Int(i64),
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
    Type(Type),
    ReservedWord(ReservedWord),
    Identifier(String),
    EOF,
    Error,
}