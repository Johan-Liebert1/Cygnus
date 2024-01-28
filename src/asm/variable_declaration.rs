use crate::semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack};

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
                    },

                    _ => {
                        // 2. If global var add it to the global section, else skip
                    }
                }
            },

            None => unreachable!("Could not find variable with name '{}' in function `variable_declaration`. This is a bug in the semantic analying step.", var_name),
        };
    }

    /// pops the top most element on the stack and assigns it to the variable
    pub fn variable_assignment(&mut self, var_name: &String, call_stack: &CallStack) {
        // 1. Check whether the variable is a local or global variable
        // 2. If global var, get it from data section, else from stack offset

        let (variable, variable_scope) = call_stack.get_var_with_name(&var_name);

        match variable {
                    Some(var) => {
                        match variable_scope {
                            ActivationRecordType::Global => {
                                    self.extend_current_label(vec![format!("pop rax"), format!("mov [{}], rax", var_name)])
                            },

                            _ => {
                                    self.extend_current_label(vec![format!("pop rax"), format!("mov [rsp + {}], rax", var.offset)])
                            }
                        }
                    },

                    None => unreachable!("Could not find variable with name '{}' in function `factor`. This is a bug in the semantic analying step.", var_name),
                };
    }
}
