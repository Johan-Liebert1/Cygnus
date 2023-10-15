use std::process::exit;

use crate::{
    lexer::Token,
    tokens::{Number, Operations, TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct BinaryOP {
    left: Box<dyn AST>,
    operator: Box<Token>,
    right: Box<dyn AST>,
}

impl BinaryOP {
    pub fn new(left: Box<dyn AST>, operator: Box<Token>, right: Box<dyn AST>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    fn get_left(&self) -> Number {
        match *self.left.visit().token {
            TokenEnum::Number(number) => {
                return number;
            }

            _ => {
                exit(1);
            }
        }
    }

    fn get_right(&self) -> Number {
        match *self.right.visit().token {
            TokenEnum::Number(number) => {
                return number;
            }

            _ => {
                exit(1);
            }
        }
    }

    fn add(&self) -> VisitResult {
        if let Number::Integer(l) = self.get_left() {
            if let Number::Integer(r) = self.get_right() {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Integer(l + r))),
                };
            }

            panic!("Cannot add Float to Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right() {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Float(l + r))),
                };
            }

            panic!("Cannot add Float to Integer");
        };

        panic!("wat add");
    }

    fn subtract(&self) -> VisitResult {
        if let Number::Integer(l) = self.get_left() {
            if let Number::Integer(r) = self.get_right() {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Integer(l - r))),
                };
            }

            panic!("Cannot subtract Float from Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right() {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Float(l - r))),
                };
            }

            panic!("Cannot subtract Float from Integer");
        };

        panic!("wat subtract");
    }

    fn multiply(&self) -> VisitResult {
        let left = self.get_left();
        let right = self.get_right();

        if let Number::Integer(l) = left {
            if let Number::Integer(r) = right {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Integer(l * r))),
                };
            }

            panic!("Cannot multiply Float with Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right() {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Float(l * r))),
                };
            }

            panic!("Cannot multiply Float with Integer");
        };

        panic!("wat multiply");
    }

    fn divide(&self) -> VisitResult {
        if let Number::Integer(l) = self.get_left() {
            if let Number::Integer(r) = self.get_right() {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Integer(l / r))),
                };
            }

            panic!("Cannot divide Float by Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right() {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Float(l / r))),
                };
            }

            panic!("Cannot divide Float by Integer");
        };

        panic!("wat divide")
    }
}

impl AST for BinaryOP {
    fn visit(&self) -> VisitResult {
        println!("Visiting BinaryOP, {:?}", &self.operator.token);

        match &self.operator.token {
            TokenEnum::Op(operation) => match operation {
                Operations::Plus => {
                    return self.add();
                }

                Operations::Minus => {
                    return self.subtract();
                }

                Operations::Divide => {
                    return self.divide();
                }

                Operations::Multiply => {
                    return self.multiply();
                }
            },

            _ => {
                unreachable!("Found a non operator in binary expression")
            }
        };
    }

    fn get_token(&self) -> &Token {
        return &self.operator;
    }

    fn print(&self) {
        self.left.print();
        println!("BinaryOP: {:?}", self.get_token());
        self.right.print();
    }
}
