use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::variable::Variable,
    lexer::{
        registers::{Register, ALL_64BIT_REGISTERS, ALL_FP_REGISTERS},
        types::VarType,
    },
    semantic_analyzer::semantic_analyzer::CallStack,
};

use super::asm::ASM;

pub const FUNCTION_RETURN_INSTRUCTIONS: [&str; 3] = [("mov rsp, rbp"), ("pop rbp"), ("ret")];

pub const FUNCTION_ARGS_REGS: [Register; 6] = [
    Register::RDI,
    Register::RSI,
    Register::RDX,
    Register::RCX,
    Register::R8,
    Register::R9,
];

pub const FUNCTION_FLOAT_ARGS_REGS: [Register; 8] = [
    Register::XMM0,
    Register::XMM1,
    Register::XMM2,
    Register::XMM3,
    Register::XMM4,
    Register::XMM5,
    Register::XMM6,
    Register::XMM7,
];

impl ASM {
    /// Checks if registers required for x86 function call already have some value in them
    /// If they have, then saves it onto the x86 stack
    pub fn function_call_prep(&mut self) {
        // Not removing the regs from the stack as we restore them after the function call
        let mut saved_regs = vec![];

        for reg in ALL_64BIT_REGISTERS {
            if self.is_reg_locked(reg) {
                self.extend_current_label(vec![
                    format!(";; Saving non float register {reg}'s value"),
                    format!("push {reg}"),
                ]);

                saved_regs.push(reg);

                // unlock this register as we've saved its value and it can be used
                self.unlock_register(reg);
            }
        }

        let rax = self.get_free_register(None);

        for reg in ALL_FP_REGISTERS {
            if self.is_reg_locked(reg) {
                self.extend_current_label(vec![
                    format!(";; Saving float register {reg}'s value"),
                    format!("movsd {rax}, {reg}"),
                    format!("push {rax}"),
                ]);

                saved_regs.push(reg);

                // unlock this register as we've saved its value and it can be used
                self.unlock_register(reg);
            }
        }

        self.unlock_register(rax);

        self.regs_saved_for_function_call.push(saved_regs);
        self.regs_locked_for_func_args.push(vec![]);
    }

    fn handle_non_float_function_call_arg(&mut self, arg_num: usize) {
        let mut instructions = vec![];

        let arg_reg = FUNCTION_ARGS_REGS[arg_num];

        let stack_member = self.stack_pop().unwrap();

        instructions.extend(vec![
            format!(";; Moving argument number {}", arg_num + 1),
            format!("mov {}, {}", arg_reg, stack_member),
        ]);

        self.unlock_register_from_stack_value(&stack_member);

        self.lock_register(arg_reg);

        match self.regs_locked_for_func_args.last_mut() {
            Some(last_mut) => last_mut.push(arg_reg),

            None => {
                let vec = vec![arg_reg];
                self.regs_locked_for_func_args.push(vec);
            }
        }

        self.extend_current_label(instructions);
    }

    fn handle_float_function_call_arg(&mut self, arg_num: usize) {
        let stack_member = self.stack_pop().unwrap();

        let arg_reg = FUNCTION_FLOAT_ARGS_REGS[arg_num];

        self.extend_current_label(vec![
            format!(";; Moving float argument number {}", arg_num + 1),
            format!("movsd {arg_reg}, {stack_member}"),
        ]);

        self.unlock_register_from_stack_value(&stack_member);

        self.lock_register(arg_reg);

        match self.regs_locked_for_func_args.last_mut() {
            Some(last_mut) => last_mut.push(arg_reg),

            None => {
                let vec = vec![arg_reg];
                self.regs_locked_for_func_args.push(vec);
            }
        }
    }

    /// arg_reg = non_float or the float arg num
    /// func(1, 2, 3, 4.5)
    ///
    /// arg num for 1 = 0, 2 = 1, 3 = 2, 4.5 = 1
    pub fn function_call_add_arg(&mut self, arg_num: usize, arg_type: VarType) {
        if matches!(arg_type, VarType::Float) {
            return self.handle_float_function_call_arg(arg_num);
        }

        self.handle_non_float_function_call_arg(arg_num);
    }

    /// Restores registers that were saved for the function call.
    /// Unlocks all registers in 'regs_locked_for_func_args' and 'regs_saved_for_function_call'
    pub fn function_call_register_restore(&mut self, instructions: &mut Vec<String>) {
        match self.regs_locked_for_func_args.pop() {
            Some(locked_regs) => {
                for reg in locked_regs {
                    self.unlock_register(reg);
                }
            }

            None => {
                // Nothing. No register was locked, i.e. func with no args
            }
        }

        match self.regs_saved_for_function_call.pop() {
            Some(saved_regs) => {
                // Iterating in reverse as we had pushed these into the vec treating it as a stack
                for reg in saved_regs.iter().rev() {
                    self.lock_register(*reg);

                    instructions.extend(vec![
                        format!(";; popping saved register value into {reg}"),
                        format!("pop {reg}"),
                    ]);
                }
            }

            None => {
                // Nothing. No register was saved
            }
        }
    }

