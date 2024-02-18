use crate::{ast::variable::Variable, types::ASTNode};

use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

use crate::{
    ast::abstract_syntax_tree::AST,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::VariableEnum,
};

// #[derive(Debug)]
// pub struct ARVariable {
//     pub var: VariableEnum,
//     pub offset: usize,
//     pub times_dereferenced: usize,
// }
//
// type ActivationRecordVariables = HashMap<String, ARVariable>;

pub type ARVariable = Variable;
type ActivationRecordVariablesValue = ARVariable;
type ActivationRecordVariables = HashMap<String, ActivationRecordVariablesValue>;

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
    var_size_sum: usize,
}

impl ActivationRecord {
    pub fn new(name: String, record_type: ActivationRecordType) -> Self {
        Self {
            name,
            record_type,
            variable_members: HashMap::new(),
            var_size_sum: 0,
        }
    }
}

#[derive(Debug)]
pub struct CallStack {
    call_stack: Vec<ActivationRecord>,
    current_function_name: Option<String>,
    loop_number: usize,
}

impl CallStack {
    pub fn length(&self) -> usize {
        return self.call_stack.len();
    }

    pub fn loop_num(&self) -> usize {
        return self.loop_number;
    }

    fn push_record(&mut self, record: ActivationRecord) {
        if let ActivationRecordType::Function = record.record_type {
            self.current_function_name = Some(record.name.clone());
        }

        if let ActivationRecordType::Loop = record.record_type {
            self.loop_number += 1;
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

                if let ActivationRecordType::Loop = record.record_type {
                    self.loop_number -= 1;
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

    pub fn get_var_with_name(&self, var_name: &String) -> (Option<&Variable>, &ActivationRecordType) {
        for record in self.call_stack.iter().rev() {
            match record.variable_members.get(var_name) {
                Some(var) => return (Some(&var), &record.record_type),

                None => continue,
            }
        }

        return (None, &ActivationRecordType::Global);
    }

    pub fn insert_variable_in_most_recent_function(&mut self, mut variable: ActivationRecordVariablesValue) {
        variable.offset = self.update_function_variable_size_and_get_offset(&variable);

        let var_name = &variable.var_name;

        let mut inserted = false;

        for record in self.call_stack.iter_mut().rev() {
            if let ActivationRecordType::Function = record.record_type {
                inserted = true;

                if record.variable_members.get(var_name).is_some() {
                    panic!("Variable '{}' is already defined", var_name);
                }

                record.variable_members.insert(var_name.into(), variable);

                break;
            }
        }

        if !inserted {
            panic!("Could not find function to insert variable");
        }
    }

    fn update_function_variable_size_and_get_offset(&mut self, var: &ARVariable) -> usize {
        let mut offset = 8;

        for record in self.call_stack.iter_mut().rev() {
            if let ActivationRecordType::Function = record.record_type {
                match &self.current_function_name {
                    Some(fname) => {
                        if fname == &record.name {
                            offset += record.var_size_sum;

                            record.var_size_sum += var.size();
                        }
                    }

                    None => unreachable!(
                        "'self.current_function_name' is None even though there's a Function in the call stack"
                    ),
                }
            }
        }

        return offset;
    }

    pub fn insert_variable(&mut self, mut variable: ActivationRecordVariablesValue) {
        variable.offset = self.update_function_variable_size_and_get_offset(&variable);

        let var_name = &variable.var_name;

        match self.call_stack.last_mut() {
            Some(last_record) => {
                if last_record.variable_members.get(var_name).is_some() {
                    panic!("Variable '{}' is already defined", var_name);
                }

                last_record.variable_members.insert(var_name.into(), variable);
            }

            None => {
                panic!("Call stack is empty");
            }
        }
    }

    pub fn get_func_var_stack_size(&self, function_name: &String) -> usize {
        for record in self.call_stack.iter().rev() {
            if let ActivationRecordType::Function = record.record_type {
                if &record.name == function_name {
                    return record.var_size_sum;
                }
            }
        }

        return 0;
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
                call_stack: vec![ActivationRecord::new("".into(), ActivationRecordType::Global)],
                current_function_name: None,
                loop_number: 0,
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
