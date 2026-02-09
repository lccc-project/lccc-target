use target_tuples::{Architecture, Environment, OS, ObjectFormat};

use crate::properties::link::Link;

pub mod clever;
pub mod x86;

pub mod lilium;

pub mod linux;

/// Obtains the [`Link`] properties for a given arch, os, env triple
pub const fn from_target(
    arch: Architecture,
    os: OS,
    env: Option<Environment>,
    objfmt: Option<ObjectFormat>,
) -> Option<&'static Link> {
    match (arch, os, env, objfmt) {
        (Architecture::X86_64, OS::Lilium, Some(Environment::Kernel), _) => {
            Some(&lilium::X86_64_LILIUM_KERNEL_LINK)
        }
        (Architecture::X86_64, OS::Lilium, _, _) => Some(&lilium::X86_64_LILIUM_LINK),
        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            OS::Lilium,
            Some(Environment::Kernel),
            _,
        ) => Some(&lilium::X86_32_LILIUM_KERNEL_LINK),
        (Architecture::I686 | Architecture::I786, OS::Lilium, _, _) => {
            Some(&lilium::X86_32_LILIUM_LINK)
        }
        (Architecture::X86_64, OS::Linux, Some(Environment::GNU), _) => {
            Some(&linux::X86_64_LINUX_GNU_LINK)
        }
        (Architecture::X86_64, OS::Linux, Some(Environment::GNUX32), _) => {
            Some(&linux::X86_64_LINUX_GNUX32_LINK)
        }
        (Architecture::X86_64, _, _, Some(ObjectFormat::Elf)) => {
            Some(&x86::ELF_X86_64_FREESTANDING_LINK)
        }

        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            OS::Linux,
            Some(Environment::GNU),
            _,
        ) => Some(&linux::X86_32LINUX_GNU_LINK),
        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            _,
            _,
            Some(ObjectFormat::Elf),
        ) => Some(&x86::ELF_X86_32_FREESTANDING_LINK),
        (Architecture::Clever, OS::Lilium | OS::CleverOS, Some(Environment::Kernel), _) => {
            Some(&lilium::CLEVER_LILIUM_KERNEL_LINK)
        }
        (Architecture::Clever, OS::Lilium | OS::CleverOS, _, _) => {
            Some(&lilium::CLEVER_LILIUM_LINK)
        }
        (Architecture::Clever, _, _, Some(ObjectFormat::Elf)) => {
            Some(&clever::ELF_CLEVER_FREESTANDING_LINK)
        }
        _ => None,
    }
}
