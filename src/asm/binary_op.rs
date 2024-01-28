use crate::types::ASTNode;

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
            }

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
            }

            Operations::ShiftLeft => {
                // 1 << 10
                // push 1
                // push 10
                // 1 should be popped into rax and 10 in rcx
                vec![
                    format!(";; get the two operands from the stack"),
                    // the shl instruction in x86 assembly does not allow the use of a general-purpose register like bl
                    // for the count operand when the destination is a 64-bit register like rax.
                    // The count operand must be either an immediate value (a constant) or the cl register.
                    format!("pop rcx"),
                    format!("pop rax"),
                    format!(";; We can only shift left or right by 8 bits"),
                    format!("shl rax, cl"),
                    format!(";; push the result back onto the stack"),
                    format!("push rax"),
                ]
            }

            Operations::ShiftRight => {
                // push 1
                // push 10
                // 1 should be popped into rax and 10 in rcx
                vec![
                    format!(";; get the two operands from the stack"),
                    // the shl instruction in x86 assembly does not allow the use of a general-purpose register like bl
                    // for the count operand when the destination is a 64-bit register like rax.
                    // The count operand must be either an immediate value (a constant) or the cl register.
                    format!("pop rcx"),
                    format!("pop rax"),
                    format!(";; We can only shift left or right by 8 bits"),
                    format!("shr rax, cl"),
                    format!(";; push the result back onto the stack"),
                    format!("push rax"),
                ]
            }

            Operations::Modulo => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("xor rdx, rdx"),
                    format!("pop rbx"),
                    format!("pop rax"),
                    format!("div rbx"),
                    format!(";; push the remainder result back onto the stack"),
                    format!("push rdx"),
                ]
            }
        };

        self.extend_current_label(first);
    }
}
