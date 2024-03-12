use core::panic;
use std::{fmt::Display, process::exit};

use crate::{helpers::compiler_error, trace};

use super::{lexer::Token, tokens::AllOperations};

#[derive(Debug, Clone)]
pub enum VarType {
    Int,
    Str,
    Float,
    Char,
    Ptr(Box<VarType>),
    Array(Box<VarType>, usize),
    Unknown,
}

impl PartialEq for VarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VarType::Int, VarType::Int)
            | (VarType::Float, VarType::Float)
            | (VarType::Char, VarType::Char)
            | (VarType::Unknown, VarType::Unknown) => true,

            (VarType::Str, VarType::Str) => true,

            (VarType::Str, VarType::Ptr(boxed)) | (VarType::Ptr(boxed), VarType::Str) => {
                matches!(**boxed, VarType::Char)
            }

            (VarType::Ptr(a), VarType::Ptr(b)) => a == b,

            (VarType::Array(a, s1), VarType::Array(b, s2)) => a == b && s1 == s2,

            _ => false,
        }
    }
}

impl VarType {
    pub fn get_pointer_type(&self) -> VarType {
        match self {
            VarType::Ptr(inner) => inner.get_pointer_type(),
            r => r.clone(),
        }
    }

    pub fn get_actual_type(&self, times_dereferenced: usize, token: &Token) -> VarType {
        return match self {
            VarType::Ptr(ptr_var_type) => {
                if times_dereferenced > 0 {
                    ptr_var_type.get_actual_type(times_dereferenced - 1, token)
                } else {
                    self.clone()
                }
            }

            // Dereferencing a string should give you a character
            VarType::Str => match times_dereferenced {
                0 => VarType::Str,
                1 => VarType::Char,
                _ => {
                    compiler_error(format!("Cannot dereference Character"), token);
                    exit(1);
                }
            },

            t => {
                if times_dereferenced > 0 {
                    compiler_error(format!("Cannot dereference {self}"), token);
                    exit(1);
                } else {
                    t.clone()
                }
            }
        };
    }

    pub fn figure_out_type(&self, other: &VarType, op: AllOperations) -> VarType {
        return match (self, other) {
            (VarType::Int, VarType::Int) => VarType::Int,

            (VarType::Float, VarType::Float) => match op {
                AllOperations::Op(_) => todo!(),
                AllOperations::Comparator(_) => todo!(),
                AllOperations::LogicalOp(_) => todo!(),
            },

            (VarType::Int, VarType::Ptr(p)) | (VarType::Ptr(p), VarType::Int) => {
                match **p {
                    VarType::Int => VarType::Ptr(p.clone()),

                    // if a string ptr is added to an integer, it's now a pointer to a character
                    VarType::Str => VarType::Ptr(p.clone()), // VarType::Ptr(Box::new(VarType::Char)),

                    VarType::Char => VarType::Ptr(p.clone()),

                    VarType::Float => todo!(),
                    VarType::Ptr(_) => todo!(),
                    VarType::Array(..) => todo!(),
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

            (VarType::Ptr(ptr1), VarType::Ptr(ptr2)) => ptr1.figure_out_type(ptr2, op),

            (VarType::Char, VarType::Char) => match op {
                AllOperations::Op(_) => todo!(),
                AllOperations::Comparator(_) => VarType::Int,
                AllOperations::LogicalOp(_) => todo!(),
            } ,

            (VarType::Ptr(_), VarType::Str)
            | (VarType::Str, VarType::Ptr(_))
            | (VarType::Ptr(_), VarType::Float)
            | (VarType::Float, VarType::Ptr(_)) => panic!("'{op}' not defined for '{self}' and '{other}'"),

            (l, r) => {
                trace!("l: {}", l);
                trace!("r: {}", r);

                VarType::Unknown
            }
        };
    }

    pub fn get_size(&self) -> usize {
        return match self {
            // 64 bit integer
            VarType::Int => 8,
            // 8 bytes for length + 8 bytes for pointer to the start of the string
            VarType::Str => 16,
            VarType::Float => todo!(),
            // char is only 1 byte
            VarType::Char => 1,
            // Pointer will always consume 8 bytes
            VarType::Ptr(_) => 8,
            VarType::Unknown => todo!(),

            VarType::Array(type_, elements) => type_.get_size() * elements,
        };
    }

    pub fn get_underlying_type_size(&self) -> usize {
        return match self {
            VarType::Ptr(type_) => type_.get_size(),
            VarType::Array(type_, _) => type_.get_size(),

            _ => self.get_size(),
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
            VarType::Array(var_type, sz) => format!("Array of {} of size {sz}", *var_type),
        };

        write!(f, "{}", msg)
    }
}

// types
pub const TYPE_INT: &str = "int";
pub const TYPE_FLOAT: &str = "float";
pub const TYPE_STRING: &str = "str";
pub const TYPE_CHAR: &str = "char";

pub const TYPES: [&str; 4] = [TYPE_INT, TYPE_FLOAT, TYPE_STRING, TYPE_CHAR];
