use crate::properties::os::Os;

/// Windows
pub static WINDOWS: Os = Os {
    name: cowstr!("windows"),
    family_names: slice![cowstr!("windows")],
    is_windows_like: true,
    is_unix_like: false,
    os_extended_properties: slice![],
};
