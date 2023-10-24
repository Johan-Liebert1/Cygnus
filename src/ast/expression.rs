use crate::lexer::lexer::Token;

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Expression {
    left: Box<dyn AST>,
    operand: Box<Token>,
    right: Box<dyn AST>,
}

impl AST for Expression {
    fn visit(&self) -> VisitResult {
        println!("Expression visit");
        VisitResult {
            token: Box::new(self.operand.token.clone()),
        }
    }

    fn get_token(&self) -> &Token {
        return &self.operand;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}

impl Expression {
    pub fn new(left: Box<dyn AST>, operand: Box<Token>, right: Box<dyn AST>) -> Self {
        Expression {
            left,
            operand,
            right,
        }
    }
}
