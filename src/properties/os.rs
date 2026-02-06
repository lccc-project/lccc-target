//! Os properties

use crate::properties::{CowSlice, CowStr, ExtPropertyValue};

/// Properties about the OS
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Os {
    /// The canonical name of the OS
    pub name: CowStr,

    /// The list of os family names
    pub family_names: CowSlice<CowStr>,
    /// Whether or not the OS is unix-like (e.g. Linux)
    pub is_unix_like: bool,
    /// Whether or not the OS is windows-like (e.g. Windows)
    pub is_windows_like: bool,

    /// Extended properties set by the OS
    pub os_extended_properties: CowSlice<(CowStr, ExtPropertyValue)>,
}
