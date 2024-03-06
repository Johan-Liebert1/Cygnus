use crate::{
    lexer::{tokens::Operations, types::VarType},
    trace,
};

use super::asm::ASM;

impl ASM {
    pub fn binary_op_nums(&mut self, op: Operations, times_dereferenced: usize, result_type: &VarType) {
        let mut instructions = match op {
            Operations::Plus => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("pop rax"),
                    format!("pop rbx"),
                    format!("add rax, rbx"),
                ]
            }

            Operations::Minus => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("pop rbx"),
                    format!("pop rax"),
                    format!("sub rax, rbx"),
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
                ]
            }

            Operations::Multiply => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("xor rdx, rdx"),
                    format!("pop rax"),
                    format!("pop rbx"),
                    format!("mul rbx"),
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
                ]
            }

            Operations::Modulo => {
                vec![
                    format!(";; get the two operands from the stack"),
                    format!("xor rdx, rdx"),
                    format!("pop rbx"),
                    format!("pop rax"),
                    format!("div rbx"),
                    format!("mov rax, rdx"),
                ]
            }
        };

        // result will always be in rax
        // We will also never dereference a string as we want the character address
        trace!("result_type: {}", result_type);

        if *result_type != VarType::Ptr(Box::new(VarType::Str)) && *result_type != VarType::Ptr(Box::new(VarType::Char))
        {
            instructions.extend(std::iter::repeat(format!("mov rax, [rax]")).take(times_dereferenced));
            instructions.push(format!("push rax"));
        } else if times_dereferenced > 0 || *result_type == VarType::Ptr(Box::new(VarType::Char)) {
            instructions.push(format!("push rax"));
            instructions.push(format!("push 1"));
        }

        self.extend_current_label(instructions);
    }
}
