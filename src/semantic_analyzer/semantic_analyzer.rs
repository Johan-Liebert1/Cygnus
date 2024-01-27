use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::abstract_syntax_tree::AST,
    interpreter::interpreter::{Functions, Variables},
};

pub enum ActivationRecordType {
    Function,
    IfElse,
    Loop,
}

pub struct ActivationRecord {
    name: String,
    record_type: ActivationRecordType,
    members: Variables,
}

impl ActivationRecord {
    pub fn new(name: String, record_type: ActivationRecordType) -> Self {
        Self {
            name,
            record_type,
            members: HashMap::new(),
        }
    }
}

pub struct CallStack {
    call_stack: Vec<ActivationRecord>,
}

impl CallStack {
    pub fn insert_record(&mut self, record: ActivationRecord) {
        self.call_stack.push(record);
    }

    pub fn var_with_name_found(&self, var_name: &String) -> bool {
        for record in self.call_stack.iter().rev() {
            match record.members.get(var_name) {
                Some(_) => return true,
                None => continue,
            }
        }

        return false;
    }

    pub fn insert_member(&mut self) {
    }
}

pub struct SemanticAnalyzer {
    pub call_stack: CallStack,
    pub ast: Rc<Box<dyn AST>>,
    pub functions: Rc<RefCell<Functions>>,
}

impl SemanticAnalyzer {
    pub fn new(ast: Rc<Box<dyn AST>>, functions: Rc<RefCell<Functions>>) -> Self {
        Self {
            call_stack: CallStack { call_stack: vec![] },
            ast,
            functions,
        }
    }

    pub fn analyze(&mut self) {
        self.ast
            .semantic_visit(&mut self.call_stack, Rc::clone(&self.functions));
    }
}
