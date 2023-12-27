use std::rc::Rc;

use crate::{
    ast::{abstract_syntax_tree::AST, function_call::FunctionCall},
    lexer::tokens::{Bracket, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// FUNCTION_CALL -> VAR_NAME LPAREN (COMPARISON_EXPRESSION)* RPAREN
    pub fn parse_function_call(&mut self, name: String) -> Rc<Box<dyn AST>> {
        // We parse from the LPAREN
        // consume the LPAREN
        self.get_next_token();

        let mut arguments: Vec<Rc<Box<dyn AST>>> = vec![];

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

        return Rc::new(Box::new(FunctionCall::new(name, arguments)));
    }
}
