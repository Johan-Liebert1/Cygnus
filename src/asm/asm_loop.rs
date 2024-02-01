use crate::semantic_analyzer::semantic_analyzer::CallStack;

use super::asm::ASM;

impl ASM {
    pub fn gen_loop_start(&mut self, loop_number: usize, call_stack: &CallStack) {
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

        let loop_start = vec![
            format!("pop rcx"), // step
            format!("pop rbx"), // to
            format!("pop rax"), // from
           
            format!("mov [rbp - {}], rcx", step_offset),
            format!("mov [rbp - {}], rbx", to_offset),
            format!("mov [rbp - {}], rax", from_offset),

            format!(".loop_{}:", loop_number),
            format!("mov rcx, [rbp - {}]", step_offset), // step
            format!("mov rbx, [rbp - {}]", to_offset), // to
            format!("mov rax, [rbp - {}]", from_offset), // from

            format!("add rax, rcx"),
            format!("dec rax"),
            // now compare rax to rbx - 1 and if they're equal jump to the end
            format!("dec rbx"),
            format!("cmp rax, rbx"),
            format!("jg .loop_end_{}", loop_number),
            format!("inc rax"),
            format!("inc rbx"),

            format!("mov [rbp - {}], rbx", to_offset),
            format!("mov [rbp - {}], rax", from_offset),
        ];

        self.extend_current_label(loop_start);
    }

    pub fn gen_loop_end(&mut self, loop_number: usize) {
        let loop_end = vec![
            // unconditional jump to loop start
            format!("jmp .loop_{}", loop_number),
            // we jump here when the loop ends
            format!(".loop_end_{}:", loop_number),
        ];

        self.extend_current_label(loop_end);
    }

    pub fn loop_break(&mut self, loop_number: usize) {
        // encountered a break, so an unconditional jump to the end of the loop
        // self.num_loops - 1 as we increment the loop number as soon as we enter the loop
        // and break statement is outside of the loop
        let instructions = vec![
            format!(";; --- break ----"),
            format!("jmp .loop_end_{}", loop_number),
        ];

        self.extend_current_label(instructions);
    }
}
