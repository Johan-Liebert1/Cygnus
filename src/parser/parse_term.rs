use crate::types::ASTNode;

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::binary_op::BinaryOP,
    lexer::tokens::{Operations, TokenEnum},
};

use super::parser::Parser;

impl Parser {
    /// TERM -> FACTOR (( * | /  | << | >> | % ) FACTOR)*
    pub fn parse_term(&mut self) -> ASTNode {
        let mut result = self.parse_factor();

        // trace!("parse_term result = {:?}", result);

        loop {
            let next_token = self.peek_next_token();

            match &next_token.token {
                TokenEnum::Op(op) => match op {
                    Operations::Divide
                    | Operations::Multiply
                    | Operations::ShiftLeft
                    | Operations::ShiftRight
                    | Operations::Modulo => {
                        let token = self.get_next_token();

                        // let type_cast = self.parse_type_cast();

                        // reassign the result
                        // if we have 1*2*3
                        // in the first iteration, result is (left: 1, op: *, right: 2)
                        // in the next iteration, result is
                        // [left: (left: 1, op: *, right: 2), op: *, right: 3]
                        // and so on
                        result = Rc::new(RefCell::new(Box::new(BinaryOP::new(
                            result,
                            token,
                            self.parse_factor(),
                            // multiplying a pointer or dividing it or shifting left/right doesn't
                            // make any sense
                            0,
                        ))));
                    }

                    _ => {
                        return result;
                    }
                },

                _ => {
                    return result;
                }
            }
        }
    }
}
