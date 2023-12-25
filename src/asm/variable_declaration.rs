use crate::ast::variable::Variable;

use super::asm::ASM;

impl ASM {
    pub fn variable_declaration(&mut self, var_name: &String) {
        self.data.push(format!("{} db 0", var_name));
    }

    /// pops the top most element on the stack and assigns it to the variable
    pub fn variable_assignment(&mut self, var_name: &String) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(vec![
                    format!("pop rax"),
                    format!("mov [{}], rax", var_name),
                ]);

                break;
            }
        }
    }
}
