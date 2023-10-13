#[derive(Debug)]
pub enum Operations {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug)]
pub enum TokenType {
    Integer(i32),
    Float(f32),

    Op(Operations),

    Equals,

    LParen,
    RParen,

    Unknown,
    EOF,
}
