//! linking properties common to x86 targets

use crate::properties::link::{
    FILENAMES_ELF, Link, LinkFormat, SEARCH_UNIX_DEFAULT, SupportedArtifacts,
};

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

/// Elf format for Freestanding Clever-ISA
pub static ELF_CLEVER_FREESTANDING: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf64-clever"),
    exec_binfmt: cowstr!("elf64-clever"),
    staticlib_format: crate::properties::link::StaticLibraryFormat::Archive(
        crate::properties::link::ArchiveFormat::SysV,
    ),
    supported_artifacts: SupportedArtifacts::NO_DEFAULT_PIE,
    default_linker_format: crate::properties::link::LinkerFlavour::Ld,
};

/// Linking for Freestanding Clever-ISA
pub static ELF_CLEVER_FREESTANDING_LINK: Link = Link {
    formats: cow!(ELF_CLEVER_FREESTANDING),
    search: cow!(SEARCH_UNIX_DEFAULT),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: crate::properties::link::NxStackMode::Default,
    dynlinker_name: None,
    default_libraries: None,
};
