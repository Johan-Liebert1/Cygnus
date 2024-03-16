use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{structs::Struct, variable::Variable},
    helpers::unexpected_token,
    lexer::tokens::{Bracket, TokenEnum},
    trace,
    types::ASTNode,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_struct(&mut self, parse_struct_declaration: bool) -> ASTNode {
        let mut name = String::from("");

        if parse_struct_declaration {
            let next_token = self.get_next_token();

            if let TokenEnum::Variable(var_name) = next_token.token {
                name = var_name;
            } else {
                unexpected_token(&next_token, Some(&TokenEnum::Variable("".into())));
            }
        } else {
            self.validate_token(TokenEnum::Bracket(Bracket::LCurly));
        }

        let mut members: Vec<Variable> = vec![];

        loop {
            if matches!(self.peek_next_token().token, TokenEnum::Bracket(Bracket::RCurly)) {
                break;
            }

            members.push(self.parse_variable());

            if matches!(self.peek_next_token().token, TokenEnum::Comma) {
                self.get_next_token();
            }
        }

        self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

        return Rc::new(RefCell::new(Box::new(Struct::new(name, members))));
    }
}
