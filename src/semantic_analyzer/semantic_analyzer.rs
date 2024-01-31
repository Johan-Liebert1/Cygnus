use crate::types::ASTNode;

use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

use crate::{
    ast::abstract_syntax_tree::AST,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::VariableEnum,
};

#[derive(Debug)]
pub struct ARVariable {
    pub var: VariableEnum,
    pub offset: usize,
}

type ActivationRecordVariables = HashMap<String, ARVariable>;

#[derive(Debug)]
pub enum ActivationRecordType {
    Global,
    Function,
    IfElse,
    Loop,
}

#[derive(Debug)]
pub enum PopTypes {
    EarlyReturn,
    LoopBreak,
}

#[derive(Debug)]
pub struct ActivationRecord {
    name: String,
    record_type: ActivationRecordType,
    variable_members: ActivationRecordVariables,
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
    current_function_name: Option<String>,
}

impl CallStack {
    pub fn length(&self) -> usize {
        return self.call_stack.len();
    }

    pub fn push_record(&mut self, record: ActivationRecord) {
        if let ActivationRecordType::Function = record.record_type {
            self.current_function_name = Some(record.name.clone());
        }

        self.call_stack.push(record);
    }

    pub fn push(&mut self, name: String, record_type: ActivationRecordType) {
        self.push_record(ActivationRecord::new(name, record_type));
    }

    pub fn pop(&mut self) {
        match self.call_stack.pop() {
            Some(record) => {
                if let ActivationRecordType::Function = record.record_type {
                    self.current_function_name = None;
                }
            }

            None => panic!("Pop from empty stack"),
        };
    }

    pub fn peek(&mut self) -> Option<&ActivationRecord> {
        self.call_stack.last()
    }

    pub fn pop_special(&mut self, pop_type: PopTypes) {
        let mut index_to_slice_from: Option<usize> = None;

        for (index, record) in self.call_stack.iter().enumerate().rev() {
            match record.record_type {
                ActivationRecordType::Function => {
                    if let PopTypes::EarlyReturn = pop_type {
                        // adding 1 here as at the end of the stack we'll pop the function
                        // stack
                        index_to_slice_from = Some(index + 1);
                        break;
                    }
                }

                ActivationRecordType::Loop => {
                    if let PopTypes::LoopBreak = pop_type {
                        index_to_slice_from = Some(index + 1);
                        break;
                    }
                }

                _ => continue,
            }
        }

        match index_to_slice_from {
            Some(index_to_slice_from) => {
                self.call_stack.drain(index_to_slice_from..);
            }

            None => panic!("Nothing to pop"),
        }
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

    pub fn get_var_with_name(
        &self,
        var_name: &String,
    ) -> (Option<&ARVariable>, &ActivationRecordType) {
        for record in self.call_stack.iter().rev() {
            match record.variable_members.get(var_name) {
                Some(var) => return (Some(var), &record.record_type),
                None => continue,
            }
        }

        return (None, &ActivationRecordType::Global);
    }

    pub fn insert_variable_in_most_recent_function(&mut self, var_name: &String, variable_enum: VariableEnum) {
        let mut offset = 8;

        if let Some(function_name) = &self.current_function_name {
            offset = self.get_func_var_stack_size(function_name);
        }

        let mut inserted = false;

        for record in self.call_stack.iter_mut().rev() {
            if let ActivationRecordType::Function = record.record_type {
                inserted = true;

                if record.variable_members.get(var_name).is_some() {
                    panic!("Variable '{}' is already defined", var_name);
                }

                record.variable_members.insert(
                    var_name.into(),
                    ARVariable {
                        var: variable_enum,
                        offset,
                    },
                );

                break;
            }
        }

        if !inserted {
            panic!("Could not find function to insert variable");
        }
    }

    pub fn insert_variable(&mut self, var_name: &String, variable_enum: VariableEnum) {
        let mut offset = 8;

        if let Some(function_name) = &self.current_function_name {
            offset = self.get_func_var_stack_size(function_name);
        }

        match self.call_stack.last_mut() {
            Some(last_record) => {
                if last_record.variable_members.get(var_name).is_some() {
                    panic!("Variable '{}' is already defined", var_name);
                }

                last_record.variable_members.insert(
                    var_name.into(),
                    ARVariable {
                        var: variable_enum,
                        offset,
                    },
                );
            }

            None => {
                panic!("Call stack is empty");
            }
        }
    }

    pub fn get_func_var_stack_size(&self, function_name: &String) -> usize {
        let mut size = 0;

        for record in self.call_stack.iter().rev() {
            for (_, ar_var) in record.variable_members.iter() {
                size += ar_var.var.size();
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
    pub ast: ASTNode,
    pub functions: Rc<RefCell<Functions>>,
}

impl SemanticAnalyzer {
    pub fn new(ast: ASTNode, functions: Rc<RefCell<Functions>>) -> Self {
        Self {
            call_stack: CallStack {
                call_stack: vec![ActivationRecord::new(
                    "".into(),
                    ActivationRecordType::Global,
                )],
                current_function_name: None,
            },
            ast,
            functions,
        }
    }

    pub fn analyze(&mut self) {
        self.ast
            .borrow_mut()
            .semantic_visit(&mut self.call_stack, Rc::clone(&self.functions));
    }
}
