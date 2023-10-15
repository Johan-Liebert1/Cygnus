use crate::lexer::Token;

use super::abstract_syntax_tree::{VisitResult, AST};

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
    fn visit(&self) -> VisitResult {
        VisitResult {
            token: Box::new(self.token.token.clone()),
        }
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("Factor: {:?}", self.get_token());
    }
}
