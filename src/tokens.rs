#[derive(Debug, Clone)]
pub enum Operations {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug, Clone)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

#[derive(Debug, Clone)]
pub enum Bracket {
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
pub enum TokenEnum {
    Number(Number),

    Op(Operations),

    Equals,

    Bracket(Bracket),

    Unknown,
    EOF,
}
