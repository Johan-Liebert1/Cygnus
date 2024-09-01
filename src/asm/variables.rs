use core::panic;
use std::{cell::RefCell, fmt::format, rc::Rc, string};

use crate::{
    ast::variable::{self, Variable},
    interpreter::interpreter::Functions,
    lexer::{
        registers::{get_register_name_for_bits, Register},
        tokens::VariableEnum,
        types::{StructMemberType, VarType, TYPE_FLOAT, TYPE_INT},
    },
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
    trace,
};

use super::asm::ASM;

struct RequiredVarFields<'a> {
    dereference: bool,
    store_address: bool,
    times_dereferenced: usize,
    var_name: &'a String,
    member_access: &'a Vec<String>,
}

impl ASM {
    fn handle_local_ptr_int_float(
        &mut self,
        var_type: &Box<VarType>,
        variable: &RequiredVarFields,
        ar_var_offset: usize,
    ) {
        if variable.dereference {
            let rax = self.get_free_register(None);
            let rax_actual_name = var_type.get_register_name(rax);

            let rbx = self.get_free_register(None);

            let mut v = vec![
                format!(";; Dereferencing variable {}", variable.var_name),
                format!("mov {rbx}, [rbp - {}]", ar_var_offset),
            ];

            v.extend(std::iter::repeat(format!("mov {rbx}, [{rbx}]")).take(variable.times_dereferenced));

            v.extend([
                format!("xor {rax}, {rax}"),
                format!("mov {rax_actual_name}, {}", var_type.get_register_name(rbx)),
            ]);

            self.extend_current_label(v);

            // already locked by self.get_free_register
            self.stack_push(String::from(rax));

            self.unlock_register(rbx);

            return;
        }

        if variable.store_address {
            let rax = self.get_free_register(None);
            self.extend_current_label(vec![format!("lea {rax}, [rbp - {}]", ar_var_offset)]);
            // already locked by self.get_free_register
            self.stack_push(String::from(rax));
            return;
        }

        let rax = self.get_free_register(None);
        let rax_actual_name = var_type.get_register_name(rax);

        self.extend_current_label(vec![format!("mov {}, [rbp - {}]", rax_actual_name, ar_var_offset)]);

        // already locked by self.get_free_register
        self.stack_push(String::from(rax));
    }

    fn handle_local_ptr_str(&mut self, var_type: &Box<VarType>, variable: &RequiredVarFields, ar_var_offset: usize) {
        if variable.dereference {
            let rax = self.get_free_register(None);
            let rbx = self.get_free_register(None);

            let mut v = vec![
                format!(";; Dereferencing variable {}. handle_local_ptr_str", variable.var_name),
                format!("mov {rax}, [rbp - {}]", ar_var_offset),
                // now rax contains the address of the pointer to the
                // string
                // now we move the length of the string into rbx
                format!("mov {rbx}, [{rax} - 8]"), // now rbx = length of
                                                   // the string
            ];
            v.extend(std::iter::repeat(format!("mov {rax}, [{rax}]")).take(variable.times_dereferenced));
            v.extend([format!(";; Finish dereferencing variable {}", variable.var_name)]);

            self.stack_push(String::from(rax));
            self.stack_push(String::from(rbx));

            self.extend_current_label(v);
            return;
        }

        let rax = self.get_free_register(None);

        if variable.store_address {
            self.extend_current_label(vec![format!("mov {rax}, [rbp - {}]", ar_var_offset)]);
            self.stack_push(String::from(rax));
            return;
        }

        self.extend_current_label(vec![format!("mov {rax}, [rbp - {}]", ar_var_offset)]);
        self.stack_push(String::from(rax));
    }

