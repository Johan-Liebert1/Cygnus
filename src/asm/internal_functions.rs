use crate::{
    ast::variable::Variable,
    lexer::{registers::Register, types::VarType},
    semantic_analyzer::semantic_analyzer::CallStack,
};

use super::asm::ASM;

pub const SYSCALL_ARGS_REGS: [Register; 7] = [
    Register::RAX,
    Register::RDI,
    Register::RSI,
    Register::RDX,
    Register::R10,
    Register::R8,
    Register::R9,
];

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
    fn get_vec_for_write_number(&mut self, type_: VarType) -> Vec<String> {
        // we pop this anyway because in binary op we push "rax" to stack no matter what
        let stack_member = self.stack_pop().unwrap();

        self.unlock_register_from_stack_value(&stack_member);

        let mut instructions = vec![];

        if !Register::is_reg(&stack_member) || !Register::is(&Register::RAX, Register::from_string(&stack_member)) {
            instructions.extend(vec![
                format!(";; get_vec_for_write_number. stack_member: {stack_member}"),
                if matches!(type_, VarType::Int) {
                    format!("xor rax, rax")
                } else {
                    format!(";; no xor here")
                },
                format!("mov {}, {}", type_.get_register_name(Register::RAX), stack_member),
            ]);
        }

        // TODO: Also check here that there's nothing in rax
        instructions.push(String::from("call _printRAX"));

        instructions
    }

    pub fn func_write_number(&mut self, type_: VarType) {
        let vec = self.get_vec_for_write_number(type_);
        self.extend_current_label(vec);
    }

    pub fn func_write_float(&mut self) {
        let stack_member = self.stack_pop().unwrap();

        self.extend_current_label(vec![
            format!(";; write float"),
            format!("movsd [float_imm], {stack_member}"),
            format!("mov rax, [float_imm]"),
            format!("call _printRAX"),
        ]);

        self.unlock_register_from_stack_value(&stack_member);
    }

    pub fn func_exit(&mut self) {
        self.extend_current_label(vec![format!("pop rdi"), format!("mov rax, 60"), format!("syscall")]);
    }

    pub fn func_write_string(&mut self) {
        let mut str_len = self.stack_pop().unwrap();
        let mut str_addr = self.stack_pop().unwrap();

        let mut instructions = vec![];

        let used_regs = [Register::RAX, Register::RDI].to_vec();
        let used_regs_string: Vec<String> = used_regs.clone().into_iter().map(|x| String::from(x)).collect();

        if used_regs_string.contains(&str_len) {
            let rax = self.get_free_register(Some(&used_regs));
            self.unlock_register_from_stack_value(&str_len);

            instructions.push(format!("mov {rax}, {str_len}"));

            str_len = String::from(rax);
        }

        if used_regs_string.contains(&str_addr) {
            let rax = self.get_free_register(Some(&used_regs));
            self.unlock_register_from_stack_value(&str_addr);

            instructions.push(format!("mov {rax}, {str_addr}"));

            str_addr = String::from(rax);
        }

        self.unlock_register_from_stack_value(&str_len);
        self.unlock_register_from_stack_value(&str_addr);

        instructions.extend(vec![
            "mov rax, 1".into(),
            "mov rdi, 1".into(),
            format!("mov rsi, {}", str_addr),
            format!("mov rdx, {}", str_len),
            "syscall".into(),
        ]);

        self.extend_current_label(instructions);
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
                // This is fine as a pointer is always 8 bytes
                self.get_vec_for_write_number(VarType::Int)
            }

            VarType::Float => {
                // we pop this anyway because in binary op we push "rax" to stack no matter what
                let stack_member = self.stack_pop().unwrap();

                self.unlock_register_from_stack_value(&stack_member);

                // TODO: Also check here that there's nothing in rax
                vec![
                    format!(";; Writing ptr -> float"),
                    format!("movsd [float_imm], {}", stack_member),
                    format!("mov rax, [float_imm]"),
                    String::from("call _printRAX"),
                ]
            }

            // TODO: Check here whether the pointer is dereferenced or not
            VarType::Str => {
                if times_dereferenced > 0 {
                    self.func_write_string();
                    vec![]
                } else {
                    self.get_vec_for_write_number(VarType::Int)
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
                        VarType::Struct(_, members) => {
                            let memebers_borrow = members.borrow();
                            let var = variable.expect("Expected a variable to be passed in");
                            let found = memebers_borrow.iter().find(|x| x.name == var.member_access[0]);

                            match found {
                                Some(struct_member) => match struct_member.member_type {
                                    VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                                        self.func_write_number(struct_member.member_type.clone());
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

    /// Simply takes the arg_num (argument number) and stores in the
    /// registers which corresponds to that argument number in accordance with x86 calling
    /// conventions
    pub fn func_syscall_add_arg(&mut self, arg_num: usize) {
        let mut instructions = vec![];

        let stack_member = self.stack_pop().unwrap();

        let arg_reg = SYSCALL_ARGS_REGS[arg_num];

        instructions.push(format!("mov {}, {}", arg_reg, stack_member));

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

    pub fn func_syscall_call(&mut self, is_result_assigned: bool) {
        self.add_to_current_label("syscall".into());

        let mut instructions = vec![];

        if is_result_assigned {
            self.handle_function_return_value(&mut instructions, &String::from("sycall"), &VarType::Int);
        }

        self.function_call_register_restore(&mut instructions);

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
                        let stack_member = self.stack_pop().unwrap();

                        self.unlock_register_from_stack_value(&stack_member);

                        vec![
                            format!(";; {}:{}", file!(), line!()),
                            format!(
                                "mov {}, {}",
                                var.var_type.get_register_name(Register::RAX),
                                stack_member
                            ),
                            format!("call _printRAX"),
                        ]
                    }

                    VarType::Str => {
                        self.func_write_string();
                        vec![]
                    }

                    VarType::Char => WRITE_CHAR_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec(),

                    VarType::Ptr(pointer_var_type) => self.func_write_pointer_internal(
                        pointer_var_type,
                        var.times_dereferenced,
                        call_stack,
                        Some(var),
                    ),

                    VarType::Float => {
                        let value = self.stack_pop().unwrap();

                        let mut inst = vec![format!(";; Writing float variable")];

                        if value.starts_with("[") {
                            // movsd [memory], [memory] is not allowed
                            inst.extend(vec![format!("mov rax, {value}"), format!("call _printRAX")])
                        } else {
                            inst.extend(vec![
                                format!("movsd [float_imm], {value}"),
                                format!("mov rax, [float_imm]"),
                                format!("call _printRAX"),
                            ])
                        }

                        self.unlock_register_from_stack_value(&value);

                        inst
                    }

                    // This will print the address to the array which is 8 bytes
                    VarType::Array(..) => self.get_vec_for_write_number(VarType::Int),

                    VarType::Struct(_, member_access) => {
                        let borrow = member_access.borrow();
                        let found = borrow.iter().find(|x| x.name == var.member_access[0]);

                        match found {
                            Some(struct_member_type) => match &struct_member_type.member_type {
                                VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                                    self.get_vec_for_write_number(struct_member_type.member_type.clone())
                                }

                                VarType::Str => {
                                    self.func_write_string();
                                    vec![]
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
