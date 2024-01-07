use std::{cell::RefCell, rc::Rc};

use crate::{ast::abstract_syntax_tree::AST, interpreter::interpreter::VariableHashMap};

#[derive(Debug)]
pub struct CallStackRecord {
    variables: Rc<RefCell<VariableHashMap>>,
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

    pub fn analyze(&self, variables: &VariableHashMap, function_variables: &VariableHashMap) {
        // trace!("Variables: {:#?}", variables);
        // trace!("Func Variables: {:#?}", function_variables);
    }
}
