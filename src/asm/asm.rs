use core::panic;

use crate::{
    lexer::registers::{Register, ALL_REGISTERS},
    trace,
};

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub code: Vec<String>,
}

#[derive(Debug)]
pub struct ASM {
    pub include: Vec<&'static str>,
    pub text: Vec<String>,
    pub data: Vec<String>,
    pub bss: Vec<String>,
    pub labels: Vec<Label>,
    pub comparison_num: usize,
    pub num_strings: usize,
    pub num_floats: usize,
    pub num_ifs: usize,

    stack: Vec<String>,
    function_argument_number: Option<usize>,
    current_label: String,

    used_regsiters: Vec<Register>,

    pub regs_locked_for_function_call: Vec<Register>,
}

impl Default for ASM {
    fn default() -> Self {
        Self {
            num_strings: 0,
            num_floats: 0,
            comparison_num: 0,
            current_label: "_start".to_string(),
            num_ifs: 0,

            include: vec![r#"%include "std.asm""#],

            text: vec![String::from("global _start")],

            data: vec![
                String::from(";; For floating point operations"),
                String::from("float_imm dq 0"),
            ],

            bss: vec![
                // for printing numbers
                String::from("digitSpace resb 100"),
                String::from("digitSpacePos resb 8"),
                String::from("argc resb 8"),
            ],

            labels: vec![Label {
                name: String::from("_start"),
                code: vec![
                    // save argc and argv

                    // top of stack -> argc argv0 argv1 argv2 ...
                    format!("mov [argc], rsp"),
                ],
            }],

            function_argument_number: None,
            stack: vec![],

            used_regsiters: vec![],
            regs_locked_for_function_call: vec![],
        }
    }
}

impl ASM {
    pub fn change_current_label(&mut self, new_label: String) {
        self.current_label = new_label.clone();

        for l in &self.labels {
            if l.name == new_label {
                return;
            }
        }

        self.labels.push(Label {
            name: new_label,
            code: vec![],
        });
    }

    pub fn extend_current_label(&mut self, vec: Vec<String>) {
        for label in &mut self.labels {
            if label.name == self.current_label {
                label.code.extend(vec);
                label.code.push(format!(""));
                break;
            }
        }
    }

    pub fn add_to_current_label(&mut self, line: String) {
        for label in &mut self.labels {
            if label.name == self.current_label {
                label.code.push(line);
                label.code.push(format!(""));
                break;
            }
        }
    }

    fn get_current_label_code(&self) -> Option<&Label> {
        for label in &self.labels {
            if label.name == self.current_label {
                return Some(&label);
            }
        }

        None
    }

    pub fn current_label(&self) -> String {
        return self.current_label.clone();
    }

    pub fn inc_num_ifs(&mut self) {
        self.num_ifs += 1;
    }

    pub fn start_parsing_function_args(&mut self) {
        match self.function_argument_number {
            Some(_) => panic!("Already parsing function arguments"),

            None => self.function_argument_number = Some(0),
        }
    }

    pub fn end_parsing_function_args(&mut self) {
        match self.function_argument_number {
            Some(_) => self.function_argument_number = None,

            None => panic!("Not parsing function args"),
        }
    }

    pub fn parsing_next_function_arg(&mut self) {
        match self.function_argument_number {
            Some(num) => self.function_argument_number = Some(num + 1),

            None => panic!("Cannot call `parsing_next_function_arg` when not parsing function args"),
        }
    }

    pub fn stack_pop(&mut self) -> Option<String> {
        self.stack.pop()
    }

    pub fn stack_push(&mut self, to_push: String) {
        self.stack.push(to_push);
    }

    pub fn replace_reg_on_stack(&mut self, reg_to_replace: Register, reg_to_replace_with: Register) {
        for reg in &mut self.stack {
            if *reg == String::from(reg_to_replace) {
                *reg = String::from(reg_to_replace_with);
            }
        }
    }

    pub fn stack_extend(&mut self, to_push: Vec<String>) {
        self.stack.extend(to_push);
    }

    pub fn lock_register(&mut self, reg_name: Register) {
        let idx = self.used_regsiters.iter().find(|x| **x == reg_name);

        match idx {
            Some(..) => {
                panic!(
                    "Lock Register Failed: Register {reg_name} already in list.\nList: {:#?}\nASM:{:#?}",
                    self.used_regsiters, self.labels
                )
            }

            None => {
                self.used_regsiters.push(reg_name);
            }
        }
    }

    pub fn unlock_register(&mut self, reg_name: Register) {
        let idx = self.used_regsiters.iter().enumerate().find(|x| *x.1 == reg_name);

        match idx {
            Some((idx, _)) => {
                self.used_regsiters.remove(idx);
            }

            None => {
                panic!(
                    "UnLock Register Failed: Register {reg_name} not found in list. List: {:#?}",
                    self.used_regsiters
                )
            }
        }
    }

    pub fn unlock_register_from_stack_value(&mut self, stack_pop_result: &String) {
        let (is_reg, reg_name) = self.is_reg_name(stack_pop_result);

        if !is_reg {
            return;
        }

        self.unlock_register(reg_name)
    }

    /// Returns a free register and locks it
    /// Always returns in this order
    /// [RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8, R9, R10, R11]
    pub fn get_free_register(&mut self, skip_list: Option<&Vec<Register>>) -> Register {
        for reg in ALL_REGISTERS {
            if !self.used_regsiters.contains(&reg) {
                if let Some(skip_list) = skip_list {
                    if skip_list.contains(&reg) {
                        continue;
                    }
                }

                self.lock_register(reg);
                return reg;
            }
        }

        panic!("Ran out of registers");
    }

    pub fn is_reg_name(&mut self, name: &String) -> (bool, Register) {
        for reg_name in ALL_REGISTERS {
            if *name == String::from(reg_name) {
                return (true, reg_name);
            }
        }

        return (false, Register::R11);
    }

    pub fn is_reg_locked(&self, name: Register) -> bool {
        self.used_regsiters.contains(&name)
    }

    pub fn get_used_registers(&self) -> &Vec<Register> {
        &self.used_regsiters
    }
}
