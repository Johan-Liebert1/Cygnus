use super::asm::ASM;

impl ASM {
    pub fn add_two_numbers<T>(&mut self, a: T, b: T)
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Sub<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: std::ops::Div<Output = T>,
        T: std::fmt::Debug,
    {
        let first = vec![
            format!("\tmov rax, {:?}\n", a),
            format!("\tmov rbx, {:?}\n", b),
            format!("\tadd rax, rbx\n"),
        ];

        for label in &mut self.labels {
            if label.name == "_start" {
                label.code.extend(first);
                break;
            }
        }
    }
}
