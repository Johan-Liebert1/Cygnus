use crate::lexer::Token;

use super::abstract_syntax_tree::AST;

pub struct Expression {
    left: Box<dyn AST>,
    operand: Box<Token>,
    right: Box<dyn AST>,
}

impl AST for Expression {
    fn visit(&mut self) {
        println!("Expression visit")
    }

    fn get_token(&self) -> &Token {
        return &self.operand;
    }
}

impl Expression {
    pub fn new(left: Box<dyn AST>, operand: Box<Token>, right: Box<dyn AST>) -> Self {
        Expression { left, operand, right }
    }
}
