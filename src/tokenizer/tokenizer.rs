use crate::token::*;
use crate::buffer::*;

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
        let buf = Buffer::default();
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
        } else if (cur == ',') {
            Token::Operator(Operator::Comma)
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

        if "+-*/&|=<>!,".contains(self.lastChar) {
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