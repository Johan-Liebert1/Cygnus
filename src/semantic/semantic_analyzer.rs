use std::{rc::Rc, cell::RefCell};

use crate::{ast::abstract_syntax_tree::AST, interpreter::interpreter::Variables};

#[derive(Debug)]
pub struct CallStackRecord {
    variables: Rc<RefCell<Variables>>,
}

#[derive(Debug)]
pub struct SemanticAnalyzer {
    call_stack: Vec<CallStackRecord>,
    ast: Rc<Box<dyn AST>>,
}

impl SemanticAnalyzer {
    pub fn new(ast: Rc<Box<dyn AST>>) -> Self {
        Self {
            ast,
            call_stack: vec![],
        }
    }

    pub fn analyze(&self, variables: &Variables, function_variables: &Variables) {
        // trace!("Variables: {:#?}", variables);
        // trace!("Func Variables: {:#?}", function_variables);
    }
}
