use std::collections::linked_list::*;
use std::iter::FromIterator;
use std::str::Chars;

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
    Let(Box<AST>),
    If(Box<AST>, Box<AST>, Box<AST>),
    Loop(Box<AST>),
    ReservedWord(ReservedWord),
    None,
}

struct Buffer {
    vec: Vec<char>,
    cur: usize,
}

impl Buffer {
    fn new(iter: Chars) -> Self {
        let vec = Vec::from_iter(iter);
        let cur = 0;
        Self { vec, cur }
    }
    fn next(&mut self) -> Option<char> {
        let item = self.vec.get(self.cur);
        self.cur += 1;
        if let Some(x) = item {
            Some(*x)
        } else {
            None
        }
    }
    fn prev(&mut self) -> Option<char> {
        self.cur -= 1;
        let item = self.vec.get(self.cur);
        if let Some(x) = item {
            Some(*x)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct SourceLocation {
    line: i32,
    col: i32,
}

pub struct Tokenizer {
    lastChar: char,
    curLoc: SourceLocation,
    lexLoc: SourceLocation,
    toks: Vec<Token>,
    buf: Buffer,
}

impl Tokenizer {
    pub fn new() -> Self {
        let lastChar = ' ';
        let toks = Vec::new();
        let curLoc = SourceLocation { col: 0, line: 0 };
        let lexLoc = SourceLocation { col: 1, line: 0 };
        let buf = Buffer {
            vec: Vec::new(),
            cur: 0,
        };
        Self {
            curLoc,
            lexLoc,
            lastChar,
            toks,
            buf,
        }
    }

    fn two_operator(&mut self, cur: char) -> Token {
        if (self.lastChar == '=') {
            self.lastChar = self.advance();
            if (cur == '=') {
                return Token::Operator(Operator::Equal);
            } else if (cur == '<') {
                return Token::Operator(Operator::LTE);
            } else if (cur == '>') {
                return Token::Operator(Operator::GTE);
            } else if (cur == '!') {
                return Token::Operator(Operator::NE);
            } else {
                return Token::Error;
            }
        }

        if (cur == '+') {
            Token::Operator(Operator::Add)
        } else if (cur == '-') {
            Token::Operator(Operator::Sub)
        } else if (cur == '*') {
            Token::Operator(Operator::Mul)
        } else if (cur == '/') {
            Token::Operator(Operator::Div)
        } else if (cur == '=') {
            Token::Operator(Operator::Assign)
        } else if (cur == '&') {
            Token::Operator(Operator::And)
        } else if (cur == '|') {
            Token::Operator(Operator::Or)
        } else if (cur == '<') {
            Token::Operator(Operator::LT)
        } else if (cur == '>') {
            Token::Operator(Operator::GT)
        } else if (cur == '!') {
            Token::Operator(Operator::Not)
        } else {
            Token::Error
        }
    }

    pub fn getchar(&mut self) -> char {
        self.buf.next().unwrap_or(0 as char)
    }

    pub fn advance(&mut self) -> char {
        let LastChar = self.getchar();

        if (LastChar == '\n' || LastChar == '\r') {
            self.lexLoc.line += 1;
            self.lexLoc.col = 0;
        } else {
            self.lexLoc.col += 1;
        }
        return LastChar;
    }

    pub fn gettok(&mut self) -> Token {
        while self.lastChar.is_ascii_whitespace() {
            self.lastChar = self.advance();
        }

        self.curLoc = self.lexLoc.clone();
        if self.lastChar.is_alphabetic() {
            let mut s = String::new();
            s.push(self.lastChar);
            while {
                self.lastChar = self.advance();
                self.lastChar.is_alphanumeric()
            } {
                s.push(self.lastChar);
            }

            if s == "if" {
                return Token::ReservedWord(ReservedWord::If);
            } else if s == "else" {
                return Token::ReservedWord(ReservedWord::Else);
            } else if s == "fn" {
                return Token::ReservedWord(ReservedWord::FN);
            } else if s == "let" {
                return Token::ReservedWord(ReservedWord::Let);
            } else if s == "loop" {
                return Token::ReservedWord(ReservedWord::Loop);
            }

            return Token::Identifier(s.clone());
        }

        if self.lastChar.is_ascii_digit() {
            let mut s = String::new();
            s.push(self.lastChar);
            while {
                self.lastChar = self.advance();
                self.lastChar.is_alphanumeric()
            } {
                s.push(self.lastChar);
            }
            return Token::Type(Type::Int(s.parse().unwrap()));
        }

        match self.lastChar {
            '(' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::LParen);
            }
            ')' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::RParen);
            }
            '{' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::LBrace);
            }
            '}' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::RBrace);
            }
            _ => (),
        }

        if "+-*/&|=<>!".contains(self.lastChar) {
            let tmp = self.lastChar;
            self.lastChar = self.advance();
            return self.two_operator(tmp);
        }

        if self.lastChar == (0 as char) {
            return Token::EOF;
        }

        return Token::Error;
    }

    pub fn tokenize(&mut self, s: &str) -> Vec<Token> {
        self.buf = Buffer::new(s.chars());
        loop {
            let t = self.gettok();
            if let Token::EOF = t {
                break;
            }
            self.toks.push(t);
        }
        self.toks.clone()
    }
}

