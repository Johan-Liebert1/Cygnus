use core::panic;

use crate::{
    ast::variable::{self, Variable},
    lexer::{
        registers::Register,
        tokens::VariableEnum,
        types::{VarType, TYPE_FLOAT, TYPE_INT},
    },
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
    trace,
};

use super::asm::ASM;

impl ASM {
    fn handle_local_ptr(&mut self, var_type: &Box<VarType>, variable: &Variable, ar_var_offset: usize) {
        match *var_type.clone() {
            VarType::Int | VarType::Int16 | VarType::Int32 | VarType::Int8 | VarType::Float => {
                let reg_name = var_type.get_register_name(Register::RAX);

                if variable.dereference {
                    let mut v = vec![
                        format!(";; Dereferencing variable {}", variable.var_name),
                        format!("mov rax, [rbp - {}]", ar_var_offset),
                    ];

                    v.extend(std::iter::repeat(format!("mov rax, [rax]")).take(variable.times_dereferenced));

                    v.extend([
                        format!("push rax"),
                        format!(";; Finish dereferencing variable {}", variable.var_name),
                    ]);

                    self.extend_current_label(v);
                } else if variable.store_address {
                    self.extend_current_label(vec![format!("lea rax, [rbp - {}]", ar_var_offset), format!("push rax")]);
                } else {
                    self.extend_current_label(vec![format!("mov {}, [rbp - {}]", reg_name, ar_var_offset), format!("push rax")]);
                }
            }

            VarType::Str => {
                if variable.dereference {
                    let mut v = vec![
                        format!(";; Dereferencing variable {}", variable.var_name),
                        format!("mov rax, [rbp - {}]", ar_var_offset),
                        // now rax contains the address of the pointer to the
                        // string
                        // now we move the length of the string into rbx
                        format!("mov rbx, [rax - 8]"), // now rbx = length of
                                                       // the string
                    ];
                    v.extend(std::iter::repeat(format!("mov rax, [rax]")).take(variable.times_dereferenced));
                    v.extend([
                        format!("push rax"),
                        format!("push rbx"),
                        format!(";; Finish dereferencing variable {}", variable.var_name),
                    ]);

                    self.extend_current_label(v);
                } else if variable.store_address {
                    self.extend_current_label(vec![format!("mov rax, [rbp - {}]", ar_var_offset), format!("push rax")]);
                } else {
                    self.extend_current_label(vec![format!("mov rax, [rbp - {}]", ar_var_offset), format!("push rax")]);
                }
            }

            VarType::Char => {
                // TODO: Differentiate btw pointer to the first char of a string and a pointer to a
                // single char
                if variable.dereference {
                    // mov al as we only want 8 bytes
                    self.extend_current_label(vec![
                        format!("mov rbx, [rbp - {}]", ar_var_offset),
                        format!("xor rax, rax"),
                        format!("mov al, [rbx]"),
                        format!("push rax"),
                    ]);
                } else if variable.store_address {
                    todo!()
                } else {
                    self.extend_current_label(vec![format!("mov rax, [rbp - {}]", ar_var_offset), format!("push rax")]);
                }
            }

            type_ => {
                todo!("var_type '{type_}' not handled")
            }
        }
    }

