use crate::{
    ast::variable::Variable,
    helpers::compiler_error,
    interpreter::interpreter::Variables,
    lexer::{
        tokens::VariableEnum,
        types::{VarType, TYPE_INT, TYPE_STRING},
    },
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
};

use super::{
    asm::ASM,
    functions::{FUNCTION_ARGS_REGS, SYSCALL_ARGS_REGS},
};

// const WRITE_STRING_ASM_INSTRUCTIONS: [&str; 9] = [
//     ";; Assuming length is pushed last",
//     "pop r8",
//     ";; Assuming string address is pushed first",
//     "pop r9",
//     "mov rax, 1",
//     "mov rdi, 1",
//     "mov rsi, r9",
//     "mov rdx, r8",
//     "syscall",
// ];

const WRITE_CHAR_ASM_INSTRUCTIONS: [&str; 8] = [
    ";; Writing a character",
    "mov r8, 1",
    "pop r9",
    "mov rax, 1",
    "mov rdi, 1",
    "mov rsi, r9",
    "mov rdx, r8",
    "syscall",
];

impl ASM {
    fn get_vec_for_write_number(&mut self) -> Vec<String> {
        // we pop this anyway because in binary op we push "rax" to stack no matter what
        let stack_member = self.stack.pop().unwrap();

        vec![
            format!("mov rax, {}", stack_member),
            String::from("call _printRAX"),
        ]
    }

    pub fn func_write_number(&mut self, is_binary_op_result: bool) {
        let vec = self.get_vec_for_write_number();
        self.extend_current_label(vec);

        // if !is_binary_op_result {
        //     self.extend_current_label(vec![
        //         format!("mov rax, {}", stack_member),
        //         String::from("call _printRAX"),
        //     ]);
        // } else {
        //     // the value is already in rax
        //     self.extend_current_label(vec![String::from("call _printRAX")]);
        // }
    }

    pub fn func_exit(&mut self) {
        self.extend_current_label(vec![format!("pop rdi"), format!("mov rax, 60"), format!("syscall")]);
    }

    pub fn func_write_string(&mut self) {
        // TODO: Remove
        //
        // self.extend_current_label(WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec());

        let str_len = self.stack.pop().unwrap();
        let str_addr = self.stack.pop().unwrap();

        self.extend_current_label(vec![
            "mov rax, 1".into(),
            "mov rdi, 1".into(),
            format!("mov rsi, {}", str_addr),
            format!("mov rdx, {}", str_len),
            "syscall".into(),
        ]);
    }

    pub fn func_write_pointer(
        &mut self,
        pointer_var_type: &Box<VarType>,
        times_dereferenced: usize,
        call_stack: &CallStack,
        var: Option<&Variable>,
    ) {
        let vec = self.func_write_pointer_internal(pointer_var_type, times_dereferenced, call_stack, var);
        self.extend_current_label(vec);
    }

