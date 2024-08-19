use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{abstract_syntax_tree::ASTNodeEnum, variable::Variable},
    lexer::{
        registers::Register,
        tokens::{AssignmentTypes, VariableEnum},
        types::{StructMemberType, VarType},
    },
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
    trace,
    types::ASTNode,
};

use super::asm::ASM;

impl ASM {
    fn assign_local_string(&mut self, var_offset: usize) {
        let mut instructions = vec![];

        // pop the string pointer into rax
        // the string len should be in rbx as string len is pushed
        // last
        // Move the string length into the mem address above the addr
        // containing the string pointer
        instructions.extend([
            format!("pop rbx"),
            format!("pop rax"),
            format!("mov [rbp - {}], rbx", var_offset - 8),
            format!("mov [rbp - {}], rax", var_offset),
        ]);

        self.extend_current_label(instructions);
    }

    fn assign_local_number(&mut self, var_offset: usize, is_function_call_assign: bool, var_type: &VarType) {
        let reg_name = var_type.get_register_name(Register::RAX);

        let mut instructions = vec![];

        // if it is a function call assignment, then the value is
        // already in rax
        if !is_function_call_assign {
            instructions.extend([
                format!(";; assign_local_number of type {}", var_type),
                format!("xor rax, rax"),
                format!("pop rax"),
            ])
        }

        instructions.push(format!("mov [rbp - {}], {}", var_offset, reg_name));

        self.extend_current_label(instructions);
    }

    fn assign_local_array(
        &mut self,
        var_offset: usize,
        array_access_index: &Option<ASTNode>,
        type_: &Box<VarType>,
        size: &usize,
    ) {
        let mut instructions = vec![];

        match array_access_index {
            Some(index) => {
                // we visit the right side first and then the left
                // side. So the array index is at topand the
                // actual value to set is at the top - 1 of the stack
                instructions.extend([
                    // array[1] = 10

                    // rcx stores the index, rdx has the actual
                    // value
                    format!(";; rbx stores the index, rcx has the actual value"),
                    format!("pop rbx"),                                       // rcx has 1
                    format!("pop rcx"),                                       // rdx has 10
                    format!("mov rax, {}", type_.get_underlying_type_size()), // rax = 8
                    format!("mul rbx"),                                       // now rax has = rax * rbx
                    // = 1 * 8 = 8
                    format!("mov rbx, rbp"),
                    format!("add rbx, rax"),
                    format!("mov [rbx - {}], rcx", var_offset),
                ]);
            }

            None => {
                // Assignment to the array variable itself
                for i in 0..*size {
                    instructions.extend([
                        format!("pop rax"),
                        format!("mov [rbp - {}], rax", var_offset - type_.get_underlying_type_size() * i),
                    ]);
                }
            }
        }

        self.extend_current_label(instructions);
    }

    fn assign_local_pointer(&mut self, var_ptr_type: &Box<VarType>, var_offset: usize, times_dereferenced: usize) {
        let mut instructions = vec![];

        let mut is_ptr_deref = false;

        match *var_ptr_type.clone() {
            VarType::Ptr(ptr_tpye) => self.assign_local_pointer(&ptr_tpye, var_offset, times_dereferenced),

            VarType::Unknown => todo!(),

            // assignment to ptr to a character
            // basically a CStr
            //
            // TODO: Also handle things like
            // def ch: char = 'a';
            // def ch_ptr: *char = &ch;
            t => {
                is_ptr_deref = times_dereferenced > 0;

                // Let's say the following code
                //
                // mem array 1024 --> array starts at addr 500
                // def thing: *int = array + 1; --> thing is at rbp - 8
                // and [rbp - 8] now contains addr 501
                //
                // *thing = 60;
                // derefed thing should contain 60, i.e. [rbp - 8]
                // should not contain 60, but addr 501 should now
                // contain 60
                //
                // we should only deref once which can be done by
                // mov rbx, [rbp - 8]
                // mov [rbx], rax

                instructions.push(format!(";; assign_local_pointer of type {}", t));
                instructions.push(format!("pop rax"));

                if is_ptr_deref {
                    instructions.push(format!("mov rbx, [rbp - {}]", var_offset));
                }

                if times_dereferenced > 1 {
                    instructions.extend(std::iter::repeat(format!("mov rbx, [rbx]")).take(times_dereferenced - 1));
                }

                if is_ptr_deref {
                    instructions.push(format!("mov [rbx], {}", t.get_register_name(Register::RAX)));
                }

                // instructions.extend(vec![format!("pop rbx"), format!("mov rax, rbx")]);
            }
        };

        // This is assignment to the pointer itself not to the value to which the pointer is
        // pointing to
        if !is_ptr_deref {
            instructions.push(format!("mov [rbp - {}], rax", var_offset));
        }

        self.extend_current_label(instructions);
    }

