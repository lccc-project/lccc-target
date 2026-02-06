use crate::properties::os::Os;

/// Linux
pub static LINUX: Os = Os {
    name: cowstr!("linux"),
    family_names: slice![cowstr!("linux")],
    is_windows_like: false,
    is_unix_like: true,
    os_extended_properties: slice![],
};
