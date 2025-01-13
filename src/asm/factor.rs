use crate::{
    helpers::compiler_error,
    lexer::{
        lexer::Token,
        tokens::{Number, TokenEnum},
        types::VarType,
    },
};

use super::asm::ASM;

impl ASM {
    /// Pushes whatever token's in here onto the stack
    pub fn generate_asm_factor(&mut self, token: &Token, result_type: &VarType) {
        let mut instructions: Vec<String> = vec![];

        match &token.token {
            TokenEnum::Number(n) => match n {
                Number::Integer(i) => self.stack_push(format!("{i}")),

                // We cannot have immediate float values in nasm
                Number::Float(f) => {
                    let xmm0 = self.get_free_float_register(None);

                    // add the floating point in the data segement
                    self.data.push(format!("float_{} dq {f}", self.num_floats));

                    instructions.extend(vec![format!("movsd {xmm0}, [float_{}]", self.num_floats)]);

                    self.stack_push(String::from(xmm0));

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
                                    None => compiler_error("String cannot end with a \\", token),
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
                    "string_{} db {} ;; {token:?}",
                    self.num_strings,
                    chars.join(",")
                ));

                match result_type {
                    VarType::Str => {
                        self.stack_extend(vec![format!("string_{}", self.num_strings), format!("{}", chars.len())])
                    },

                    VarType::Ptr(boxed) if **boxed == VarType::Char => {
                        self.stack_push(format!("string_{}", self.num_strings))
                    },

                    _ => unreachable!("Found type of token to be {result_type} when it's a string. This must be a bug in the semantic analysis step.")
                }

                self.num_strings += 1;
            }

            _ => unreachable!(),
        }

        self.extend_current_label(instructions);
    }
}
