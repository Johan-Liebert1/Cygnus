use core::panic;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Str,
    Float,
    Ptr(Box<VarType>),
    Unknown,
}

impl VarType {
    pub fn figure_out_type(&self, other: &VarType) -> VarType {
        return match (self, other) {
            (VarType::Int, VarType::Int) => VarType::Int,
            (VarType::Float, VarType::Float) => VarType::Float,

            (VarType::Int, VarType::Ptr(p)) | (VarType::Ptr(p), VarType::Int) => VarType::Ptr(p.clone()),

            (VarType::Int, VarType::Float) | (VarType::Float, VarType::Int) => panic!("Cannot add float to int"),

            (VarType::Str, VarType::Float) |
            (VarType::Str, VarType::Str) |
            (VarType::Float, VarType::Str) => panic!("Addition for string is not defined"),

            (VarType::Int, VarType::Str) | (VarType::Str, VarType::Int) => {
                panic!("Adding string and int is not allowed")
            }

            (VarType::Ptr(_), VarType::Str)
            | (VarType::Str, VarType::Ptr(_))
            | (VarType::Ptr(_), VarType::Float)
            | (VarType::Ptr(_), VarType::Ptr(_))
            | (VarType::Float, VarType::Ptr(_)) => panic!("Pointers can only be added to integers"),

            _ => VarType::Unknown,
        };
    }
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
