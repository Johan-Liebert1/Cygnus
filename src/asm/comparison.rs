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
            Comparators::LessThan => format!("jl .skip_{}", self.comparison_num),
            Comparators::GreaterThan =>format!( "jg .skip_{}", self.comparison_num),
            Comparators::LessThanEq =>format!( "jle .skip_{}", self.comparison_num),
            Comparators::GreaterThanEq =>format!( "jge .skip_{}", self.comparison_num),
        };

        instructions.push(String::from(inst));

        instructions.extend(vec![
            format!("mov rax, 0"),
            format!("jmp .skip_n{}", self.comparison_num),
            format!(".skip_{}:", self.comparison_num),
            format!("mov rax, 1"),
            format!(".skip_n{}:", self.comparison_num),

            format!(";; push onto the stack whatever's in rax so rest of the program can use it"),
            format!("push rax")
        ]);

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }

        self.comparison_num += 1;
    }
}
