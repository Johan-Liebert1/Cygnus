use core::panic;

use crate::{
    ast::variable::{self, Variable},
    lexer::{
        keywords::{TYPE_FLOAT, TYPE_INT, TYPE_STRING},
        tokens::VariableEnum,
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
                    ActivationRecordType::Global => match ar_var.var {
                        VariableEnum::Number(_) => {
                            if variable.dereference {
                                panic!("Cannot dereference a number")
                            } else if variable.store_address {
                                self.extend_current_label(vec![format!("lea rax, {var_name}"), format!("push rax")]);
                            } else {
                                self.extend_current_label(vec![format!("mov rax, [{var_name}]"), format!("push rax")])
                            }
                        }

                        VariableEnum::String(_) => {
                            if variable.dereference {
                                panic!("Cannot dereference a string")
                            } else if variable.store_address {
                                todo!()
                            } else {
                                todo!("need to get the string length as well");
                                self.extend_current_label(vec![format!("mov rax, [{var_name}]"), format!("push rax")])
                            }
                        }

                        VariableEnum::Pointer(_) => todo!(),
                    },

                    _ => {
                        match &ar_var.var {
                            VariableEnum::Number(_) => {
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

                            VariableEnum::String(_) => {
                                if variable.dereference {
                                    panic!("Cannot dereference a string")
                                } else if variable.store_address {
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
                            VariableEnum::Pointer(var_type) => match var_type.as_str() {
                                TYPE_INT | TYPE_FLOAT => {
                                    if variable.dereference {
                                        self.extend_current_label(vec![
                                            format!("mov rax, [rbp - {}]", ar_var.offset),
                                            format!("mov rax, [rax]"),

                                            format!("push rax"),
                                            format!("push rbx"),
                                        ]);
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

                                TYPE_STRING => {
                                    if variable.dereference {
                                        self.extend_current_label(vec![
                                            format!("mov rax, [rbp - {}]", ar_var.offset),
                                            // now rax contains the address of the pointer to the
                                            // string
                                            // now we move the length of the string into rbx
                                            format!("mov rbx, [rax - 8]"), // now rbx = length of
                                            // the string
                                            
                                            format!("mov rax, [rax]"),

                                            format!("push rax"),
                                            format!("push rbx"),
                                        ]);
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
