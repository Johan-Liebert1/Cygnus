use crate::semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack};

use super::asm::ASM;

impl ASM {
    pub fn variable_declaration(&mut self, var_name: &String, call_stack: &CallStack) {
        // 1. Check whether the variable is a local or global variable
        // 2. If global var add it to the global section, else skip

        let (variable, variable_scope) = call_stack.get_var_with_name(var_name);

        match variable {
            Some(var) => {
                match variable_scope {
                    ActivationRecordType::Global => {
                        // db will only allocate 1 byte while dq allocates a word
                        self.data.push(format!(
                            "{var_name} {} 0",
                            var.borrow().var_type.get_data_segment_size()
                        ));
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
}
