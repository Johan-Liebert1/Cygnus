use crate::lexer::tokens::LogicalOps;

use super::asm::ASM;

impl ASM {
    pub fn gen_logical_statement(&mut self, op: LogicalOps) {
        let instructions = match op {
            LogicalOps::Or | LogicalOps::And => {
                let rax = self.get_free_register(None);
                let rbx = self.get_free_register(None);

                let first = self.stack_pop().unwrap();
                let second = self.stack_pop().unwrap();

                let instructions = vec![
                    format!(";; gen_logical_statement"),
                    format!("xor {rax}, {rax}"),
                    format!("mov {rax}, {first}"),
                    format!("xor {rbx}, {rbx}"),
                    format!("mov {rbx}, {second}"),
                    format!("{op} {rax}, {rbx}"),
                ];

                self.unlock_register_from_stack_value(&first);
                self.unlock_register_from_stack_value(&second);

                self.unlock_register(rbx);

                self.stack_push(String::from(rax));

                instructions
            }

            LogicalOps::Not => {
                let rax = self.get_free_register(None);
                let first = self.stack_pop().unwrap();

                let instructions = vec![format!("{op} {rax}")];

                self.unlock_register_from_stack_value(&first);

                self.stack_push(String::from(rax));

                instructions
            }
        };

        self.extend_current_label(instructions);
    }
}
