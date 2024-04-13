use crate::{
    asm::structs, ast::variable::Variable, helpers::compiler_error, lexer::types::VarType,
    parser::parser::UserDefinedType, trace, types::ASTNode,
};

use core::panic;
use std::{cell::RefCell, cmp::min, collections::HashMap, process::exit, rc::Rc, usize};

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
    Function(usize), // stack_var_size
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
    current_offset: usize,
}

impl ActivationRecord {
    pub fn new(name: String, record_type: ActivationRecordType) -> Self {
        Self {
            name,
            record_type,
            variable_members: HashMap::new(),
            var_size_sum: 0,
            current_offset: 0,
        }
    }
}

#[derive(Debug)]
pub struct CallStack<'a> {
    call_stack: Vec<ActivationRecord>,
    current_function_name: Option<String>,
    loop_number: usize,
    pub user_defined_types: &'a Vec<UserDefinedType>,
}

impl<'a> CallStack<'a> {
    pub fn length(&self) -> usize {
        return self.call_stack.len();
    }

    pub fn loop_num(&self) -> usize {
        return self.loop_number;
    }

    fn push_record(&mut self, mut record: ActivationRecord) {
        if let ActivationRecordType::Function(stack_var_size) = record.record_type {
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
                if let ActivationRecordType::Function(..) = record.record_type {
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
            if let ActivationRecordType::Function(..) = record.record_type {
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
        let actual_var_size = var.var_type.get_size_handle_array_and_struct(&var);
        let mut offset = 0;

        for record in self.call_stack.iter_mut().rev() {
            if let ActivationRecordType::Function(stack_var_size) = record.record_type {
                match &self.current_function_name {
                    Some(fname) => {
                        if fname == &record.name {
                            // This means semantic analysis has been finished and we're on the
                            // visiting stage
                            if stack_var_size > 0 {
                                offset = stack_var_size - record.current_offset;

                                trace!(
                                    "funcname: {}, get_size = {}, offset = {}, stack_var_size = {}",
                                    fname,
                                    actual_var_size,
                                    offset,
                                    stack_var_size
                                );

                                if let Some(..) = self.user_defined_types.iter().find(|x| x.type_ == var.var_type) {
                                    if var.member_access.len() != 0 {
                                        record.current_offset += actual_var_size;
                                    }
                                } else {
                                    record.current_offset += actual_var_size;
                                }
                            } else {
                                offset = record.var_size_sum;
                                offset += actual_var_size;
                            }

                            record.var_size_sum += actual_var_size;
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
                match last_record.variable_members.get(var_name) {
                    Some(var) => {
                        compiler_error(
                            format!(
                                "Variable '{}' is already defined on line {}",
                                var_name,
                                var.get_token().line_number
                            ),
                            variable.get_token(),
                        );
                        exit(1);
                    }

                    None => {
                        if let VarType::Struct(_, struct_members) = &mut variable.result_type {
                            if struct_members.borrow().len() == 0 {
                                last_record.variable_members.insert(var_name.into(), variable);
                                return;
                            }

                            // the struct has members
                            struct_members.borrow_mut().first_mut().unwrap().offset = variable.offset;

                            let mut prev_member_offset = variable.offset;
                            let mut prev_member_size = struct_members.borrow().first().unwrap().member_type.get_size();

                            // variable.offset will be equal to the first struct member
                            for member in struct_members.borrow_mut().iter_mut().skip(1) {
                                member.offset = prev_member_offset - prev_member_size;

                                prev_member_offset = member.offset;
                                prev_member_size = member.member_type.get_size();
                            }
                        }

                        last_record.variable_members.insert(var_name.into(), variable)
                    }
                }; // last_record.variable_members.get(var_name) end
            }

            None => {
                panic!("Call stack is empty");
            }
        };
    }

    pub fn get_func_var_stack_size(&self, function_name: &String) -> usize {
        for record in self.call_stack.iter().rev() {
            if let ActivationRecordType::Function(..) = record.record_type {
                if &record.name == function_name {
                    // TODO: Implement a proper fix for this
                    // Since we set the initial offset to 8 (i.e. the largest size a variable can
                    // have) if we have int8 or int16, then the offset goes beyond the record.var_size_sum
                    return if record.var_size_sum >= 8 {
                        record.var_size_sum
                    } else {
                        record.var_size_sum + 8
                    };
                }
            }
        }

        return 0;
    }
}

pub struct SemanticAnalyzer<'a> {
    pub call_stack: CallStack<'a>,
    pub ast: ASTNode,
    pub functions: Rc<RefCell<Functions>>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(ast: ASTNode, functions: Rc<RefCell<Functions>>, user_defined_types: &'a Vec<UserDefinedType>) -> Self {
        Self {
            call_stack: CallStack {
                call_stack: vec![ActivationRecord::new("".into(), ActivationRecordType::Global)],
                current_function_name: None,
                loop_number: 0,
                user_defined_types,
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
