use crate::types::ASTNode;

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::binary_op::BinaryOP,
    lexer::tokens::{Operations, TokenEnum},
};

use super::parser::Parser;

impl Parser {
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

                        let term = self.parse_term();

                        // If we have *a + b, term would be a Variable('a') and bracket_stack would
                        // have the same number of elements as before us parsing the term
                        //
                        // if the current bracket_stack_len != the old bracket_stack_len then we
                        // are not dereferencing a Variable but an expression. This is because, if
                        // we were dereferencing a Variable then it'd have been of type *(((((((a)))))))
                        // which makes sure all brackets are properly closed.
                        //
                        // if let ASTNodeEnumMut::Variable(ref mut var) = term.borrow_mut().get_node_mut() {
                        //     if self.bracket_stack.len() == bracket_stack_len {
                        //         trace!("dereferencing the Variable: {:#?}", var.var_name);
                        //         var.times_dereferenced = self.times_dereferenced;
                        //         self.times_dereferenced = 0;
                        //     }
                        // }

                        // reassign the result
                        // if we have 1+2+3
                        // in the first iteration, result is (left: 1, op: +, right: 2)
                        // in the next iteration, result is
                        // [left: (left: 1, op: +, right: 2), op: +, right: 3]
                        // and so on
                        result = Rc::new(RefCell::new(Box::new(BinaryOP::new(
                            result,
                            Box::new(next_token),
                            term,
                            self.times_dereferenced,
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