    fn handle_asm_for_array(&mut self, var_type: &Box<VarType>, variable: &Variable, ar_var: &Variable) {
        if variable.array_aceess_index.is_none() {
            // if it's just printing the array, then print the address
            self.extend_current_label(vec![format!("lea rax, [rbp - {}]", ar_var.offset), format!("push rax")]);
            return;
        }

        match *var_type.clone() {
            VarType::Int => {
                self.extend_current_label(vec![
                    format!(";; Start array index access"),
                    // rax has the index into the array
                    format!("pop rax"),
                    format!("mov rbx, {}", variable.result_type.get_underlying_type_size()),
                    format!("mul rbx"),
                    // now rax has index * 8
                    format!("mov rbx, rbp"),
                    format!("add rbx, rax"),
                    format!("mov rax, [rbx - {}]", ar_var.offset),
                    format!("push rax"),
                ]);
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
        }
    }

    // need the var_type for struct member access as struct_name.member as the type 'struct_name'
    fn handle_local_int(&mut self, variable: &Variable, ar_var_offset: usize, actual_var_type: &VarType) {
        if variable.dereference {
            panic!("Cannot dereference a number")
        } else if variable.store_address {
            self.extend_current_label(vec![format!("lea rax, [rbp - {}]", ar_var_offset), format!("push rax")]);
        } else {
            let reg_name = actual_var_type.get_register_name(Register::RAX);

            self.extend_current_label(vec![
                format!("xor {}, {}", Register::RAX, Register::RAX),
                format!("mov {}, [rbp - {}]", reg_name, ar_var_offset),
                format!("push rax"),
            ]);
        }
    }

    fn handle_local_str(&mut self, variable: &Variable, ar_var_offset: usize) {
        if variable.dereference {
            let mut v = vec![
                format!(";; Dereferencing variable {}", variable.var_name),
                format!("mov rax, [rbp - {}]", ar_var_offset),
                // now rax contains the address of the pointer to the
                // string
                // now we move the length of the string into rbx
                // format!("mov rbx, 1"), // now rbx = length of
                // the string
                //
                // NOTE: Not doing the above as a string derefed should only be the first character
            ];
            v.extend(std::iter::repeat(format!("mov rax, [rax]")).take(variable.times_dereferenced - 1));
            v.extend([
                format!("push rax"),
                // format!("push rbx"),
            ]);

            self.extend_current_label(v);
        } else if variable.store_address {
            // the pointer to the string is stored below the length
            // --- top of stack ---
            // .
            // .
            // --- length ---
            // --- pointer to string ---
            self.extend_current_label(vec![format!("lea rax, [rbp - {}]", ar_var_offset), format!("push rax")]);
        } else {
            self.extend_current_label(vec![
                format!("mov rax, [rbp - {}]", ar_var_offset),
                format!("push rax"),
                // length is pushed last
                format!("mov rax, [rbp - {}]", ar_var_offset - 8),
                format!("push rax"),
            ])
        }
    }

    fn handle_global_ptr(&mut self, variable: &Variable, ar_var: &Variable) {
        let var_name = &variable.var_name;

        if ar_var.is_memory_block {
            // this will be in the bss section
            if variable.dereference {
                let mut v = vec![
                    format!(";; Dereferencing variable {}", var_name),
                    format!("mov rax, {}", var_name),
                ];
                v.extend(std::iter::repeat(format!("mov rax, [rax]")).take(variable.times_dereferenced));
                v.extend([
                    format!("push rax"),
                    format!(";; Finish dereferencing variable {}", var_name),
                ]);

                self.extend_current_label(v);
            } else if variable.store_address {
                self.extend_current_label(vec![format!("lea rax, {}", var_name), format!("push rax")]);
            } else {
                self.extend_current_label(vec![format!("mov rax, {}", var_name), format!("push rax")]);
            }
        } else {
            todo!()
        }
    }

    pub fn gen_asm_for_var(&mut self, variable: &Variable, call_stack: &CallStack) {
        let var_name = &variable.var_name;

        let (variable_from_stack, variable_scope) = call_stack.get_var_with_name(var_name);

        match variable_from_stack {
            Some(ar_var) => {
                match variable_scope {
                    ActivationRecordType::Global => match variable.var_type {
                        VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                            let register_name = variable.var_type.get_register_name(Register::RAX);

                            if variable.dereference {
                                panic!("Cannot dereference a number")
                            } else if variable.store_address {
                                self.extend_current_label(vec![format!("lea rax, {var_name}"), format!("push rax")]);
                            } else {
                                self.extend_current_label(vec![
                                    format!("mov {}, [{var_name}]", register_name),
                                    format!("push rax"),
                                ])
                            }
                        }

                        VarType::Str => {
                            if variable.dereference {
                                panic!("Cannot dereference a string")
                            } else if variable.store_address {
                                todo!()
                            } else {
                                todo!("need to get the string length as well");
                                self.extend_current_label(vec![format!("mov rax, [{var_name}]"), format!("push rax")])
                            }
                        }

                        VarType::Float => todo!(),
                        VarType::Char => todo!(),

                        VarType::Int8 => todo!(),
                        VarType::Int16 => todo!(),
                        VarType::Int32 => todo!(),

                        VarType::Ptr(_) => self.handle_global_ptr(variable, ar_var),

                        VarType::Array(..) => todo!(),
                        VarType::Unknown => todo!(),
                        VarType::Struct(_, _) => todo!(),
                    }, // global scope end

                    _ => {
                        // cannot use ar_var here as it does not have the computed types
                        match &variable.var_type {
                            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                                self.handle_local_int(variable, ar_var.offset, &variable.var_type)
                            }

                            VarType::Str => self.handle_local_str(variable, ar_var.offset),

                            // TODO: Handle pointer to pointer to something
                            VarType::Ptr(var_type) => self.handle_local_ptr(var_type, variable, ar_var.offset),

                            VarType::Float => todo!(),
                            VarType::Char => todo!(),

                            VarType::Array(var_type, _) => self.handle_asm_for_array(var_type, variable, ar_var),

                            VarType::Struct(_, member_access) => {
                                let first = &member_access.borrow()[0];

                                // 'variable' has the member access properties
                                // 'variable_from_stack' has the offset
                                // it's the entire struct
                                // print the memory address
                                if variable.member_access.len() == 0 {
                                    self.extend_current_label(vec![
                                        format!("lea rax, [rbp - {}]", first.offset),
                                        format!("push rax"),
                                    ]);

                                    return;
                                }

                                let offset = first;

                                // TODO: handle struct inside struct here
                                let borrow = member_access.borrow();
                                let found = borrow.iter().find(|x| x.name == variable.member_access[0]);

                                match found {
                                    Some(struct_member_type) => match &struct_member_type.member_type {
                                        VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => self
                                            .handle_local_int(
                                                variable,
                                                struct_member_type.offset,
                                                &struct_member_type.member_type,
                                            ),

                                        VarType::Str => self.handle_local_str(variable, struct_member_type.offset),
                                        VarType::Ptr(var_type) => {
                                            self.handle_local_ptr(var_type, variable, struct_member_type.offset)
                                        }

                                        VarType::Int8 => todo!(),
                                        VarType::Int16 => todo!(),
                                        VarType::Int32 => todo!(),

                                        VarType::Float => todo!(),
                                        VarType::Char => todo!(),
                                        VarType::Array(_, _) => todo!(),
                                        VarType::Struct(_, _) => todo!(),
                                        VarType::Unknown => todo!(),
                                    },

                                    None => unreachable!(
                                        "Could not find memeber {} of struct while generating ASM",
                                        variable.member_access[0]
                                    ),
                                }
                            }

                            VarType::Unknown => todo!(),
                        }
                    }
                }
            }

            None => unreachable!(
                "Could not find variable with name '{}' in function `variable_declaration`. \
            This is a bug in the semantic analying step.",
                var_name,
            ),
        };
    }
}
