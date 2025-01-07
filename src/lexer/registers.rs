// R8 through R15 (new registers introduced in x86_64):
//
// 64-bit: r8 to r15
// 32-bit: r8d to r15d
// 16-bit: r8w to r15w
// 8-bit: r8b to r15b (lower 8 bits)

use core::panic;
use std::fmt::{Debug, Display};

use super::types::VarType;

#[derive(Clone, Copy, PartialEq)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    R8,
    R9,
    R10,
    R11,

    // Float regs
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
}

pub const ALL_REGISTERS: [Register; 11] = [
    Register::RAX,
    Register::RBX,
    Register::RCX,
    Register::RDX,
    Register::RSI,
    Register::RDI,
    Register::RBP,
    Register::R8,
    Register::R9,
    Register::R10,
    Register::R11,
];

pub const ALL_FP_REGISTERS: [Register; 8] = [
    Register::XMM0,
    Register::XMM1,
    Register::XMM2,
    Register::XMM3,
    Register::XMM4,
    Register::XMM5,
    Register::XMM6,
    Register::XMM7,
];

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
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",

            Register::XMM0 => "xmm0",
            Register::XMM1 => "xmm1",
            Register::XMM2 => "xmm2",
            Register::XMM3 => "xmm3",
            Register::XMM4 => "xmm4",
            Register::XMM5 => "xmm5",
            Register::XMM6 => "xmm6",
            Register::XMM7 => "xmm7",
        };

        write!(f, "{}", s)
    }
}

impl Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Register> for String {
    fn from(value: Register) -> Self {
        format!("{value}")
    }
}

impl Register {
    pub fn from_string(reg_str: &String) -> Self {
        match reg_str.as_str() {
            "rax" => Register::RAX,
            "rbx" => Register::RBX,
            "rcx" => Register::RCX,
            "rdx" => Register::RDX,
            "rsi" => Register::RSI,
            "rdi" => Register::RDI,
            "rbp" => Register::RBP,
            "r8" => Register::R8,
            "r9" => Register::R9,
            "r10" => Register::R10,
            "r11" => Register::R11,

            "xmm0" => Register::XMM0,
            "xmm1" => Register::XMM1,
            "xmm2" => Register::XMM2,
            "xmm3" => Register::XMM3,
            "xmm4" => Register::XMM4,
            "xmm5" => Register::XMM5,
            "xmm6" => Register::XMM6,
            "xmm7" => Register::XMM7,

            r => panic!("{r} is not a register"),
        }
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

        _ => todo!(),
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

            Self::Ptr(..) => get_register_name_for_bits(&register, 64),

            v => panic!("get_register_name not implemented for '{}'", v),
        };

        return thing;
    }
}
