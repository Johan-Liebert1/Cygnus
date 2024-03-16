use crate::{
    ast::abstract_syntax_tree::ASTNodeEnum,
    lexer::{
        tokens::{AssignmentTypes, VariableEnum},
        types::VarType,
    },
    semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack},
    trace,
    types::ASTNode,
};

use super::asm::ASM;

impl ASM {
    pub fn variable_declaration(&mut self, var_name: &String, call_stack: &CallStack) {
        // 1. Check whether the variable is a local or global variable
        // 2. If global var add it to the global section, else skip

        let (variable, variable_scope) = call_stack.get_var_with_name(var_name);

        match variable {
            Some(_) => {
                match variable_scope {
                    ActivationRecordType::Global => {
                        // this needs to be dq, as we are assuming all integers are 64 bits
                        // db will only allocate 1 byte while dq allocates a word
                        self.data.push(format!("{} dq 0", var_name));
                    }

                    _ => {
                        // 2. If global var add it to the global section, else skip
                    }
                }
            }

            None => unreachable!(
                "Could not find variable with name '{}' in function `variable_declaration`. \
                    This is a bug in the semantic analying step.",
                var_name,
            ),
        };
    }

    /// pops the top most element on the stack and assigns it to the variable
    pub fn variable_assignment(
        &mut self,
        var_name: &String,
        assignment_type: &AssignmentTypes,
        call_stack: &CallStack,
        times_dereferenced: usize,
        is_function_call_assign: bool,
        array_access_index: &Option<ASTNode>,
    ) {
        // 1. Check whether the variable is a local or global variable
        // 2. If global var, get it from data section, else from stack offset
        let (var_from_call_stack, variable_scope) = call_stack.get_var_with_name(&var_name);

        let mut instructions = vec![];

        let mut is_string = false;
        let mut is_ptr_deref = false;
        let mut is_array = false;

        match var_from_call_stack {
            Some(var) => {
                match variable_scope {
                    ActivationRecordType::Global => {
                        match assignment_type {
                            AssignmentTypes::Equals => {
                                match &var.var_type {
                                    VarType::Int => todo!(),
                                    VarType::Struct(_, _) => todo!(),

                                    VarType::Str => {
                                        // pop the string pointer into rax
                                        // the string len should be in rbx as string len is pushed
                                        // last
                                        instructions.extend([format!("pop rbx"), format!("pop rax")]);

                                        is_string = true;
                                    }

                                    VarType::Ptr(ptr_var_type) => {
                                        trace!("{}", var.var_type);

                                        match **ptr_var_type {
                                            VarType::Struct(_, _) => todo!(),
                                            VarType::Int => {
                                                // Store whatever's on the top of the stack into
                                                // this memory location
                                                instructions
                                                    .extend([format!("pop rax"), format!("mov rbx, {}", var.var_name)]);
                                                instructions.extend(
                                                    std::iter::repeat(format!("mov rbx, [rbx]"))
                                                        .take(times_dereferenced),
                                                );

                                                instructions.push(format!("mov rbx, rax"));
                                            }

                                            VarType::Str => todo!(),

                                            VarType::Array(..) => todo!(),
                                            VarType::Float => todo!(),
                                            VarType::Char => todo!(),
                                            VarType::Ptr(_) => todo!(),
                                            VarType::Unknown => todo!(),
                                        }
                                    }

                                    VarType::Array(..) => todo!(),
                                    VarType::Float => todo!(),
                                    VarType::Char => todo!(),
                                    VarType::Unknown => todo!(),
                                }
                            }

                            AssignmentTypes::PlusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("add rax, rbx")])
                            }

                            AssignmentTypes::MinusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("sub rax, rbx")])
                            }
                        }

                        instructions.push(format!("mov [{}], rax", var_name));
                    }

                    // local variable
                    _ => {
                        match assignment_type {
                            AssignmentTypes::Equals => {
                                match &var.var_type {
                                    VarType::Struct(_, _) => todo!(),
                                    VarType::Int | VarType::Float => {
                                        // if it is a function call assignment, then the value is
                                        // already in rax
                                        if !is_function_call_assign {
                                            instructions.push(format!("pop rax"))
                                        }
                                    }

                                    VarType::Str => {
                                        // pop the string pointer into rax
                                        // the string len should be in rbx as string len is pushed
                                        // last
                                        instructions.extend([format!("pop rbx"), format!("pop rax")]);

                                        is_string = true;
                                    }

                                    VarType::Char => {
                                        // TODO: Update this
                                        //
                                        // pop the string pointer into rax
                                        // the string len should be in rbx as string len is pushed
                                        // last
                                        // Treat a character as a string with length of 1
                                        instructions.extend([format!("mov rbx, 1"), format!("pop rax")]);

                                        is_string = true;
                                    }

                                    // Assignment to a pointer should be simple enough
                                    VarType::Ptr(var_ptr_type) => match **var_ptr_type {
                                        VarType::Ptr(_) => todo!(),

                                        // assignment to ptr to a character
                                        // basically a CStr
                                        //
                                        // TODO: Also handle things like
                                        // def ch: char = 'a';
                                        // def ch_ptr: *char = &ch;
                                        VarType::Char => {}

                                        VarType::Unknown => todo!(),

                                        _ => {
                                            is_ptr_deref = times_dereferenced > 0;

                                            // Let's say the following code
                                            //
                                            // mem array 1024 --> array starts at addr 500
                                            // def thing: *int = array + 1; --> thing is at rbp - 8
                                            // and [rbp - 8] now contains addr 501
                                            //
                                            // *thing = 60;
                                            // derefed thing should contain 60, i.e. [rbp - 8]
                                            // should not contain 60, but addr 501 should now
                                            // contain 60
                                            //
                                            // we should only deref once which can be done by
                                            // mov rbx, [rbp - 8]
                                            // mov [rbx], rax

                                            instructions.push(format!("pop rax"));

                                            if is_ptr_deref {
                                                instructions.push(format!("mov rbx, [rbp - {}]", var.offset));
                                            }

                                            if times_dereferenced > 1 {
                                                instructions.extend(
                                                    std::iter::repeat(format!("mov rbx, [rbx]"))
                                                        .take(times_dereferenced - 1),
                                                );
                                            }

                                            if is_ptr_deref {
                                                instructions.push(format!("mov [rbx], rax"));
                                            }

                                            // instructions.extend(vec![format!("pop rbx"), format!("mov rax, rbx")]);
                                        }
                                    },

                                    VarType::Array(type_, size) => {
                                        is_array = true;

                                        match array_access_index {
                                            Some(index) => {
                                                // we visit the right side first and then the left
                                                // side. So the array index is at topand the
                                                // actual value to set is at the top - 1 of the stack
                                                instructions.extend([
                                                    // array[1] = 10

                                                    // rcx stores the index, rdx has the actual
                                                    // value
                                                    format!(";; rbx stores the index, rcx has the actual value"),
                                                    format!("pop rbx"), // rcx has 1
                                                    format!("pop rcx"), // rdx has 10
                                                    format!("mov rax, {}", type_.get_underlying_type_size()), // rax = 8
                                                    format!("mul rbx"), // now rax has = rax * rbx
                                                    // = 1 * 8 = 8
                                                    format!("mov rbx, rbp"),
                                                    format!("sub rbx, rax"),
                                                    format!("mov [rbx - {}], rcx", var.offset),
                                                ]);
                                            }

                                            None => {
                                                // Assignment to the array variable itself
                                                for i in 0..*size {
                                                    instructions.extend([
                                                        format!("pop rax"),
                                                        format!(
                                                            "mov [rbp - {}], rax",
                                                            var.offset + type_.get_underlying_type_size() * i
                                                        ),
                                                    ]);
                                                }
                                            }
                                        }
                                    }

                                    VarType::Unknown => todo!(),
                                }
                            }

                            AssignmentTypes::PlusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("add rax, rbx")])
                            }

                            AssignmentTypes::MinusEquals => {
                                instructions.extend([format!("pop rax"), format!("pop rbx"), format!("sub rax, rbx")])
                            }
                        }

                        if is_string {
                            // Move the string length into the mem address above the addr
                            // containing the string pointer
                            instructions.push(format!("mov [rbp - {}], rbx", var.offset + 8));
                        }

                        if !is_ptr_deref && !is_array {
                            instructions.push(format!("mov [rbp - {}], rax", var.offset));
                        }
                    }
                }
            }

            None => unreachable!(
                "Could not find variable with name '{}' in function `factor`. \
            This is a bug in the semantic analying step.",
                var_name
            ),
        };

        self.extend_current_label(instructions);
    }
}
