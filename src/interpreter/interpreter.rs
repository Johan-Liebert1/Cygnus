use std::collections::HashMap;

use crate::{
    ast::abstract_syntax_tree::{VisitResult, AST},
    lexer::tokens::Number,
};

pub type Variables = HashMap<String, Number>;
pub type Functions = HashMap<String, Box<dyn AST>>;

pub struct Interpreter {
    ast: Box<dyn AST>,
    pub variables: Variables,
    pub functions: Functions,
}

impl Interpreter {
    pub fn new(ast: Box<dyn AST>) -> Self {
        Self {
            ast,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> VisitResult {
        return self.ast.visit(&mut self.variables, &mut self.functions);
    }
}
