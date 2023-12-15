use crate::lexer::tokens::Comparators;

use super::asm::ASM;

impl ASM {
    pub fn compare_two_numbers<T>(&mut self, l: T, r: T, op: Comparators)
    where
        T: PartialOrd + std::fmt::Debug,
    {
        let mut instructions = vec![
            format!("mov rax, {:?}", l),
            format!("mov rbx, {:?}", r),
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
            format!("mov rax, 1"),
            format!(".skip:"),
            format!("mov rax, 0"),
        ]);

        for label in &mut self.labels {
            if label.name == "_start" {
                label.code.extend(instructions);
                break;
            }
        }
    }
}
