use crate::lexer::tokens::Comparators;

use super::asm::ASM;

impl ASM {
    pub fn compare_two_numbers(&mut self, op: Comparators)
    {
        let mut instructions = vec![
            format!(";; We pop in the opposite order of comparison as we push onto the stack"),
            format!("pop rbx"),
            format!("pop rax"),
            format!("cmp rax, rbx"),
        ];

        let inst = match op {
            Comparators::LessThan => "jl .skip",
            Comparators::GreaterThan => "jg .skip",
            Comparators::LessThanEq => "jle .skip",
            Comparators::GreaterThanEq => "jge .skip",
        };

        instructions.push(String::from(inst));

        instructions.extend(vec![
            format!("mov rax, 0"),
            format!("jmp .skip_n"),
            format!(".skip:"),
            format!("mov rax, 1"),
            format!(".skip_n:")
        ]);

        for label in &mut self.labels {
            if label.name == "_start" {
                label.code.extend(instructions);
                break;
            }
        }
    }
}
