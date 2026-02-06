//! Linux

use crate::{
    builtin::link::x86::{
        ELF_X86_32, ELF_X86_32_MULTILIB, ELF_X86_64, ELF_X86_64_MULTILIB, ELF_X86_64_MULTILIBX32,
        ELF_X86_64_X32,
    },
    properties::link::{DefaultLinking, FILENAMES_ELF, Link},
};

/// Link filenames for linux
pub static LINUX_FILENAMES: DefaultLinking = DefaultLinking {
    start_files: slice![cowstr!("crt1.o"), cowstr!("crti.o")],
    end_files: slice![cowstr!("crto.o")],
    libraries: slice![cowstr!("c")],
};

/// Linking for x86_64-linux-gnu
pub static X86_64_LINUX_GNU_LINK: Link = Link {
    formats: cow!(ELF_X86_64),
    search: cow!(ELF_X86_64_MULTILIB),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: crate::properties::link::NxStackMode::GnuStack,
    dynlinker_name: Some(cowstr!("/lib/ld-linux-x86_64.so.2")),
    default_libraries: cow!(LINUX_FILENAMES),
};

/// Linking for x86_64-linux-gnux32
pub static X86_64_LINUX_GNUX32_LINK: Link = Link {
    formats: cow!(ELF_X86_64_X32),
    search: cow!(ELF_X86_64_MULTILIBX32),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: crate::properties::link::NxStackMode::GnuStack,
    dynlinker_name: Some(cowstr!("/lib/ld-linux-x32.so.2")),
    default_libraries: cow!(LINUX_FILENAMES),
};

/// Linking for i*86-linux-gnu
pub static X86_32LINUX_GNU_LINK: Link = Link {
    formats: cow!(ELF_X86_32),
    search: cow!(ELF_X86_32_MULTILIB),
    output_filename: cow!(FILENAMES_ELF),
    nx_stack: crate::properties::link::NxStackMode::GnuStack,
    dynlinker_name: Some(cowstr!("/lib/ld-linux.so.2")),
    default_libraries: cow!(LINUX_FILENAMES),
};
