use crate::{
    interpreter::interpreter::Variables,
    lexer::tokens::{Number, TokenEnum},
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
};

use super::asm::ASM;

impl ASM {
    /// Pushes whatever token's in here onto the stack
    pub fn generate_asm_factor(
        &mut self,
        token: &TokenEnum,
        vars: &Variables,
        call_stack: &CallStack,
    ) {
        let mut instructions: Vec<String> = vec![];

        match token {
            TokenEnum::Number(n) => match n {
                Number::Integer(i) => instructions.push(format!("push {i}")),
                Number::Float(_) => todo!(),
            },

            TokenEnum::StringLiteral(s) => {
                let mut chars = vec![];

                let mut char_iter = s.chars();

                loop {
                    match char_iter.next() {
                        Some(c) => match c {
                            '\\' => {
                                match char_iter.next() {
                                    Some(_c) => {
                                        // TODO: Handle all escape sequences
                                        chars.push(('\n' as u8).to_string())
                                    }

                                    // string literal ends with a backslash
                                    None => panic!("String cannot end with a \\"),
                                }
                            }

                            _ => {
                                chars.push((c as u8).to_string());
                            }
                        },

                        None => break,
                    }
                }

                // add the string literal in the data segement
                self.data.push(format!(
                    "string_{} db {}",
                    self.num_strings,
                    chars.join(",")
                ));

                instructions.extend(vec![
                    format!("mov rax, string_{}", self.num_strings),
                    format!("push rax"),
                    format!("push {}", chars.len()),
                ]);

                self.num_strings += 1;
            }

            TokenEnum::Variable(var_name) => {
                let (variable, variable_scope) = call_stack.get_var_with_name(&var_name);

                match variable {
                    Some(var) => {
                        match variable_scope {
                            ActivationRecordType::Global => {
                                instructions
                                    .extend(vec![format!("mov rax, [{var_name}]"), format!("push rax")])
                            },

                            _ => {
                                instructions
                                    .extend(vec![format!("mov rax, [rsp + {}]", var.offset), format!("push rax")])
                            }
                        }
                    },

                    None => unreachable!("Could not find variable with name '{}' in function `factor`. This is a bug in the semantic analying step or the call stacks in semantic analysis and compilation don't match.", var_name),
                };
            }

            TokenEnum::LogicalOp(..) => todo!(),
            TokenEnum::Equals => todo!(),
            TokenEnum::Comma => todo!(),
            TokenEnum::SemiColon => todo!(),
            TokenEnum::Colon => todo!(),
            TokenEnum::Bracket(_) => todo!(),
            TokenEnum::Op(_) => todo!(),
            TokenEnum::Comparator(_) => todo!(),
            TokenEnum::Bool(_) => todo!(),
            TokenEnum::Keyword(_) => todo!(),
            TokenEnum::Type(_) => todo!(),
            TokenEnum::Unknown(_) => todo!(),
            TokenEnum::EOF => todo!(),
        }

        self.extend_current_label(instructions);
    }
}
