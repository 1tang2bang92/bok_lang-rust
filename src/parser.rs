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

struct Buffer<T: Clone> {
    vec: Vec<T>,
    cur: usize,
}

impl<T: Clone> Buffer<T> {
    fn new<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let vec: Vec<T> = Vec::from_iter(iter);
        let cur = 0;
        Self { vec, cur }
    }

    fn has_next(&mut self) -> bool {
        if self.vec.len() == self.cur {
            false
        } else {
            true
        }
    }

    fn next(&mut self) -> Option<T> {
        let item = self.vec.get(self.cur);
        self.cur += 1;
        if let Some(x) = item {
            Some(x.clone())
        } else {
            None
        }
    }
    fn prev(&mut self) -> Option<T> {
        self.cur -= 1;
        let item = self.vec.get(self.cur);
        if let Some(x) = item {
            Some(x.clone())
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
    buf: Buffer<char>,
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
            },
            ')' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::RParen);
            },
            '{' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::LBrace);
            },
            '}' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::RBrace);
            },
            ':' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::Collon);
            },
            ';' => {
                self.lastChar = self.advance();
                return Token::ReservedWord(ReservedWord::SemiCollon);
            },
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
    buf: Buffer<Token>,
    ast: AST,
}

impl Parser {
    pub fn new() -> Self {
        let ast = AST::None;
        let buf = Buffer {
            vec: Vec::new(),
            cur: 0,
        };
        Self { buf, ast }
    }

    fn parse_var_expression(&mut self) -> AST {
        let tok = self.buf.next().expect("Expect Identifier");

        let id = if let Token::Identifier(x) = tok {
            AST::Identifier(x)
        } else {
            panic!("Expect Identifier");
        };

        let value = if self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            let value = if let Token::Operator(Operator::Assign) = tok {
                let tok = self.buf.next().unwrap();
                if let Token::Type(x) = tok {
                    AST::Value(x)
                } else {
                    panic!("Expect Value");
                    AST::None
                }
            } else {
                self.buf.prev();
                AST::None
            };
            value
        } else {
            AST::None
        };

