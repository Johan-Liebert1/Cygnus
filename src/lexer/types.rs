use core::panic;
use std::fmt::Display;

use crate::trace;

use super::tokens::AllOperations;

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Str,
    Float,
    Ptr(Box<VarType>),
    Unknown,
}

impl VarType {
    pub fn figure_out_type(&self, other: &VarType, op: AllOperations) -> VarType {
        trace!("self: {self}, other: {other}");

        return match (self, other) {
            (VarType::Int, VarType::Int) => VarType::Int,
            (VarType::Float, VarType::Float) => VarType::Float,

            (VarType::Int, VarType::Ptr(p)) | (VarType::Ptr(p), VarType::Int) => VarType::Ptr(p.clone()),

            (VarType::Int, VarType::Float) | (VarType::Float, VarType::Int) => {
                panic!("'{op}' not defined for '{self}' and '{other}'")
            }

            (VarType::Str, VarType::Float) | (VarType::Str, VarType::Str) | (VarType::Float, VarType::Str) => {
                panic!("'{op}' not defined for '{self}' and '{other}'")
            }

            (VarType::Int, VarType::Str) | (VarType::Str, VarType::Int) => {
                panic!("'{op}' not defined for '{self}' and '{other}'")
            }

            (VarType::Ptr(_), VarType::Str)
            | (VarType::Str, VarType::Ptr(_))
            | (VarType::Ptr(_), VarType::Float)
            | (VarType::Ptr(_), VarType::Ptr(_))
            | (VarType::Float, VarType::Ptr(_)) => panic!("'{op}' not defined for '{self}' and '{other}'"),

            _ => VarType::Unknown,
        };
    }
}

impl Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            VarType::Int => "Integer",
            VarType::Str => "String",
            VarType::Float => "Floating Point",
            VarType::Ptr(_) => "Pointer",
            VarType::Unknown => "Unknown",
        };

        write!(f, "{}", msg)
    }
}

// types
pub const TYPE_INT: &str = "int";
pub const TYPE_FLOAT: &str = "float";
pub const TYPE_STRING: &str = "str";

pub const TYPES: [&str; 3] = [TYPE_INT, TYPE_FLOAT, TYPE_STRING];
