use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Bracket {
    LParen,
    RParen,
    LCurly,
    RCurly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparators {
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Boolean {
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Number(Number),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenEnum {
    Equals,

    Number(Number),
    Bracket(Bracket),
    Op(Operations),
    Comparator(Comparators),

    Bool(bool),
    Keyword(String),
    Variable(String),
    Type(String),

    Colon,

    Unknown(String),
    EOF,
}

pub struct OperandConversionError(TokenEnum);

impl Display for OperandConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} cannot be used as an operand", self.0)
    }
}

impl TokenEnum {
    pub fn is_number(&self) -> bool {
        match self {
            TokenEnum::Number(..) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            TokenEnum::Number(n) => match n {
                Number::Integer(..) => true,
                Number::Float(..) => false,
            },
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            TokenEnum::Number(n) => match n {
                Number::Float(..) => true,
                Number::Integer(..) => false,
            },
            _ => false,
        }
    }

    pub fn get_operand(&self) -> Result<Operand, OperandConversionError> {
        match self {
            TokenEnum::Number(n) => return Ok(Operand::Number(n.clone())),

            TokenEnum::Variable(n) => return Ok(Operand::Variable(n.to_string())),

            _ => Err(OperandConversionError(self.clone())),
        }
    }

    pub fn new_float(f: f32) -> TokenEnum {
        return TokenEnum::Number(Number::Float(f));
    }

    pub fn new_integer(f: i32) -> TokenEnum {
        return TokenEnum::Number(Number::Integer(f));
    }
}
