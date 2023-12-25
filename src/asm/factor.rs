use crate::{
    interpreter::interpreter::Variables,
    lexer::tokens::{Number, TokenEnum},
};

use super::asm::ASM;

impl ASM {
    /// Pushes whatever token's in here onto the stack
    pub fn generate_asm_factor(&mut self, token: &TokenEnum, vars: &Variables) {
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
                                    Some(c) => {
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
                match vars.get(var_name) {
                    Some(..) => {
                        instructions.extend(vec![
                            format!("mov rax, [{var_name}]"),
                            format!("push rax"),
                        ]);
                    },

                    None => {
                        println!("Variable {var_name} is not defined")
                    },
                };
            }

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

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }
}
