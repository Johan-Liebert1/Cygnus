use crate::lexer::tokens::Operations;

use super::asm::ASM;

impl ASM {
    pub fn binary_op_nums(&mut self, op: Operations) {
        let first = match op {
            Operations::Plus => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("pop rax"),
                    format!("pop rbx"),
                    format!("add rax, rbx"),
                    format!(";; push the result back onto the stack"),
                    format!("push rax"),
                ]
            }

            Operations::Minus => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("pop rbx"),
                    format!("pop rax"),
                    format!("sub rax, rbx"),
                    format!(";; push the result back onto the stack"),
                    format!("push rax"),
                ]
            }

            Operations::Divide => {
                vec![
                    // 40 / 5
                    // push 40
                    // push 5
                    format!(";; clean up rdx as this might mess up the final output"),
                    format!("xor rdx, rdx"),
                    format!(";; get the two operands from the stack"),
                    format!("pop rbx"),
                    format!("pop rax"),
                    format!("div rbx"),
                    format!(";; push the result back onto the stack"),
                    format!("push rax"),
                ]
            },

            Operations::Multiply => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("xor rdx, rdx"),
                    format!("pop rax"),
                    format!("pop rbx"),
                    format!("mul rbx"),
                    format!(";; push the result back onto the stack"),
                    format!("push rax"),
                ]
            },
        };

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(first);
                break;
            }
        }
    }
}
