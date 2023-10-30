use crate::lexer::lexer::Token;

use super::{abstract_syntax_tree::{AST, VisitResult}, variable::Variable};

#[derive(Debug)]
pub struct AssignmentStatement {
    left: Variable,
    right: Box<dyn AST>,
}

impl AssignmentStatement {
    pub fn new(left: Variable, right: Box<dyn AST>) -> Self {
        Self { left, right }
    }
}

impl AST for AssignmentStatement {
    fn visit(&self) -> VisitResult {
        todo!()
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
