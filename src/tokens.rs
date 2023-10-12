#[derive(Debug)]
pub enum Tokens {
    Integer(i32),
    Float(f32),
    Plus,
    Minus,
    Divide,
    Multiply,
    Equals,
    Unknown,
}
