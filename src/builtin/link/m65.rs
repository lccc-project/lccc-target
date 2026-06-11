//! Linking for MOS 6502 and derivatives 

use crate::{helpers::CowSlice, properties::link::{ArchiveFormat, FILENAMES_ELF, LibrarySearch, Link, LinkFormat, LinkerFlavour, NxStackMode, StaticLibraryFormat, SupportedArtifacts}};

/// ELF Format for w65
pub static W65_ELF: LinkFormat = LinkFormat {
    object_binfmt: cowstr!("elf32-w65"),
    exec_binfmt: cowstr!("elf32-w65"),
    staticlib_format: StaticLibraryFormat::Archive(ArchiveFormat::SysV),
    supported_artifacts: SupportedArtifacts::NO_DYNLINKER,
    default_linker_format: LinkerFlavour::Ld,
};

/// Default Search for w65
pub static W65_SEARCH: LibrarySearch = LibrarySearch {
    base_dirs: slice![cowstr!("/")],
    search_dirs: slice![cowstr!("lib")],
    staticlib_prefixes: slice![cowstr!("lib")],
    staticlib_suffixes: slice![cowstr!(".a"), cowstr!(".lib")],
    dylib_prefixes: slice![],
    dylib_suffixes: slice![],
    use_target_stem_dirs: true,
};

/// Freestanding Linking for w65-elf
pub static W65_ELF_FREESTANDING_LINK: Link = Link {
    formats: cow!(W65_ELF),
    search: cow!(W65_SEARCH),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: NxStackMode::Unsupported,
    dynlinker_name: None,
    default_libraries: None,
};

