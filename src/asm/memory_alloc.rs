
use super::asm::ASM;

impl ASM {
    pub fn generate_memory_alloc(&mut self, name: &String, size: usize) {
        // this will be in the bss section
        self.bss.push(format!("{} resb {}", name, size));
    }
}
