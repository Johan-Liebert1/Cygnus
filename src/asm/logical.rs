use crate::lexer::tokens::LogicalOps;

use super::asm::ASM;

impl ASM {
    pub fn gen_logical_statement(&mut self, op: LogicalOps) {
        let mut instructions = vec![
            format!("xor rax, rax"),
            format!("pop rax"),
            format!("xor rbx, rbx"),
            format!("pop rbx"),
        ];

        let thing = match op {
            LogicalOps::Or => format!("or rax, rbx"),
            LogicalOps::And => format!("and rax, rbx"),
            LogicalOps::Not => {
                instructions.pop();
                instructions.pop();
                format!("not rax")
            },
        };


        instructions.push(thing);
        instructions.push(format!("push rax"));

        self.extend_current_label(instructions);
    }
}
