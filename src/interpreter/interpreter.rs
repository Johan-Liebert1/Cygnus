use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{abstract_syntax_tree::{VisitResult, AST}, function_def::FunctionDefinition},
    lexer::tokens::Number,
};

pub type Variables = HashMap<String, Number>;
pub type Functions = HashMap<String, Rc<FunctionDefinition>>;

pub struct Interpreter{
    ast: Box<dyn AST>,
    pub variables: Variables,
    pub functions: Rc<RefCell<Functions>>,
}

impl Interpreter{
    pub fn new(ast: Box<dyn AST>) -> Self {
        Self {
            ast,
            variables: HashMap::new(),
            functions: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn interpret(&mut self) -> VisitResult {
        return self
            .ast
            .visit(&mut self.variables, Rc::clone(&self.functions));
    }
}

fn x(f: Rc<RefCell<HashMap<i32, i32>>>) {
    let mut c = f.borrow_mut();
    c.insert(1, 2);
}
