use std::process::exit;

use crate::{lexer::Token, tokens::{Operations, TokenEnum, Number}};

use super::abstract_syntax_tree::AST;

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

    fn get_left(&self) -> &Number {
        match &self.left.get_token().token {
            TokenEnum::Number(number) => {
                return number;
            },

            _ => {
                exit(1);
            }
        }
    }

    fn get_right(&self) -> &Number {
        match &self.right.get_token().token {
            TokenEnum::Number(number) => {
                return number;
            },

            _ => {
                exit(1);
            }
        }
    }

    fn add(&self) {
        if let Number::Integer(l) = self.get_left() {
            if let Number::Integer(r) = self.get_right()  {
                return println!("{}", l + r);
            }

            return println!("Cannot add Float to Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right()  {
                return println!("{}", l + r);
            }

            return println!("Cannot add Float to Integer");
        };
    }

    fn subtract(&self) {
        if let Number::Integer(l) = self.get_left() {
            if let Number::Integer(r) = self.get_right()  {
                return println!("{}", l - r);
            }

            return println!("Cannot subtract Float from Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right()  {
                return println!("{}", l - r);
            }

            return println!("Cannot subtract Float from Integer");
        };
    }

    fn multiply(&self) {
        if let Number::Integer(l) = self.get_left() {
            if let Number::Integer(r) = self.get_right()  {
                return println!("{}", l * r);
            }

            return println!("Cannot multiply Float with Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right()  {
                return println!("{}", l * r);
            }

            return println!("Cannot multiply Float with Integer");
        };
    }

    fn divide(&self) {
        if let Number::Integer(l) = self.get_left() {
            if let Number::Integer(r) = self.get_right()  {
                return println!("{}", l / r);
            }

            return println!("Cannot divide Float by Integer");
        };

        if let Number::Float(l) = self.get_left() {
            if let Number::Float(r) = self.get_right()  {
                return println!("{}", l / r);
            }

            return println!("Cannot divide Float by Integer");
        };
    }
}

impl AST for BinaryOP {
    fn visit(&mut self) {
        match &self.operator.token {
            TokenEnum::Op(operation) => {
                match operation {
                    Operations::Plus => {
                        self.add();
                    },

                    Operations::Minus => {
                        self.subtract();
                    },

                    Operations::Divide => {
                        self.divide();
                    },

                    Operations::Multiply => {
                        self.multiply();
                    },
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
}
