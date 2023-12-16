use super::asm::ASM;

impl ASM {
    pub fn func_write(&mut self) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.push(String::from("call _printRAX"));
            }
        }
    }
}
