//! Lilium OS and Lilium-like

use crate::{
    builtin::link::{
        clever::ELF_CLEVER,
        x86::{ELF_X86_32, ELF_X86_64},
    },
    properties::link::{DefaultLinking, FILENAMES_ELF, Link, NxStackMode, SEARCH_UNIX_DEFAULT},
};

/// Library set for Lilium
pub static LILIUM_LIBRARIES: DefaultLinking = DefaultLinking {
    start_files: slice![cowstr!("liblilium-init.o")],
    end_files: slice![],
    libraries: slice![cowstr!("c"), cowstr!("usi"), cowstr!("usi-support")],
};

/// Linking for x86_64-lilium
pub static X86_64_LILIUM_LINK: Link = Link {
    formats: cow!(ELF_X86_64),
    search: cow!(SEARCH_UNIX_DEFAULT),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: NxStackMode::Default,
    dynlinker_name: Some(cowstr!("/lib/ld-lilium-x86_64.so.0")),
    default_libraries: cow!(LILIUM_LIBRARIES),
};

/// Linking for i686-lilium and i786-lilium
pub static X86_32_LILIUM_LINK: Link = Link {
    formats: cow!(ELF_X86_32),
    search: cow!(SEARCH_UNIX_DEFAULT),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: NxStackMode::Default,
    dynlinker_name: Some(cowstr!("/lib/ld-lilium-i686.so.0")),
    default_libraries: cow!(LILIUM_LIBRARIES),
};

/// Linking for clever-lilium and clever-cleveros
pub static CLEVER_LILIUM_LINK: Link = Link {
    formats: cow!(ELF_CLEVER),
    search: cow!(SEARCH_UNIX_DEFAULT),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: NxStackMode::Default,
    dynlinker_name: Some(cowstr!("/lib/ld-lilium-clever.so.0")),
    default_libraries: cow!(LILIUM_LIBRARIES),
};
