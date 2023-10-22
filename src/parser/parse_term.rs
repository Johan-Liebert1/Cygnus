use crate::{
    ast::{abstract_syntax_tree::AST, binary_op::BinaryOP},
    tokens::{Operations, TokenEnum}, constants,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// TERM -> FACTOR (*|/) FACTOR
    pub fn parse_term(&mut self) -> Box<dyn AST> {
        let mut result = self.parse_factor();

        loop {
            let next_token = self.peek_next_token();

            if constants::PARSER_DEBUG {
                println!("parse_term next_token {:#?}", next_token);
            }

            match &next_token.token {
                TokenEnum::Op(op) => match op {
                    Operations::Divide | Operations::Multiply => {
                        let token = self.get_next_token();

                        // reassign the result
                        // if we have 1*2*3
                        // in the first iteration, result is (left: 1, op: *, right: 2)
                        // in the next iteration, result is
                        // [left: (left: 1, op: *, right: 2), op: *, right: 3]
                        // and so on
                        result =
                            Box::new(BinaryOP::new(result, Box::new(token), self.parse_factor()));
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
