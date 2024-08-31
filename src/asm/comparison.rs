use crate::lexer::{registers::get_register_name_for_bits, tokens::Comparators, types::VarType};

use super::asm::ASM;

impl ASM {
    /// Only needed for comsd
    ///
    /// CMPEQSD    xmm1, xmm2	    CMPSD xmm1, xmm2, 0
    /// CMPLTSD    xmm1, xmm2	    CMPSD xmm1, xmm2, 1
    /// CMPLESD    xmm1, xmm2	    CMPSD xmm1, xmm2, 2
    /// CMPUNORDSD xmm1, xmm2	    CMPSD xmm1, xmm2, 3
    /// CMPNEQSD   xmm1, xmm2	    CMPSD xmm1, xmm2, 4
    /// CMPNLTSD   xmm1, xmm2	    CMPSD xmm1, xmm2, 5
    /// CMPNLESD   xmm1, xmm2	    CMPSD xmm1, xmm2, 6
    /// CMPORDSD   xmm1, xmm2	    CMPSD xmm1, xmm2, 7
    fn get_cmpsd_immediate_for_comparison_op(&self, op: &Comparators) -> i8 {
        match op {
            Comparators::DoubleEquals => 0,
            Comparators::NotEquals => 4,
            Comparators::LessThan => 1,
            Comparators::GreaterThan => 6,
            Comparators::LessThanEq => 2,
            Comparators::GreaterThanEq => 5,
        }
    }

    pub fn compare_ints(&mut self) -> Vec<String> {
        let first = self.stack_pop().unwrap();
        let second = self.stack_pop().unwrap();

        self.unlock_register_from_stack_value(&first);
        self.unlock_register_from_stack_value(&second);

        let rax = self.get_free_register(None);
        let rbx = self.get_free_register(None);

        self.unlock_register(rax);
        self.unlock_register(rbx);

        vec![
            format!(";; We pop in the opposite order of comparison as we push onto the stack"),
            format!("mov {rbx}, {first}"),
            format!("mov {rax}, {second}"),
            format!("cmp {rax}, {rbx}"),
        ]
    }

    pub fn compare_floats(&self) -> Vec<String> {
        vec![
            format!(";; Floating point comparison"),
            format!(";; Get the first operand"),
            format!("pop QWORD [float_imm]"),
            format!("movsd xmm1, [float_imm]"),
            format!(";; Get the second operand"),
            format!("pop QWORD [float_imm]"),
            format!("movsd xmm0, [float_imm]"),
            format!(";; floating point comparison"),
            format!("ucomisd xmm0, xmm1",),
        ]
    }

    pub fn compare_two_numbers(&mut self, op: Comparators, result_type: &VarType) {
        let mut instructions = match result_type {
            VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Char => self.compare_ints(),
            VarType::Float => self.compare_floats(),

            VarType::Ptr(inner_type) => match **inner_type {
                VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 | VarType::Char => self.compare_ints(),
                VarType::Float => self.compare_floats(),

                _ => {
                    unreachable!("Found type {result_type} while generating ASM for comparison op for a pointer type. This must be a bug in the semantic analysis step")
                }
            },

            _ => {
                unreachable!("Found type {result_type} while generating ASM for comparison op. This must be a bug in the semantic analysis step")
            }
        };

        let rax = self.get_free_register(None);
        let rbx = self.get_free_register(None);

        instructions.extend(
            vec![
                format!(";; Not xor-ing here as it sets flags"),
                format!("mov {rax}, 0"), 
                format!("mov {rbx}, 1")
            ]);

        instructions.push(match op {
            Comparators::LessThan => format!("cmovl {rax}, {rbx}"),
            Comparators::GreaterThan => format!("cmovg {rax}, {rbx}"),
            Comparators::LessThanEq => format!("cmovle {rax}, {rbx}"),
            Comparators::GreaterThanEq => format!("cmovge {rax}, {rbx}"),
            Comparators::DoubleEquals => format!("cmove {rax}, {rbx}"),
            Comparators::NotEquals => format!("cmovne {rax}, {rbx}"),
        });

        self.unlock_register(rbx);

        self.extend_current_label(instructions);
        self.stack_push(String::from(rax));

        self.comparison_num += 1;
    }
}
