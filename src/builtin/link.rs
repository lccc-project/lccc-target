use target_tuples::{Architecture, Environment, OS};

use crate::properties::link::Link;

pub mod clever;
pub mod x86;

pub mod lilium;

pub mod linux;

/// Obtains the [`Link`] properties for a given arch, os, env triple
pub const fn from_target(arch: Architecture, os: OS, env: Environment) -> Option<&'static Link> {
    match (arch, os, env) {
        (Architecture::X86_64, OS::Lilium, _) => Some(&lilium::X86_64_LILIUM_LINK),
        (Architecture::I686 | Architecture::I786, OS::Lilium, _) => {
            Some(&lilium::X86_32_LILIUM_LINK)
        }
        (Architecture::X86_64, OS::Linux, Environment::GNU) => Some(&linux::X86_64_LINUX_GNU_LINK),
        (Architecture::X86_64, OS::Linux, Environment::GNUX32) => {
            Some(&linux::X86_64_LINUX_GNUX32_LINK)
        }
        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            OS::Linux,
            Environment::GNU,
        ) => Some(&linux::X86_32LINUX_GNU_LINK),
        (Architecture::Clever, OS::Lilium | OS::CleverOS, _) => Some(&lilium::CLEVER_LILIUM_LINK),
        _ => None,
    }
}
