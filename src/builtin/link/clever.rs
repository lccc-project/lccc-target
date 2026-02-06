//! linking properties common to x86 targets

use crate::properties::link::{LinkFormat, SupportedArtifacts};

/// Elf format for Clever-ISA
pub static ELF_CLEVER: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf64-clever"),
    exec_binfmt: cowstr!("elf64-clever"),
    staticlib_format: crate::properties::link::StaticLibraryFormat::Archive(
        crate::properties::link::ArchiveFormat::SysV,
    ),
    supported_artifacts: SupportedArtifacts::all(),
    default_linker_format: crate::properties::link::LinkerFlavour::Ld,
};
