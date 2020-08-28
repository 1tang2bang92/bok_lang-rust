use crate::token::*;
use bok_utils::*;

#[derive(Clone, Debug)]
struct SourceLocation {
    line: i32,
    col: i32,
}

pub struct Tokenizer {
    last_char: char,
    cur_loc: SourceLocation,
    lex_loc: SourceLocation,
    toks: Vec<Token>,
    buf: Buffer<char>,
}

impl Tokenizer {
    pub fn new() -> Self {
        let last_char = ' ';
        let toks = Vec::new();
        let cur_loc = SourceLocation { col: 0, line: 0 };
        let lex_loc = SourceLocation { col: 1, line: 0 };
        let buf = Buffer::default();
        Self {
            cur_loc,
            lex_loc,
            last_char,
            toks,
            buf,
        }
    }

    fn two_operator(&mut self, cur: char) -> Token {
        if self.last_char == '=' {
            self.last_char = self.advance();
            if cur == '=' {
                return Token::Operator(Operator::Equal);
            } else if cur == '<' {
                return Token::Operator(Operator::LTE);
            } else if cur == '>' {
                return Token::Operator(Operator::GTE);
            } else if cur == '!' {
                return Token::Operator(Operator::NE);
            }
        }

        if cur == '+' {
            Token::Operator(Operator::Add)
        } else if cur == '-' {
            Token::Operator(Operator::Sub)
        } else if cur == '*' {
            Token::Operator(Operator::Mul)
        } else if cur == '/' {
            Token::Operator(Operator::Div)
        } else if cur == '=' {
            Token::Operator(Operator::Assign)
        } else if cur == '&' {
            Token::Operator(Operator::And)
        } else if cur == '|' {
            Token::Operator(Operator::Or)
        } else if cur == '<' {
            Token::Operator(Operator::LT)
        } else if cur == '>' {
            Token::Operator(Operator::GT)
        } else if cur == '!' {
            Token::Operator(Operator::Not)
        } else if cur == ',' {
            Token::Operator(Operator::Comma)
        } else {
            Token::Error
        }
    }

    pub fn getchar(&mut self) -> char {
        self.buf.next().unwrap_or(0 as char)
    }

    pub fn advance(&mut self) -> char {
        let last_char = self.getchar();

        if last_char == '\n' || last_char == '\r' {
            self.lex_loc.line += 1;
            self.lex_loc.col = 0;
        } else {
            self.lex_loc.col += 1;
        }
        return last_char;
    }

    pub fn gettok(&mut self) -> Token {
        while self.last_char.is_ascii_whitespace() {
            self.last_char = self.advance();
        }

        self.cur_loc = self.lex_loc.clone();
        if self.last_char.is_alphabetic() {
            let mut s = String::new();
            s.push(self.last_char);
            while {
                self.last_char = self.advance();
                self.last_char.is_alphanumeric()
            } {
                s.push(self.last_char);
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

        if self.last_char.is_ascii_digit() {
            let mut s = String::new();
            s.push(self.last_char);
            while {
                self.last_char = self.advance();
                self.last_char.is_alphanumeric()
            } {
                s.push(self.last_char);
            }
            return Token::Value(Type::Int, Value::Int(s.parse().unwrap()));
        }

        match self.last_char {
            '(' => {
                self.last_char = self.advance();
                return Token::ReservedWord(ReservedWord::LParen);
            },
            ')' => {
                self.last_char = self.advance();
                return Token::ReservedWord(ReservedWord::RParen);
            },
            '{' => {
                self.last_char = self.advance();
                return Token::ReservedWord(ReservedWord::LBrace);
            },
            '}' => {
                self.last_char = self.advance();
                return Token::ReservedWord(ReservedWord::RBrace);
            },
            ':' => {
                self.last_char = self.advance();
                return Token::ReservedWord(ReservedWord::Collon);
            },
            ';' => {
                self.last_char = self.advance();
                return Token::ReservedWord(ReservedWord::SemiCollon);
            },
            _ => (),
        }

        if "+-*/&|=<>!,".contains(self.last_char) {
            let tmp = self.last_char;
            self.last_char = self.advance();
            return self.two_operator(tmp);
        }

        if self.last_char == (0 as char) {
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