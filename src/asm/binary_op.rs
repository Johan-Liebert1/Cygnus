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
                    format!(";; Plus get the two operands from the stack"),
                    format!("pop rax"),
                    format!("pop rbx"),
                    format!("add rax, rbx"),
                ]
            }

            Operations::Minus => {
                vec![
                    format!(";; Minus get the two operands from the stack"),
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
                    format!(";; Divide clean up rdx as this might mess up the final output"),
                    format!("xor rdx, rdx"),
                    format!(";; get the two operands from the stack"),
                    format!("pop rbx"),
                    format!("pop rax"),
                    format!("div rbx"),
                ]
            }

            Operations::Multiply => {
                vec![
                    format!(";; Multiply get the two operands from the stack"),
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
                    format!(";; ShiftLeft get the two operands from the stack"),
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
                    format!(";; ShiftRight get the two operands from the stack"),
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
                    format!(";; Modulo get the two operands from the stack"),
                    format!("xor rdx, rdx"),
                    format!("pop rbx"),
                    format!("pop rax"),
                    format!("div rbx"),
                    format!("mov rax, rdx"),
                ]
            }
        };

        // trace!(
        //     "result_type: {}, times_dereferenced: {}",
        //     result_type,
        //     times_dereferenced
        // );

        // if *result_type == VarType::Ptr(Box::new(VarType::Str)) {
        // }

        // result will always be in rax
        // We will also never dereference a string as we want the character address
        match result_type {
            VarType::Int => {
                instructions.push(format!("push rax"));
            }

            VarType::Str => todo!(),
            VarType::Float => todo!(),

            // this is basically an integer, a u8 to be precise
            VarType::Char => {
                instructions.push(format!("push rax"));
            }

            VarType::Ptr(type_) => match **type_ {
                VarType::Int => {
                    instructions.extend(std::iter::repeat(format!("mov rax, [rax]")).take(times_dereferenced));
                    instructions.push(format!("push rax"));
                }

                VarType::Char => {
                    instructions.push(format!(";; binary op ptr -> char"));

                    if times_dereferenced > 0 {
                        instructions.extend(vec![
                            format!("mov rbx, rax"),
                            format!("xor rax, rax"),
                            format!("mov al, [rbx]"),
                            format!("push rax"),
                        ]);
                    } else {
                        instructions.push(format!("push rax"));
                    }
                }

                VarType::Str => todo!(),

                VarType::Float => todo!(),
                VarType::Ptr(_) => todo!(),
                VarType::Array(_, _) => todo!(),
                VarType::Struct(_, _) => todo!(),
                VarType::Unknown => todo!(),
            },

            VarType::Array(_, _) => todo!(),
            VarType::Struct(_, _) => todo!(),
            VarType::Unknown => todo!(),
        };

        self.extend_current_label(instructions);
    }
}
