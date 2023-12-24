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

    pub fn func_exit(&mut self) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(vec![
                    format!("pop rdi"),
                    format!("mov rax, 60"),
                    format!("syscall"),
                ]);
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

    pub fn func_write_var(&mut self, var_name: &String) {
        // TODO: Un-hardcode this

        let instructions = match var_name.as_str() {
            "argc" => vec![
                // argc contains the address of rsp
                format!("mov rax, [argc]"),
                // since argc contained the address, we need to dereference it again to get the
                // actual value
                format!("mov rax, [rax]"),
                format!("call _printRAX"),
            ],

            "argv" => vec![
                // argc contains the address of rsp
                format!("mov rax, [argc]"),
                // since argc contained the address, we need to dereference it again to get the
                // actual value
                format!("mov rax, [rax + 8]"),
                // now rax has the string
                // we need to calculate its length
                // it's NULL terminated

                // save rax
                format!("push rax"),
                format!("mov rbx, 0"),
                format!(".strlen:"),
                format!("inc rax"),
                format!("inc rbx"),
                format!("mov cl, [rax]"),
                format!("cmp cl, 0"),
                format!("jne .strlen"),
                format!("push rbx"),
                format!("pop r8"),
                format!("pop r9"),
                format!("mov rax, 1"),
                format!("mov rdi, 1"),
                format!("mov rsi, r9"),
                format!("mov rdx, r8"),
                format!("syscall"),
            ],

            _ => vec![],
        };

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }
}
