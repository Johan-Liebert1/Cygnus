use super::asm::ASM;

pub enum ConditionalJumpTo {
    IfEnd,
    ElifEnd,
    Else,
    Elif,
}

impl ASM {
    pub fn if_start(&mut self, jump_to: ConditionalJumpTo) {
        self.change_current_label(".if".into());

        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => ".if_end",
            ConditionalJumpTo::ElifEnd => panic!("Cannot jump to elif end from if"),
            ConditionalJumpTo::Elif => ".elif_0",
            ConditionalJumpTo::Else => ".else",
        };

        let instructions = vec![
            format!("pop rcx"),
            format!(";; if the comparison value is false, jump to the next label altogether"),
            format!("jz {}", jump_to_label),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }

    pub fn if_end(&mut self) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.push(".if_end:".into());
                break;
            }
        }
    }

    /// The label names for all elifs will be of the same format, i.e. <elif label name>_<elif_number>
    /// so that we can easily jump around
    pub fn elif_start(&mut self, label_name: String, elif_number: usize, jump_to: ConditionalJumpTo) {
        self.change_current_label(format!(".{}_{}", label_name, elif_number));

        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => panic!("cannot jump to if from elif"),
            ConditionalJumpTo::ElifEnd => format!(".elif_{}_end", elif_number),
            ConditionalJumpTo::Elif => format!(".elif_{}", elif_number + 1),
            ConditionalJumpTo::Else => ".else".into(),
        };

        let instructions = vec![
            format!("pop rcx"),
            format!(";; if the comparison value is false, jump to the next label altogether"),
            format!("jz {}", jump_to_label),
        ];

        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.extend(instructions);
                break;
            }
        }
    }

    pub fn elif_end(&mut self, elif_number: usize) {
        let current_label = self.current_label();

        for label in &mut self.labels {
            if label.name == current_label {
                label.code.push(format!(".elif_{}_end:", elif_number));
                break;
            }
        }
    }

    /// The label name for else will be unique
    pub fn gen_else(&mut self, label_name: String) {
        self.change_current_label(format!(".{}", label_name));
    }
}
