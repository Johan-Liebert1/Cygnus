use crate::types::ASTNode;

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::function_call::FunctionCall,
    lexer::tokens::{Bracket, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// FUNCTION_CALL -> VAR_NAME LPAREN (COMPARISON_EXPRESSION)* RPAREN
    pub fn parse_function_call(&mut self, name: String) -> ASTNode {
        // We parse from the LPAREN
        // consume the LPAREN
        self.get_next_token();

        let mut arguments: Vec<ASTNode> = vec![];

        loop {
            let token = self.peek_next_token();

            match &token.token {
                TokenEnum::Bracket(b) => match b {
                    Bracket::RParen => {
                        self.get_next_token();
                        break;
                    }

                    Bracket::LParen => {
                        let factor = self.parse_logical_expression();
                        arguments.push(factor);
                    }

                    _ => panic!("Unexpected token {:#?}", token),
                },

                TokenEnum::Comma => {
                    self.get_next_token();
                    continue;
                }

                _ => {
                    let factor = self.parse_logical_expression();
                    arguments.push(factor);
                }
            };
        }

        return Rc::new(RefCell::new(Box::new(FunctionCall::new(name, arguments))));
    }
}
