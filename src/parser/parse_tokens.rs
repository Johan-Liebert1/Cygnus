use crate::lexer::{lexer::Token, tokens::TokenEnum};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn peek_next_token(&mut self) -> Token {
        return self.lexer.peek_next_token();
    }

    /// Peeks the nth token from now
    pub fn peek_nth_token(&mut self, n: usize) -> Token {
        let current_index = self.lexer.index;

        let mut token: Token = Token {
            token: TokenEnum::EOF,
            line_number: self.lexer.line_number,
            col_number: self.lexer.col_number,
        };

        for _ in 0..n {
            token = self.lexer.peek_next_token();
            self.lexer.index += 1;
        }

        self.lexer.index = current_index;

        return token;
    }

    pub fn get_next_token(&mut self) -> Token {
        return self.lexer.get_next_token();
    }
}
