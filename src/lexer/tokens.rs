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
pub enum TokenEnum {
    Number(Number),

    Op(Operations),

    Equals,

    Bracket(Bracket),

    Comparator(Comparators),

    Bool(bool),

    Keyword(String),

    Variable(String),

    Unknown,
    EOF,
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
}
