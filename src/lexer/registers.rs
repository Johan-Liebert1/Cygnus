// R8 through R15 (new registers introduced in x86_64):
//
// 64-bit: r8 to r15
// 32-bit: r8d to r15d
// 16-bit: r8w to r15w
// 8-bit: r8b to r15b (lower 8 bits)

use core::panic;
use std::fmt::Display;

use super::types::VarType;

const RAX: &str = "rax";
const RBX: &str = "rbx";
const RCX: &str = "rcx";
const RDX: &str = "rdx";
const RSI: &str = "rsi";
const RDI: &str = "rdi";
const RBP: &str = "rbp";

#[derive(Clone, Copy)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Register::RAX => "rax",
            Register::RBX => "rbx",
            Register::RCX => "rcx",
            Register::RDX => "rdx",
            Register::RSI => "rsi",
            Register::RDI => "rdi",
            Register::RBP => "rbp",
        };

        write!(f, "{}", s)
    }
}

pub fn get_register_name_for_bits(register: &Register, bits: u8) -> &'static str {
    return match register {
        Register::RAX => match bits {
            8 => "al",
            16 => "ax",
            32 => "eax",
            _ => "rax",
        },
        Register::RBX => match bits {
            8 => "bl",
            16 => "bx",
            32 => "ebx",
            _ => "rbx",
        },
        Register::RCX => match bits {
            8 => "cl",
            16 => "cx",
            32 => "ecx",
            _ => "rcx",
        },
        Register::RDX => match bits {
            8 => "dl",
            16 => "dx",
            32 => "edx",
            _ => "rdx",
        },
        Register::RSI => match bits {
            8 => "sil",
            16 => "si",
            32 => "esi",
            _ => "rsi",
        },
        Register::RDI => match bits {
            8 => "dil",
            16 => "di",
            32 => "edi",
            _ => "rdi",
        },
        Register::RBP => match bits {
            8 => "bpl",
            16 => "bp",
            32 => "ebp",
            _ => "rbp",
        },

        _ => panic!(),
    };
}

impl VarType {
    pub fn get_register_name(&self, register: Register) -> &'static str {
        let thing = match self {
            Self::Int => get_register_name_for_bits(&register, 64),
            Self::Int32 => get_register_name_for_bits(&register, 32),
            Self::Int16 => get_register_name_for_bits(&register, 16),
            Self::Int8 => get_register_name_for_bits(&register, 8),
            Self::Char => get_register_name_for_bits(&register, 8),
            Self::Float => get_register_name_for_bits(&register, 64),

            v => panic!("get_register_name not implemented for '{}'", v),
        };

        return thing;
    }
}
