use super::asm::ASM;

impl ASM {
    pub fn function_call(&mut self, function_name: &String) {
        let instructions = vec![
            format!("call _{function_name}"),
        ];

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
    }

    pub fn function_def_end(&mut self, function_name: &String) {
        let instructions = vec![
            format!("ret"),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }

        self.change_current_label("_start".into());
    }
}
