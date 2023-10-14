use crate::lexer::Token;

use super::abstract_syntax_tree::AST;

/// FACTOR -> INTEGER | FLOAT
pub struct Factor {
    token: Box<Token>,
}

impl Factor {
    pub fn new(token: Box<Token>) -> Self {
        Self { token }
    }

}

impl AST for Factor {
    /// no implementation for this
    fn visit(&mut self) {
        ()
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }
}
