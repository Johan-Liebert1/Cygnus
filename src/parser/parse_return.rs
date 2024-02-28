use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    ast::jump::{Jump, JumpType},
    helpers::{compiler_error, unexpected_token},
    lexer::{
        lexer::Token,
        tokens::{Bracket, TokenEnum},
    },
    types::ASTNode,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_return_statement(&mut self, current_token: &Token) -> ASTNode {
        if self.inside_function_depth == 0 {
            compiler_error("Found `return` outside of a function", &current_token);
        }

        let peek_next = self.peek_next_token();

        let return_ast_node = match &peek_next.token {
            TokenEnum::Number(..) | TokenEnum::Variable(..) => Some(self.parse_logical_expression()),

            TokenEnum::Bracket(b) => match b {
                Bracket::LParen => Some(self.parse_logical_expression()),

                Bracket::RCurly => {
                    // this is fine as
                    // fun test() { return } is fine
                    // and the next token after return is '}'
                    None
                }

                _ => {
                    unexpected_token(&peek_next, None);
                    exit(1);
                }
            },

            TokenEnum::Bool(_) => todo!(),
            TokenEnum::StringLiteral(_) => todo!(),

            TokenEnum::SemiColon => None,

            _ => {
                unexpected_token(&peek_next, None);
                exit(1);
            }
        };

        let jump_statement: ASTNode = Rc::new(RefCell::new(Box::new(Jump::new(
            JumpType::Return,
            0,
            self.current_function_being_parsed.clone(),
            return_ast_node,
            current_token.clone(),
        ))));

        return jump_statement;
    }
}
