//! Full target property database
use std::collections::{HashMap, HashSet};

use crate::{
    helpers::{CowPtr, CowSlice, CowStr},
    properties::{
        ExtPropertyValue,
        abi::{Abi, PrimitiveLayouts},
        arch::{Arch, Machine},
        link::Link,
        os::Os,
    },
};

/// The target properties
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Target {
    /// Canonical name of the environemtn, if any
    pub env_name: Option<CowStr>,
    /// The architecture properties
    pub arch: CowPtr<'static, Arch>,
    /// The OS properties
    pub os: CowPtr<'static, Os>,
    /// The default ("C") call tag
    pub default_tag: CowStr,
    /// The system ("system") call tag.
    pub system_tag: CowStr,
    /// The primitive layouts
    pub primitive_layout: CowPtr<'static, PrimitiveLayouts>,
    /// The abi overrides
    pub abi: CowPtr<'static, Abi>,
    /// The link settings
    pub link: CowPtr<'static, Link>,
    /// Sets or unsets features implied by the current machine
    pub override_features: CowSlice<(CowStr, bool)>,
    /// Extended Properties set by the target as a whole.
    /// These properties override all others (except those set by a CLI specified machine or by CLI flags), and additionally:
    /// * The OS properties override the architecture properties, unless they start with `arch.` or the architecture name
    /// * The Machine properties (including default machine) overrides the architecture properties.
    pub extended_properties: CowSlice<(CowStr, ExtPropertyValue)>,
}

impl Target {
    /// Determines the set of default property values for the target
    /// `mach` is set to the explicit machine passed by the `-march` flag
    pub fn compile_default_properties(
        &self,
        mach: Option<&Machine>,
    ) -> HashMap<CowStr, ExtPropertyValue> {
        let mut working = HashMap::new();

        for (name, val) in &self.arch.arch_extended_properties {
            working.insert(name.clone(), val.clone());
        }

        if mach.is_none() {
            for (name, val) in &self.arch.default_machine.mach_extended_properties {
                working.insert(name.clone(), val.clone());
            }
        }

        for (name, val) in &self.os.os_extended_properties {
            if (name.starts_with("arch.") || name.starts_with(&*self.arch.name))
                && working.contains_key(name)
            {
                continue;
            }
            working.insert(name.clone(), val.clone());
        }

        for (name, val) in &self.extended_properties {
            working.insert(name.clone(), val.clone());
        }

        if let Some(mach) = mach {
            for (name, val) in &mach.mach_extended_properties {
                working.insert(name.clone(), val.clone());
            }
        }

        working
    }

    /// Compiles the list of target features set by default on this target. `mach` is a machine passed in explicity by the `-march` flag
    pub fn compile_target_features(&self, mach: Option<&Machine>) -> HashSet<CowStr> {
        let mach = mach.unwrap_or(&self.arch.default_machine);

        let mut working = HashSet::new();

        for feature in &mach.features {
            working.insert(feature.clone());
        }

        for (feature, over) in &self.override_features {
            if *over {
                working.insert(feature.clone());
            } else {
                working.remove(feature);
            }
        }

        working
    }
}
