#[derive(Debug)]
pub enum TokenType {
    Integer(i32),
    Float(f32),

    Plus,
    Minus,
    Divide,
    Multiply,
    Equals,

    LParen,
    RParen,

    Unknown,
    EOF,
}
