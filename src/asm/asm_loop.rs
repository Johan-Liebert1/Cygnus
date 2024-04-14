use core::panic;

use crate::{ast::variable::Variable, semantic_analyzer::semantic_analyzer::CallStack, trace};

use super::asm::ASM;

impl ASM {
    pub fn gen_inf_loop_start(&mut self, loop_number: usize) {
        self.add_to_current_label(format!(".loop_{}:", loop_number));
    }

    pub fn gen_inf_loop_end(&mut self, loop_number: usize) {
        self.extend_current_label(vec![
            // unconditional jump to loop start
            format!("jmp .loop_{}", loop_number),
            // we jump here when the loop ends
            format!(".loop_end_{}:", loop_number),
        ]);
    }

    pub fn gen_loop_start(&mut self, loop_number: usize, call_stack: &CallStack, with_var: &Option<Variable>) {
        // we should have in the stack
        //
        // step <- stack top
        // to
        // from

        // 1. The comparison will be done inside the .loop: label, at the very top
        // 2. If it's false, we jump to .loop_end, else keep executing
        // 3. Just before .loop_end: there's an unconditional jump to .loop:

        let from = call_stack.get_var_with_name(&format!("loop_{}_from", loop_number));
        let to = call_stack.get_var_with_name(&format!("loop_{}_to", loop_number));
        let step = call_stack.get_var_with_name(&format!("loop_{}_step", loop_number));

        let (mut from_offset, mut to_offset, mut step_offset) = (0, 0, 0);

        match (from, to, step) {
            ((Some(from), _), (Some(to), _), (Some(step), _)) => {
                from_offset = from.offset;
                to_offset = to.offset;
                step_offset = step.offset;
            }

            _ => {
                panic!("'from', 'to' or 'step' not defined");
            }
        };

        let mut loop_start: Vec<String> = vec![
            format!("pop rcx"), // step
            format!("pop rbx"), // to
            format!("pop rax"), // from
            format!("mov [rbp - {}], rcx", step_offset),
            format!("mov [rbp - {}], rbx", to_offset),
            format!("mov [rbp - {}], rax", from_offset),
        ];

        let mut call_stack_var = None;

        if let Some(v) = with_var {
            (call_stack_var, _) = call_stack.get_var_with_name(&v.var_name);

            if call_stack_var.is_none() {
                panic!("`call_stack_var` is none but loop has a variable")
            }

            // here rax contains the from value
            loop_start.extend(vec![format!("mov [rbp - {}], rax", call_stack_var.unwrap().offset)]);
        }

        loop_start.push(format!(".loop_{}:", loop_number));

        self.extend_current_label(loop_start);
    }

    pub fn gen_loop_end(&mut self, loop_number: usize, call_stack: &CallStack, with_var: &Option<Variable>) {
        let mut loop_end: Vec<String> = vec![];

        let (from, _) = call_stack.get_var_with_name(&format!("loop_{}_from", loop_number));
        let (to, _) = call_stack.get_var_with_name(&format!("loop_{}_to", loop_number));
        let (step, _) = call_stack.get_var_with_name(&format!("loop_{}_step", loop_number));

        let (mut from_offset, mut to_offset, mut step_offset) = (0, 0, 0);

        match (from, to, step) {
            (Some(from), Some(to), Some(step)) => {
                from_offset = from.offset;
                to_offset = to.offset;
                step_offset = step.offset;
            }

            _ => {
                panic!("'from', 'to' or 'step' not defined");
            }
        };

        if let Some(v) = with_var {
            let (call_stack_var, _) = call_stack.get_var_with_name(&v.var_name);

            if call_stack_var.is_none() {
                panic!("`call_stack_var` is none but loop has a variable")
            }

            // add step to variable
            loop_end.extend([
                format!(";; inc the loop variable"),
                format!("mov rdx, [rbp - {}]", call_stack_var.unwrap().offset),
                format!("mov rcx, [rbp - {}]", step_offset),
                format!("add rdx, rcx"),
                format!("mov [rbp - {}], rdx", call_stack_var.unwrap().offset),
            ]);
        }

        loop_end.extend(vec![
            format!(";; check exit condition"),
            format!("mov rcx, [rbp - {}] ;; step", step_offset), // step
            format!("mov rbx, [rbp - {}] ;; to", to_offset),     // to
            format!("mov rax, [rbp - {}] ;; from", from_offset), // from
            format!("add rax, rcx"),
            // now compare rax to rbx - 1 and if they're equal jump to the end
            format!("dec rbx"),
            format!("cmp rax, rbx"),
            format!("jg .loop_end_{}", loop_number),
            format!("mov [rbp - {}], rax", from_offset),
            // unconditional jump to loop start
            format!("jmp .loop_{}", loop_number),
            // we jump here when the loop ends
            format!(".loop_end_{}:", loop_number),
        ]);

        self.extend_current_label(loop_end);
    }

    pub fn loop_break(&mut self, loop_number: usize) {
        // encountered a break, so an unconditional jump to the end of the loop
        // self.num_loops - 1 as we increment the loop number as soon as we enter the loop
        // and break statement is outside of the loop
        let instructions = vec![format!(";; --- break ----"), format!("jmp .loop_end_{}", loop_number)];

        self.extend_current_label(instructions);
    }
}
