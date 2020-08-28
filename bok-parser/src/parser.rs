use bok_utils::*;
use bok_tokenizer::*;
use crate::ast::*;

pub struct Parser {
    buf: Buffer<Token>,
    //ast: AST,
}

impl Parser {
    pub fn new() -> Self {
        //let ast = AST::None;
        let buf = Buffer::default();
        Self { 
            buf, 
            //ast, 
        }
    }

    fn parse_let_expression(&mut self) -> AST {
        let tok = self.buf.next().expect("Expect Identifier");

        let id = if let Token::Identifier(x) = tok {
            x
        } else {
            panic!("Expect Identifier");
        };

        let value = if self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            let value = if let Token::Operator(Operator::Assign) = tok {
                self.expression()
            } else {
                self.buf.prev();
                AST::None
            };
            value
        } else {
            AST::None
        };

        AST::Variable(id, Type::None, Box::new(value))
    }

    fn parse_fn_expression(&mut self) -> AST {
        let tok = self.buf.next().expect("Expect Identifier");

        let id = if let Token::Identifier(x) = tok {
            x
        } else {
            panic!("Expect Identifier");
        };

        let tok = self.buf.next().expect("Expect LParen");
        if let Token::ReservedWord(ReservedWord::LParen) = tok {
        } else {
            panic!("Expect LParen");
        };

        let mut vec = Vec::new();
        let mut vid = String::new();
        let mut ty = Type::None;
        let mut colon = false;
        while self.buf.has_next() {
            match self.buf.next().unwrap() {
                Token::ReservedWord(ReservedWord::RParen) => {
                    vec.push(AST::Variable(vid.clone(), ty.clone(), Box::new(AST::None)));
                    break;
                },
                Token::Operator(Operator::Comma) => {
                    vec.push(AST::Variable(vid.clone(), ty.clone(), Box::new(AST::None)));
                    colon = false;
                },
                Token::Identifier(x) => {
                    if colon == false {
                        vid = x;
                        ty = Type::None;
                    } else {
                        ty = Type::Id(x);
                    }
                },
                Token::ReservedWord(ReservedWord::Collon) => {
                    colon = true;
                },
                x => {
                    panic!("Unexpected Token {:?}", x)
                },
            }
        }
        AST::Function(id, vec, Box::new(self.statement()))
    }

    fn parse_loop_expression(&mut self) -> AST {
        return AST::Loop(Box::new(self.statement()));
    }

    fn parse_if_expression(&mut self) -> AST {
        let none = AST::None;
        let condition = self.expression();
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
        let tree = self.expression();
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

    fn parse_call_expression(&mut self, s: String) -> AST {
        let mut vec = Vec::new();
        while self.buf.has_next() {
            match self.buf.next().unwrap() {
                Token::ReservedWord(ReservedWord::RParen) => {
                    break;
                },
                Token::Operator(Operator::Comma) => {
                    continue;
                },
                _ => {
                    self.buf.prev();
                    vec.push(self.expression());
                },
            }
        }
        AST::Call(s, vec)
    }

    fn factor(&mut self) -> AST {
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::Identifier(x) => {
                    if self.buf.has_next() {
                        let tok = self.buf.next().unwrap();
                        if let Token::ReservedWord(ReservedWord::LParen) = tok {
                            return self.parse_call_expression(x);
                        } else {
                            self.buf.prev();
                            return AST::Identifier(x);
                        }
                    } else {
                        return AST::Identifier(x);
                    }
                },
                Token::Value(Type::Int, x) => {
                    return AST::Value(Type::Int, x);
                },
                Token::ReservedWord(x) => match x {
                    ReservedWord::Let => {
                        return self.parse_let_expression();
                    },
                    ReservedWord::FN => {
                        return self.parse_fn_expression();
                    },
                    ReservedWord::Loop => {
                        return self.parse_loop_expression();
                    },
                    ReservedWord::RBrace => {
                        return AST::None;
                    },
                    ReservedWord::LParen => {
                        return self.parse_paran_expression();
                    },
                    ReservedWord::If => {
                        return self.parse_if_expression();
                    },
                    ReservedWord::SemiCollon => {
                        return self.factor();
                    },
                    x => {
                        panic!("Undefined Reserved Word '{:?}'", x);
                    },
                },
                Token::Operator(x) => match x {
                    Operator::Add => {
                        return self.factor();
                    },
                    Operator::Sub => {
                        return AST::Unary(Operator::Sub, Box::new(self.factor()));
                    },
                    x => {
                        panic!("Unexpected Toekn {:?}", x);
                    },
                },
                _ => {
                    panic!("Token Type Error");
                },
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
                    tree1 = AST::Binary(Operator::Mul, Box::new(tree1), Box::new(tree2));
                }
                Token::Operator(Operator::Div) => {
                    let tree2 = self.factor();
                    tree1 = AST::Binary(Operator::Div, Box::new(tree1), Box::new(tree2));
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
                    tree1 = AST::Binary(Operator::Add, Box::new(tree1), Box::new(tree2));
                },
                Token::Operator(Operator::Sub) => {
                    let tree2 = self.product();
                    tree1 = AST::Binary(Operator::Sub, Box::new(tree1), Box::new(tree2));
                },
                _ => {
                    self.buf.prev();
                    return tree1;
                },
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
                    tree1 = AST::Binary(Operator::And, Box::new(tree1), Box::new(tree2));
                },
                Token::Operator(Operator::Or) => {
                    let tree2 = self.sum();
                    tree1 = AST::Binary(Operator::Or, Box::new(tree1), Box::new(tree2));
                },
                _ => {
                    self.buf.prev();
                    return tree1;
                },
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
                    return AST::Binary(Operator::Equal, Box::new(tree1), Box::new(tree2));
                },
                Token::Operator(Operator::NE) => {
                    let tree2 = self.bit();
                    return AST::Binary(Operator::NE, Box::new(tree1), Box::new(tree2));
                },
                Token::Operator(Operator::LT) => {
                    let tree2 = self.bit();
                    return AST::Binary(Operator::LT, Box::new(tree1), Box::new(tree2));
                },
                Token::Operator(Operator::LTE) => {
                    let tree2 = self.bit();
                    return AST::Binary(Operator::LTE, Box::new(tree1), Box::new(tree2));
                },
                Token::Operator(Operator::GT) => {
                    let tree2 = self.bit();
                    return AST::Binary(Operator::GT, Box::new(tree1), Box::new(tree2));
                },
                Token::Operator(Operator::GTE) => {
                    let tree2 = self.bit();
                    return AST::Binary(Operator::GTE, Box::new(tree1), Box::new(tree2));
                },
                _ => {
                    self.buf.prev();
                    return tree1;
                },
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
                    tree1 = AST::Binary(Operator::Assign, Box::new(tree1), Box::new(tree2));
                },
                _ => {
                    self.buf.prev();
                    return tree1;
                },
            }
        }
        tree1
    }
    
    fn expression(&mut self)  -> AST {
        self.assign()
    }

    fn statement(&mut self) -> AST {
        while self.buf.has_next() {
            let tok = self.buf.next().unwrap();
            match tok {
                Token::ReservedWord(ReservedWord::LBrace) => {
                    let mut vec = Vec::new();
                    while self.buf.has_next() {
                        let tok = self.buf.next().unwrap();
                        match tok {
                            Token::ReservedWord(ReservedWord::RBrace) => {
                                return AST::Statement(vec);
                            },
                            _ => {
                                self.buf.prev();
                                let tree = self.expression();
                                vec.push(tree);
                            },
                        }
                    }
                },
                _ => {
                    self.buf.prev();
                    return self.expression();
                },
            }
        }
        self.expression()
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