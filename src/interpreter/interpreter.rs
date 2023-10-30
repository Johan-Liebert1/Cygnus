use std::collections::HashMap;

use crate::{ast::abstract_syntax_tree::{AST, VisitResult}, lexer::tokens::Number};

pub struct Interpreter {
    ast: Box<dyn AST>,
    variables: HashMap<String, Number>
}

impl Interpreter {
    pub fn new(ast: Box<dyn AST>) -> Self {
        Self {
            ast,
            variables: HashMap::new()
        }
    }

    pub fn interpret(&self) -> VisitResult {
        return self.ast.visit();
    }
}
