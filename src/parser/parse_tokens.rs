use crate::lexer::lexer::Token;

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn peek_next_token(&mut self) -> Token {
        return self.lexer.peek_next_token();
    }

    pub fn get_next_token(&mut self) -> Token {
        return self.lexer.get_next_token();
    }
}