pub struct Parser {
    ast: AST,
}

impl Parser {
    pub fn new() -> Self {
        let ast = AST::None;
        Self { ast }
    }

    fn factor(&mut self, mut toks: Vec<Token>) -> (AST, Vec<Token>) {
        if toks.is_empty() {
            return (AST::None, toks);
        }
        let tok = toks[0].clone();
        match tok {
            Token::Type(x) => {
                toks.remove(0);
                (AST::Value(x), toks)
            },
            Token::ReservedWord(ReservedWord::RParen) => {
                (AST::None, toks)
            },
            Token::Identifier(x) => {
                toks.remove(0);
                if toks.is_empty() {
                    (AST::Identifier(x), toks)
                } else {
                    let tok = toks[0].clone();
                    if let Token::ReservedWord(ReservedWord::LParen) = tok {
                        toks.remove(0);
                        let (tree, mut toks) = self.statement(toks);
                        if toks.is_empty() {
                            panic!("Expected ')'");
                        }
                        let tok = toks[0].clone();
                        if let Token::ReservedWord(ReservedWord::RParen) = tok {
                            toks.remove(0);
                            (
                                AST::Call(Box::new(AST::Identifier(x)), Box::new(tree)),
                                toks,
                            )
                        } else {
                            panic!("Expected ')'");
                        }
                    } else {
                        (AST::Identifier(x), toks)
                    }
                }
            }
            Token::ReservedWord(ReservedWord::LParen) => {
                toks.remove(0);
                let (ast, mut toks) = self.assign(toks);
                if toks.is_empty() {
                    return (AST::None, toks);
                }
                let tok = toks[0].clone();
                if let Token::ReservedWord(ReservedWord::RParen) = tok {
                    toks.remove(0);
                    (ast, toks)
                } else {
                    panic!("Expected ')'");
                }
            }
            Token::ReservedWord(ReservedWord::Let) => {
                toks.remove(0);
                let (tree, toks) = self.assign(toks);
                (AST::Let(Box::new(tree)), toks)
            }
            Token::ReservedWord(ReservedWord::FN) => {
                toks.remove(0);
                let (tree1, mut toks) = self.statement(toks);
                let tok = toks[0].clone();
                if let Token::ReservedWord(ReservedWord::LParen) = tok {
                    toks.remove(0);
                } else {
                    panic!("Expected '('");
                }
                let (tree2, mut toks) = self.statement(toks);
                if toks.is_empty() {
                    return (AST::None, toks);
                }
                let tok = toks[0].clone();
                if let Token::ReservedWord(ReservedWord::RParen) = tok {
                    toks.remove(0);
                } else {
                    panic!("Expected ')'");
                }
                let (tree3, toks) = self.statement(toks);
                (
                    AST::FN(Box::new(tree1), Box::new(tree2), Box::new(tree3)),
                    toks,
                )
            }
            Token::ReservedWord(ReservedWord::If) => {
                toks.remove(0);
                let (tree1, toks) = self.compare(toks);
                let (tree2, mut toks) = self.statement(toks);
                if toks.is_empty() {
                    return (
                        AST::If(Box::new(tree1), Box::new(tree2), Box::new(AST::None)),
                        toks,
                    );
                }
                let tok = toks[0].clone();
                if let Token::ReservedWord(ReservedWord::Else) = tok {
                    toks.remove(0);
                    let (tree3, toks) = self.statement(toks);
                    (
                        AST::If(Box::new(tree1), Box::new(tree2), Box::new(tree3)),
                        toks,
                    )
                } else {
                    (
                        AST::If(Box::new(tree1), Box::new(tree2), Box::new(AST::None)),
                        toks,
                    )
                }
            }
            Token::ReservedWord(x) => (AST::ReservedWord(x), toks),
            _ => panic!("Parser Factor Error"),
        }
    }

