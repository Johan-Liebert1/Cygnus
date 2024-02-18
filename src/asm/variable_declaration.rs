use crate::{
    lexer::{tokens::{AssignmentTypes, VariableEnum}, types::VarType},
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
};

use super::asm::ASM;

impl ASM {
    pub fn variable_declaration(&mut self, var_name: &String, call_stack: &CallStack) {
        // 1. Check whether the variable is a local or global variable
        // 2. If global var add it to the global section, else skip

        let (variable, variable_scope) = call_stack.get_var_with_name(var_name);

        match variable {
            Some(_) => {
                match variable_scope {
                    ActivationRecordType::Global => {
                        // this needs to be dq, as we are assuming all integers are 64 bits
                        // db will only allocate 1 byte while dq allocates a word
                        self.data.push(format!("{} dq 0", var_name));
                    }

                    _ => {
                        // 2. If global var add it to the global section, else skip
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

    /// pops the top most element on the stack and assigns it to the variable
    pub fn variable_assignment(
        &mut self,
        var_name: &String,
        assignment_type: &AssignmentTypes,
        call_stack: &CallStack,
    ) {
        // 1. Check whether the variable is a local or global variable
        // 2. If global var, get it from data section, else from stack offset

        let (variable, variable_scope) = call_stack.get_var_with_name(&var_name);

        let mut instructions = vec![];

        let mut is_string = false;

        match variable {
            Some(var) => {
                match variable_scope {
                    ActivationRecordType::Global => {
                        match assignment_type {
                            AssignmentTypes::Equals => {
                                match var.var_type {
                                    VarType::Int => {}

                                    VarType::Str => {
                                        // pop the string pointer into rax
                                        // the string len should be in rbx as string len is pushed
                                        // last
                                        instructions.extend([format!("pop rbx"), format!("pop rax")]);

                                        is_string = true;
                                    }

                                    VarType::Ptr(_) => {}

                                    VarType::Float => todo!(),
                                    VarType::Char => todo!(),
                                    VarType::Unknown => todo!(),
                                }
                            }

                            AssignmentTypes::PlusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("add rax, rbx")])
                            }
                            AssignmentTypes::MinusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("sub rax, rbx")])
                            }
                        }

                        instructions.push(format!("mov [{}], rax", var_name));
                    }

                    _ => {
                        match assignment_type {
                            AssignmentTypes::Equals => {
                                match var.var_type {
                                    VarType::Int | VarType::Float => {
                                        instructions.push(format!("pop rax"))
                                    }

                                    VarType::Str => {
                                        // pop the string pointer into rax
                                        // the string len should be in rbx as string len is pushed
                                        // last
                                        instructions.extend([format!("pop rbx"), format!("pop rax")]);

                                        is_string = true;
                                    }

                                    VarType::Char => {
                                        // pop the string pointer into rax
                                        // the string len should be in rbx as string len is pushed
                                        // last
                                        // Treat a character as a string with length of 1
                                        instructions.extend([format!("mov rbx, 1"), format!("pop rax")]);

                                        is_string = true;
                                    }

                                    VarType::Ptr(..) => {}
                                    VarType::Unknown => todo!(),
                                }
                            }

                            AssignmentTypes::PlusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("add rax, rbx")])
                            }

                            AssignmentTypes::MinusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("sub rax, rbx")])
                            }
                        }

                        instructions.push(format!("mov [rbp - {}], rax", var.offset));

                        if is_string {
                            // Move the string length into the mem address above the addr
                            // containing the string pointer
                            instructions.push(format!("mov [rbp - {}], rbx", var.offset + 8));
                        }
                    }
                }
            }

            None => unreachable!(
                "Could not find variable with name '{}' in function `factor`. \
            This is a bug in the semantic analying step.",
                var_name
            ),
        };

        self.extend_current_label(instructions);
    }
}
