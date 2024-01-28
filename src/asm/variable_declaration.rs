use crate::types::ASTNode;

use crate::ast::variable::Variable;

use super::asm::ASM;

impl ASM {
    pub fn variable_declaration(&mut self, var_name: &String) {
        // this needs to be dq, as we are assuming all integers are 64 bits
        // db will only allocate 1 byte while dq allocates a word
        self.data.push(format!("{} dq 0", var_name));
    }

    /// pops the top most element on the stack and assigns it to the variable
    pub fn variable_assignment(&mut self, var_name: &String) {
        self.extend_current_label(vec![format!("pop rax"), format!("mov [{}], rax", var_name)]);
    }
}
