use super::asm::ASM;

impl ASM {
    pub fn gen_loop_start(&mut self) {
        // we should have in the stack
        //
        // step <- stack top
        // to
        // from

        // 1. The comparison will be done inside the .loop: label, at the very top
        // 2. If it's false, we jump to .loop_end, else keep executing
        // 3. Just before .loop_end: there's an unconditional jump to .loop:

        let loop_start = vec![
            format!(".loop:"),
            format!("pop rcx"), // step
            format!("pop rbx"), // to
            format!("pop rax"), // from
            format!("add rax, rcx"),
            format!("dec rax"),
            // now compare rax to rbx - 1 and if they're equal jump to the end
            format!("dec rbx"),
            format!("cmp rax, rbx"),
            format!("jg .loop_end"),
            format!("inc rax"),
            format!("inc rbx"),
            format!("push rax"),
            format!("push rbx"),
            format!("push rcx"),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(loop_start);
                break;
            }
        }
    }

    pub fn gen_loop_end(&mut self) {
        let loop_end = vec![
            // unconditional jump to loop start
            format!("jmp .loop"),
            // we jump here when the loop ends
            format!(".loop_end:"),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(loop_end);
                break;
            }
        }
    }
}
