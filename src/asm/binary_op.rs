use super::asm::ASM;

impl ASM {
    pub fn add_two_numbers(&mut self) {
        let first = vec![
            format!(";; get the two operands from the stack"),
            format!("pop rax"),
            format!("pop rbx"),
            format!("add rax, rbx"),
            format!(";; push the result back onto the stack"),
            format!("push rax"),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(first);
                break;
            }
        }
    }
}
