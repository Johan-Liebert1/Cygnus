use crate::{
    lexer::{keywords::AS, tokens::LogicalOps},
    types::{ASTNode, TypeCast},
};

use std::{cell::RefCell, rc::Rc};

use crate::{ast::logical_exp::LogicalExpression, lexer::tokens::TokenEnum};

use super::parser::Parser;

impl Parser {
    pub fn parse_type_cast(&mut self) -> TypeCast {
        let peeked_token = self.peek_next_token();

        // trace!("in parse_type_cast peeked_token: {peeked_token:#?}");
        // let backtrace = std::backtrace::Backtrace::capture();
        // println!("{:#?}", backtrace);
        // trace!("\n\n\n\n");

        if !matches!(&peeked_token.token, TokenEnum::Keyword(keyword) if keyword == AS) {
            return None;
        }

        // consume 'as'
        self.consume_token();

        let var_type = self.parse_var_type();

        return Some(var_type);
    }

    /// LOGICAL_EXPRESSION -> (not)* COMPARISON_EXPRESSION ((and | or) COMPARISON_EXPRESSION)* (as TYPE)*
    pub fn parse_logical_expression(&mut self) -> ASTNode {
        while matches!(self.peek_next_token().token, TokenEnum::LogicalOp(LogicalOps::Not)) {
            let tok = self.consume_token();

            return Rc::new(RefCell::new(Box::new(LogicalExpression::new(
                None,
                tok,
                self.parse_logical_expression(),
                self.parse_type_cast(),
            ))));
        }

        let left = self.parse_comparison_expression();

        loop {
            let next_token = self.peek_next_token();

            match next_token.token {
                TokenEnum::LogicalOp(LogicalOps::And) | TokenEnum::LogicalOp(LogicalOps::Or) => {
                    let type_cast = self.parse_type_cast();

                    return Rc::new(RefCell::new(Box::new(LogicalExpression::new(
                        Some(left),
                        self.consume_token(),
                        self.parse_logical_expression(),
                        type_cast,
                    ))));
                }

                _ => {
                    return left;
                }
            };
        }
    }
}
