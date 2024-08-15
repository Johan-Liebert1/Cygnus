use std::{cell::RefCell, rc::Rc};

use crate::{ast::variable::Variable, lexer::types::VarType, semantic_analyzer::semantic_analyzer::CallStack};

use super::asm::ASM;

pub const FUNCTION_RETURN_INSTRUCTIONS: [&str; 3] = [("mov rsp, rbp"), ("pop rbp"), ("ret")];

pub const FUNCTION_ARGS_REGS: [&str; 7] = ["rax", "rdi", "rsi", "rdx", "r10", "r8", "r9"];

impl ASM {
    pub fn function_call(&mut self, function_name: &String, num_args: usize, func_return_type: &VarType) {
        let mut instructions = vec![];

        for i in 0..num_args {
            instructions.push(format!("pop {}", FUNCTION_ARGS_REGS[i]))
        }

        instructions.push(format!("call _{function_name}"));

        // if the function returns anything, push it onto the stack
        if !matches!(func_return_type, VarType::Unknown) {
            instructions.push(format!("push rax"));
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
            format!("sub rsp, {}", local_var_size),
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

    pub fn function_def_end(&mut self, _function_name: &String) {
        // mov rsp, rbp        ; Reset stack pointer
        // pop rbp             ; Restore old base pointer

        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
        self.change_current_label("_start".into());
    }

    pub fn function_return(&mut self, return_value_exists: bool) {
        self.add_to_current_label(format!("pop rax"));
        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
    }
}
