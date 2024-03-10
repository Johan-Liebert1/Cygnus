use core::panic;

use crate::{
    ast::variable::Variable,
    interpreter::interpreter::Variables,
    lexer::{
        tokens::VariableEnum,
        types::{VarType, TYPE_INT, TYPE_STRING},
    },
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
};

use super::{asm::ASM, functions::FUNCTION_ARGS_REGS};

const WRITE_STRING_ASM_INSTRUCTIONS: [&str; 9] = [
    ";; Assuming length is pushed last",
    "pop r8",
    ";; Assuming string address is pushed first",
    "pop r9",
    "mov rax, 1",
    "mov rdi, 1",
    "mov rsi, r9",
    "mov rdx, r8",
    "syscall",
];

impl ASM {
    pub fn func_write_number(&mut self) {
        self.extend_current_label(vec![String::from("pop rax"), String::from("call _printRAX")]);
    }

    pub fn func_exit(&mut self) {
        self.extend_current_label(vec![format!("pop rdi"), format!("mov rax, 60"), format!("syscall")]);
    }

    pub fn func_write_string(&mut self) {
        self.extend_current_label(WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec());
    }

    pub fn func_write_pointer(&mut self, pointer_var_type: &Box<VarType>, times_dereferenced: usize) {
        let vec = self.func_write_pointer_internal(pointer_var_type, times_dereferenced);
        self.extend_current_label(vec);
    }

    fn func_write_pointer_internal(
        &mut self,
        pointer_var_type: &Box<VarType>,
        times_dereferenced: usize,
    ) -> Vec<String> {
        match **pointer_var_type {
            VarType::Int => {
                vec![format!("pop rax"), format!("call _printRAX")]
            }

            // TODO: Check here whether the pointer is dereferenced or not
            VarType::Str => {
                if times_dereferenced > 0 {
                    WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec()
                } else {
                    vec![format!("pop rax"), format!("call _printRAX")]
                }
            }

            VarType::Char => {
                if times_dereferenced > 0 {
                    WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec()
                } else {
                    vec![format!("pop rax"), format!("call _printRAX")]
                }
            }

            _ => panic!("Unknown type '{pointer_var_type}'"),
        }
    }

    pub fn func_syscall(&mut self, num_args: usize) {
        let mut instructions = vec![];

        for i in 0..num_args {
            instructions.push(format!("pop {}", FUNCTION_ARGS_REGS[i]));
        }

        instructions.push("syscall".into());

        self.extend_current_label(instructions);
    }

    pub fn func_write_var(&mut self, var: &Variable) {
        let var_name = &var.var_name;

        let instructions = match var_name.as_str() {
            "argc" => vec![
                // argc contains the address of rsp
                format!("mov rax, [argc]"),
                // since argc contained the address, we need to dereference it again to get the
                // actual value
                format!("mov rax, [rax]"),
                format!("call _printRAX"),
            ],

            "argv" => vec![
                // argc contains the address of rsp
                format!("mov rax, [argc]"),
                // since argc contained the address, we need to dereference it again to get the
                // actual value
                format!("mov rax, [rax + 8]"),
                // now rax has the string
                // we need to calculate its length
                // it's NULL terminated

                // save rax
                format!("push rax"),
                format!("mov rbx, 0"),
                format!(".strlen:"),
                format!("inc rax"),
                format!("inc rbx"),
                format!("mov cl, [rax]"),
                format!("cmp cl, 0"),
                format!("jne .strlen"),
                format!("push rbx"),
                format!("pop r8"),
                format!("pop r9"),
                format!("mov rax, 1"),
                format!("mov rdi, 1"),
                format!("mov rsi, r9"),
                format!("mov rdx, r8"),
                format!("syscall"),
            ],

            _ => {
                // the variable value or its address will be pushed onto the stack
                // We don't need to check the scope here as the variable value is already
                // pushed into rax beforehand in `factor` AST
                match &var.var_type {
                    VarType::Int => {
                        vec![format!("pop rax"), format!("call _printRAX")]
                    }

                    VarType::Str => WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec(),

                    VarType::Char => {
                        let mut a = vec![format!("push 1")];
                        a.extend(WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec());

                        a
                    }

                    VarType::Ptr(pointer_var_type) => {
                        self.func_write_pointer_internal(pointer_var_type, var.times_dereferenced)
                    }

                    VarType::Float => todo!(),

                    VarType::Array(..) => {
                        vec![format!("pop rax"), format!("call _printRAX")]
                    }

                    VarType::Unknown => todo!(),
                }
            }
        };

        self.extend_current_label(instructions);
    }
}
