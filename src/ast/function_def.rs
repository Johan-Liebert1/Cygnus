use crate::{interpreter::interpreter::Variables, lexer::lexer::Token};

use super::abstract_syntax_tree::{VisitResult, AST};

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
    fn visit(&self, i: &mut Variables) -> VisitResult {
        self.block.visit(i)
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}
