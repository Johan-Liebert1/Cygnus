use super::asm::ASM;

impl ASM {
    pub fn function_call(&mut self, function_name: &String) {
        self.add_to_current_label(format!("call _{function_name}"));
    }

    pub fn function_def(&mut self, function_name: &String) {
        self.change_current_label(format!("_{function_name}"));

        // push rbp            ; Save old base pointer
        // mov rbp, rsp        ; Set base pointer to current stack pointer
        // sub rsp, 16         ; Allocate 16 bytes for local variables

        let instructions = vec![format!("push rbp"), format!("mov rbp, rsp")];

        self.extend_current_label(instructions);
    }

    pub fn function_def_end(&mut self, function_name: &String) {
        // mov rsp, rbp        ; Reset stack pointer
        // pop rbp             ; Restore old base pointer

        let instructions = vec![format!("mov rsp, rbp"), format!("pop rbp"), format!("ret")];
        self.extend_current_label(instructions);

        self.change_current_label("_start".into());
    }

    pub fn function_return(&mut self) {
        self.add_to_current_label(format!("ret"));
    }
}
