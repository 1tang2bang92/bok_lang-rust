use std::collections::linked_list::*;
use std::iter::FromIterator;
use std::str::Chars;

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mode,
    Ref,
    Deref,
}

enum Type {
    Str(String),
    Int(i64),
    Float(f64),
} //FN 겹치면 안댐?

enum ReservedWord {
    If,
    Loop,
    FN,
    Let,
}

enum Token {
    Operator(Operator),
    Type(Type),
    ReservedWord(ReservedWord),
}

pub struct Tokenizer {
    Toks: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Self {
        let Toks = Vec::new();
        Self { Toks }
    }

    fn chars_to_int(s: char, chars: &mut Cursor<'_, char>) -> i64 {
        let mut result: i64 = s.to_digit(10).unwrap() as i64;
        for x in chars {
            if !x.is_ascii_digit() {
                break;
            }
            result *= 10;
            result += x.to_digit(10).unwrap() as i64;
        }
        chars.move_prev();
        result
    }

    pub fn tokenize(&mut self, s: &str) {
        let len = s.len();
        let mut char_list: LinkedList<char> = LinkedList::from_iter(s.chars());
        let mut cur: Cursor<'_, char> = char_list.cursor_front();
        let mut c = 0;
        loop {
            let ch = chars.next();
            if (ch.is_none()) {
                break;
            }
            let ch = ch.unwrap();
            if (ch.is_ascii_digit()) {
                println!("{}", chars_to_int(ch, chars: &mut Chars));
            }
        }
    }
}
