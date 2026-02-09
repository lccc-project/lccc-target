//! Properties for the link step, such as how to produce object files, and how to invoke the linker

use crate::helpers::{CowPtr, CowSlice, CowStr};

bitflags::bitflags! {
    /// Supported types for artifact files
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct SupportedArtifacts : u8 {
        /// Executable files supported.
        /// This should be default in all but the most extreme environments
        const EXE = 0x01;
        /// Dynamically linked libraries are supported
        const DYLIB = 0x02;
        /// Position independant code is supported.
        /// This should be true wherever dylibs are supported
        const PIC = 0x04;
        /// Position Independant Executables are supported.
        const PIE = 0x08;
        /// The default executable type is Position Independant.
        const DEFAULT_PIE = 0x10;
        /// Position Independant Executables are supported without a dynamic linker
        const STATIC_PIE = 0x20;
    }
}

impl SupportedArtifacts {
    /// Default set of [`SupportedArtifacts`] where PIE is not treated as default
    pub const NO_DEFAULT_PIE: SupportedArtifacts = SupportedArtifacts::EXE
        .union(SupportedArtifacts::DYLIB)
        .union(SupportedArtifacts::PIC)
        .union(SupportedArtifacts::PIE)
        .union(SupportedArtifacts::STATIC_PIE);
    /// Default set of [`SupportedArtifacts`] where a dynamic linker is not available
    pub const NO_DYNLINKER: SupportedArtifacts =
        SupportedArtifacts::EXE.union(SupportedArtifacts::STATIC_PIE);
}

/// Linker properties
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Link {
    /// Formats for linking
    pub formats: CowPtr<'static, LinkFormat>,
    /// Information about how to search for libraries (`-l` and the stdlib)
    pub search: CowPtr<'static, LibrarySearch>,
    /// The formatting of output file names
    pub output_filename: CowPtr<'static, FileNames>,

    /// Controls what is needed to make the stack non-executable
    pub nx_stack: NxStackMode,

    /// The name of the dynamic linker/program interpreter for elf platforms
    pub dynlinker_name: Option<CowStr>,

    /// The default libraries on the platform
    pub default_libraries: Option<CowPtr<'static, DefaultLinking>>,
}

/// Default libraries/startfiles behaviour
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DefaultLinking {
    /// The list of startfile objects that appear on the link line first if `-nostartfiles` is not provided
    pub start_files: CowSlice<CowStr>,
    /// The list of endfile objects that appear on the link line last if `-nostartfiles` is not provided
    pub end_files: CowSlice<CowStr>,
    /// The list of libraries that are added to the link line after all inputs if `-nostdlib` is not provided
    pub libraries: CowSlice<CowStr>,
}

/// The format of link outputs
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LinkFormat {
    /// The binary format (format-arch) for object files
    pub object_binfmt: CowStr,
    /// The binary format (format-arch) for executables, dylibs, and [`StaticLibraryFormat::Object`] static libs
    pub exec_binfmt: CowStr,
    /// The format of static libraries. Most static libraries use the ar archive format of various flavours.
    pub staticlib_format: StaticLibraryFormat,
    /// Supported Artifact kinds
    pub supported_artifacts: SupportedArtifacts,
    /// The default flavour for the linker executable (ld, link, etc.)
    pub default_linker_format: LinkerFlavour,
}

/// Output Filenames
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FileNames {
    /// The prefix to prepend to the default filename for object files
    pub obj_prefix: CowStr,
    /// The suffix to append to the default filename for object files
    pub obj_suffix: CowStr,
    /// The prefix to prepend to the default filename for executable files
    pub exe_prefix: CowStr,
    /// The suffix to append to the default filename for executable files
    pub exe_suffix: CowStr,
    /// The prefix to prepend to the default filename for dynamic libraries
    pub dylib_prefix: CowStr,
    /// The suffix to append to the default filename for dynamic libraries
    pub dylib_suffix: CowStr,
    /// The prefix to prepend to the default filename for static libraries
    pub staticlib_prefix: CowStr,
    /// The suffix to append to the default filename for static libraries
    pub staticlib_suffix: CowStr,
}

