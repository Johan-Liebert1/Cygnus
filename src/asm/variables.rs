use crate::{
    ast::variable,
    lexer::{tokens::VariableEnum, keywords::{TYPE_INT, TYPE_FLOAT, TYPE_STRING}},
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
};

use super::asm::ASM;

impl ASM {
    pub fn gen_asm_for_var(&mut self, var_name: &String, call_stack: &CallStack) {
        let (variable, variable_scope) = call_stack.get_var_with_name(var_name);

        match variable {
            Some(ar_var) => {
                match variable_scope {
                    ActivationRecordType::Global => {
                        match ar_var.var {
                            // TODO: Move this here, currently
                            // this is handled in factor
                            VariableEnum::Number(_) => {
                                todo!()
                            },

                            // TODO: Move this here, currently
                            // this is handled in factor
                            VariableEnum::String(_) => todo!(),

                            VariableEnum::Pointer(_) => todo!(),
                        }
                        
                    },

                    _ => {
                        match &ar_var.var {
                            // TODO: Move this here, currently
                            // this is handled in factor
                            VariableEnum::Number(_) => {
                                todo!()
                            },

                            // TODO: Move this here, currently
                            // this is handled in factor
                            VariableEnum::String(_) => todo!(),

                            // TODO: Handle pointer to pointer to something

                            // TODO: Handle &a, currently we assume it's always gonna be *a
                            VariableEnum::Pointer(var_type) => {
                                match var_type.as_str() {
                                    TYPE_INT | TYPE_FLOAT => {
                                        self.extend_current_label(vec![format!("mov rax, [rbp - {}]", ar_var.offset)]);
                                    }

                                    TYPE_STRING => todo!(),

                                    type_ => {
                                        todo!("var_type '{type_}' not handled")
                                    }
                                }
                            },

                        }
                    }
                }
            },

            None => unreachable!("Could not find variable with name '{}' in function `variable_declaration`. This is a bug in the semantic analying step.", var_name),
        };
    }
}
