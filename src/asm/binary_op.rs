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
        println!("add_two_numbers a = {:?}, b = {:?}", a, b);

        let first = vec![
            format!("mov rax, {:?}", a),
            format!("mov rbx, {:?}", b),
            format!("add rax, rbx"),
        ];

        for label in &mut self.labels {
            if label.name == "_start" {
                label.code.extend(first);
                break;
            }
        }
    }
}
