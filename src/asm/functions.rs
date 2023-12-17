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
                label.code.push(String::from("call _printString"));
            }
        }
    }
}
