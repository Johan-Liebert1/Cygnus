use core::panic;
use std::fmt::Display;

use crate::trace;

use super::tokens::AllOperations;

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Str,
    Float,
    Char,
    Ptr(Box<VarType>),
    Unknown,
}

impl VarType {
    pub fn get_pointer_type(&self) -> VarType {
        match self {
            VarType::Ptr(inner) => inner.get_pointer_type(),
            r => r.clone(),
        }
    }

    pub fn figure_out_type(&self, other: &VarType, op: AllOperations) -> VarType {
        return match (self, other) {
            (VarType::Int, VarType::Int) => VarType::Int,
            (VarType::Float, VarType::Float) => VarType::Float,

            (VarType::Int, VarType::Ptr(p)) | (VarType::Ptr(p), VarType::Int) => {
                match **p {
                    VarType::Int => VarType::Ptr(p.clone()),
                    // if a string ptr is added to an integer, it's now a pointer to a character
                    VarType::Str => VarType::Ptr(p.clone()), // VarType::Ptr(Box::new(VarType::Char)),
                    VarType::Float => todo!(),
                    VarType::Char => todo!(),
                    VarType::Ptr(_) => todo!(),
                    VarType::Unknown => todo!(),
                }
            }

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
            VarType::Int => "Integer".to_string(),
            VarType::Str => "String".to_string(),
            VarType::Float => "Floating Point".to_string(),
            VarType::Ptr(var_type) => format!("Pointer -> {}", *var_type),
            VarType::Char => "Character".to_string(),
            VarType::Unknown => "Unknown".to_string(),
        };

        write!(f, "{}", msg)
    }
}

// types
pub const TYPE_INT: &str = "int";
pub const TYPE_FLOAT: &str = "float";
pub const TYPE_STRING: &str = "str";

pub const TYPES: [&str; 3] = [TYPE_INT, TYPE_FLOAT, TYPE_STRING];