    fn assign_local_struct(
        &mut self,
        struct_offset: usize,
        struct_assign_order: Option<Vec<&String>>,
        struct_name: &String,
        call_stack: &CallStack,
        is_function_call_assign: bool,
    ) {
        if struct_assign_order.is_none() {
            panic!("Need struct_assign_order")
        }

        let var_type = call_stack.user_defined_types.iter().find(|x| x.name == *struct_name);

        if var_type.is_none() {
            unreachable!("Did not find type with name {struct_name} in ASM generator.")
        }

        if let VarType::Struct(_, member_types) = &var_type.unwrap().type_ {
            for order in struct_assign_order.unwrap() {
                // this has to exist
                let borrow = member_types.borrow();
                let member_type = borrow.iter().find(|x| x.name == *order).unwrap();

                match &member_type.member_type {
                    VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => self.assign_local_number(
                        struct_offset - member_type.offset,
                        is_function_call_assign,
                        &member_type.member_type,
                    ),

                    VarType::Float => todo!(),

                    VarType::Str => self.assign_local_string(struct_offset - member_type.offset),

                    // times_dereferenced = 0 as you cannot dereference a struct member while
                    // initializing
                    VarType::Ptr(inner_type) => {
                        self.assign_local_pointer(&inner_type, struct_offset - member_type.offset, 0)
                    }
                    VarType::Array(type_, size) => {
                        self.assign_local_array(struct_offset - member_type.offset, &None, &type_, &size)
                    }

                    VarType::Char => todo!(),
                    VarType::Struct(_, _) => todo!(),
                    VarType::Unknown => todo!(),
                };
            }
        } else {
            unreachable!("Found non struct type for struct")
        }
    }

