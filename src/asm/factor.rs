use crate::{
    helpers::compiler_error,
    interpreter::interpreter::Variables,
    lexer::tokens::{Number, TokenEnum, VariableEnum},
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
    trace,
};

use super::asm::ASM;

impl ASM {
    /// Pushes whatever token's in here onto the stack
    pub fn generate_asm_factor(&mut self, token: &TokenEnum, call_stack: &CallStack) {
        let mut instructions: Vec<String> = vec![];

        match token {
            TokenEnum::Number(n) => match n {
                Number::Integer(i) => instructions.push(format!("push {i}")),

                // We cannot have immediate float values in nasm
                Number::Float(f) => {
                    // add the floating point in the data segement
                    self.data.push(format!("float_{} db {f}", self.num_floats));

                    instructions.extend(vec![
                        format!("mov rax, float_{}", self.num_floats),
                        // rax contains the address of the float
                        format!("push rax"),
                    ]);

                    self.num_floats += 1;
                }
            },

            TokenEnum::StringLiteral(s) => {
                let mut chars = vec![];

                let mut char_iter = s.chars();

                loop {
                    match char_iter.next() {
                        Some(c) => match c {
                            '\\' => {
                                match char_iter.next() {
                                    Some(c) => match c {
                                        'n' => chars.push(('\n' as u8).to_string()),
                                        '0' => chars.push('0'.into()),
                                        'r' => chars.push(('\r' as u8).to_string()),

                                        _ => unimplemented!(),
                                    },

                                    // string literal ends with a backslash
                                    None => {
                                        panic!("String cannot end with a \\")
                                    }
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
                self.data
                    .push(format!("string_{} db {}", self.num_strings, chars.join(",")));

                instructions.extend(vec![
                    format!("mov rax, string_{}", self.num_strings),
                    format!("push rax"),
                    format!("push {}", chars.len()),
                ]);

                self.num_strings += 1;
            }

            TokenEnum::Variable(..) => todo!(),
            TokenEnum::LogicalOp(..) => todo!(),
            TokenEnum::Equals => todo!(),
            TokenEnum::PlusEquals => todo!(),
            TokenEnum::MinusEquals => todo!(),
            TokenEnum::Ampersand => todo!(),
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
            TokenEnum::FunctionReturnIndicator => todo!(),
            TokenEnum::Comment => todo!(),
            TokenEnum::EOF => todo!(),
            TokenEnum::Dot => todo!(),
        }

        self.extend_current_label(instructions);
    }
}
