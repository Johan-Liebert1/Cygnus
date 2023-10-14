#[derive(Debug)]
pub enum Operations {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

#[derive(Debug)]
pub enum TokenEnum {
    Number(Number),

    Op(Operations),

    Equals,

    LParen,
    RParen,

    Unknown,
    EOF,
}
