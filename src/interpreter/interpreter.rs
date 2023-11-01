use std::collections::HashMap;

use crate::{
    ast::abstract_syntax_tree::{VisitResult, AST},
    lexer::tokens::Number,
};

pub type Variables = HashMap<String, Number>;

pub struct Interpreter {
    ast: Box<dyn AST>,
    pub variables: Variables,
}

impl Interpreter {
    pub fn new(ast: Box<dyn AST>) -> Self {
        Self {
            ast,
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> VisitResult {
        return self.ast.visit(&mut self.variables);
    }
}