    pub fn handle_function_return_value(
        &mut self,
        instructions: &mut Vec<String>,
        function_name: &String,
        func_return_type: &VarType,
    ) {
        // if the function returns something we need to save rax
        // else it'll be overwritten by the return value of the function
        // if the function returns anything, push it onto the stack
        if !matches!(func_return_type, VarType::Unknown) {
            // Also, here we check if rax was saved onto the stack before this call
            // if it was we will save the return value to another register as we will
            // need to restore the pushed value into rax
            let is_rax_locked = self.is_reg_locked(Register::RAX);

            let is_rax_saved = self
                .regs_saved_for_function_call
                .last()
                .unwrap_or(&vec![])
                .contains(&Register::RAX);

            if !is_rax_locked && !is_rax_saved {
                // Return value is in RAX
                // Simply push it onto the stack

                self.lock_register(Register::RAX);
                self.stack_push(String::from(Register::RAX));
                return;
            }

            // could be the fact that RAX is not locked, but saved
            // in which case `get_free_register` will return RAX but we need to prevent that
            let rbx = self.get_free_register(Some(&vec![Register::RAX]));

            instructions.extend(vec![
                format!(";; Moving function '{function_name}' return value"),
                format!("mov {rbx}, rax"),
            ]);

            // if is_rax_locked {
            //     self.unlock_register(Register::RAX);
            // }

            self.stack_push(String::from(rbx));
        }
    }

    pub fn function_call(
        &mut self,
        function_name: &String,
        _num_args: usize,
        func_return_type: &VarType,
        is_function_pointer_call: bool,
        call_stack: &CallStack,
        is_extern_func: bool,
        is_assigned_to_var: bool,
    ) {
        let mut instructions = vec![];

        if !is_function_pointer_call {
            if is_extern_func {
                instructions.push(format!("call {function_name}"));
            } else {
                instructions.push(format!("call _{function_name}"));
            }
        } else {
            // we get the function pointer stored on the stack and call that
            // the function name is the variable name of the function pointer

            let (ar_var, _) = call_stack.get_var_with_name(function_name);

            let rax = self.get_free_register(None);

            if let Some(ar_var) = ar_var {
                instructions.extend(vec![
                    format!("mov {rax}, [rbp - {}]", ar_var.borrow().offset),
                    format!("call {rax}"),
                ]);

                self.unlock_register(rax);
            } else {
                unreachable!("Function pointer variable '{function_name}' not found on call stack. This must be a bug in the semantic analysis step.");
            }
        }

        if is_assigned_to_var {
            self.handle_function_return_value(&mut instructions, function_name, func_return_type);
        }

        self.function_call_register_restore(&mut instructions);

        self.extend_current_label(instructions);
    }

    pub fn function_def(
        &mut self,
        call_stack: &CallStack,
        function_name: &String,
        local_var_size: usize,
        func_params: &Vec<Rc<RefCell<Variable>>>,
    ) {
        self.change_current_label(format!("_{function_name}"));

        // push rbp            ; Save old base pointer
        // mov rbp, rsp        ; Set base pointer to current stack pointer
        // sub rsp, 16         ; Allocate 16 bytes for local variables

        let mut instructions = vec![
            format!("push rbp"),
            format!("mov rbp, rsp"),
            format!(";; Make sure that the stack pointer is 16 byte aligned always"),
            format!("sub rsp, {}", local_var_size + local_var_size % 16),
        ];

        let mut float_arg_num: i32 = -1;
        let mut non_float_arg_num: i32 = -1;

        for var in func_params.iter() {
            // we have to get the variable from the call stack as call stack's where we set the
            // variable offset and stuff. The func_params themselves all have an offset of 0

            let (call_stack_var, _) = call_stack.get_var_with_name(&var.borrow().var_name);

            match call_stack_var {
                Some(var) => {
                    let (operation, register) = if matches!(var.borrow().var_type, VarType::Float) {
                        float_arg_num += 1;
                        ("movsd", FUNCTION_FLOAT_ARGS_REGS[float_arg_num as usize])
                    } else {
                        non_float_arg_num += 1;
                        ("mov", FUNCTION_ARGS_REGS[non_float_arg_num as usize])
                    };

                    instructions.extend([
                        format!(
                            ";; param name {}. Param type {}",
                            var.borrow().var_name,
                            var.borrow().var_type
                        ),
                        format!("{operation} [rbp - {}], {}", var.borrow().offset, register),
                    ]);
                }

                None => unreachable!(
                    "Did not find variable {} in the call stack. Must be a bug in function definition AST",
                    var.borrow().var_name
                ),
            }
        }

        self.extend_current_label(instructions);
    }

    pub fn extern_function_def(&mut self, function_name: &String) {
        self.data.push(format!("extern {function_name}"))
    }

    pub fn function_def_end(&mut self) {
        // mov rsp, rbp        ; Reset stack pointer
        // pop rbp             ; Restore old base pointer
        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
        self.change_current_label("_start".into());
    }

    pub fn function_return(&mut self, return_value_exists: bool) {
        // move the return value into rax
        if return_value_exists {
            let stack_member = self.stack_pop().unwrap();

            // We won't check if RAX is locked or not before inserting the return value in it
            self.extend_current_label(vec![
                format!(";; Returning from function"),
                format!("mov {}, {stack_member}", Register::RAX),
            ]);

            // We're only locking RAX once here as there could be multiple return statements
            // and locking RAX in the first return will have this function panic when
            // it tries to lock RAX for the second return

            // if !self.is_reg_locked(Register::RAX) {
            //     self.lock_register(Register::RAX)
            // }

            // Only unlock if the register was not rax
            self.unlock_register_from_stack_value(&stack_member);
        }

        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
    }

    pub fn debug_all_variable_offsets_for_function(&mut self, call_stack: &CallStack) {
        if let Some(ar) = call_stack.peek() {
            for (_, variable) in &ar.variable_members {
                let var_borrow = variable.borrow();

                self.add_to_current_label(format!(
                    ";; '{}' at '[rbp - {}]'",
                    var_borrow.var_name, var_borrow.offset
                ));
            }
        }
    }
}
