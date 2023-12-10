#[derive(Debug)]
struct Label {
    name: String,
    code: String,
}

#[derive(Debug)]
pub struct ASM {
    text: String,
    bss: String,
    labels: Vec<Label>,
}

impl Default for ASM {
    fn default() -> Self {
        Self {
            text: String::new(),
            bss: String::new(),
            labels: Vec::new(),
        }
    }
}
