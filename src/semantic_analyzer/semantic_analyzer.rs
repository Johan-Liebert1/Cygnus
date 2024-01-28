use core::panic;
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::abstract_syntax_tree::AST,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::VariableEnum,
};

#[derive(Debug)]
pub enum ActivationRecordType {
    Global,
    Function,
    IfElse,
    Loop,
}

#[derive(Debug)]
pub struct ActivationRecord {
    name: String,
    record_type: ActivationRecordType,
    variable_members: Variables,
}

impl ActivationRecord {
    pub fn new(name: String, record_type: ActivationRecordType) -> Self {
        Self {
            name,
            record_type,
            variable_members: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct CallStack {
    call_stack: Vec<ActivationRecord>,
}

impl CallStack {
    pub fn push(&mut self, record: ActivationRecord) {
        self.call_stack.push(record);
    }

    pub fn pop(&mut self) {
        self.call_stack.pop();
    }

    pub fn var_with_name_found(&self, var_name: &String) -> bool {
        for record in self.call_stack.iter().rev() {
            match record.variable_members.get(var_name) {
                Some(_) => return true,
                None => continue,
            }
        }

        return false;
    }

    pub fn insert_variable(&mut self, var_name: &String, variable_enum: VariableEnum) {
        match self.call_stack.last_mut() {
            Some(last_record) => {
                last_record
                    .variable_members
                    .insert(var_name.into(), variable_enum);
            }

            None => {
                panic!("Call stack is empty");
            }
        }
    }

    pub fn get_func_var_stack_size(&self, function_name: &String) -> usize {
        let mut size = 0;

        for record in self.call_stack.iter().rev() {
            for (_, var_enum) in record.variable_members.iter() {
                size += var_enum.size();
            }

            if let ActivationRecordType::Function = record.record_type {
                if &record.name == function_name {
                    break;
                }
            }
        }

        return size;
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
            call_stack: CallStack {
                call_stack: vec![ActivationRecord::new(
                    "".into(),
                    ActivationRecordType::Global,
                )],
            },
            ast,
            functions,
        }
    }

    pub fn analyze(&mut self) {
        self.ast
            .semantic_visit(&mut self.call_stack, Rc::clone(&self.functions));
    }
}
