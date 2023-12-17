use super::asm::ASM;

impl ASM {
    pub fn func_write_number(&mut self) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.push(String::from("pop rax"));
                label.code.push(String::from("call _printRAX"));
            }
        }
    }

    pub fn func_write_string(&mut self) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                // TODO: There's some weird stack alloc issue when I try to do this. So this takes a
                // backseat for now
                // label.code.push(String::from("call _printString"));

                label.code.extend(vec![
                    format!(";; Assuming length is pushed last"),
                    format!("pop r8"),
                    format!(";; Assuming string address is pushed first"),
                    format!("pop r9"),
                    format!("mov rax, 1"),
                    format!("mov rdi, 1"),
                    format!("mov rsi, r9"),
                    format!("mov rdx, r8"),
                    format!("syscall"),
                ]);
            }
        }
    }
}