    fn handle_local_ptr_struct(
        &mut self,
        var_type: &Box<VarType>,
        variable: &RequiredVarFields,
        ar_var_offset: usize,
        struct_name: &String,
        members: Rc<RefCell<Vec<StructMemberType>>>,
    ) {
        if variable.member_access.len() == 0 {
            let rax = self.get_free_register(None);

            // it's of the type
            // def var: *MyStruct = &another_var;

            let first = &members.borrow()[0];

            self.extend_current_label(vec![
                format!(
                    ";; Storing address of struct {} for variable {} in handle_local_ptr",
                    struct_name, variable.var_name
                ),
                format!("lea {rax}, [rbp - {}]", first.offset),
            ]);

            self.stack_push(String::from(rax));

            return;
        }

        let borrow = members.borrow();
        let found = borrow.iter().find(|x| x.name == variable.member_access[0]);

        self.add_to_current_label(format!(";; Handling ptr to struct for Ptr -> {}", struct_name));

        match found {
            Some(struct_member_type) => match &struct_member_type.member_type {
                // accessing an integer on a ptr to a structure
                // We will dereference it automatically here
                VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                    let rax = self.get_free_register(None);
                    let rbx = self.get_free_register(None);

                    let rax_actual_name = struct_member_type.member_type.get_register_name(rax);

                    println!("struct_member_type: {:#?}", struct_member_type);

                    self.extend_current_label(vec![
                        format!("mov {rbx}, [rbp - {}]", ar_var_offset),
                        format!("add {rbx}, {}", struct_member_type.offset),
                        // Since memeber type is an integer
                        format!("xor {rax}, {rax}"),
                        format!("mov {rax_actual_name}, [{rbx}]"),
                    ]);

                    self.unlock_register(rbx);

                    self.stack_push(String::from(rax));
                }

                VarType::Str => {
                    let rax = self.get_free_register(None);
                    let rbx = self.get_free_register(None);
                    let rcx = self.get_free_register(None);

                    self.extend_current_label(vec![
                        format!("mov {rbx}, [rbp - {}]", ar_var_offset),
                        format!("add {rbx}, {}", struct_member_type.offset),
                        format!("xor {rax}, {rax}"),
                        format!("mov {rax}, [{rbx}]"),
                        // format!("push rax"),
                        // length is pushed last
                        // we add 8 here instead of subtracting 8 because we are
                        // calculating this offset from the beginning of the struct and not
                        // the beginning of the address where the address of the string is
                        // kept
                        format!("mov {rcx}, [{rbx} + 8]"),
                        // format!("push rax"),
                    ]);

                    self.stack_push(String::from(rax));
                    self.stack_push(String::from(rcx));

                    self.unlock_register(rbx);
                }

                VarType::Ptr(var_type) => self.handle_local_ptr(var_type, variable, struct_member_type.offset),

                VarType::Float => todo!(),
                VarType::Char => todo!(),
                VarType::Array(_, _) => todo!(),
                VarType::Struct(_, _) => todo!(),
                VarType::Unknown => todo!(),
                VarType::Function(_, _, _) => todo!(),
            },

            None => unreachable!(
                "Could not find memeber {} of struct while generating ASM",
                variable.member_access[0]
            ),
        }
    }

    fn handle_local_ptr_char(&mut self, var_type: &Box<VarType>, variable: &RequiredVarFields, ar_var_offset: usize) {
        // TODO: Differentiate btw pointer to the first char of a string and a pointer to a
        // single char
        if variable.dereference {
            let rax = self.get_free_register(None);
            let rbx = self.get_free_register(None);

            // mov al as we only want 8 bytes
            self.extend_current_label(vec![
                format!("mov {rbx}, [rbp - {}]", ar_var_offset),
                format!("xor {rax}, {rax}"),
                format!("mov {}, [{rbx}]", get_register_name_for_bits(&rax, 8)),
            ]);

            self.stack_push(String::from(rax));

            self.unlock_register(rbx);

            return;
        }

        if variable.store_address {
            todo!();
            return;
        }

        let rax = self.get_free_register(None);

        self.extend_current_label(vec![format!("mov {rax}, [rbp - {}]", ar_var_offset)]);
        self.stack_push(String::from(rax));
    }

    fn handle_local_ptr(&mut self, var_type: &Box<VarType>, variable: &RequiredVarFields, ar_var_offset: usize) {
        match *var_type.clone() {
            VarType::Int | VarType::Int16 | VarType::Int32 | VarType::Int8 | VarType::Float => {
                self.handle_local_ptr_int_float(var_type, variable, ar_var_offset)
            }

            VarType::Str => self.handle_local_ptr_str(var_type, variable, ar_var_offset),

            VarType::Char => self.handle_local_ptr_char(var_type, variable, ar_var_offset),

            VarType::Struct(struct_name, members) => {
                self.handle_local_ptr_struct(var_type, variable, ar_var_offset, &struct_name, members)
            }

            type_ => {
                todo!("var_type '{type_}' not handled")
            }
        }
    }

    fn handle_asm_for_array(&mut self, var_type: &Box<VarType>, variable: &Variable, ar_var: &Variable) {
        if variable.array_aceess_index.is_none() {
            let rax = self.get_free_register(None);

            // if it's just printing the array, then print the address
            self.extend_current_label(vec![format!("lea {rax}, [rbp - {}]", ar_var.offset)]);

            self.stack_push(String::from(rax));
            return;
        }

        match *var_type.clone() {
            VarType::Int => {
                let mut instructions = vec![];

                let regs_to_skip = vec![Register::RDX];

                let rax = if self.is_reg_locked(Register::RAX) {
                    let rbx = self.get_free_register(Some(&regs_to_skip));

                    trace!("RAX was locked so rbx = {rbx}. Used registers: {:#?}", self.get_used_registers());

                    instructions.extend(vec![format!("mov {rbx}, rax")]);

                    self.replace_reg_on_stack(Register::RAX, rbx);

                    self.unlock_register(Register::RAX);

                    self.get_free_register(Some(&regs_to_skip))
                } else {
                    self.get_free_register(Some(&regs_to_skip))
                };

                // we have to get this after we update the stack otherwise we're working with stale
                // data
                let index = self.stack_pop().unwrap();

                let rbx = self.get_free_register(None);

                trace!("used regs: {:#?}", self.get_used_registers());
                trace!("index: {:#?}", index);
                trace!("rax: {:#?}", rax);

                instructions.extend(vec![
                    format!(";; Start array index access"),
                    // rax has the index into the array
                    format!("mov {rax}, {} ;; move index into {rax}", index),
                    format!("mov {rbx}, {} ;; move var size into {rbx}", variable.result_type.get_underlying_type_size()),
                    format!("mul {rbx} ;; now rax = rbx * rax = index * var_size"),
                    // now rax has index * 8
                    format!("mov {rbx}, rbp"),
                    format!("add {rbx}, {rax}"),
                    format!("mov {rax}, [{rbx} - {}]", ar_var.offset),
                ]);

                self.unlock_register_from_stack_value(&index);

                self.extend_current_label(instructions);

                self.unlock_register(rbx);

                self.stack_push(String::from(rax));
            }

            VarType::Int8 => todo!(),
            VarType::Int16 => todo!(),
            VarType::Int32 => todo!(),

            VarType::Float => todo!(),
            VarType::Struct(_, _) => todo!(),

            VarType::Str => todo!(),
            VarType::Char => todo!(),
            VarType::Ptr(_) => todo!(),
            VarType::Array(_, _) => todo!(),
            VarType::Unknown => todo!(),
            VarType::Function(_, _, _) => todo!(),
        }
    }

    // need the var_type for struct member access as struct_name.member as the type 'struct_name'
    fn handle_local_int_float(&mut self, variable: &Variable, ar_var_offset: usize, actual_var_type: &VarType) {
        if variable.dereference {
            panic!("Cannot dereference a number");
            return;
        }

        if variable.store_address {
            let rax = self.get_free_register(None);

            self.extend_current_label(vec![
                format!("lea {rax}, [rbp - {}]", ar_var_offset), // format!("push rax")
            ]);

            self.stack_push(String::from(rax));
            return;
        }

        self.stack_push(format!("[rbp - {}]", ar_var_offset));
    }

    fn handle_local_str(&mut self, variable: &Variable, ar_var_offset: usize) {
        if variable.dereference {
            let rax = self.get_free_register(None);

            let mut v = vec![
                format!(";; Dereferencing variable {}. handle_local_str", variable.var_name),
                format!("mov {rax}, [rbp - {}]", ar_var_offset),
                // now rax contains the address of the pointer to the
                // string
                // now we move the length of the string into rbx
                // format!("mov rbx, 1"), // now rbx = length of
                // the string
                //
                // NOTE: Not doing the above as a string derefed should only be the first character
            ];

            v.extend(std::iter::repeat(format!("mov {rax}, [{rax}]")).take(variable.times_dereferenced - 1));

            self.stack_push(String::from(rax));

            self.extend_current_label(v);
            return;
        }

        if variable.store_address {
            let rax = self.get_free_register(None);

            // the pointer to the string is stored below the length
            // --- top of stack ---
            // .
            // .
            // --- length ---
            // --- pointer to string ---
            self.extend_current_label(vec![
                format!("lea {rax}, [rbp - {}]", ar_var_offset), //  format!("push rax")
            ]);
            self.stack_push(String::from(rax));
            return;
        }

        self.stack_extend(vec![
            format!("[rbp - {}]", ar_var_offset),
            // length is pushed last
            format!("[rbp - {}]", ar_var_offset - 8),
        ]);
    }

    fn handle_global_ptr(&mut self, variable: &Variable, ar_var: &Variable) {
        let var_name = &variable.var_name;

        if ar_var.is_memory_block {
            // this will be in the bss section
            if variable.dereference {
                let rax = self.get_free_register(None);
                let rax_actual_name = variable.var_type.get_pointer_type().get_register_name(rax);

                let rbx = self.get_free_register(None);

                let mut v = vec![
                    format!(";; Dereferencing variable {}. handle_global_ptr", var_name),
                    format!("mov {rbx}, {}", var_name),
                ];

                v.extend(std::iter::repeat(format!("mov {rbx}, [{rbx}]")).take(variable.times_dereferenced));

                v.extend([
                    format!("xor {rax}, {rax}"),
                    format!(
                        "mov {rax_actual_name}, {}",
                        variable.var_type.get_pointer_type().get_register_name(rbx)
                    ),
                    // format!("push rax"),
                    format!(";; Finish dereferencing variable {}", var_name),
                ]);

                self.stack_push(String::from(rax));

                self.unlock_register(rbx);

                self.extend_current_label(v);
                return;
            }

            if variable.store_address {
                let rax = self.get_free_register(None);

                self.extend_current_label(vec![format!("lea {rax}, {}", var_name)]);

                self.stack_push(String::from(rax));

                return;
            }

            trace!("Global ptr outside {var_name}: {:#?}", self.get_used_registers());

            let rax = self.get_free_register(None);
            let rax_actual_name = variable.var_type.get_pointer_type().get_register_name(rax);

            self.extend_current_label(vec![
                format!("xor {rax}, {rax}"),
                format!("mov {rax_actual_name}, {}", var_name),
            ]);

            self.stack_push(String::from(rax));

            return;
        }

        todo!("not implemented for non memory block. {variable:#?}")
    }

    fn gen_asm_for_var_global_scope(&mut self, variable: &Variable, ar_var: &Rc<RefCell<Variable>>) {
        let var_name = &variable.var_name;

        match variable.var_type {
            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                let rax = self.get_free_register(None);
                let rax_actual_name = variable.var_type.get_register_name(rax);

                if variable.dereference {
                    panic!("Cannot dereference a number")
                } else if variable.store_address {
                    self.add_to_current_label(format!("lea {rax}, {var_name}"));
                    self.stack_push(format!("{rax}"));
                } else {
                    self.add_to_current_label(format!("mov {rax_actual_name}, [{var_name}]"));
                    self.stack_push(format!("{rax}"));
                }
            }

            VarType::Str => {
                let rax = self.get_free_register(None);

                if variable.dereference {
                    panic!("Cannot dereference a string")
                } else if variable.store_address {
                    todo!()
                } else {
                    todo!("need to get the string length as well");
                    self.add_to_current_label(format!("mov {rax}, [{var_name}]"));
                    self.stack_push(String::from(rax));
                }
            }

            VarType::Float => todo!(),
            VarType::Char => todo!(),

            VarType::Int8 => todo!(),
            VarType::Int16 => todo!(),
            VarType::Int32 => todo!(),

            VarType::Ptr(_) => self.handle_global_ptr(variable, &ar_var.borrow()),

            VarType::Array(..) => todo!(),
            VarType::Unknown => todo!(),
            VarType::Struct(_, _) => todo!(),
            VarType::Function(_, _, _) => todo!(),
        }
    }

    fn handle_local_function_pointer(&mut self, variable: &Variable, ar_var: &Rc<RefCell<Variable>>) {
        let rax = self.get_free_register(None);

        self.extend_current_label(vec![format!("mov {rax}, [rbp - {}]", ar_var.borrow().offset)]);

        self.stack_push(String::from(rax));
    }

    fn gen_asm_for_var_local_scope(&mut self, variable: &Variable, ar_var: &Rc<RefCell<Variable>>) {
        let required_var = RequiredVarFields {
            dereference: variable.dereference,
            times_dereferenced: variable.times_dereferenced,
            member_access: &variable.member_access,
            var_name: &variable.var_name,
            store_address: variable.store_address,
        };

        // cannot use ar_var here as it does not have the computed types
        match &variable.var_type {
            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Float => {
                self.handle_local_int_float(variable, ar_var.borrow().offset, &variable.var_type)
            }

            VarType::Str => self.handle_local_str(variable, ar_var.borrow().offset),

            // TODO: Handle pointer to pointer to something
            VarType::Ptr(var_type) => self.handle_local_ptr(var_type, &required_var, ar_var.borrow().offset),

            VarType::Char => todo!(),

            VarType::Array(var_type, _) => self.handle_asm_for_array(var_type, variable, &ar_var.borrow()),

            VarType::Struct(struct_name, member_access) => {
                let first = &member_access.borrow()[0];

                // 'variable' has the member access properties
                // 'variable_from_stack' has the offset
                // it's the entire struct
                // print the memory address
                if variable.member_access.len() == 0 {
                    let rax = self.get_free_register(None);

                    self.extend_current_label(vec![
                        format!(
                            ";; Storing address of struct {} for variable {} not in handle_local_ptr",
                            struct_name, variable.var_name
                        ),
                        format!("lea {rax}, [rbp - {}]", ar_var.borrow().offset + first.offset),
                    ]);

                    self.stack_push(String::from(rax));

                    return;
                }

                let offset = first;

                // TODO: handle struct inside struct here
                let borrow = member_access.borrow();
                let found = borrow.iter().find(|x| x.name == variable.member_access[0]);

                match found {
                    Some(struct_member_type) => match &struct_member_type.member_type {
                        VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Float => self
                            .handle_local_int_float(
                                variable,
                                ar_var.borrow().offset - struct_member_type.offset,
                                &struct_member_type.member_type,
                            ),

                        VarType::Str => {
                            self.handle_local_str(variable, ar_var.borrow().offset - struct_member_type.offset)
                        }

                        VarType::Ptr(var_type) => self.handle_local_ptr(
                            var_type,
                            &required_var,
                            ar_var.borrow().offset - struct_member_type.offset,
                        ),

                        VarType::Char => todo!(),

                        VarType::Array(_, _) => todo!(),
                        VarType::Struct(_, _) => todo!(),
                        VarType::Unknown => todo!(),
                        VarType::Function(_, _, _) => todo!(),
                    },

                    None => unreachable!(
                        "Could not find memeber {} of struct while generating ASM",
                        variable.member_access[0]
                    ),
                }
            }

            VarType::Function(_, _, _) => {
                self.handle_local_function_pointer(variable, ar_var);
            }

            VarType::Unknown => todo!(),
        }
    }

    pub fn gen_asm_for_var(&mut self, variable: &Variable, functions: Rc<RefCell<Functions>>, call_stack: &CallStack) {
        let var_name = &variable.var_name;

        let (variable_from_stack, variable_scope) = call_stack.get_var_with_name(var_name);

        match variable_from_stack {
            Some(ar_var) => {
                match variable_scope {
                    ActivationRecordType::Global => self.gen_asm_for_var_global_scope(&variable, ar_var), // global scope end
                    _ => self.gen_asm_for_var_local_scope(&variable, ar_var),
                }
            }

            // Might be a function pointer
            None => match functions.borrow().get(var_name) {
                Some(..) => {
                    // var name is the name of the function
                    // TODO: This won't work if we are passing around this pointer

                    let rax = self.get_free_register(None);

                    self.extend_current_label(vec![
                        format!(";; Function pointer {var_name}"),
                        format!("mov {rax}, _{var_name}"),
                    ]);

                    self.stack_push(String::from(rax));
                }

                None => unreachable!(
                    "Could not find variable with name '{}' in function `variable_declaration`. \
                        This is a bug in the semantic analying step.",
                    var_name,
                ),
            },
        };
    }
}
