use crate::lexer::lexer::Token;

use super::parser::Parser;

impl Parser {
    pub fn peek_next_token(&mut self) -> Token {
        return self.lexer.peek_next_token();
    }

    /// Peeks the nth token from now
    pub fn peek_nth_token(&mut self, n: usize) -> Token {
        return self.lexer.peek_nth_token(n);
    }

    pub fn get_next_token(&mut self) -> Token {
        return self.lexer.get_next_token();
    }
}
