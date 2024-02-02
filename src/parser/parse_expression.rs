use crate::types::ASTNode;

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::binary_op::BinaryOP,
    lexer::tokens::{Operations, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// EXPRESSION -> BINARY_OP (+|-) BINARY_OP
    /// for precedence as term will be calculated first
    pub fn parse_expression(&mut self) -> ASTNode {
        let mut result = self.parse_term();

        loop {
            let next_token = self.peek_next_token();

            match &next_token.token {
                TokenEnum::Op(op) => match op {
                    Operations::Plus | Operations::Minus => {
                        self.get_next_token();

                        // reassign the result
                        // if we have 1+2+3
                        // in the first iteration, result is (left: 1, op: +, right: 2)
                        // in the next iteration, result is
                        // [left: (left: 1, op: +, right: 2), op: +, right: 3]
                        // and so on
                        result = Rc::new(RefCell::new(Box::new(BinaryOP::new(
                            result,
                            Box::new(next_token),
                            self.parse_term(),
                        ))));
                    }

                    _ => {
                        return result;
                    }
                },

                _ => {
                    return result;
                }
            };
        }
    }
}
