use crate::{interpreter::interpreter::Variables, lexer::lexer::Token};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Variable {
    token: Box<Token>,
    pub var_name: String,
    pub var_type: String,
}

impl Variable {
    pub fn new(token: Box<Token>, var_type: String, var_name: String) -> Self {
        Self {
            token,
            var_type,
            var_name,
        }
    }
}

impl AST for Variable {
    fn visit(&self, _: &mut Variables) -> VisitResult {
        todo!()
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
