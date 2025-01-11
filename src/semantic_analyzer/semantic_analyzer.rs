use crate::{
    ast::{typedef::Typedef, variable::Variable},
    helpers::compiler_error,
    lexer::types::VarType,
    parser::parser::UserDefinedType,
    trace,
    types::ASTNode,
};

use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

use crate::{
    ast::abstract_syntax_tree::AST,
    interpreter::interpreter::Functions,
};

// #[derive(Debug)]
// pub struct ARVariable {
//     pub var: VariableEnum,
//     pub offset: usize,
//     pub times_dereferenced: usize,
// }
//
// type ActivationRecordVariables = HashMap<String, ARVariable>;

pub type ARVariable = Rc<RefCell<Variable>>;
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
    pub variable_members: ActivationRecordVariables,
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
    pub type_aliases: &'a Vec<Typedef>,
}

impl<'a> CallStack<'a> {
    pub fn length(&self) -> usize {
        return self.call_stack.len();
    }

    pub fn loop_num(&self) -> usize {
        return self.loop_number;
    }

    fn push_record(&mut self, record: ActivationRecord) {
        if let ActivationRecordType::Function(..) = record.record_type {
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

    pub fn peek(&self) -> Option<&ActivationRecord> {
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

    pub fn get_var_with_name(&self, var_name: &String) -> (Option<&Rc<RefCell<Variable>>>, &ActivationRecordType) {
        for record in self.call_stack.iter().rev() {
            match record.variable_members.get(var_name) {
                Some(var) => {
                    return (Some(var), &record.record_type);
                }

                None => continue,
            }
        }

        return (None, &ActivationRecordType::Global);
    }

    pub fn insert_variable_in_most_recent_function(&mut self, variable: ActivationRecordVariablesValue) {
        variable.borrow_mut().offset = self.update_function_variable_size_and_get_offset(&variable);

        let immutable_variable = variable.borrow();
        let var_name = &immutable_variable.var_name;

        let mut inserted = false;

        for record in self.call_stack.iter_mut().rev() {
            if let ActivationRecordType::Function(..) = record.record_type {
                inserted = true;

                if record.variable_members.get(var_name).is_some() {
                    panic!("Variable '{}' is already defined", var_name);
                }

                record.variable_members.insert(var_name.into(), Rc::clone(&variable));

                break;
            }
        }

        if !inserted {
            panic!("Could not find function to insert variable");
        }
    }

    // Have to make sure memory alignment is correct
    // start from 8 byte aligned values
    // then to 4 bytes
    // then to 2 bytes
    // then to 1 byte
    fn update_function_variable_size_and_get_offset(&mut self, var: &ARVariable) -> usize {
        let borrowed_var = var.borrow();

        let actual_var_size = borrowed_var.var_type.get_mem_aligned_size(&borrowed_var);
        let var_mem_alignment = borrowed_var.var_type.get_mem_alignment();

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

                                let user_defined_type = self
                                    .user_defined_types
                                    .iter()
                                    .find(|x| x.type_ == var.borrow().var_type);

                                if let Some(..) = user_defined_type {
                                    if borrowed_var.member_access.len() != 0 {
                                        record.current_offset += actual_var_size;
                                    }
                                } else {
                                    record.current_offset += actual_var_size;
                                }
                            } else {
                                offset = record.var_size_sum;
                                offset += actual_var_size;
                            }

                            // make sure every offset is properly aligned
                            if offset % var_mem_alignment != 0 {
                                offset += var_mem_alignment - offset % var_mem_alignment;
                            }

                            record.var_size_sum = offset;
                        }
                    }

                    None => unreachable!(
                        "'self.current_function_name' is None even though there's a Function in the call stack"
                    ),
                }
            }
        }

        // trace!("offset {offset:?}\n\n");

        return offset;
    }

    pub fn insert_variable(&mut self, variable: ActivationRecordVariablesValue) {
        variable.borrow_mut().offset = self.update_function_variable_size_and_get_offset(&variable);

        let var_name = &variable.borrow().var_name;

        match self.call_stack.last_mut() {
            Some(last_record) => {
                match last_record.variable_members.get(var_name) {
                    Some(var) => {
                        compiler_error(
                            format!(
                                "Variable '{}' is already defined on line {}",
                                var_name,
                                var.borrow().get_token().line_number
                            ),
                            variable.borrow().get_token(),
                        );
                    }

                    None => {
                        if let VarType::Struct(_, struct_members) = &variable.borrow().result_type {
                            if struct_members.borrow().len() == 0 {
                                last_record
                                    .variable_members
                                    .insert(var_name.into(), Rc::clone(&variable));
                                return;
                            }

                            // the struct has members
                            let mut prev_member_offset = 0;
                            let mut prev_member_size = 0;

                            // variable.offset will be equal to the first struct member
                            for member in struct_members.borrow_mut().iter_mut() {
                                let var_mem_alignment = member.member_type.get_mem_alignment();

                                member.offset = prev_member_offset + prev_member_size;

                                // make sure every offset is properly aligned
                                if member.offset % var_mem_alignment != 0 {
                                    member.offset += var_mem_alignment - member.offset % var_mem_alignment;
                                }

                                prev_member_size = member.member_type.get_mem_aligned_size(&variable.borrow());
                                prev_member_offset = member.offset;
                            }
                        }

                        last_record
                            .variable_members
                            .insert(var_name.into(), Rc::clone(&variable))
                    }
                }; // last_record.variable_members.get(var_name) end
            }

            None => {
                panic!("Call stack is empty");
            }
        };

        if var_name == "bb" {
            trace!("insert_variable: {:#?}", variable.borrow());
        }
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
    pub fn new(
        ast: ASTNode,
        functions: Rc<RefCell<Functions>>,
        user_defined_types: &'a Vec<UserDefinedType>,
        type_aliases: &'a Vec<Typedef>,
    ) -> Self {
        Self {
            call_stack: CallStack {
                call_stack: vec![ActivationRecord::new("".into(), ActivationRecordType::Global)],
                current_function_name: None,
                loop_number: 0,
                user_defined_types,
                type_aliases,
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
