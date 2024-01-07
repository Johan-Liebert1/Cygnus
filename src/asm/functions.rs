use super::asm::ASM;

pub const FUNCTION_RETURN_INSTRUCTIONS: [&str; 3] = [("mov rsp, rbp"), ("pop rbp"), ("ret")];

impl ASM {
    pub fn function_call(&mut self, function_name: &String) {
        self.add_to_current_label(format!("call _{function_name}"));
    }

    pub fn function_def(&mut self, function_name: &String, local_var_size: usize) {
        self.change_current_label(format!("_{function_name}"));

        // push rbp            ; Save old base pointer
        // mov rbp, rsp        ; Set base pointer to current stack pointer
        // sub rsp, 16         ; Allocate 16 bytes for local variables

        let instructions = vec![
            format!("push rbp"),
            format!("mov rbp, rsp"),
            format!("sub rsp, {}", local_var_size),
        ];

        self.extend_current_label(instructions);
    }

    pub fn function_def_end(&mut self, function_name: &String) {
        // mov rsp, rbp        ; Reset stack pointer
        // pop rbp             ; Restore old base pointer

        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
        self.change_current_label("_start".into());
    }

    pub fn function_return(&mut self) {
        self.extend_current_label(FUNCTION_RETURN_INSTRUCTIONS.map(|x| x.into()).to_vec());
    }
}