/// The format of static libraries
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum StaticLibraryFormat {
    /// An archive, using the specified format
    Archive(ArchiveFormat),
    /// An Object, using the same format as [`Link::exec_binfmt`]. Usually requires special linker support
    Object,
}

/// The format of archives for static libraries
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArchiveFormat {
    /// System V archive format, using SysV long name and symbol files
    SysV,
    /// System V archive format with the windows secondary symbol file
    SysVWin,
    /// BSD Format
    Bsd,
}

/// The flavour of a linker command line
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum LinkerFlavour {
    /// Like unix ld
    Ld,
    /// MacOS ld
    MachLd,
    /// Like microsoft `link.exe`
    Link,
    /// Wasm linker
    WasmLd,
}

/// The support for non-executable stack
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum NxStackMode {
    /// Indicates that non-executable stack is the default (or only) behaviour
    Default,
    /// Requires emitting `.note.GNU-stack` for non-executable stack
    GnuStack,
    /// Indicates that non-executable stack is not supported
    Unsupported,
}

/// Library search properties
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LibrarySearch {
    /// Base directories.
    /// These start with a `/` and are used to resolve the `search_dirs`.
    /// They are resolved relative to the sysroot.
    pub base_dirs: CowSlice<CowStr>,
    /// The search directories.
    /// These do not start with a slash, and are usually one component.
    /// They are resolved relative to the base_dirs (with target stems if [`LibrarySearch::use_target_stem_dirs`] is also set)
    pub search_dirs: CowSlice<CowStr>,
    /// The prefixes to prepend when resolving `-l<name>`, when static libraries are available for linking
    pub staticlib_prefixes: CowSlice<CowStr>,
    /// The suffixes to append when resolving `-l<name>`, when static libraries are available for linking
    pub staticlib_suffixes: CowSlice<CowStr>,
    /// The prefixes to prepend when resolving `-l<name>`, when dynamic libraries are available for linking
    pub dylib_prefixes: CowSlice<CowStr>,
    /// The suffixes to append when resolving `-l<name>`, when dynamic libraries are available for linking
    pub dylib_suffixes: CowSlice<CowStr>,
    /// Whether or not [`LibrarySearch::base_dirs`] are searched using the target stem (usually <arch>-<sys>) when linking
    pub use_target_stem_dirs: bool,
}

/// The Default search for Unix-like platforms (and platforms with similar filesystem layouts)
pub const SEARCH_UNIX_DEFAULT: LibrarySearch = LibrarySearch {
    base_dirs: slice![cowstr!("/"), cowstr!("/usr"), cowstr!("/usr/local")],
    search_dirs: slice![cowstr!("lib")],
    staticlib_prefixes: slice![cowstr!("lib")],
    staticlib_suffixes: slice![cowstr!(".a")],
    dylib_prefixes: slice![cowstr!("lib")],
    dylib_suffixes: slice![cowstr!(".so")],
    use_target_stem_dirs: true,
};

/// The Default search for Unix-like platforms (and platforms with similar filesystem layouts)
pub static FILENAMES_ELF: FileNames = FileNames {
    obj_prefix: cowstr!(""),
    obj_suffix: cowstr!(".o"),
    exe_prefix: cowstr!(""),
    exe_suffix: cowstr!(""),
    dylib_prefix: cowstr!("lib"),
    dylib_suffix: cowstr!(".so"),
    staticlib_prefix: cowstr!("lib"),
    staticlib_suffix: cowstr!(".a"),
};

/// The Default search for Unix-like platforms (and platforms with similar filesystem layouts)
pub static FILENAMES_PE: FileNames = FileNames {
    obj_prefix: cowstr!(""),
    obj_suffix: cowstr!(".obj"),
    exe_prefix: cowstr!(""),
    exe_suffix: cowstr!(""),
    dylib_prefix: cowstr!(""),
    dylib_suffix: cowstr!(".dll"),
    staticlib_prefix: cowstr!("lib"),
    staticlib_suffix: cowstr!(".lib"),
};
