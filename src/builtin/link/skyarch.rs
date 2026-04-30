//! Linking for skyarch
use crate::properties::link::{FILENAMES_ELF, Link, LinkFormat, SEARCH_UNIX_DEFAULT, SupportedArtifacts};

/// Elf format for Freestanding Clever-ISA
pub static ELF_CLEVER_FREESTANDING: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf64-skyarch"),
    exec_binfmt: cowstr!("elf64-skyarch"),
    staticlib_format: crate::properties::link::StaticLibraryFormat::Object,
    supported_artifacts: SupportedArtifacts::NO_DYNLINKER,
    default_linker_format: crate::properties::link::LinkerFlavour::Ld,
};

/// Linking for Freestanding Clever-ISA
pub static ELF_SKYARCH_FREESTANDING_LINK: Link = Link {
    formats: cow!(ELF_CLEVER_FREESTANDING),
    search: cow!(SEARCH_UNIX_DEFAULT),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: crate::properties::link::NxStackMode::Default,
    dynlinker_name: None,
    default_libraries: None,
};