        AST::Let(Box::new(id), Box::new(value))
    }

    fn parse_loop_expression(&mut self) -> AST {
        return AST::Loop(Box::new(self.statement()));
    }

    fn parse_if_expression(&mut self) -> AST {
        let none = AST::None;
        let condition = self.assign();
        let then = self.statement();
        let el = if self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            if let Token::ReservedWord(ReservedWord::Else) = tok {
                self.statement()
            } else {
                none
            }
        } else {
            none
        };

        return AST::If(Box::new(condition), Box::new(then), Box::new(el));
    }

    fn parse_paran_expression(&mut self) -> AST {
        let tree = self.assign();
        if self.buf.has_next() {
            if let Token::ReservedWord(ReservedWord::RParen) = self.buf.next().unwrap() {
                return tree;
            } else {
                panic!("Expected ')'");
            }
        } else {
            panic!("Expected ')'");
        }
    }

    fn factor(&mut self) -> AST {
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::Identifier(x) => {
                    if self.buf.has_next() {
                        let tok = self.buf.next().unwrap();
                        if let Token::ReservedWord(ReservedWord::LParen) = tok {
                            return AST::None;
                        } else {
                            self.buf.prev();
                            return AST::Identifier(x);
                        }
                    } else {
                        return AST::Identifier(x);
                    }
                },
                Token::Type(x) => match x {
                    Type::Int(x) => {
                        return AST::Value(Type::Int(x));
                    }
                    _ => {
                        panic!("Type Value Error");
                    }
                },
                Token::ReservedWord(x) => match x {
                    ReservedWord::Let => {
                        return self.parse_var_expression();
                    }
                    ReservedWord::Loop => {
                        return self.parse_loop_expression();
                    }
                    ReservedWord::RBrace => {
                        return AST::None;
                    }
                    ReservedWord::LParen => {
                        return self.parse_paran_expression();
                    }
                    ReservedWord::If => {
                        return self.parse_if_expression();
                    }
                    x => {
                        panic!("Undefined Reserved Word '{:?}'", x);
                    }
                },
                _ => {
                    panic!("Token Type Error");
                }
            }
        }
        AST::None
    }

    fn product(&mut self) -> AST {
        let mut tree1 = self.factor();
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::Operator(Operator::Mul) => {
                    let tree2 = self.factor();
                    tree1 = AST::Product(Operator::Mul, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::Div) => {
                    let tree2 = self.factor();
                    tree1 = AST::Product(Operator::Div, Box::new(tree1), Box::new(tree2));
                }
                _ => {
                    self.buf.prev();
                    return tree1;
                }
            }
        }
        tree1
    }

    fn sum(&mut self) -> AST {
        let mut tree1 = self.product();
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::Operator(Operator::Add) => {
                    let tree2 = self.product();
                    tree1 = AST::Sum(Operator::Add, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::Sub) => {
                    let tree2 = self.product();
                    tree1 = AST::Sum(Operator::Sub, Box::new(tree1), Box::new(tree2));
                }
                _ => {
                    self.buf.prev();
                    return tree1;
                }
            }
        }
        tree1
    }

    fn bit(&mut self) -> AST {
        let mut tree1 = self.sum();
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::Operator(Operator::And) => {
                    let tree2 = self.sum();
                    tree1 = AST::Bit(Operator::And, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::Or) => {
                    let tree2 = self.sum();
                    tree1 = AST::Bit(Operator::Or, Box::new(tree1), Box::new(tree2));
                }
                _ => {
                    self.buf.prev();
                    return tree1;
                }
            }
        }
        tree1
    }

    fn compare(&mut self) -> AST {
        let tree1 = self.bit();
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::Operator(Operator::Equal) => {
                    let tree2 = self.bit();
                    return AST::Compare(Operator::Equal, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::NE) => {
                    let tree2 = self.bit();
                    return AST::Compare(Operator::NE, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::LT) => {
                    let tree2 = self.bit();
                    return AST::Compare(Operator::LT, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::LTE) => {
                    let tree2 = self.bit();
                    return AST::Compare(Operator::LTE, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::GT) => {
                    let tree2 = self.bit();
                    return AST::Compare(Operator::GT, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::GTE) => {
                    let tree2 = self.bit();
                    return AST::Compare(Operator::GTE, Box::new(tree1), Box::new(tree2));
                }
                _ => {
                    self.buf.prev();
                    return tree1;
                }
            }
        }
        tree1
    }

    fn assign(&mut self) -> AST {
        let mut tree1 = self.compare();
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::Operator(Operator::Assign) => {
                    let tree2 = self.assign();
                    tree1 = AST::Assign(Operator::Assign, Box::new(tree1), Box::new(tree2));
                }
                _ => {
                    self.buf.prev();
                    return tree1;
                }
            }
        }
        tree1
    }

    fn statement(&mut self) -> AST {
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::ReservedWord(ReservedWord::LBrace) => {
                    let mut vec = Vec::new();
                    while (self.buf.has_next()) {
                        let tok = self.buf.next().unwrap();
                        match tok {
                            Token::ReservedWord(ReservedWord::RBrace) => {
                                return AST::Statement(vec);
                            }
                            _ => {
                                self.buf.prev();
                                let tree = self.assign();
                                vec.push(tree);
                            }
                        }
                    }
                }
                _ => {
                    self.buf.prev();
                    return self.assign();
                }
            }
        }
        self.assign()
    }

    pub fn parse(&mut self, toks: Vec<Token>) -> AST {
        self.buf = Buffer::new(toks);
        let mut vec = Vec::new();
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            if let Token::EOF = tok {
                return AST::Statement(vec);
            } else {
                self.buf.prev();
                let tree = self.statement();
                vec.push(tree);
            }
        }
        AST::Statement(vec)
        //self.ast.clone()
    }
}
