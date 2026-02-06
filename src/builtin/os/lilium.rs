use crate::properties::os::Os;

/// LiliumOS: https://github.com/LiliumOS
pub static LILIUM: Os = Os {
    name: cowstr!("lilium"),
    family_names: slice![cowstr!("lilium")],
    is_unix_like: false,
    is_windows_like: false,
    os_extended_properties: slice![],
};

/// Clever-ISA 1.0 Test OS - Lilium-like
pub static CLEVEROS: Os = Os {
    name: cowstr!("cleveros"),
    family_names: slice![cowstr!("lilium")],
    is_unix_like: false,
    is_windows_like: false,
    os_extended_properties: slice![],
};
