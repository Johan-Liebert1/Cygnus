use crate::lexer::lexer::Token;

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Variable {
    token: Box<Token>,
}

impl Variable {
    pub fn new(token: Box<Token>) -> Self {
        Self { token }
    }
}

impl AST for Variable {
    fn visit(&self) -> VisitResult {
        todo!()
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