    fn handle_local_eq_assignment(
        &mut self,
        ar_var: &Rc<RefCell<Variable>>,
        call_stack: &CallStack,
        variable_assigned_to: &Variable,
        struct_assign_order: Option<Vec<&String>>,
        is_function_call_assign: bool,
        times_dereferenced: usize,
        array_access_index: &Option<ASTNode>,
    ) {
        let mut instructions = vec![];
        let mut is_string = false;

        // var = variable from call stack
        match &ar_var.borrow().var_type {
            VarType::Struct(name, members) => {
                // Assignment to the struct variable
                if variable_assigned_to.member_access.len() == 0 {
                    self.assign_local_struct(
                        ar_var.borrow().offset,
                        struct_assign_order,
                        name,
                        call_stack,
                        is_function_call_assign,
                    )
                } else {
                    match &variable_assigned_to.result_type {
                        VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                            let borrowed = members.borrow();

                            let member_offset = borrowed
                                .iter()
                                .find(|x| x.name == variable_assigned_to.member_access[0])
                                .unwrap();

                            self.assign_local_number(
                                member_offset.offset,
                                is_function_call_assign,
                                &variable_assigned_to.result_type,
                            )
                        }

                        v => unimplemented!("Assignment to var_type '{}' inside struct not handled", v),
                    };
                }
            }

            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => self.assign_local_number(
                ar_var.borrow().offset,
                is_function_call_assign,
                &ar_var.borrow().var_type,
            ),

            VarType::Float => todo!(),

            VarType::Str => self.assign_local_string(ar_var.borrow().offset),

            VarType::Char => {
                // TODO: Update this
                //
                // pop the string pointer into rax
                // the string len should be in rbx as string len is pushed
                // last
                // Treat a character as a string with length of 1
                instructions.extend([
                    format!("mov rbx, 1"),
                    format!("pop rax"),
                    format!("mov [rbp - {}], rax", ar_var.borrow().offset),
                ]);

                is_string = true;
            }

            // Assignment to a pointer should be simple enough
            VarType::Ptr(var_ptr_type) => {
                self.assign_local_pointer(var_ptr_type, ar_var.borrow().offset, times_dereferenced)
            }
            VarType::Array(type_, size) => {
                self.assign_local_array(ar_var.borrow().offset, &array_access_index, type_, size)
            }

            VarType::Unknown => todo!(),
        }
    }

    fn handle_global_eq_assignment(
        &mut self,
        ar_var: &Rc<RefCell<Variable>>,
        times_dereferenced: usize,
        instructions: &mut Vec<String>,
    ) {
        let mut is_string = false;

        match &ar_var.borrow().var_type {
            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => instructions.extend([format!("pop rax")]),

            VarType::Struct(_, _) => todo!(),

            VarType::Str => {
                // pop the string pointer into rax
                // the string len should be in rbx as string len is pushed
                // last
                instructions.extend([format!("pop rbx"), format!("pop rax")]);

                is_string = true;
            }

            VarType::Ptr(ptr_var_type) => {
                trace!("{}", ar_var.borrow().var_type);

                match **ptr_var_type {
                    VarType::Struct(_, _) => todo!(),
                    VarType::Int => {
                        // Store whatever's on the top of the stack into
                        // this memory location
                        instructions.extend([format!("pop rax"), format!("mov rbx, {}", ar_var.borrow().var_name)]);
                        instructions.extend(std::iter::repeat(format!("mov rbx, [rbx]")).take(times_dereferenced));

                        instructions.push(format!("mov rbx, rax"));
                    }

                    VarType::Str => todo!(),

                    VarType::Int8 => todo!(),
                    VarType::Int16 => todo!(),
                    VarType::Int32 => todo!(),
                    VarType::Array(..) => todo!(),
                    VarType::Float => todo!(),
                    VarType::Char => todo!(),
                    VarType::Ptr(_) => todo!(),
                    VarType::Unknown => todo!(),
                }
            }

            VarType::Array(..) => todo!(),
            VarType::Float => todo!(),
            VarType::Char => todo!(),
            VarType::Unknown => todo!(),
        }
    }

    /// pops the top most element on the stack and assigns it to the variable
    pub fn variable_assignment(
        &mut self,
        var_name: &String,
        assignment_type: &AssignmentTypes,
        call_stack: &CallStack,
        times_dereferenced: usize,
        is_function_call_assign: bool,
        array_access_index: &Option<ASTNode>,
        struct_assign_order: Option<Vec<&String>>,
        // This is not from the call stack. Only required for structs.
        // TODO: Fix the inconsistency between call_stack variables and actual variables
        variable_assigned_to: &Variable,
    ) {
        // 1. Check whether the variable is a local or global variable
        // 2. If global var, get it from data section, else from stack offset
        let (var_from_call_stack, variable_scope) = call_stack.get_var_with_name(&var_name);

        let mut instructions = vec![];
        let mut is_string = false;

        match var_from_call_stack {
            Some(ar_var) => {
                match variable_scope {
                    ActivationRecordType::Global => {
                        match assignment_type {
                            AssignmentTypes::Equals => {
                                self.handle_global_eq_assignment(ar_var, times_dereferenced, &mut instructions)
                            }

                            AssignmentTypes::PlusEquals => instructions.extend([
                                format!(";; Global PlusEquals {}", var_name),
                                format!("mov rax, [{}]", var_name),
                                format!("pop rbx"),
                                format!("add rax, rbx"),
                            ]),

                            AssignmentTypes::MinusEquals => instructions.extend([
                                format!("mov rax, [{}]", var_name),
                                format!("pop rbx"),
                                format!("sub rax, rbx"),
                            ]),
                        }

                        instructions.push(format!("mov [{}], rax", var_name));

                        self.extend_current_label(instructions);
                    }

                    // local variable
                    _ => match assignment_type {
                        AssignmentTypes::Equals => self.handle_local_eq_assignment(
                            ar_var,
                            call_stack,
                            variable_assigned_to,
                            struct_assign_order,
                            is_function_call_assign,
                            times_dereferenced,
                            array_access_index,
                        ),

                        AssignmentTypes::PlusEquals => self.extend_current_label(
                            [
                                format!("mov rax, [rbp - {}]", ar_var.borrow().offset),
                                format!("pop rbx"),
                                format!("add rax, rbx"),
                                format!("mov [rbp - {}], rax", ar_var.borrow().offset),
                            ]
                            .into(),
                        ),

                        AssignmentTypes::MinusEquals => self.extend_current_label(
                            [
                                format!("mov rax, [rbp - {}]", ar_var.borrow().offset),
                                format!("pop rbx"),
                                format!("sub rax, rbx"),
                                format!("mov [rbp - {}], rax", ar_var.borrow().offset),
                            ]
                            .into(),
                        ),
                    },
                }
            }

            None => unreachable!(
                "Could not find variable with name '{}' in function `factor`. \
            This is a bug in the semantic analying step.",
                var_name
            ),
        };
    }
}
