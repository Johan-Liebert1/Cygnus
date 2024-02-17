use std::fmt::Display;

use super::types::VarType;

#[derive(Debug, Clone, PartialEq)]
pub enum VariableEnum {
    Number(Number),
    String(String),
    /// Pointer(TypeName) -> Pointer("INT") etc..
    Pointer(String),
}

impl VariableEnum {
    pub fn size(&self) -> usize {
        match self {
            VariableEnum::Number(n) => match n {
                // 64 bit integer
                Number::Integer(_) => 8,
                Number::Float(_) => todo!(),
            },

            // 8 bytes for length + 8 bytes for pointer to the start of the string
            VariableEnum::String(_) => 16,

            // Pointer will always consume 8 bytes
            VariableEnum::Pointer(..) => 8,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentTypes {
    Equals,
    PlusEquals,
    MinusEquals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    Plus,
    Minus,
    Divide,
    Multiply,
    ShiftLeft,
    ShiftRight,
    Modulo,
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
    DoubleEquals,
    NotEquals,
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
pub enum LogicalOps {
    Or,
    And,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenEnum {
    Equals,
    PlusEquals,
    MinusEquals,
    Comma,
    Colon,
    SemiColon,
    Ampersand,

    Number(Number),
    Bracket(Bracket),
    Op(Operations),
    Comparator(Comparators),
    LogicalOp(LogicalOps),

    Bool(bool),
    Keyword(String),
    Variable(String),
    Type(VarType),

    StringLiteral(String),

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

    pub fn get_assignment_type(&self) -> AssignmentTypes {
        return match self {
            TokenEnum::PlusEquals => AssignmentTypes::PlusEquals,
            TokenEnum::MinusEquals => AssignmentTypes::MinusEquals,
            TokenEnum::Equals => AssignmentTypes::Equals,
            _ => panic!("{:?} is not an assignment token", self)
        };
    }
}
