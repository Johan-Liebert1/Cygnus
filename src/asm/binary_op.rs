use crate::lexer::{
    lexer::Token,
    registers::{get_register_name_for_bits, Register},
    tokens::Operations,
    types::VarType,
};

use super::asm::ASM;

impl ASM {
    fn clear_registers_if_needed(&self, reg: Register, instructions: &mut Vec<String>) {
        // We need to clear only if we're not using all 64 bits of the register
        // Also, not xoring here as we don't want to set some unnecessary flags
        if !reg.is_64bit_reg() {
            instructions.push(format!("mov {}, 0 ;; clearing {reg}", reg.get_64bit_version()))
        }
    }

    pub fn binary_op_nums(
        &mut self,
        op: Operations,
        times_dereferenced: usize,
        result_type: &VarType,
        token: &Token,
        left_type: &VarType,
        right_type: &VarType,
    ) {
        let mut reg_to_lock: Register;

        let mut instructions = match op {
            Operations::Plus => {
                if matches!(result_type, VarType::Float) {
                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let xmm0 = self.get_free_float_register(None);
                    let xmm1 = self.get_free_float_register(None);

                    let inst = vec![
                        format!(";; Plus get the two operands from the stack"),
                        format!("movsd {xmm0}, {}", right),
                        format!("movsd {xmm1}, {}", left),
                        format!("addsd {xmm0}, {xmm1}"),
                    ];

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    self.unlock_register(xmm1);

                    reg_to_lock = xmm0;

                    inst
                } else {
                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let rax = self.get_free_register(None);
                    let rbx = self.get_free_register(None);

                    let rax_actual = right_type.get_register_name(rax);
                    let rbx_actual = left_type.get_register_name(rbx);

                    let mut inst = vec![format!(
                        ";; Plus get the two operands from the stack. Result type: {result_type}. Token: {token:?}"
                    )];

                    self.clear_registers_if_needed(rax_actual, &mut inst);
                    self.clear_registers_if_needed(rbx_actual, &mut inst);

                    inst.extend(vec![
                        format!("mov {rax_actual}, {}", right),
                        format!("mov {rbx_actual}, {}", left),
                        format!("add {rax_actual}, {rbx_actual}"),
                        format!(
                            ";; will lock {rax}. first = {right}. second = {left}. Locked: {:?}",
                            self.get_used_registers()
                        ),
                    ]);

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    self.unlock_register(rbx);

                    reg_to_lock = rax_actual.into();

                    inst
                }
            }

            Operations::Minus => {
                if matches!(result_type, VarType::Float) {
                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let xmm0 = self.get_free_float_register(None);
                    let xmm1 = self.get_free_float_register(None);

                    let inst = vec![
                        format!(";; Minus get the two operands from the stack"),
                        format!("movsd {xmm0}, {}", left),
                        format!("movsd {xmm1}, {}", right),
                        format!("subsd {xmm0}, {xmm1}"),
                    ];

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    self.unlock_register(xmm1);

                    reg_to_lock = xmm0;

                    inst
                } else {
                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let rax = self.get_free_register(None);
                    let rbx = self.get_free_register(None);

                    let rax_actual = left_type.get_register_name(rax);
                    let rbx_actual = right_type.get_register_name(rbx);

                    let mut inst = vec![format!(
                        ";; Minus get the two operands from the stack. Result type: {result_type}. Token: {token:?}"
                    )];

                    self.clear_registers_if_needed(rax_actual, &mut inst);
                    self.clear_registers_if_needed(rbx_actual, &mut inst);

                    inst.extend(vec![
                        format!("mov {rax_actual}, {}", left),
                        format!("mov {rbx_actual}, {}", right),
                        format!("sub {rax_actual}, {rbx_actual}"),
                        format!(
                            ";; will lock {rax}. first = {right}. second = {left}. Locked: {:?}",
                            self.get_used_registers()
                        ),
                    ]);

                    self.unlock_register(rbx);

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    reg_to_lock = rax_actual.into();

                    inst
                }
            }

            Operations::Divide => {
                if matches!(result_type, VarType::Float) {
                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let xmm0 = self.get_free_float_register(None);
                    let xmm1 = self.get_free_float_register(None);

                    let inst = vec![
                        format!(";; Plus get the two operands from the stack"),
                        format!("movsd {xmm0}, {}", left),
                        format!("movsd {xmm1}, {}", right),
                        format!("divsd {xmm0}, {xmm1}"),
                    ];

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    self.unlock_register(xmm1);

                    reg_to_lock = xmm0;

                    inst
                } else {
                    let mut instructions = vec![];

                    // cannot use rdx here as it will get cleared on multiplication
                    let regs_to_skip = vec![Register::RDX];

                    // We need 'rax' to be free here for the multiplication
                    let rax = if self.is_reg_locked(Register::RAX) {
                        let rbx = self.get_free_register(Some(&regs_to_skip));

                        instructions.extend(vec![format!(
                            "mov {rbx}, rax ;; moving rax into {rbx} as rax is needed"
                        )]);

                        self.replace_reg_on_stack(Register::RAX, rbx);

                        self.unlock_register(Register::RAX);

                        self.get_free_register(Some(&regs_to_skip))
                    } else {
                        self.get_free_register(Some(&regs_to_skip))
                    };

                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let rbx = self.get_free_register(Some(&regs_to_skip));

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    let rax_actual = right_type.get_register_name(rax);
                    let rbx_actual = left_type.get_register_name(rbx);

                    instructions.extend(vec![
                        // 40 / 5
                        // push 40
                        // push 5
                        format!(";; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'"),
                        format!("xor rdx, rdx"),
                    ]);

                    self.clear_registers_if_needed(rax_actual, &mut instructions);
                    self.clear_registers_if_needed(rbx_actual, &mut instructions);

                    instructions.extend(vec![
                        format!(";; get the two operands from the stack"),
                        format!("mov {rbx_actual}, {}", right),
                        format!("mov {rax_actual}, {}", left),
                        format!("div {rbx}"),
                    ]);

                    self.unlock_register(rbx);

                    reg_to_lock = rax_actual.into();

                    instructions
                }
            }

            Operations::Multiply => {
                if matches!(result_type, VarType::Float) {
                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let xmm0 = self.get_free_float_register(None);
                    let xmm1 = self.get_free_float_register(None);

                    let inst = vec![
                        format!(";; Plus get the two operands from the stack"),
                        format!("movsd {xmm0}, {}", right),
                        format!("movsd {xmm1}, {}", left),
                        format!("mulsd {xmm0}, {xmm1}"),
                    ];

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    self.unlock_register(xmm1);

                    reg_to_lock = xmm0;

                    inst
                } else {
                    let mut instructions = vec![];

                    // cannot use rdx here as it will get cleared on multiplication
                    let regs_to_skip = vec![Register::RDX];

                    // We need 'rax' to be free here for the multiplication
                    let rax = if self.is_reg_locked(Register::RAX) {
                        let rbx = self.get_free_register(Some(&regs_to_skip));

                        instructions.extend(vec![format!("mov {rbx}, rax ;; Saving rax")]);

                        self.replace_reg_on_stack(Register::RAX, rbx);

                        self.unlock_register(Register::RAX);

                        self.get_free_register(Some(&regs_to_skip))
                    } else {
                        self.get_free_register(Some(&regs_to_skip))
                    };

                    let right = self.stack_pop().unwrap();
                    let left = self.stack_pop().unwrap();

                    let rbx = self.get_free_register(Some(&regs_to_skip));

                    let rax_actual = right_type.get_register_name(rax);
                    let rbx_actual = left_type.get_register_name(rbx);

                    instructions.extend(vec![
                        format!(";; Multiply get the two operands from the stack. Type: {result_type}"),
                        format!("xor rdx, rdx"),
                    ]);

                    self.clear_registers_if_needed(rax_actual, &mut instructions);
                    self.clear_registers_if_needed(rbx_actual, &mut instructions);

                    instructions.extend(vec![
                        format!("mov {rax_actual}, {}", right),
                        format!("mov {rbx_actual}, {}", left),
                        format!("mul {rbx}"),
                    ]);

                    self.unlock_register_from_stack_value(&right);
                    self.unlock_register_from_stack_value(&left);

                    self.unlock_register(rbx);

                    reg_to_lock = rax_actual.into();

                    instructions
                }
            }

            Operations::ShiftLeft => {
                // We need rcx here
                // the shl instruction in x86 assembly does not allow the use of a general-purpose register like bl
                // for the count operand when the destination is a 64-bit register like rax.
                // The count operand must be either an immediate value (a constant) or the cl register.

                let mut instructions = vec![];

                let rcx = if self.is_reg_locked(Register::RCX) {
                    let rbx = self.get_free_register(None);

                    instructions.extend(vec![format!("mov {rbx}, rcx")]);

                    self.replace_reg_on_stack(Register::RCX, rbx);

                    self.unlock_register(Register::RCX);

                    self.get_certain_free_register(Register::RCX)
                } else {
                    self.get_certain_free_register(Register::RCX)
                };

                let rax = self.get_free_register(None);

                let first = self.stack_pop().unwrap();
                let shift_by = self.stack_pop().unwrap();

                self.unlock_register_from_stack_value(&first);
                self.unlock_register_from_stack_value(&shift_by);

                // 1 << 10
                // push 1
                // push 10
                // 1 should be popped into rax and 10 in rcx
                instructions.extend(vec![
                    format!(";; ShiftLeft get the two operands from the stack"),
                    format!("xor {rax}, {rax}"),
                    format!("xor {rcx}, {rcx}"),
                    format!("mov {rax}, {first}"),
                    format!("mov {rcx}, {shift_by}"),
                    format!(";; We can only shift left or right by 8 bits"),
                    format!("shl {rax}, cl"),
                ]);

                self.unlock_register(Register::RCX);

                reg_to_lock = rax;

                instructions
            }

            Operations::ShiftRight => {
                // We need rcx here
                // the shr instruction in x86 assembly does not allow the use of a general-purpose register like bl
                // for the count operand when the destination is a 64-bit register like rax.
                // The count operand must be either an immediate value (a constant) or the cl register.

                let mut instructions = vec![];

                let rcx = if self.is_reg_locked(Register::RCX) {
                    let rbx = self.get_free_register(None);

                    instructions.extend(vec![format!("mov {rbx}, rcx")]);

                    self.replace_reg_on_stack(Register::RCX, rbx);

                    self.unlock_register(Register::RCX);

                    self.get_certain_free_register(Register::RCX)
                } else {
                    self.get_certain_free_register(Register::RCX)
                };

                let rax = self.get_free_register(None);

                let first = self.stack_pop().unwrap();
                let shift_by = self.stack_pop().unwrap();

                self.unlock_register_from_stack_value(&first);
                self.unlock_register_from_stack_value(&shift_by);

                // 1 << 10
                // push 1
                // push 10
                // 1 should be popped into rax and 10 in rcx
                instructions.extend(vec![
                    format!(";; ShiftLeft get the two operands from the stack"),
                    format!("xor {rax}, {rax}"),
                    format!("xor {rcx}, {rcx}"),
                    format!("mov {rax}, {first}"),
                    format!("mov {rcx}, {shift_by}"),
                    format!(";; We can only shift left or right by 8 bits"),
                    format!("shr {rax}, cl"),
                ]);

                self.unlock_register(Register::RCX);

                reg_to_lock = rax;

                instructions
            }

            Operations::Modulo => {
                let mut instructions = vec![];

                let regs_to_skip = vec![Register::RDX];

                // We need 'rax' to be free here for the multiplication
                let rax = if self.is_reg_locked(Register::RAX) {
                    let rbx = self.get_free_register(Some(&regs_to_skip));

                    // trace!(
                    //     "RAX was locked so rbx = {rbx}. Used registers: {:#?}",
                    //     self.get_used_registers()
                    // );

                    instructions.extend(vec![format!("mov {rbx}, rax")]);

                    self.replace_reg_on_stack(Register::RAX, rbx);

                    self.unlock_register(Register::RAX);

                    self.get_free_register(Some(&regs_to_skip))
                } else {
                    self.get_free_register(Some(&regs_to_skip))
                };

                // We need 'rdx' to be free here for the modulo
                let rdx = if self.is_reg_locked(Register::RDX) {
                    let rbx = self.get_free_register(Some(&regs_to_skip));

                    instructions.extend(vec![format!("mov {rbx}, rdx")]);

                    self.replace_reg_on_stack(Register::RDX, rbx);

                    self.unlock_register(Register::RDX);

                    self.get_certain_free_register(Register::RDX)
                } else {
                    self.get_certain_free_register(Register::RDX)
                };

                let first = self.stack_pop().unwrap();
                let second = self.stack_pop().unwrap();

                let rbx = self.get_free_register(Some(&regs_to_skip));

                instructions.extend(vec![
                    format!(";; Modulo get the two operands from the stack"),
                    format!("xor {rdx}, {rdx}"),
                    format!("mov {rbx}, {first}"),
                    format!("mov {rax}, {second}"),
                    format!("div {rbx}"),
                ]);

                reg_to_lock = rdx;

                self.unlock_register_from_stack_value(&first);
                self.unlock_register_from_stack_value(&second);

                self.unlock_register(rbx);
                self.unlock_register(rax);

                instructions
            }
        };

        // Don't need to lock as get_free_register locks it
        //
        // We might've preformed OP on two int8s, but casted it to an integer
        // So, we need to push the register that will hold an integer

        let actual_reg_to_lock = Register::from(result_type.get_register_name(reg_to_lock));

        // if actual_reg_to_lock != reg_to_lock {
        //     let bit64_reg = actual_reg_to_lock.get_64bit_version();

        //     instructions.extend(vec![
        //         format!("push {bit64_reg}"),
        //         format!("mov {bit64_reg}, 0"),
        //         format!("pop {bit64_reg}")
        //     ]);
        // }

        reg_to_lock = actual_reg_to_lock;

        // result will always be in rax
        // We will also never dereference a string as we want the character address
        match result_type {
            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Char | VarType::Float => {
                self.stack_push(String::from(reg_to_lock));
            }

            VarType::Str => todo!(),

            VarType::Ptr(type_) => match **type_ {
                VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                    instructions.extend(
                        std::iter::repeat(format!("mov {reg_to_lock}, [{reg_to_lock}]")).take(times_dereferenced),
                    );

                    self.stack_push(String::from(reg_to_lock));
                }

                VarType::Char => {
                    instructions.push(format!(";; binary op ptr -> char"));

                    if times_dereferenced > 0 {
                        let rbx = self.get_free_register(None);

                        let reg_to_lock_actual = get_register_name_for_bits(&reg_to_lock, 8);

                        instructions.extend(vec![
                            format!("mov {rbx}, {reg_to_lock}"),
                            format!("xor {reg_to_lock}, {reg_to_lock}"),
                            format!("mov {reg_to_lock_actual}, [{rbx}]",),
                        ]);

                        self.unlock_register(rbx);

                        self.stack_push(String::from(reg_to_lock_actual));
                    } else {
                        self.stack_push(String::from(reg_to_lock));
                    }
                }

                VarType::Str => {
                    instructions.push(format!(";; binary op ptr -> str"));

                    if times_dereferenced > 0 {
                        let rbx = self.get_free_register(None);

                        instructions.extend(vec![
                            format!("mov {rbx}, {reg_to_lock}"),
                            format!("xor {reg_to_lock}, {reg_to_lock}"),
                            format!("xor {reg_to_lock}, [{rbx}]"),
                        ]);
                        self.unlock_register(rbx);

                        self.stack_push(String::from(reg_to_lock));
                    } else {
                        self.stack_push(String::from(reg_to_lock));
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
