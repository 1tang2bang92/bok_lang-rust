use std::collections::linked_list::*;
use std::iter::FromIterator;
use std::str::Chars;

#[derive(Clone,Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    And,
    Or,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    AndAssign,
    OrAssign,
    Compare,
}

#[derive(Clone,Debug)]
pub enum Type {
    Str(String),
    Int(i64),
    Float(f64),
}

#[derive(Clone,Debug)]
pub enum ReservedWord {
    If,
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

#[derive(Clone,Debug)]
pub enum Token {
    Operator(Operator),
    Type(Type),
    ReservedWord(ReservedWord),
    Identifier(String),
    EOF,
    Error,
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
        if  item.is_some() {
            Some(*item.unwrap())
        } else {
            None
        }
    }
    fn prev(&mut self) -> Option<char> {
        self.cur -= 1;
        let item = self.vec.get(self.cur);
        if  item.is_some() {
            Some(*item.unwrap())
        } else {
            None
        }
    }
}

#[derive(Clone,Debug)]
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
    s: String,
    num: i64,
}

impl Tokenizer {
    pub fn new() -> Self {
        let lastChar = ' ';
        let toks = Vec::new();
        let curLoc = SourceLocation { col: 0, line: 0 };
        let lexLoc = SourceLocation  { col: 1, line: 0 };
        let buf = Buffer { vec: Vec::new(), cur: 0};
        Self { curLoc, lexLoc, lastChar ,toks, buf, s: String::new(), num: 0 }
    }

    fn two_operator(&mut self, cur: char) -> Token {
        if (x == '=') {
            if (cur == '+') {
                return Token::Operator(Operator::AddAssign);
            } else if (cur == '-') {
                return Token::Operator(Operator::SubAssign);
            } else if (cur == '*') {
                return Token::Operator(Operator::MulAssign);
            } else if (cur == '/') {
                return Token::Operator(Operator::DivAssign);
            } else if (cur == '=') {
                return Token::Operator(Operator::Compare);
            } else if (cur == '&') {
                return Token::Operator(Operator::AndAssign);
            } else if (cur == '|') {
                return Token::Operator(Operator::OrAssign);
            } else {
                return Token::Error;
            }
        }

        if (cur == '+') {
            Token::Operator(Operator::Add))
        } else if (cur == '-') {
            Token::Operator(Operator::Sub))
        } else if (cur == '*') {
            Some(Token::Operator(Operator::Mul))
        } else if (cur == '/') {
            Some(Token::Operator(Operator::Div))
        } else if (cur == '=') {
            Some(Token::Operator(Operator::Assign))
        } else if (cur == '&') {
            Some(Token::Operator(Operator::And))
        } else if (cur == '|') {
            Some(Token::Operator(Operator::Or))
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
        };

        self.curLoc = self.lexLoc.clone();
        
        if self.lastChar.is_alphabetic() {
            self.s = String::new();
            self.s.push(self.lastChar);
            while {
                self.lastChar = self.advance();
                self.lastChar.is_alphanumeric()
            } {
                self.s.push(self.lastChar);
            }
            return Token::Identifier(self.s.clone());
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
            '(' => {self.lastChar = self.advance();return Token::ReservedWord(ReservedWord::LParen);},
            ')' => {self.lastChar = self.advance();return Token::ReservedWord(ReservedWord::RParen);},
            '{' => {self.lastChar = self.advance();return Token::ReservedWord(ReservedWord::LBrace);},
            '}' => {self.lastChar = self.advance();return Token::ReservedWord(ReservedWord::RBrace);},
            _ => (),
        }

        match self.lastChar {
            '+' => {self.lastChar = self.advance();return Token::Operator(Operator::Add);},
            '-' => {self.lastChar = self.advance();return Token::Operator(Operator::Sub);},
            '*' => {self.lastChar = self.advance();return Token::Operator(Operator::Mul);},
            '/' => {self.lastChar = self.advance();return Token::Operator(Operator::Div);},
            '%' => {self.lastChar = self.advance();return Token::Operator(Operator::Mod);},
            '&' => {self.lastChar = self.advance();return Token::Operator(Operator::And);},
            '|' => {self.lastChar = self.advance();return Token::Operator(Operator::Or);},
            _ => (),
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
