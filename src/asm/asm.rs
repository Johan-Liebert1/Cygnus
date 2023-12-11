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
}

impl Default for ASM {
    fn default() -> Self {
        Self {
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
