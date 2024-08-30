use crate::{
    lexer::{tokens::Operations, types::VarType},
    trace,
};

use super::asm::ASM;

impl ASM {
    fn reg_to_use(&self, reg_from_stack: &String) -> &str {
        if reg_from_stack == "rax" {
            "rcx"
        } else {
            "rax"
        }
    }

    pub fn binary_op_nums(&mut self, op: Operations, times_dereferenced: usize, result_type: &VarType) {
        let mut reg_to_use = "rax";

        let mut instructions = match op {
            Operations::Plus => {
                if matches!(result_type, VarType::Float) {
                    vec![
                        format!(";; Floating point addition"),
                        format!(";; Get the first operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm0, [float_imm]"),
                        format!(";; Get the second operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm1, [float_imm]"),
                        format!(";; floating point addition"),
                        format!("addsd xmm0, xmm1"),
                        format!("movsd [float_imm], xmm0"),
                        format!("mov rax, [float_imm]"),
                    ]
                } else {
                    // TODO: Remove
                    // vec![
                    //     format!(";; Plus get the two operands from the stack"),
                    //     format!("pop rax"),
                    //     format!("pop rbx"),
                    //     format!("add rax, rbx"),
                    // ]

                    let first = self.stack.pop().unwrap();
                    let second = self.stack.pop().unwrap();

                    let mut inst = vec![
                        format!(";; Plus get the two operands from the stack"),
                        // format!("mov {}, {}", reg_to_use, first),
                        // format!("mov rbx, {}", second),
                        // format!("add {}, rbx", reg_to_use),
                    ];

                    inst.push(format!("mov rbx, {second}"));

                    if first != "rax" {
                        inst.push(format!("mov rax, {first}"));
                    }

                    inst.push("add rax, rbx".into());

                    inst
                }
            }

            Operations::Minus => {
                if matches!(result_type, VarType::Float) {
                    vec![
                        format!(";; Floating point subtraction"),
                        format!(";; Get the first operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm1, [float_imm]"),
                        format!(";; Get the second operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm0, [float_imm]"),
                        format!(";; floating point subtraction"),
                        format!("subsd xmm0, xmm1"),
                        format!("movsd [float_imm], xmm0"),
                        format!("mov rax, [float_imm]"),
                    ]
                } else {
                    let first = self.stack.pop().unwrap();
                    let second = self.stack.pop().unwrap();

                    // TODO: Remove
                    //
                    // vec![
                    //     format!(";; Minus get the two operands from the stack"),
                    //     format!("pop rbx"),
                    //     format!("pop rax"),
                    //     format!("sub rax, rbx"),
                    // ]

                    let mut inst = vec![
                        format!(";; Minus get the two operands from the stack"),
                        format!("mov rbx, {}", first),
                        format!("mov rax, {}", second),
                        format!("sub {}, rbx", reg_to_use),
                    ];

                    inst
                }
            }

            Operations::Divide => {
                if matches!(result_type, VarType::Float) {
                    vec![
                        format!(";; Floating point division"),
                        format!(";; Get the first operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm1, [float_imm]"),
                        format!(";; Get the second operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm0, [float_imm]"),
                        format!(";; floating point subtraction"),
                        format!("divsd xmm0, xmm1"),
                        format!("movsd [float_imm], xmm0"),
                        format!("mov rax, [float_imm]"),
                    ]
                } else {
                    // TODO: Remove

                    // vec![
                    //     // 40 / 5
                    //     // push 40
                    //     // push 5
                    //     format!(";; Divide clean up rdx as this might mess up the final output"),
                    //     format!("xor rdx, rdx"),
                    //     format!(";; get the two operands from the stack"),
                    //     format!("pop rbx"),
                    //     format!("pop rax"),
                    //     format!("div rbx"),
                    // ]
                    //

                    let first = self.stack.pop().unwrap();
                    let second = self.stack.pop().unwrap();

                    vec![
                        // 40 / 5
                        // push 40
                        // push 5
                        format!(";; Divide clean up rdx as this might mess up the final output as 'div' divides rdx:rax with the register we pass to div inst"),
                        format!("xor rdx, rdx"),
                        format!(";; get the two operands from the stack"),
                        format!("mov rbx, {}", first),
                        format!("mov rax, {}", second),
                        format!("div rbx"),
                    ]
                }
            }

            Operations::Multiply => {
                if matches!(result_type, VarType::Float) {
                    vec![
                        format!(";; Floating point multiplication"),
                        format!(";; Get the first operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm0, [float_imm]"),
                        format!(";; Get the second operand"),
                        format!("pop QWORD [float_imm]"),
                        format!("movsd xmm1, [float_imm]"),
                        format!(";; floating point addition"),
                        format!("mulsd xmm0, xmm1"),
                        format!("movsd [float_imm], xmm0"),
                        format!("mov rax, [float_imm]"),
                    ]
                } else {
                    // TODO: Remove
                    //
                    // vec![
                    //     format!(";; Multiply get the two operands from the stack"),
                    //     format!("xor rdx, rdx"),
                    //     format!("pop rax"),
                    //     format!("pop rbx"),
                    //     format!("mul rbx"),
                    // ]

                    let first = self.stack.pop().unwrap();
                    let second = self.stack.pop().unwrap();

                    let mut inst = vec![
                        format!(";; Multiply get the two operands from the stack"),
                        format!("xor rdx, rdx"),
                        // format!("mov rax, {}", first),
                        // format!("mov rbx, {}", second),
                        // format!("mul rbx"),
                    ];

                    inst.push(format!("mov rbx, {second}"));

                    if first != "rax" {
                        inst.push(format!("mov rax, {first}"));
                    }

                    inst.push("mul rbx".into());

                    inst
                }
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
                    format!("xor rax, rax"),
                    format!("xor rcx, rcx"),
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
                    format!("xor rax, rax"),
                    format!("xor rcx, rcx"),
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

        self.stack.push("rax".into());
        // self.stack.push(reg_to_use.into());

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
            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                // instructions.push(format!("push rax"));
            }

            VarType::Str => todo!(),

            VarType::Float => instructions.push("push rax".into()),

            // this is basically an integer, a u8 to be precise
            VarType::Char => {
                instructions.push(format!("push rax"));
            }

            VarType::Ptr(type_) => match **type_ {
                VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
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

                VarType::Str => {
                    instructions.push(format!(";; binary op ptr -> char"));

                    if times_dereferenced > 0 {
                        instructions.extend(vec![
                            format!("mov rbx, rax"),
                            format!("xor rax, rax"),
                            format!("mov rax, [rbx]"),
                            format!("push rax"),
                        ]);
                    } else {
                        instructions.push(format!("push rax"));
                    }
                }

                VarType::Float => todo!(),
                VarType::Ptr(_) => todo!(),
                VarType::Array(_, _) => todo!(),
                VarType::Struct(_, _) => todo!(),
                VarType::Unknown => todo!(),
                VarType::Function(_, _, _) => todo!(),
            },

            VarType::Array(_, _) => todo!(),
            VarType::Struct(_, _) => todo!(),
            VarType::Unknown => todo!(),
            VarType::Function(_, _, _) => todo!(),
        };

        self.extend_current_label(instructions);
    }
}
