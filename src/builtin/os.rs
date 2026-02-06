use crate::properties::{os::Os, target};

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
pub const fn from_target(os: target_tuples::OS) -> Option<&'static Os> {
    match os {
        target_tuples::OS::Linux => Some(&linux::LINUX),
        target_tuples::OS::Win32 => Some(&windows::WINDOWS),
        target_tuples::OS::CleverOS => Some(&lilium::CLEVEROS),
        target_tuples::OS::Lilium => Some(&lilium::LILIUM),
        target_tuples::OS::SNES
        | target_tuples::OS::NES
        | target_tuples::OS::None
        | target_tuples::OS::Null => Some(&OS_STANDALONE),
        _ => todo!(),
    }
}
