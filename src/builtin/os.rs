use crate::properties::{os::Os, target};

use target_tuples::pieces;

/// Standalone/Freestanding target with no OS
pub static OS_STANDALONE: Os = Os {
    name: cowstr!("none"),
    family_names: slice![],
    is_unix_like: false,
    is_windows_like: false,
    os_extended_properties: slice![],
};

/// Linux
pub mod linux;

/// WIndows and related targets
pub mod windows;

/// Lilium-like targets
pub mod lilium;

/// Obtains the [`Os`] properties from the target name
pub const fn from_target(os: pieces::OS) -> Option<&'static Os> {
    match os {
        pieces::OS::Linux => Some(&linux::LINUX),
        pieces::OS::Win32 => Some(&windows::WINDOWS),
        pieces::OS::CleverOS => Some(&lilium::CLEVEROS),
        pieces::OS::Lilium => Some(&lilium::LILIUM),
        pieces::OS::SNES | pieces::OS::NES | pieces::OS::None => Some(&OS_STANDALONE),
        _ => todo!(),
    }
}
