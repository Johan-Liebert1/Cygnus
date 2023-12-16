#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub code: Vec<String>,
}

#[derive(Debug)]
pub struct ASM {
    pub include: Vec<&'static str>,
    pub text: Vec<String>,
    pub bss: Vec<String>,
    pub labels: Vec<Label>,
    pub comparison_num: usize,
    current_label: String,
}

impl Default for ASM {
    fn default() -> Self {
        Self {
            comparison_num: 0,
            current_label: "_start".to_string(),

            include: vec![
                r#"%include "std.asm""#
            ],

            text: vec![
                String::from("\tglobal _start")
            ],

            bss: vec![
                // for printing numbers
                String::from("\tdigitSpace resb 100"),
                String::from("\tdigitSpacePos resb 8"),
            ],

            labels: vec![Label {
                name: String::from("_start"),
                code: vec![],
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

        self.labels.push(Label { name: new_label, code: vec![] });
    }

    pub fn current_label(&self) -> String {
        return self.current_label.clone();
    }
}
