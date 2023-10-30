use std::process::exit;

use crate::{
    constants,
    interpreter::interpreter::Variables,
    lexer::{
        lexer::Token,
        tokens::{Number, Operations, TokenEnum},
    },
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

    fn get_left(&self, i: &mut Variables) -> Number {
        match *self.left.visit(i).token {
            TokenEnum::Number(number) => {
                return number;
            }

            _ => {
                exit(1);
            }
        }
    }

    fn get_right(&self, i: &mut Variables) -> Number {
        match *self.right.visit(i).token {
            TokenEnum::Number(number) => {
                return number;
            }

            _ => {
                exit(1);
            }
        }
    }

    fn evaluate<T>(&self, l: T, r: T) -> T
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Sub<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: std::ops::Div<Output = T>,
    {
        match &self.operator.token {
            TokenEnum::Op(op) => match op {
                Operations::Plus => l + r,
                Operations::Minus => l - r,
                Operations::Divide => l / r,
                Operations::Multiply => l * r,
            },

            _ => {
                unreachable!("WTF!!")
            }
        }
    }
}

impl AST for BinaryOP {
    fn visit(&self, i: &mut Variables) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:#?}", &self);
            println!("===============================================");
        }

        if let Number::Integer(left) = self.get_left(i) {
            if let Number::Integer(right) = self.get_right(i) {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Integer(
                        self.evaluate(left, right),
                    ))),
                };
            }

            panic!("Cannot add Float to Integer");
        };

        if let Number::Float(left_op) = self.get_left(i) {
            if let Number::Float(right_op) = self.get_right(i) {
                return VisitResult {
                    token: Box::new(TokenEnum::Number(Number::Float(
                        self.evaluate(left_op, right_op),
                    ))),
                };
            }

            panic!("Cannot add Float to Integer");
        };

        panic!("wat add");
    }

    fn get_token(&self) -> &Token {
        return &self.operator;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
