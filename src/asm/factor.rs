use crate::lexer::tokens::{TokenEnum, Number};

use super::asm::ASM;

impl ASM {
    /// Pushes whatever token's in here onto the stack
    pub fn generate_asm_factor(&mut self, token: &TokenEnum) {
        let mut instructions: Vec<String> = vec![];

        match token {
            TokenEnum::Number(n) => match n {
                Number::Integer(i) => instructions.push(format!("push {i}")),
                Number::Float(_) => todo!(),
            },

            TokenEnum::Equals => todo!(),
            TokenEnum::Comma => todo!(),
            TokenEnum::Colon => todo!(),
            TokenEnum::Bracket(_) => todo!(),
            TokenEnum::Op(_) => todo!(),
            TokenEnum::Comparator(_) => todo!(),
            TokenEnum::Bool(_) => todo!(),
            TokenEnum::Keyword(_) => todo!(),
            TokenEnum::Variable(_) => todo!(),
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
