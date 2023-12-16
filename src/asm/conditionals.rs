use super::asm::ASM;

pub enum ConditionalJumpTo {
    IfEnd,
    ElifEnd,
    Else,
    Elif,
}

impl ASM {
    pub fn if_start(&mut self, jump_to: ConditionalJumpTo) {
        // self.change_current_label(".if".into());

        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => ".if_end",
            ConditionalJumpTo::ElifEnd => panic!("Cannot jump to elif end from if start"),
            ConditionalJumpTo::Elif => ".elif_0",
            ConditionalJumpTo::Else => ".else",
        };

        let instructions = vec![
            // if label
            format!(".if:"),

            format!("pop rcx"),
            format!("cmp rcx, 0"),
            format!(";; if the comparison value is false, jump to the next label altogether"),
            format!("je {}", jump_to_label),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }

    pub fn if_end(&mut self, jump_to: ConditionalJumpTo, elif_len: usize) {
        let current_label = self.current_label();

        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => panic!("Cannot jump to if end from if end"),
            ConditionalJumpTo::ElifEnd => format!(".elif_{}_end", elif_len - 1),
            ConditionalJumpTo::Elif => panic!("Cannot jump to elif start from if end"),
            ConditionalJumpTo::Else => ".else_end".into(),
        };

        // if we ever enter the if block, then that's it, we can jump straight to the end of the else or the elif block
        let instructions = vec![format!("jmp {}", jump_to_label), ".if_end:".into()];

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }

    /// The label names for all elifs will be of the same format, i.e. <elif label name>_<elif_number>
    /// so that we can easily jump around
    pub fn elif_start(
        &mut self,
        label_name: String,
        elif_number: usize,
        jump_to: ConditionalJumpTo,
    ) {
        // self.change_current_label(format!(".{}_{}", label_name, elif_number));

        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => panic!("cannot jump to if from elif"),
            ConditionalJumpTo::ElifEnd => format!(".elif_{}_end", elif_number),
            ConditionalJumpTo::Elif => format!(".elif_{}", elif_number + 1),
            ConditionalJumpTo::Else => ".else".into(),
        };

        let instructions = vec![
            // elif label
            format!(".{}_{}:", label_name, elif_number),

            format!("pop rcx"),
            format!("cmp rcx, 0"),
            format!(";; if the comparison value is false, jump to the next label altogether"),
            format!("je {}", jump_to_label),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }

    // we need jump_to in case there is not else
    pub fn elif_end(&mut self, elif_number: usize, jump_to: ConditionalJumpTo) {
        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => panic!("Cannot jump to if end from elif end"),
            ConditionalJumpTo::ElifEnd => format!(".elif_{}_end", elif_number),
            ConditionalJumpTo::Elif => panic!("Cannot jump to elif start from elif end"),
            ConditionalJumpTo::Else => ".else_end".into(),
        };

        // if we ever enter the if block, then that's it, we can jump straight to the end of the else or the elif block
        let instructions = vec![
            format!("jmp {}", jump_to_label),
            format!(".elif_{}_end:", elif_number),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }

    /// The label name for else will be unique
    pub fn else_start(&mut self, label_name: String) {
        // self.change_current_label();

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.push(format!(".{}:", label_name));
                break;
            }
        }
    }

    pub fn else_end(&mut self, label_name: String) {
        // self.change_current_label(format!(".{}_end", label_name));

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.push(format!(".{}_end:", label_name));
                break;
            }
        }
    }
}
