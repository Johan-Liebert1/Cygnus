use core::fmt;
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

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Operations::Plus => "+",
            Operations::Minus => "-",
            Operations::Divide => "/",
            Operations::Multiply => "*",
            Operations::ShiftLeft => "<<",
            Operations::ShiftRight => ">>",
            Operations::Modulo => "%",
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(_) => write!(f, "{}", "Integer"),
            Number::Float(_) => write!(f, "{}", "Float"),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        // only compare the enum variant and not the value inside it
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Bracket {
    LParen,
    RParen,
    LCurly,
    RCurly,
}

impl Display for Bracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bracket::LParen => write!(f, "{}", "("),
            Bracket::RParen => write!(f, "{}", ")"),
            Bracket::LCurly => write!(f, "{}", "{"),
            Bracket::RCurly => write!(f, "{}", "}"),
        }
    }
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

impl Display for Comparators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Comparators::DoubleEquals => "==",
            Comparators::NotEquals => "!=",
            Comparators::LessThan => "<",
            Comparators::GreaterThan => ">",
            Comparators::LessThanEq => "<=",
            Comparators::GreaterThanEq => ">=",
        };

        write!(f, "{}", msg)
    }
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

impl Display for LogicalOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LogicalOps::Or => "or",
            LogicalOps::And => "and",
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AllOperations {
    Op(Operations),
    Comparator(Comparators),
    LogicalOp(LogicalOps),
}

impl Display for AllOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllOperations::Op(op) => fmt::Display::fmt(&op, f),
            AllOperations::Comparator(com) => fmt::Display::fmt(&com, f),
            AllOperations::LogicalOp(lo) => fmt::Display::fmt(&lo, f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenEnum {
    Equals,
    PlusEquals,
    MinusEquals,
    Comma,
    Colon,
    SemiColon,
    Ampersand,
    FunctionReturnIndicator,

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

impl Display for TokenEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenEnum::Equals => write!(f, "{}", "="),
            TokenEnum::PlusEquals => write!(f, "{}", "+="),
            TokenEnum::MinusEquals => write!(f, "{}", "-="),
            TokenEnum::Comma => write!(f, "{}", ","),
            TokenEnum::Colon => write!(f, "{}", ":"),
            TokenEnum::SemiColon => write!(f, "{}", ";"),
            TokenEnum::Ampersand => write!(f, "{}", "&"),
            TokenEnum::FunctionReturnIndicator => write!(f, "{}", "->"),
            TokenEnum::Number(token) => write!(f, "{}", token),
            TokenEnum::Bracket(token) => write!(f, "{}", token),
            TokenEnum::Op(token) => write!(f, "{}", token),
            TokenEnum::Comparator(token) => write!(f, "{}", token),
            TokenEnum::LogicalOp(token) => write!(f, "{}", token),
            TokenEnum::Bool(token) => write!(f, "{}", token),
            TokenEnum::Keyword(token) => write!(f, "{}", token),
            TokenEnum::Variable(token) => write!(f, "{}", token),
            TokenEnum::Type(token) => write!(f, "{}", token),
            TokenEnum::StringLiteral(token) => write!(f, "{}", token),
            TokenEnum::Unknown(token) => write!(f, "{}", token),
            TokenEnum::EOF => write!(f, "{}", "EOF"),
        }
        
    }
}

impl PartialEq for TokenEnum {
    fn eq(&self, other: &Self) -> bool {
        use TokenEnum::*;

        match (self, other) {
            (Number(a), Number(b)) => a == b,
            (Op(a), Op(b)) => a == b,
            (Comparator(a), Comparator(b)) => a == b,
            (LogicalOp(a), LogicalOp(b)) => a == b,

            // only compare the enum variant and not the value inside it
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
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
            _ => panic!("{:?} is not an assignment token", self),
        };
    }
}
