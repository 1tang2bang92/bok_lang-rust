use crate::buffer::*;
use crate::token::*;
use crate::ast::*;

pub struct Parser {
    buf: Buffer<Token>,
    ast: AST,
}

impl Parser {
    pub fn new() -> Self {
        let ast = AST::None;
        let buf = Buffer::default();
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