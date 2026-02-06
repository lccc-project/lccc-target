use target_tuples::Architecture;

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
        Architecture::I86 | Architecture::I8086 | Architecture::I086 | Architecture::I186 => {
            Some(&x86::A8086)
        }
        Architecture::I286 => Some(&x86::I286),
        Architecture::I386 => Some(&x86::I386),
        Architecture::I486 => Some(&x86::I486),
        Architecture::I586 => Some(&x86::I586),
        Architecture::I686 => Some(&x86::I686),
        Architecture::I786 => Some(&x86::I786),
        Architecture::X86_64 => Some(&x86::X86_64),
        Architecture::Wc65c816 => Some(&m65::W65),
        Architecture::M6502 => Some(&m65::M6502),
        Architecture::M65C02 => Some(&m65::M65C02),
        Architecture::Clever => Some(&clever::CLEVER),
        _ => None,
    }
}
