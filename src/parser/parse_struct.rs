use crate::{
    lexer::tokens::{Bracket, TokenEnum},
    trace,
    types::ASTNode,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_struct(&mut self) -> ASTNode {
        self.validate_token(TokenEnum::Bracket(Bracket::LCurly));

        loop {
            if matches!(self.peek_next_token().token, TokenEnum::Bracket(Bracket::RCurly)) {
                break;
            }

            let member = self.parse_variable();
            trace!("{:#?}", member);

            if matches!(self.peek_next_token().token, TokenEnum::Comma) {
                self.get_next_token();
            }
        }

        self.validate_token(TokenEnum::Bracket(Bracket::RCurly));
        todo!()
    }
}
