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
#[rustfmt::skip]
pub enum Register {
    RAX, EAX, AX, AL,
    RBX, EBX, BX, BL,
    RCX, ECX, CX, CL,
    RDX, EDX, DX, DL,
    RSI, ESI, SI, SIL,
    RDI, EDI, DI, DIL,
    RBP, EBP, BP, BPL,
    R8,  R8D, R8W, R8B,
    R9,  R9D, R9W, R9B,
    R10, R10D, R10W, R10B,
    R11, R11D, R11W, R11B,

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

pub const ALL_64BIT_REGISTERS: [Register; 11] = [
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

#[rustfmt::skip]
pub const ALL_REGISTERS: [Register; 44] = [
    Register::RAX, Register::EAX,  Register::AX,   Register::AL,
    Register::RBX, Register::EBX,  Register::BX,   Register::BL,
    Register::RCX, Register::ECX,  Register::CX,   Register::CL,
    Register::RDX, Register::EDX,  Register::DX,   Register::DL,
    Register::RSI, Register::ESI,  Register::SI,   Register::SIL,
    Register::RDI, Register::EDI,  Register::DI,   Register::DIL,
    Register::RBP, Register::EBP,  Register::BP,   Register::BPL,
    Register::R8,  Register::R8D,  Register::R8W,  Register::R8B,
    Register::R9,  Register::R9D,  Register::R9W,  Register::R9B,
    Register::R10, Register::R10D, Register::R10W, Register::R10B,
    Register::R11, Register::R11D, Register::R11W, Register::R11B,
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
        write!(f, "{}", self.as_ref())
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

impl AsRef<str> for Register {
    fn as_ref(&self) -> &str {
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

            Register::EAX => "eax",
            Register::AX => "ax",
            Register::AL => "al",
            Register::EBX => "ebx",
            Register::BX => "bx",
            Register::BL => "bl",
            Register::ECX => "ecx",
            Register::CX => "cx",
            Register::CL => "cl",
            Register::EDX => "edx",
            Register::DX => "dx",
            Register::DL => "dl",
            Register::ESI => "esi",
            Register::SI => "si",
            Register::SIL => "sil",
            Register::EDI => "edi",
            Register::DI => "di",
            Register::DIL => "dil",
            Register::EBP => "ebp",
            Register::BP => "bp",
            Register::BPL => "bpl",
            Register::R8D => "r8d",
            Register::R8W => "r8w",
            Register::R8B => "r8b",
            Register::R9D => "r9d",
            Register::R9W => "r9w",
            Register::R9B => "r9b",
            Register::R10D => "r10d",
            Register::R10W => "r10w",
            Register::R10B => "r10b",
            Register::R11D => "r11d",
            Register::R11W => "r11w",
            Register::R11B => "r11b",
        };

        return s;
    }
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        return Register::from_string(&value.to_string());
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
            "eax" => Register::EAX,
            "ax" => Register::AX,
            "al" => Register::AL,
            "ebx" => Register::EBX,
            "bx" => Register::BX,
            "bl" => Register::BL,
            "ecx" => Register::ECX,
            "cx" => Register::CX,
            "cl" => Register::CL,
            "edx" => Register::EDX,
            "dx" => Register::DX,
            "dl" => Register::DL,
            "esi" => Register::ESI,
            "si" => Register::SI,
            "sil" => Register::SIL,
            "edi" => Register::EDI,
            "di" => Register::DI,
            "dil" => Register::DIL,
            "ebp" => Register::EBP,
            "bp" => Register::BP,
            "bpl" => Register::BPL,
            "r8d" => Register::R8D,
            "r8w" => Register::R8W,
            "r8b" => Register::R8B,
            "r9d" => Register::R9D,
            "r9w" => Register::R9W,
            "r9b" => Register::R9B,
            "r10d" => Register::R10D,
            "r10w" => Register::R10W,
            "r10b" => Register::R10B,
            "r11d" => Register::R11D,
            "r11w" => Register::R11W,
            "r11b" => Register::R11B,

            r => panic!("{r} is not a register"),
        }
    }

    pub fn is_reg(string: &String) -> bool {
        return ALL_REGISTERS.iter().any(|r| format!("{}", r) == *string)
            || ALL_FP_REGISTERS.iter().any(|fr| format!("{}", fr) == *string);
    }

    pub fn is_64bit_reg(&self) -> bool {
        return ALL_64BIT_REGISTERS.contains(self);
    }

    pub fn get_all_register_variants(&self) -> Vec<Register> {
        let mut all_regs = vec![];

        for bits in [8, 16, 32, 64] {
            all_regs.push(get_register_name_for_bits(self, bits));
        }

        return all_regs;
    }