    fn product(&mut self, mut toks: Vec<Token>) -> (AST, Vec<Token>) {
        let (tree1, mut toks) = self.factor(toks);
        if toks.is_empty() {
            return (tree1, toks);
        }
        let tok = toks[0].clone();
        match tok {
            Token::Operator(Operator::Mul) => {
                toks.remove(0);
                let (tree2, toks) = self.product(toks);
                (
                    AST::Product(Operator::Mul, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::Div) => {
                toks.remove(0);
                let (tree2, toks) = self.product(toks);
                (
                    AST::Product(Operator::Div, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            _ => (tree1, toks),
        }
    }

    fn sum(&mut self, mut toks: Vec<Token>) -> (AST, Vec<Token>) {
        let (tree1, mut toks) = self.product(toks);
        if toks.is_empty() {
            return (tree1, toks);
        }
        let tok = toks[0].clone();
        match tok {
            Token::Operator(Operator::Add) => {
                toks.remove(0);
                let (tree2, toks) = self.sum(toks);
                (
                    AST::Sum(Operator::Add, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::Sub) => {
                toks.remove(0);
                let (tree2, toks) = self.sum(toks);
                (
                    AST::Sum(Operator::Sub, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            _ => (tree1, toks),
        }
    }

    fn bit(&mut self, mut toks: Vec<Token>) -> (AST, Vec<Token>) {
        let (tree1, mut toks) = self.sum(toks);
        if toks.is_empty() {
            return (tree1, toks);
        }
        let tok = toks[0].clone();
        match tok {
            Token::Operator(Operator::And) => {
                toks.remove(0);
                let (tree2, toks) = self.bit(toks);
                (
                    AST::Bit(Operator::And, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::Or) => {
                toks.remove(0);
                let (tree2, toks) = self.bit(toks);
                (
                    AST::Bit(Operator::Or, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            _ => (tree1, toks),
        }
    }

    fn compare(&mut self, mut toks: Vec<Token>) -> (AST, Vec<Token>) {
        let (tree1, mut toks) = self.bit(toks);
        if toks.is_empty() {
            return (tree1, toks);
        }
        let tok = toks[0].clone();
        match tok {
            Token::Operator(Operator::Equal) => {
                toks.remove(0);
                let (tree2, toks) = self.compare(toks);
                (
                    AST::Compare(Operator::Equal, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::NE) => {
                toks.remove(0);
                let (tree2, toks) = self.compare(toks);
                (
                    AST::Compare(Operator::NE, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::LT) => {
                toks.remove(0);
                let (tree2, toks) = self.compare(toks);
                (
                    AST::Compare(Operator::LT, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::LTE) => {
                toks.remove(0);
                let (tree2, toks) = self.compare(toks);
                (
                    AST::Compare(Operator::LTE, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::GT) => {
                toks.remove(0);
                let (tree2, toks) = self.compare(toks);
                (
                    AST::Compare(Operator::GT, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            Token::Operator(Operator::GTE) => {
                toks.remove(0);
                let (tree2, toks) = self.compare(toks);
                (
                    AST::Compare(Operator::GTE, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            _ => (tree1, toks),
        }
    }

    fn assign(&mut self, mut toks: Vec<Token>) -> (AST, Vec<Token>) {
        let (tree1, mut toks) = self.compare(toks);
        if toks.is_empty() {
            return (tree1, toks);
        }
        let tok = toks[0].clone();
        match tok {
            Token::Operator(Operator::Assign) => {
                toks.remove(0);
                let (tree2, toks) = self.assign(toks);
                (
                    AST::Assign(Operator::Assign, Box::new(tree1), Box::new(tree2)),
                    toks,
                )
            }
            _ => (tree1, toks),
        }
    }

    fn statement(&mut self, mut toks: Vec<Token>) -> (AST, Vec<Token>) {
        let (tree1, mut toks) = self.assign(toks);
        if toks.is_empty() {
            return (tree1, toks);
        }
        let tok = toks[0].clone();
        match tok {
            Token::ReservedWord(ReservedWord::LBrace) => {
                toks.remove(0);
                let (tree2, mut toks) = self.statement(toks);
                let tok = toks[0].clone();
                if let Token::ReservedWord(ReservedWord::RBrace) = tok {
                    toks.remove(0);
                    (tree2, toks)
                } else {
                    panic!("Expected '}'");
                }
            }
            _ => (tree1, toks),
        }
    }

    pub fn parse(&mut self, mut toks: Vec<Token>) -> AST {
        let (ast, tokss) = self.statement(toks);
        ast

        //self.ast.clone()
    }
}
