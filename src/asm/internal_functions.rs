use crate::{
    ast::{abstract_syntax_tree::AST, variable::Variable},
    lexer::{registers::Register, types::VarType},
    trace,
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

        // If the current value is not rax or a variant of rax, then move it into rax
        if !Register::is_reg(&stack_member) || !Register::is(&Register::RAX, Register::from_string(&stack_member)) {
            instructions.extend(vec![
                format!(";; get_vec_for_write_number. stack_member: {stack_member}, type: {type_}"),
                if !matches!(type_, VarType::Int) {
                    format!("xor rax, rax")
                } else {
                    format!(";; no xor here")
                },
                format!("mov {}, {}", type_.get_register_name(Register::RAX), stack_member),
            ]);
        }

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

    pub fn func_write_pointer(&mut self) {
        // All pointers are 64bit
        let vec = self.get_vec_for_write_number(VarType::Int);
        self.extend_current_label(vec);
    }

    // fn func_write_pointer_internal(
    //     &mut self,
    //     pointer_var_type: &VarType,
    //     call_stack: &CallStack,
    //     variable: Option<&Variable>,
    // ) -> Vec<String> {
    //     match *pointer_var_type {
    //         VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Char => {
    //             self.get_vec_for_write_number(pointer_var_type.clone())
    //         }

    //         VarType::Float => {
    //             // we pop this anyway because in binary op we push "rax" to stack no matter what
    //             let stack_member = self.stack_pop().unwrap();

    //             self.unlock_register_from_stack_value(&stack_member);

    //             // TODO: Also check here that there's nothing in rax
    //             vec![
    //                 format!(";; Writing ptr -> float"),
    //                 format!("movsd [float_imm], {}", stack_member),
    //                 format!("mov rax, [float_imm]"),
    //                 String::from("call _printRAX"),
    //             ]
    //         }

    //         VarType::Str => {
    //             self.func_write_string();
    //             vec![]
    //         }

    //         // Might be pointer to a user defined type
    //         _ => {
    //             let user_defined_type = call_stack
    //                 .user_defined_types
    //                 .iter()
    //                 .find(|x| x.name == format!("{}", pointer_var_type));

    //             match user_defined_type {
    //                 Some(user_defined_type) => match &user_defined_type.type_ {
    //                     VarType::Struct(_, members) => {
    //                         let memebers_borrow = members.borrow();
    //                         let var = variable.expect("Expected a variable to be passed in");
    //                         let found = memebers_borrow.iter().find(|x| x.name == var.member_access[0]);

    //                         match found {
    //                             Some(struct_member) => match struct_member.member_type {
    //                                 VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
    //                                     self.func_write_number(struct_member.member_type.clone());
    //                                     vec![]
    //                                 }
    //                                 VarType::Str => {
    //                                     self.func_write_string();
    //                                     vec![]
    //                                 }
    //                                 VarType::Float => todo!(),
    //                                 VarType::Char => todo!(),
    //                                 VarType::Ptr(_) => todo!(),
    //                                 VarType::Array(_, _) => todo!(),
    //                                 VarType::Struct(_, _) => todo!(),
    //                                 VarType::Unknown => todo!(),
    //                                 VarType::Function(_, _, _) => todo!(),
    //                             },

    //                             None => {
    //                                 unreachable!(
    //                                     "Member '{}' not present in struct '{}'",
    //                                     var.member_access[0], user_defined_type.type_
    //                                 );
    //                             }
    //                         }
    //                     }

    //                     type_ => {
    //                         unreachable!("Cannot have user defined primitive type: '{}'", type_);
    //                     }
    //                 },

    //                 None => {
    //                     unreachable!("Unknown type '{pointer_var_type}'")
    //                 }
    //             }
    //         }
    //     }
    // }

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
                // let actual_var_type = var.var_type.get_actual_type(var.times_dereferenced, var.get_token());

                let actual_var_type = var.get_type().0;

                // the variable value or its address will be pushed onto the stack
                // We don't need to check the scope here as the variable value is already
                // pushed into rax beforehand in `factor` AST
                match &actual_var_type {
                    VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                        self.get_vec_for_write_number(actual_var_type)
                    }

                    VarType::Str => {
                        self.func_write_string();
                        vec![]
                    }

                    VarType::Char => WRITE_CHAR_ASM_INSTRUCTIONS.map(|x| x.into()).to_vec(),

                    VarType::Ptr(..) => {
                        self.func_write_pointer();
                        vec![]
                    }

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
                    VarType::Array(..) => {
                        self.func_write_pointer();
                        // TODO: Fix these
                        vec![]
                    }

                    VarType::Struct(_, member_access) => {
                        trace!("Printing something for a struct");

                        let borrow = member_access.borrow();
                        let found = borrow.iter().find(|x| x.name == var.member_access[0]);

                        match found {
                            Some(struct_member_type) => {
                                let actual_member_type = struct_member_type
                                    .member_type
                                    .get_actual_type(var.times_dereferenced, var.get_token());

                                match &actual_member_type {
                                    VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Char => {
                                        self.get_vec_for_write_number(actual_member_type)
                                    }

                                    VarType::Str => {
                                        self.func_write_string();
                                        vec![]
                                    }

                                    VarType::Ptr(..) => {
                                        self.func_write_pointer();
                                        vec![]
                                    }

                                    VarType::Float => todo!(),
                                    VarType::Array(_, _) => todo!(),
                                    VarType::Struct(_, _) => todo!(),
                                    VarType::Unknown => todo!(),
                                    VarType::Function(_, _, _) => todo!(),
                                }
                            }

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
