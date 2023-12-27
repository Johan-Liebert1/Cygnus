use super::asm::ASM;

impl ASM {
    pub fn function_call(&mut self, function_name: &String) {
        let instructions = vec![format!("call _{function_name}")];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }

    pub fn function_def(&mut self, function_name: &String) {
        self.change_current_label(format!("_{function_name}"));

        // push rbp            ; Save old base pointer
        // mov rbp, rsp        ; Set base pointer to current stack pointer
        // sub rsp, 16         ; Allocate 16 bytes for local variables

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label
                    .code
                    .extend(vec![format!("push rbp"), format!("mov rbp, rsp")]);

                break;
            }
        }
    }

    pub fn function_def_end(&mut self, function_name: &String) {
        // mov rsp, rbp        ; Reset stack pointer
        // pop rbp             ; Restore old base pointer

        let instructions = vec![format!("mov rsp, rbp"), format!("pop rbp"), format!("ret")];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }

        self.change_current_label("_start".into());
    }

    pub fn function_return(&mut self) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.push(format!("ret"));
                break;
            }
        }
    }
}