    /// Returns if 'self' is any variant of 'other'
    /// self = eax, other = rax -> true
    pub fn is(&self, other: Self) -> bool {
        match self {
            Register::RAX | Register::EAX | Register::AX | Register::AL => {
                matches!(other, Register::RAX | Register::EAX | Register::AX | Register::AL)
            }
            Register::RBX | Register::EBX | Register::BX | Register::BL => {
                matches!(other, Register::RBX | Register::EBX | Register::BX | Register::BL)
            }
            Register::RCX | Register::ECX | Register::CX | Register::CL => {
                matches!(other, Register::RCX | Register::ECX | Register::CX | Register::CL)
            }
            Register::RDX | Register::EDX | Register::DX | Register::DL => {
                matches!(other, Register::RDX | Register::EDX | Register::DX | Register::DL)
            }
            Register::RSI | Register::ESI | Register::SI | Register::SIL => {
                matches!(other, Register::RSI | Register::ESI | Register::SI | Register::SIL)
            }
            Register::RDI | Register::EDI | Register::DI | Register::DIL => {
                matches!(other, Register::RDI | Register::EDI | Register::DI | Register::DIL)
            }
            Register::RBP | Register::EBP | Register::BP | Register::BPL => {
                matches!(other, Register::RBP | Register::EBP | Register::BP | Register::BPL)
            }
            Register::R8 | Register::R8D | Register::R8W | Register::R8B => {
                matches!(other, Register::R8 | Register::R8D | Register::R8W | Register::R8B)
            }
            Register::R9 | Register::R9D | Register::R9W | Register::R9B => {
                matches!(other, Register::R9 | Register::R9D | Register::R9W | Register::R9B)
            }
            Register::R10 | Register::R10D | Register::R10W | Register::R10B => {
                matches!(other, Register::R10 | Register::R10D | Register::R10W | Register::R10B)
            }
            Register::R11 | Register::R11D | Register::R11W | Register::R11B => {
                matches!(other, Register::R11 | Register::R11D | Register::R11W | Register::R11B)
            }

            r => *r == other,
        }
    }

    pub fn get_64bit_version(&self) -> Self {
        match self {
            Register::EAX | Register::AX | Register::AL => Register::RAX,
            Register::EBX | Register::BX | Register::BL => Register::RBX,
            Register::ECX | Register::CX | Register::CL => Register::RCX,
            Register::EDX | Register::DX | Register::DL => Register::RDX,
            Register::ESI | Register::SI | Register::SIL => Register::RSI,
            Register::EDI | Register::DI | Register::DIL => Register::RDI,
            Register::EBP | Register::BP | Register::BPL => Register::RBP,
            Register::R8D | Register::R8W | Register::R8B => Register::R8,
            Register::R9D | Register::R9W | Register::R9B => Register::R9,
            Register::R10D | Register::R10W | Register::R10B => Register::R10,
            Register::R11D | Register::R11W | Register::R11B => Register::R11,

            r => *r,
        }
    }
}

pub fn get_register_name_for_bits(register: &Register, bits: u8) -> Register {
    use Register::*;

    return match register {
        Register::EAX | Register::AX | Register::AL | Register::RAX => match bits {
            8 => AL,
            16 => AX,
            32 => EAX,
            _ => RAX,
        },
        Register::EBX | Register::BX | Register::BL | Register::RBX => match bits {
            8 => BL,
            16 => BX,
            32 => EBX,
            _ => RBX,
        },
        Register::ECX | Register::CX | Register::CL | Register::RCX => match bits {
            8 => CL,
            16 => CX,
            32 => ECX,
            _ => RCX,
        },
        Register::EDX | Register::DX | Register::DL | Register::RDX => match bits {
            8 => DL,
            16 => DX,
            32 => EDX,
            _ => RDX,
        },
        Register::ESI | Register::SI | Register::SIL | Register::RSI => match bits {
            8 => SIL,
            16 => SI,
            32 => ESI,
            _ => RSI,
        },
        Register::EDI | Register::DI | Register::DIL | Register::RDI => match bits {
            8 => DIL,
            16 => DI,
            32 => EDI,
            _ => RDI,
        },
        Register::EBP | Register::BP | Register::BPL | Register::RBP => match bits {
            8 => BPL,
            16 => BP,
            32 => EBP,
            _ => RBP,
        },
        Register::R8D | Register::R8W | Register::R8B | Register::R8 => match bits {
            8 => R8B,
            16 => R8W,
            32 => R8D,
            _ => R8,
        },
        Register::R9D | Register::R9W | Register::R9B | Register::R9 => match bits {
            8 => R9B,
            16 => R9W,
            32 => R9D,
            _ => R9,
        },
        Register::R10D | Register::R10W | Register::R10B | Register::R10 => match bits {
            8 => R10B,
            16 => R10W,
            32 => R10D,
            _ => R10,
        },
        Register::R11D | Register::R11W | Register::R11B | Register::R11 => match bits {
            8 => R11B,
            16 => R11W,
            32 => R11D,
            _ => R11,
        },

        // We only support one size of floating point numbers for now
        Register::XMM0 => XMM0,
        Register::XMM1 => XMM1,
        Register::XMM2 => XMM2,
        Register::XMM3 => XMM3,
        Register::XMM4 => XMM4,
        Register::XMM5 => XMM5,
        Register::XMM6 => XMM6,
        Register::XMM7 => XMM7,
    };
}

impl VarType {
    pub fn get_register_name(&self, register: Register) -> Register {
        let thing = match self {
            Self::Int => get_register_name_for_bits(&register, 64),
            Self::Int32 => get_register_name_for_bits(&register, 32),
            Self::Int16 => get_register_name_for_bits(&register, 16),
            Self::Int8 | Self::Char => get_register_name_for_bits(&register, 8),
            Self::Float => get_register_name_for_bits(&register, 64),

            Self::Ptr(..) => get_register_name_for_bits(&register, 64),

            Self::Str => get_register_name_for_bits(&register, 64),

            // Function pointer so this does take 8 bytes
            Self::Function(..) => get_register_name_for_bits(&register, 64),

            v => panic!("get_register_name not implemented for '{}'", v),
        };

        return thing;
    }
}
