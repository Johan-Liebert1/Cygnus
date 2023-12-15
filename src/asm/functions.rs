use super::asm::ASM;

impl ASM {
    pub fn func_write(&mut self) {
        for label in &mut self.labels {
            if label.name == "_start" {
                label.code.push(String::from("call _printRAX"));
            }
        }
    }
}
