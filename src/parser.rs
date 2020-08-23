use std::collections::linked_list::*;
use std::iter::FromIterator;
use std::str::Chars;

#[derive(Clone,Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mode,
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
    fn next(&mut self) -> Option<&char> {
        let item = self.vec.get(self.cur);
        self.cur += 1;
        item
    }
    fn prev(&mut self) -> Option<&char> {
        self.cur -= 1;
        let item = self.vec.get(self.cur);
        item
    }
}

pub struct Tokenizer {
    Toks: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Self {
        let Toks = Vec::new();
        Self { Toks }
    }

    fn two_operator(c: char, b: &mut Buffer) -> Option<Token> {
        let x = b.next();
        if x.is_some() {
            let x = x.unwrap();
            if (*x == '=') {
                if (c == '+') {
                    return Some(Token::Operator(Operator::AddAssign));
                } else if (c == '-') {
                    return Some(Token::Operator(Operator::SubAssign));
                } else if (c == '*') {
                    return Some(Token::Operator(Operator::MulAssign));
                } else if (c == '/') {
                    return Some(Token::Operator(Operator::DivAssign));
                } else if (c == '=') {
                    return Some(Token::Operator(Operator::Compare));
                } else if (c == '&') {
                    return Some(Token::Operator(Operator::AndAssign));
                } else if (c == '|') {
                    return Some(Token::Operator(Operator::OrAssign));
                } else {
                    return None;
                }
            }
        }
        b.prev();
        if (c == '+') {
            Some(Token::Operator(Operator::Add))
        } else if (c == '-') {
            Some(Token::Operator(Operator::Sub))
        } else if (c == '*') {
            Some(Token::Operator(Operator::Mul))
        } else if (c == '/') {
            Some(Token::Operator(Operator::Div))
        } else if (c == '=') {
            Some(Token::Operator(Operator::Assign))
        } else if (c == '&') {
            Some(Token::Operator(Operator::And))
        } else if (c == '|') {
            Some(Token::Operator(Operator::Or))
        } else {
            None
        }
    }

    fn get_token_kind(s: String) -> Token {
        if s == "if" {
            Token::ReservedWord(ReservedWord::If)
        } else if s == "loop" {
            Token::ReservedWord(ReservedWord::Loop)
        } else if s == "fn" {
            Token::ReservedWord(ReservedWord::FN)
        } else if s == "let" {
            Token::ReservedWord(ReservedWord::Let)
        } else if s == "continue" {
            Token::ReservedWord(ReservedWord::Continue)
        } else if s == "break" {
            Token::ReservedWord(ReservedWord::Break)
        } else {
            Token::Identifier(s)
        }
    }

    fn chars_to_id(s: char, b: &mut Buffer) -> String {
        let mut result: String = String::new();
        result.push(s);
        loop {
            let x = b.next();
            if x.is_none() {
                break;
            }
            let x = x.unwrap();
            if !x.is_alphanumeric() {
                break;
            }
            result.push(*x)
        }
        b.prev();
        result
    }

    fn chars_to_int(s: char, b: &mut Buffer) -> i64 {
        let mut result: i64 = s.to_digit(10).unwrap() as i64;
        loop {
            let x = b.next();
            if x.is_none() {
                break;
            }
            let x = x.unwrap();
            if !x.is_ascii_digit() {
                break;
            }
            result *= 10;
            result += x.to_digit(10).unwrap() as i64;
        }
        b.prev();
        result
    }

    fn chars_to_str(s: char, b: &mut Buffer) -> String {
        let mut result: String = String::new();
        loop {
            let x = b.next();
            if x.is_none() {
                break;
            }
            let x = x.unwrap();
            if *x == '"' {
                break;
            }
            result.push(*x)
        }
        result
    }

    pub fn tokenize(&mut self, s: &str) -> Vec<Token> {
        let len = s.len();
        let mut char_buffer = Buffer::new(s.chars());
        loop {
            let ch = char_buffer.next();
            if (ch.is_none()) {
                break;
            }

            let ch = ch.unwrap();
            if ch.is_ascii_whitespace() {
                continue;
            } 

            if ch.is_ascii_alphabetic() {
                let t = Self::chars_to_id(*ch, &mut char_buffer);
                self.Toks.push(Self::get_token_kind(t));
            } else if (ch.is_ascii_digit()) {
                let t = Self::chars_to_int(*ch, &mut char_buffer);
                self.Toks.push(Token::Type(Type::Int(t)));
            } else if (*ch == '"') {
                let t = Self::chars_to_str(*ch, &mut char_buffer);
                self.Toks.push(Token::Type(Type::Str(t.clone())));
            } else if (*ch == '(') {
                self.Toks.push(Token::ReservedWord(ReservedWord::LParen));
            } else if (*ch == ')') {
                self.Toks.push(Token::ReservedWord(ReservedWord::RParen));
            } else if (*ch == '{') {
                self.Toks.push(Token::ReservedWord(ReservedWord::LBrace));
            } else if (*ch == '}') {
                self.Toks.push(Token::ReservedWord(ReservedWord::RBrace));
            } else if "+-*/&|=".contains(*ch) {
                if let Some(t) = Self::two_operator(*ch, &mut char_buffer) {
                    self.Toks.push(t);
                }        
            }
        }
        self.Toks.clone()
    }
}
