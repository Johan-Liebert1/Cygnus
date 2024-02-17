use core::panic;

use crate::{
    ast::variable::{self, Variable},
    lexer::{
        tokens::VariableEnum,
        types::{TYPE_FLOAT, TYPE_INT, VarType},
    },
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
    trace,
};

use super::asm::ASM;

impl ASM {
    pub fn gen_asm_for_var(&mut self, variable: &Variable, call_stack: &CallStack) {
        let var_name = &variable.var_name;

        let (variable_from_stack, variable_scope) = call_stack.get_var_with_name(var_name);

        match variable_from_stack {
            Some(ar_var) => {
                match variable_scope {
                    ActivationRecordType::Global => match ar_var.var_type {
                        VarType::Int => {
                            if variable.dereference {
                                panic!("Cannot dereference a number")
                            } else if variable.store_address {
                                self.extend_current_label(vec![format!("lea rax, {var_name}"), format!("push rax")]);
                            } else {
                                self.extend_current_label(vec![format!("mov rax, [{var_name}]"), format!("push rax")])
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
                        VarType::Ptr(_) => todo!(),
                        VarType::Unknown => todo!(),
                    },

                    _ => {
                        match &ar_var.var_type {
                            VarType::Int => {
                                if variable.dereference {
                                    panic!("Cannot dereference a number")
                                } else if variable.store_address {
                                    self.extend_current_label(vec![
                                        format!("lea rax, [rbp - {}]", ar_var.offset),
                                        format!("push rax"),
                                    ]);
                                } else {
                                    self.extend_current_label(vec![
                                        format!("mov rax, [rbp - {}]", ar_var.offset),
                                        format!("push rax"),
                                    ]);
                                }
                            }

                            VarType::Str => {
                                if variable.dereference {
                                    let mut v = vec![
                                        format!(";; Dereferencing variable {}", var_name),
                                        format!("mov rax, [rbp - {}]", ar_var.offset),
                                        // now rax contains the address of the pointer to the
                                        // string
                                        // now we move the length of the string into rbx
                                        format!("mov rbx, 1"), // now rbx = length of
                                                               // the string
                                    ];
                                    v.extend(
                                        std::iter::repeat(format!("mov rax, [rax]"))
                                            .take(variable.times_dereferenced - 1),
                                    );
                                    v.extend([format!("push rax"), format!("push rbx")]);

                                    self.extend_current_label(v);
                                } else if variable.store_address {
                                    // the pointer to the string is stored below the length
                                    // --- top of stack ---
                                    // .
                                    // .
                                    // --- lenght ---
                                    // --- pointer to string ---
                                    self.extend_current_label(vec![format!("lea rax, [rbp - {}]", ar_var.offset)]);
                                } else {
                                    self.extend_current_label(vec![
                                        format!("mov rax, [rbp - {}]", ar_var.offset),
                                        format!("push rax"),
                                        // length is pushed last
                                        format!("mov rax, [rbp - {}]", ar_var.offset + 8),
                                        format!("push rax"),
                                    ])
                                }
                            }

                            // TODO: Handle pointer to pointer to something
                            VarType::Ptr(var_type) => match *var_type.clone() {
                                VarType::Int | VarType::Float => {
                                    if variable.dereference {
                                        let mut v = vec![
                                            format!(";; Dereferencing variable {}", var_name),
                                            format!("mov rax, [rbp - {}]", ar_var.offset),
                                        ];
                                        v.extend(
                                            std::iter::repeat(format!("mov rax, [rax]"))
                                                .take(variable.times_dereferenced),
                                        );
                                        v.extend([
                                            format!("push rax"),
                                            format!(";; Finish dereferencing variable {}", var_name),
                                        ]);

                                        self.extend_current_label(v);
                                    } else if variable.store_address {
                                        self.extend_current_label(vec![
                                            format!("lea rax, [rbp - {}]", ar_var.offset),
                                            format!("push rax"),
                                        ]);
                                    } else {
                                        self.extend_current_label(vec![
                                            format!("mov rax, [rbp - {}]", ar_var.offset),
                                            format!("push rax"),
                                        ]);
                                    }
                                }

                                VarType::Str => {
                                    if variable.dereference {
                                        let mut v = vec![
                                            format!(";; Dereferencing variable {}", var_name),
                                            format!("mov rax, [rbp - {}]", ar_var.offset),
                                            // now rax contains the address of the pointer to the
                                            // string
                                            // now we move the length of the string into rbx
                                            format!("mov rbx, [rax - 8]"), // now rbx = length of
                                                                           // the string
                                        ];
                                        v.extend(
                                            std::iter::repeat(format!("mov rax, [rax]"))
                                                .take(variable.times_dereferenced),
                                        );
                                        v.extend([
                                            format!("push rax"),
                                            format!("push rbx"),
                                            format!(";; Finish dereferencing variable {}", var_name),
                                        ]);

                                        self.extend_current_label(v);
                                    } else if variable.store_address {
                                        self.extend_current_label(vec![
                                            format!("lea rax, [rbp - {}]", ar_var.offset),
                                            format!("push rax"),
                                        ]);
                                    } else {
                                        self.extend_current_label(vec![
                                            format!("mov rax, [rbp - {}]", ar_var.offset),
                                            format!("push rax"),
                                        ]);
                                    }
                                }

                                type_ => {
                                    todo!("var_type '{type_}' not handled")
                                }
                            },

                            VarType::Float => todo!(),
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
