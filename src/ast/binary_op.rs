use crate::{lexer::Token, tokens::{Operations, TokenEnum, Number}};

use super::abstract_syntax_tree::AST;

#[derive(Debug)]
pub struct BinaryOP {
    left: Box<Token>,
    operator: Box<Token>,
    right: Box<Token>,
}

impl BinaryOP {
    pub fn new(left: Box<Token>, operator: Box<Token>, right: Box<Token>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl AST for BinaryOP {
    fn visit(&mut self) {
        match &self.operator.token {
            TokenEnum::Op(operation) => {
                match operation {
                    Operations::Plus => {
                        println!("{:?}", self.left.token as Number);
                    },

                    Operations::Minus => {
                    },

                    Operations::Divide => {
                    },

                    Operations::Multiply => {
                    },
                }
            },

            _ => {
                unreachable!("Found a non operator in binary expression")
            }
        };
    }
}
