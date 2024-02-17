use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Str,
    Float,
    Ptr(Box<VarType>),
    Unknown,
}

impl Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// types
pub const TYPE_INT: &str = "int";
pub const TYPE_FLOAT: &str = "float";
pub const TYPE_STRING: &str = "str";

pub const TYPES: [&str; 3] = [TYPE_INT, TYPE_FLOAT, TYPE_STRING];
