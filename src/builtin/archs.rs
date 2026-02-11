use target_tuples::pieces::Architecture;

use crate::{
    helpers::CowPtr,
    properties::arch::{Arch, Asm, Machine},
};

/// x86/x86-64
pub mod x86;

/// Clever-ISA
pub mod clever;

/// 6502 and derivatives
pub mod m65;

/// Determines the architecture info from the architecture target name
pub const fn from_target(arch: Architecture) -> Option<&'static Arch> {
    match arch {
        Architecture::X86_16(..2) => Some(&x86::A8086),
        Architecture::X86_16(2) => Some(&x86::I286),
        Architecture::X86_32(3) => Some(&x86::I386),
        Architecture::X86_32(4) => Some(&x86::I486),
        Architecture::X86_32(5) => Some(&x86::I586),
        Architecture::X86_32(6) => Some(&x86::I686),
        Architecture::X86_32(7..) => Some(&x86::I786),
        Architecture::X86_64 { microarch: 0 | 1 } => Some(&x86::X86_64),
        Architecture::X86_64 { microarch: 2 } => Some(&x86::X86_64_V2),
        Architecture::X86_64 { microarch: 3 } => Some(&x86::X86_64V3),
        Architecture::X86_64 { microarch: 4.. } => Some(&x86::X86_64V4),
        Architecture::Wc65c816 => Some(&m65::W65),
        Architecture::M6502 => Some(&m65::M6502),
        Architecture::M65C02 => Some(&m65::M65C02),
        Architecture::Clever => Some(&clever::CLEVER),
        _ => None,
    }
}
