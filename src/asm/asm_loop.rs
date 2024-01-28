

use super::asm::ASM;

impl ASM {
    pub fn gen_loop_start(&mut self, loop_number: usize) {
        // we should have in the stack
        //
        // step <- stack top
        // to
        // from

        // 1. The comparison will be done inside the .loop: label, at the very top
        // 2. If it's false, we jump to .loop_end, else keep executing
        // 3. Just before .loop_end: there's an unconditional jump to .loop:

        let loop_start = vec![
            format!(".loop_{}:", loop_number),
            format!("pop rcx"), // step
            format!("pop rbx"), // to
            format!("pop rax"), // from
            format!("add rax, rcx"),
            format!("dec rax"),
            // now compare rax to rbx - 1 and if they're equal jump to the end
            format!("dec rbx"),
            format!("cmp rax, rbx"),
            format!("jg .loop_end_{}", loop_number),
            format!("inc rax"),
            format!("inc rbx"),
            format!("push rax"),
            format!("push rbx"),
            format!("push rcx"),
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

    pub fn loop_break(&mut self) {
        // encountered a break, so an unconditional jump to the end of the loop
        // self.num_loops - 1 as we increment the loop number as soon as we enter the loop
        // and break statement is outside of the loop
        let instructions = vec![
            format!(";; --- break ----"),
            format!("jmp .loop_end_{}", self.num_loops - 1),
        ];

        self.extend_current_label(instructions);
    }
}
