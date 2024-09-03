use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::variable::Variable,
    lexer::{
        registers::{Register, ALL_REGISTERS},
        types::VarType,
    },
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
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

impl ASM {
    pub fn function_call_prep(&mut self) {
        // Not removing the regs from the stack as we restore them after the function call

        let mut saved_regs = vec![];

        for reg in ALL_REGISTERS {
            if self.is_reg_locked(reg) {
                self.extend_current_label(vec![format!(";; Saving register {reg}'s value"), format!("push {reg}")]);

                saved_regs.push(reg);

                self.unlock_register(reg);
            }
        }

        self.regs_saved_for_function_call.push(saved_regs);

        self.regs_locked_for_func_args.push(vec![]);
    }

    pub fn function_call_add_arg(&mut self, arg_num: usize) {
        trace!("regs: {:#?}", self.get_used_registers());

        let mut instructions = vec![];

        let mut arg_reg = FUNCTION_ARGS_REGS[arg_num];

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

    pub fn function_call(
        &mut self,
        function_name: &String,
        num_args: usize,
        func_return_type: &VarType,
        is_function_pointer_call: bool,
        call_stack: &CallStack,
        is_extern_func: bool,
        is_assigned_to_var: bool,
    ) {
        let mut instructions = vec![];

        // if the function returns something we need to save rax
        // else it'll be overwritten by the return value of the function
        if !matches!(func_return_type, VarType::Unknown) {}

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

        // if the function returns anything, push it onto the stack
        if !matches!(func_return_type, VarType::Unknown) && is_assigned_to_var {
            let rax = if self.is_reg_locked(Register::RAX) {
                let rbx = self.get_free_register(None);

                instructions.push(format!("mov {rbx}, rax"));

                self.unlock_register(Register::RAX);

                self.get_free_register(None)
            } else {
                self.get_free_register(None)
            };

            self.stack_push(String::from(rax));
        }

        match self.regs_saved_for_function_call.pop() {
            Some(saved_regs) => {
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

        for (var, register) in func_params.iter().zip(FUNCTION_ARGS_REGS) {
            // we have to get the variable from the call stack as call stack's where we set the
            // variable offset and stuff. The func_params themselves all have an offset of 0

            let (call_stack_var, _) = call_stack.get_var_with_name(&var.borrow().var_name);

            match call_stack_var {
                Some(var) => {
                    instructions.extend([
                        format!(";; param name {}", var.borrow().var_name),
                        format!("mov [rbp - {}], {}", var.borrow().offset, register),
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

    pub fn function_def_end(&mut self, _function_name: &String) {
        // mov rsp, rbp        ; Reset stack pointer
        // pop rbp             ; Restore old base pointer

        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
        self.change_current_label("_start".into());
    }

    pub fn function_return(&mut self, return_value_exists: bool) {
        // move the return value into rax
        if return_value_exists {
            let stack_member = self.stack_pop().unwrap();

            // not locking here should be fine
            self.add_to_current_label(format!("mov {}, {stack_member}", Register::RAX));

            self.unlock_register_from_stack_value(&stack_member);
        }

        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
    }
}
