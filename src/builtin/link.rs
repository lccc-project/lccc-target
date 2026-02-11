use target_tuples::pieces::{Architecture, Environment, OS, ObjectFormat, System};

use crate::properties::link::Link;

pub mod clever;
pub mod x86;

pub mod lilium;

pub mod linux;

/// Obtains the [`Link`] properties for a given arch, os, env triple
pub const fn from_target(arch: Architecture, sys: System) -> Option<&'static Link> {
    let os = match sys.os() {
        Some(os) => os,
        None => OS::None,
    };
    match (arch, os, sys.env(), sys.object_format()) {
        (Architecture::X86_64 { .. }, OS::Lilium, Some(Environment::Kernel), _) => {
            Some(&lilium::X86_64_LILIUM_KERNEL_LINK)
        }
        (Architecture::X86_64 { .. }, OS::Lilium, _, _) => Some(&lilium::X86_64_LILIUM_LINK),
        (Architecture::X86_32(_), OS::Lilium, Some(Environment::Kernel), _) => {
            Some(&lilium::X86_32_LILIUM_KERNEL_LINK)
        }
        (Architecture::X86_32(_), OS::Lilium, _, _) => Some(&lilium::X86_32_LILIUM_LINK),
        (Architecture::X86_64 { .. }, OS::Linux, Some(Environment::GNU), _) => {
            Some(&linux::X86_64_LINUX_GNU_LINK)
        }
        (Architecture::X86_64 { .. }, OS::Linux, Some(Environment::GNUX32), _) => {
            Some(&linux::X86_64_LINUX_GNUX32_LINK)
        }
        (Architecture::X86_64 { .. }, _, _, Some(ObjectFormat::Elf)) => {
            Some(&x86::ELF_X86_64_FREESTANDING_LINK)
        }

        (Architecture::X86_32(_), OS::Linux, Some(Environment::GNU), _) => {
            Some(&linux::X86_32LINUX_GNU_LINK)
        }
        (Architecture::X86_32(_), _, _, Some(ObjectFormat::Elf)) => {
            Some(&x86::ELF_X86_32_FREESTANDING_LINK)
        }
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
