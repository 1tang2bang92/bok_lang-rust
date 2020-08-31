
use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    And,
    Or,
    Not,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    AndAssign,
    OrAssign,
    Equal,
    LessThan,
    GreatThan,
    LessThanEqual,
    GreatThanEqual,
    NotEequl,
    Comma,
}

#[derive(Clone, Debug)]
pub enum Type {
    Str(String),
    Int(i64),
    Float(f64),
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
    Comment,
}

#[derive(Clone, Debug)]
pub enum Token {
    Op(Op),
    Type(Type),
    ReservedWord(ReservedWord),
    Identifier(String),
    EOF,
    Error,
}

pub struct LexError {
    pub error: &'static str,
    pub index: usize,
}

impl LexError {
    pub fn new(msg: &'static str) -> LexError {
        LexError {
            error: msg,
            index: 0,
        }
    }

    pub fn with_index(msg: &'static str, index: usize) -> LexError {
        LexError {
            error: msg,
            index: index,
        }
    }
}

pub type LexResult = Result<Token, LexError>;

pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            chars: Box::new(input.chars().peekable()),
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> LexResult {
        let chars = self.chars.deref_mut();
        let src = self.input;

        let mut pos = self.pos;

        loop {
            {
                let ch = chars.peek();
            
                if ch.is_none() {
                    self.pos = pos;

                    return Ok(Token::EOF);
                }

                if !ch.unwrap().is_whitespace() {
                    break;
                }
            }

            chars.next();
            pos += 1;
        }

        let start = pos;
        let next = chars.next();

        if next.is_none() {
            return Ok(Token::EOF);
        }

        pos += 1;

        let result = match next.unwrap() {
            '(' => Ok(Token::ReservedWord::LParen),
            ')' => Ok(Token::ReservedWord::RParen),
            ',' => Ok(Token::Op::Comma),
            '\/\/' => {
                loop {
                    let ch = chars.next();
                    pos += 1;

                    if ch == Some('\n') {
                        break;
                    }
                }

                return Ok(Token::ReservedWord::Comment);
            }

            '0'..='9' => {
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return Ok(Token::EOF);
                    };

                    if !ch.is_digit(16) {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                Ok(Token::Number)
            }
        }
    }
}
