//! Properties about the architecture, such as how asm should be supported, what target features and what abi tags exist

use crate::{
    helpers::CowPtr,
    properties::{CowSlice, CowStr, ExtPropertyValue},
};

/// A Target Feature.
/// Includes information about implied/required features
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TargetFeature {
    /// The name of the target feature
    pub name: CowStr,
    /// The features implied by this
    pub implies: CowSlice<CowStr>,
}

/// Architecture properties
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Arch {
    /// The canonical name of the architecture
    pub name: CowStr,
    /// Alias names for the arch
    pub alias_names: CowSlice<CowStr>,
    /// The list of machines
    pub machines: CowSlice<Machine>,
    /// The native width of the architecture. Note that this doesn't do anything to control the layout of primitive types
    pub raw_width: usize,
    /// The list of target features
    pub features: CowSlice<TargetFeature>,
    /// The default target machine for the architecture
    pub default_machine: CowPtr<'static, Machine>,
    /// The list of call tags (ABI strings) supported on the target
    pub call_tags: CowSlice<CowStr>,
    /// The set of extended properties implied by using the architecture
    pub arch_extended_properties: CowSlice<(CowStr, ExtPropertyValue)>,
    /// If inline asm is supported, refers to the asm properties of the target
    pub asm_spec: Option<CowPtr<'static, Asm>>,
}

/// A [`Machine`] for an architecture.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Machine {
    /// The name of the machine. This is also used for lookup purposes
    pub name: CowStr,
    /// The set of features enabled by the machine
    pub features: CowSlice<CowStr>,
    /// The set of extended properties also enabled by the `-march` flag.
    pub mach_extended_properties: CowSlice<(CowStr, ExtPropertyValue)>,
}

/// Stub type for asm description properties (no properties yet)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Asm {}

/// Helper macro for defining the features of an architecture
#[macro_export]
macro_rules! arch_features {
    {
        $(#[$meta:meta])*
        $vis:vis static $features:ident = [
            $($feature:literal $(($($implies:literal),* $(,)?))?),*
            $(,)?
        ];
    } => {
        $(#[$meta])*
        $vis static $features: &[$crate::properties::arch::TargetFeature] = &[
            $(
                $crate::properties::arch::TargetFeature {
                    name: $crate::cowstr!($feature),
                    implies: $crate::slice![$($($crate::cowstr!($implies)),*)?]
                }
            ),*
        ];
    };
}
