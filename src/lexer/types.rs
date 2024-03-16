use core::panic;
use std::{cell::RefCell, fmt::Display, process::exit, rc::Rc};

use crate::{
    ast::{abstract_syntax_tree::AST, variable::Variable},
    helpers::compiler_error,
    trace,
};

use super::{
    lexer::Token,
    tokens::{AllOperations, Comparators, Operations},
};

#[derive(Debug, Clone)]
pub struct StructMemberType {
    pub name: String,
    pub member_type: VarType,
}

#[derive(Debug, Clone)]
pub enum VarType {
    Int,
    Str,
    Float,
    Char,
    Ptr(Box<VarType>),
    Array(Box<VarType>, usize),
    Struct(String, Rc<RefCell<Vec<StructMemberType>>>), // string = name of struct
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

            (VarType::Struct(name1, members1), VarType::Struct(name2, members2)) => {
                if name1 != name2 {
                    return false;
                }

                if (members1).borrow().len() != members2.borrow().len() {
                    return false;
                }

                let member2_borrow = members2.borrow();

                for mem1 in members1.borrow().iter() {
                    let found = member2_borrow.iter().find(|mem2| mem2.name == mem1.name);

                    if found.is_none() {
                        return false;
                    }

                    if mem1.member_type != found.unwrap().member_type {
                        return false;
                    }
                }

                return true;
            }

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

    fn type_int(&self, other: &VarType) -> VarType {
        if !matches!(self, VarType::Int) {
            panic!("function `type_int` only accepts VarType::Int as the first arg")
        }

        return match other {
            VarType::Int => VarType::Int,
            VarType::Str => todo!(),
            VarType::Float => todo!(),
            VarType::Char => todo!(),
            VarType::Ptr(_) => todo!(),
            VarType::Array(_, _) => todo!(),
            VarType::Struct(..) => todo!(),
            VarType::Unknown => todo!(),
        };
    }

    pub fn figure_out_type(&self, other: &VarType, op: AllOperations) -> VarType {
        use Comparators::*;
        use Operations::*;
        use VarType::*;

        return match (self, other) {
            // No matter what the op is, the result will always be an integer
            (Int, Int) => Int,

            // No matter what the op is, the result will always be an float
            (Float, Float) => Float,

            // Incrementing a pointer
            (Int, Ptr(..)) | (Ptr(..), Int) => {
                let is_allowed = matches!(
                    op,
                    AllOperations::Op(Plus) // only addition is allowed
                        | AllOperations::Comparator(..) // all comparisons allowed
                                                        // Logical and/or not allowed
                );

                if !is_allowed {
                    panic!("'{op}' not defined for '{self}' and '{other}'")
                }

                // any pointer incremented is the same pointer to the same type unless casted
                self.clone()
            }

            (Ptr(ptr1), Ptr(ptr2)) => ptr1.figure_out_type(ptr2, op),

            (Char, Char) | (Int, Char) | (Char, Int) => {
                let is_allowed = matches!(
                    op,
                    AllOperations::Comparator(..) // only comparisons allowed
                );

                if !is_allowed {
                    panic!("'{op}' not defined for '{self}' and '{other}'")
                }

                // result of comparison is always an int
                Int
            }

            (Ptr(_), Str)
            | (Str, Ptr(_))
            | (Ptr(_), Float)
            | (Float, Ptr(_))
            | (Char, Ptr(_))
            | (Ptr(_), Char)
            | (Int, Str)
            | (Str, Int)
            | (Str, Float)
            | (Str, Str)
            | (Float, Str)
            | (Int, Float)
            | (Float, Int) => {
                panic!("'{op}' not defined for '{self}' and '{other}'")
            }

            (l, r) => {
                trace!("l: {}", l);
                trace!("r: {}", r);

                Unknown
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

            VarType::Struct(_, members) => {
                // let size = members
                //     .borrow()
                //     .iter()
                //     .map(|var| var.get_type().0.get_size())
                //     .reduce(|acc, var| var);

                let size = members
                    .borrow()
                    .iter()
                    .map(|var_type| var_type.member_type.get_size())
                    .reduce(|acc, var_type| var_type);

                trace!("Struct size: {:#?}", size);

                match size {
                    Some(s) => s,
                    None => 0,
                }
            }
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
            VarType::Struct(name, _) => name.into(),
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

pub const PREDEFINED_TYPES: [&str; 4] = [TYPE_INT, TYPE_FLOAT, TYPE_STRING, TYPE_CHAR];
