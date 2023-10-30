use crate::lexer::lexer::Token;

use super::abstract_syntax_tree::{AST, VisitResult};

#[derive(Debug)]
pub struct FunctionDefinition {
    name: String,
    arguments: Vec<String>,
    block: Box<dyn AST>,
}

impl FunctionDefinition {
    pub fn new(name: String, arguments: Vec<String>, block: Box<dyn AST>) -> Self {
        Self {
            name,
            arguments,
            block,
        }
    }
}

impl AST for FunctionDefinition {
    fn visit(&self) -> VisitResult {
        self.block.visit()
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}
