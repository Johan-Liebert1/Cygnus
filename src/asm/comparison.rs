

use crate::lexer::tokens::Comparators;

use super::asm::ASM;

impl ASM {
    pub fn compare_two_numbers(&mut self, op: Comparators) {
        let mut instructions = vec![
            format!(";; We pop in the opposite order of comparison as we push onto the stack"),
            format!("pop rbx"),
            format!("pop rax"),
            format!("cmp rax, rbx"),
        ];

        let inst = match op {
            Comparators::LessThan => format!("jl .skip_{}", self.comparison_num),
            Comparators::GreaterThan => format!("jg .skip_{}", self.comparison_num),
            Comparators::LessThanEq => format!("jle .skip_{}", self.comparison_num),
            Comparators::GreaterThanEq => format!("jge .skip_{}", self.comparison_num),
            Comparators::DoubleEquals => format!("je .skip_{}", self.comparison_num),
            Comparators::NotEquals => format!("jne .skip_{}", self.comparison_num),
        };

        instructions.extend(vec![
            inst.into(),
            // assume the comparison is true
            format!("mov rax, 0"),
            format!("jmp .skip_next{}", self.comparison_num),
            // we'll skip to here if the comparison is false
            format!(".skip_{}:", self.comparison_num),
            format!("mov rax, 1"),
            format!(".skip_next{}:", self.comparison_num),
            format!(";; push onto the stack whatever's in rax so rest of the program can use it"),
            format!("push rax"),
        ]);

        self.extend_current_label(instructions);

        self.comparison_num += 1;
    }
}
