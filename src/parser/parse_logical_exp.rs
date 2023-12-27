use std::{borrow::Borrow, rc::Rc};

use crate::{
    ast::{abstract_syntax_tree::AST, logical_exp::LogicalExpression},
    lexer::tokens::TokenEnum,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// LOGICAL_EXPRESSION -> COMPARISON_EXPRESSION ((and | or) COMPARISON_EXPRESSION)*
    pub fn parse_logical_expression(&mut self) -> Rc<Box<dyn AST>> {
        let left = self.parse_comparison_expression();

        loop {
            let next_token = self.peek_next_token();

            match next_token.token {
                TokenEnum::LogicalOp(..) => {
                    return Rc::new(Box::new(LogicalExpression::new(
                        left,
                        self.get_next_token(),
                        self.parse_logical_expression(),
                    )));
                },

                _ => {
                    return left;
                }
            };
        }

    }
}
