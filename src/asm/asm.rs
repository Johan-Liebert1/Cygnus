#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub code: Vec<String>,
}

#[derive(Debug)]
pub struct ASM {
    pub include: Vec<&'static str>,
    pub text: Vec<String>,
    pub data: Vec<String>,
    pub bss: Vec<String>,
    pub labels: Vec<Label>,
    pub comparison_num: usize,
    pub num_strings: usize,
    pub num_ifs: usize,
    pub num_loops: usize,

    current_label: String,
}

impl Default for ASM {
    fn default() -> Self {
        Self {
            num_strings: 0,
            comparison_num: 0,
            current_label: "_start".to_string(),
            num_ifs: 0,
            num_loops: 0,

            include: vec![r#"%include "std.asm""#],

            text: vec![String::from("global _start")],

            data: vec![],

            bss: vec![
                // for printing numbers
                String::from("digitSpace resb 100"),
                String::from("digitSpacePos resb 8"),
                String::from("argc resb 8"),
            ],

            labels: vec![Label {
                name: String::from("_start"),
                code: vec![
                    // save argc and argv

                    // top of stack -> argc argv0 argv1 argv2 ...
                    format!("mov [argc], rsp"),
                ],
            }],
        }
    }
}

impl ASM {
    pub fn change_current_label(&mut self, new_label: String) {
        self.current_label = new_label.clone();

        for l in &self.labels {
            if l.name == new_label {
                return;
            }
        }

        self.labels.push(Label {
            name: new_label,
            code: vec![],
        });
    }

    pub fn current_label(&self) -> String {
        return self.current_label.clone();
    }
}
