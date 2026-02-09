//! linking properties common to x86 targets

use crate::properties::link::{
    ArchiveFormat, FILENAMES_ELF, LibrarySearch, Link, LinkFormat, LinkerFlavour,
    SEARCH_UNIX_DEFAULT, StaticLibraryFormat, SupportedArtifacts,
};

/// Linking for Elf x86-64
pub static ELF_X86_64: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf64-x86_64"),
    exec_binfmt: cowstr!("elf64-x86_64"),
    staticlib_format: StaticLibraryFormat::Archive(ArchiveFormat::SysV),
    supported_artifacts: SupportedArtifacts::all(),
    default_linker_format: crate::properties::link::LinkerFlavour::Ld,
};

/// Linking for Elf x86-64 x32
pub static ELF_X86_64_X32: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf32-x86_64"),
    exec_binfmt: cowstr!("elf32-x86_64"),
    staticlib_format: StaticLibraryFormat::Archive(ArchiveFormat::SysV),
    supported_artifacts: SupportedArtifacts::all(),
    default_linker_format: crate::properties::link::LinkerFlavour::Ld,
};

/// Linking for Elf x86-32
pub static ELF_X86_32: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf32-x86"),
    exec_binfmt: cowstr!("elf32-x86"),
    staticlib_format: StaticLibraryFormat::Archive(ArchiveFormat::SysV),
    supported_artifacts: SupportedArtifacts::all(),
    default_linker_format: LinkerFlavour::Ld,
};

/// Library Search for x86_64 with gnu multilib
pub static ELF_X86_64_MULTILIB: LibrarySearch = LibrarySearch {
    search_dirs: slice![cowstr!("lib"), cowstr!("lib64")],
    ..SEARCH_UNIX_DEFAULT
};

/// Library Search for x86_64 x32 with gnu multilib
pub static ELF_X86_64_MULTILIBX32: LibrarySearch = LibrarySearch {
    search_dirs: slice![cowstr!("lib"), cowstr!("libx32")],
    ..SEARCH_UNIX_DEFAULT
};

/// Library Search for x86_32 with GNU Multilib
pub static ELF_X86_32_MULTILIB: LibrarySearch = LibrarySearch {
    search_dirs: slice![cowstr!("lib"), cowstr!("lib32")],
    ..SEARCH_UNIX_DEFAULT
};

/// Elf Format for Freestanding x86-64
pub static ELF_X86_64_FREESTANDING: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf64-x86_64"),
    exec_binfmt: cowstr!("elf64-x86_64"),
    staticlib_format: StaticLibraryFormat::Archive(ArchiveFormat::SysV),
    supported_artifacts: SupportedArtifacts::NO_DEFAULT_PIE,
    default_linker_format: LinkerFlavour::Ld,
};

/// Elf Format for Freestanding x86-32
pub static ELF_X86_32_FREESTANDING: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf32-x86"),
    exec_binfmt: cowstr!("elf32-x86"),
    staticlib_format: StaticLibraryFormat::Archive(ArchiveFormat::SysV),
    supported_artifacts: SupportedArtifacts::NO_DEFAULT_PIE,
    default_linker_format: LinkerFlavour::Ld,
};

/// Linking behaviour for Freestanding x86-64
pub static ELF_X86_64_FREESTANDING_LINK: Link = Link {
    formats: cow!(ELF_X86_64_FREESTANDING),
    search: cow!(SEARCH_UNIX_DEFAULT),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: crate::properties::link::NxStackMode::Unsupported,
    dynlinker_name: None,
    default_libraries: None,
};

/// Linking behaviour for Freestanding x86-32
pub static ELF_X86_32_FREESTANDING_LINK: Link = Link {
    formats: cow!(ELF_X86_32_FREESTANDING),
    search: cow!(SEARCH_UNIX_DEFAULT),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: crate::properties::link::NxStackMode::Unsupported,
    dynlinker_name: None,
    default_libraries: None,
};