    fn func_write_pointer_internal(
        &mut self,
        pointer_var_type: &Box<VarType>,
        times_dereferenced: usize,
        call_stack: &CallStack,
        variable: Option<&Variable>,
    ) -> Vec<String> {
        match **pointer_var_type {
            // a char is always represented as an 8 bit number
            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Char => {
                self.get_vec_for_write_number()
                // TODO: Remove
                //
                // vec![
                //     // format!("pop rax"),
                //     format!("call _printRAX"),
                // ]
            }

            // TODO: Check here whether the pointer is dereferenced or not
            VarType::Str => {
                if times_dereferenced > 0 {
                    // WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec()

                    let str_len = self.stack.pop().unwrap();
                    let str_addr = self.stack.pop().unwrap();

                    vec![
                        "mov rax, 1".into(),
                        "mov rdi, 1".into(),
                        format!("mov rsi, {}", str_addr),
                        format!("mov rdx, {}", str_len),
                        "syscall".into(),
                    ]
                } else {
                    self.get_vec_for_write_number()

                    // TODO: Remove
                    //
                    // vec![
                    //     // format!("pop rax"),
                    //     format!("call _printRAX"),
                    // ]
                }
            }

            // Might be pointer to a user defined type
            _ => {
                let user_defined_type = call_stack
                    .user_defined_types
                    .iter()
                    .find(|x| x.name == format!("{}", pointer_var_type));

                match user_defined_type {
                    Some(user_defined_type) => match &user_defined_type.type_ {
                        VarType::Struct(name, members) => {
                            let memebers_borrow = members.borrow();
                            let var = variable.expect("Expected a variable to be passed in");
                            let found = memebers_borrow.iter().find(|x| x.name == var.member_access[0]);

                            match found {
                                Some(struct_member) => match struct_member.member_type {
                                    VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                                        self.func_write_number(false);
                                        vec![]
                                    }
                                    VarType::Str => {
                                        self.func_write_string();
                                        vec![]
                                    }
                                    VarType::Float => todo!(),
                                    VarType::Char => todo!(),
                                    VarType::Ptr(_) => todo!(),
                                    VarType::Array(_, _) => todo!(),
                                    VarType::Struct(_, _) => todo!(),
                                    VarType::Unknown => todo!(),
                                    VarType::Function(_, _, _) => todo!(),
                                },

                                None => {
                                    panic!(
                                        "Member '{}' not present in struct '{}'",
                                        var.member_access[0], user_defined_type.type_
                                    );
                                }
                            }
                        }

                        type_ => {
                            panic!("Cannot have user defined primitive type: '{}'", type_);
                        }
                    },

                    None => {
                        panic!("Unknown type '{pointer_var_type}'")
                    }
                }
            }
        }
    }

    pub fn func_syscall(&mut self, num_args: usize) {
        let mut instructions = vec![];

        for i in 0..num_args {
            instructions.push(format!("pop {}", SYSCALL_ARGS_REGS[i]));
        }

        instructions.push("syscall".into());
        instructions.push("push rax".into());

        self.extend_current_label(instructions);
    }

    pub fn func_write_var(&mut self, var: &Variable, call_stack: &CallStack) {
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
                    VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                        // TODO: Remove
                        //
                        // vec![format!("pop rax"), format!("call _printRAX")]

                        let stack_member = self.stack.pop().unwrap();

                        vec![
                            format!("mov rax, {} {}", var.var_type.get_operation_size(), stack_member),
                            format!("call _printRAX"),
                        ]
                    }

                    VarType::Str => {
                        // TODO: Remove
                        // WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec()

                        let str_len = self.stack.pop().unwrap();
                        let str_addr = self.stack.pop().unwrap();

                        vec![
                            "mov rax, 1".into(),
                            "mov rdi, 1".into(),
                            format!("mov rsi, {}", str_addr),
                            format!("mov rdx, {}", str_len),
                            "syscall".into(),
                        ]
                    }

                    VarType::Char => WRITE_CHAR_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec(),

                    VarType::Ptr(pointer_var_type) => self.func_write_pointer_internal(
                        pointer_var_type,
                        var.times_dereferenced,
                        call_stack,
                        Some(var),
                    ),

                    VarType::Float => {
                        // TODO: This is just for testing
                        self.get_vec_for_write_number()
                        // vec![format!("pop rax"), format!("call _printRAX")]
                    }

                    VarType::Array(..) => {
                        self.get_vec_for_write_number()
                        // vec![format!("pop rax"), format!("call _printRAX")]
                    }

                    VarType::Struct(_, member_access) => {
                        let borrow = member_access.borrow();
                        let found = borrow.iter().find(|x| x.name == var.member_access[0]);

                        match found {
                            Some(struct_member_type) => match &struct_member_type.member_type {
                                VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                                    self.get_vec_for_write_number()
                                    // vec![
                                    //     // format!("pop rax"), 
                                    //     format!("call _printRAX")
                                    // ]
                                }

                                VarType::Str => {
                                    // TODO: Remove
                                    // WRITE_STRING_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec()

                                    let str_len = self.stack.pop().unwrap();
                                    let str_addr = self.stack.pop().unwrap();

                                    vec![
                                        "mov rax, 1".into(),
                                        "mov rdi, 1".into(),
                                        format!("mov rsi, {}", str_addr),
                                        format!("mov rdx, {}", str_len),
                                        "syscall".into(),
                                    ]
                                }

                                VarType::Ptr(var_type) => self.func_write_pointer_internal(
                                    var_type,
                                    var.times_dereferenced,
                                    call_stack,
                                    Some(var),
                                ),

                                VarType::Float => todo!(),
                                VarType::Char => todo!(),
                                VarType::Array(_, _) => todo!(),
                                VarType::Struct(_, _) => todo!(),
                                VarType::Unknown => todo!(),
                                VarType::Function(_, _, _) => todo!(),
                            },

                            None => unreachable!(
                                "Could not find memeber '{}' of struct while generating ASM",
                                var.member_access[0]
                            ),
                        }
                    }

                    VarType::Unknown => todo!(),
                    VarType::Function(_, _, _) => todo!(),
                }
            }
        };

        self.extend_current_label(instructions);
    }
}
