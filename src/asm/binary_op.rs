pub fn add_two_numbers<T>(a: &T, b: &T)
where
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::fmt::Debug,
{
    let str = format!(r#"

_start:
    mov rax, {:?}
    mov rbx, {:?}
    add rax, rbx

    "#, a, b);

    println!("{str}");
}
