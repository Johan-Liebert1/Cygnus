use crate::{lexer::lexer::Token, interpreter::interpreter::Variables};

use super::{abstract_syntax_tree::{VisitResult, AST}, variable::Variable};

#[derive(Debug)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Variable>,
    block: Box<dyn AST>,
}

impl FunctionDefinition {
    pub fn new(name: String, arguments: Vec<Variable>, block: Box<dyn AST>) -> Self {
        Self {
            name,
            parameters: arguments,
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
