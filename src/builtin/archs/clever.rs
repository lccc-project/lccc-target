use crate::helpers::{CowSlice, CowStr};
use crate::properties::ExtPropertyValue;
use crate::properties::arch::{Arch, Asm, Machine};

/// The list of Clever-ISA features
pub static CLEVER_FEATURES: &[CowStr] = &[
    cowstr!("main"),
    cowstr!("float"),
    cowstr!("float-ext"),
    cowstr!("vector"),
    cowstr!("rand"),
    cowstr!("bitmanip"),
    cowstr!("atomic-xchg"),
    cowstr!("float128"),
    cowstr!("int128"),
    cowstr!("hash-accel"),
    cowstr!("crypto"),
];

/// This list of known Clever-ISA machines
pub static CLEVER_MACHINES: &[Machine] = &[
    Machine {
        name: cowstr!("clever"),
        features: slice![cowstr!("main")],
        mach_extended_properties: slice![],
    },
    Machine {
        name: cowstr!("clever1.0f"),
        features: slice!(
            cowstr!("main"),
            cowstr!("float"),
            cowstr!("vector"),
            cowstr!("rand")
        ),
        mach_extended_properties: slice![],
    },
];

/// The ASM Specification for Clever-ISA
pub static CLEVER_ASM: Asm = Asm {};

/// Clever-ISA
pub static CLEVER: Arch = Arch {
    name: cowstr!("clever"),
    alias_names: slice![],
    raw_width: 64,
    features: cow!(*CLEVER_FEATURES),
    default_machine: cow!(CLEVER_MACHINES[0]),
    call_tags: slice![cowstr!("C")],
    arch_extended_properties: slice![(
        cowstr!("rust.abi.rustcall.vector-pass-indirect"),
        ExtPropertyValue::Bool(false)
    )],
    asm_spec: Some(cow!(CLEVER_ASM)),
    machines: cow!(*CLEVER_MACHINES),
};
