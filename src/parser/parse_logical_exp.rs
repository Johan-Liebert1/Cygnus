use crate::{lexer::tokens::LogicalOps, trace, types::ASTNode};

use std::{cell::RefCell, rc::Rc};

use crate::{ast::logical_exp::LogicalExpression, lexer::tokens::TokenEnum};

use super::parser::Parser;

impl Parser {
    /// LOGICAL_EXPRESSION -> (not)* COMPARISON_EXPRESSION ((and | or) COMPARISON_EXPRESSION)*
    pub fn parse_logical_expression(&mut self) -> ASTNode {
        let mut num_nots = 0;

        while matches!(self.peek_next_token().token, TokenEnum::LogicalOp(LogicalOps::Not)) {
            let tok = self.get_next_token();
            num_nots %= 2;

            return Rc::new(RefCell::new(Box::new(LogicalExpression::new(
                None,
                tok,
                self.parse_logical_expression(),
            ))));
        }

        let left = self.parse_comparison_expression();

        loop {
            let next_token = self.peek_next_token();

            match next_token.token {
                TokenEnum::LogicalOp(LogicalOps::And) | TokenEnum::LogicalOp(LogicalOps::Or) => {
                    return Rc::new(RefCell::new(Box::new(LogicalExpression::new(
                        Some(left),
                        self.get_next_token(),
                        self.parse_logical_expression(),
                    ))));
                }

                _ => {
                    return left;
                }
            };
        }
    }
}
