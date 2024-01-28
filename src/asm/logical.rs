use crate::lexer::tokens::LogicalOps;

use super::asm::ASM;

impl ASM {
    pub fn gen_logical_statement(&mut self, op: LogicalOps) {
        let instructions = vec![
            format!("pop rax"),
            format!("pop rbx"),
            match op {
                LogicalOps::Or => format!("or rax, rbx"),
                LogicalOps::And => format!("and rax, rbx"),
            },
            format!("push rax"),
        ];

        self.extend_current_label(instructions);
    }
}
