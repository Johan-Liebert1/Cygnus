use crate::lexer::{tokens::Comparators, types::VarType};

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

        vec![
            format!(";; We pop in the opposite order of comparison as we push onto the stack"),
            format!("mov rbx, {first}"),
            format!("mov rax, {second}"),
            format!("cmp rax, rbx"),
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

        // TODO: Remove
        //
        // let inst = match op {
        //     Comparators::LessThan => format!("jl .skip_{}", self.comparison_num),
        //     Comparators::GreaterThan => format!("jg .skip_{}", self.comparison_num),
        //     Comparators::LessThanEq => format!("jle .skip_{}", self.comparison_num),
        //     Comparators::GreaterThanEq => format!("jge .skip_{}", self.comparison_num),
        //     Comparators::DoubleEquals => format!("je .skip_{}", self.comparison_num),
        //     Comparators::NotEquals => format!("jne .skip_{}", self.comparison_num),
        // };

        let inst = match op {
            Comparators::LessThan => format!("setl al"),
            Comparators::GreaterThan => format!("setg al"),
            Comparators::LessThanEq => format!("setle al"),
            Comparators::GreaterThanEq => format!("setge al"),
            Comparators::DoubleEquals => format!("sete al"),
            Comparators::NotEquals => format!("setne al"),
        };

        // TODO: Remove
        //
        // instructions.extend(vec![
        //     inst.into(),
        //     // assume the comparison is true
        //     format!("mov rax, 0"),
        //     format!("jmp .skip_next{}", self.comparison_num),
        //     // we'll skip to here if the comparison is false
        //     format!(".skip_{}:", self.comparison_num),
        //     format!("mov rax, 1"),
        //     format!(".skip_next{}:", self.comparison_num),
        //     format!(";; push onto the stack whatever's in rax so rest of the program can use it"),
        //     format!("push rax"),
        // ]);

        instructions.extend(vec![inst]);

        self.extend_current_label(instructions);

        self.stack_push("al".into());

        self.comparison_num += 1;
    }
}
