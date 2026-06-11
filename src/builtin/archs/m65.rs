
use crate::{
    arch_features,
    properties::{abi::{Abi, IEEE754_DOUBLE, LE_IP16, LE_IP16_NEAR_FAR, LE_LP32_NEAR_FAR, PrimitiveLayouts}, arch::{Arch, Asm, Machine}},
};

arch_features! {
    /// Arch features for m6502
    pub static M6502_FEATURES = [
        "ext-instr",
        "cmos",
    ];
}

/// Machines for the m6502
pub static M6502_MACHINES: &[Machine] = &[
    Machine {
        name: cowstr!("m6502"),
        features: slice![],
        mach_extended_properties: slice![],
    },
    Machine {
        name: cowstr!("m6502x"),
        features: slice![cowstr!("instr-ext")],
        mach_extended_properties: slice![],
    },
    Machine {
        name: cowstr!("m65c02"),
        features: slice![cowstr!("cmos")],
        mach_extended_properties: slice![],
    },
];

/// w65 machines. Currently there's only the one
pub static W65_MACHINES: &[Machine] = &[Machine {
    name: cowstr!("wdc65c16"),
    features: slice![],
    mach_extended_properties: slice![],
}];

/// Asm spec for m6502
pub static M6502_ASM: Asm = Asm {};
/// Asm spec for w65
pub static W65_ASM: Asm = Asm {};

/// m6502
pub static M6502: Arch = Arch {
    name: cowstr!("m6502"),
    alias_names: slice![],
    machines: cow!(*M6502_MACHINES),
    raw_width: 8,
    features: cow!(*M6502_FEATURES),
    default_machine: cow!(M6502_MACHINES[0]),
    call_tags: slice![cowstr!("C"), cowstr!("interrupt")],
    arch_extended_properties: slice![],
    asm_spec: Some(cow!(M6502_ASM)),
};

/// m65c02 - CMOS 6502
pub static M65C02: Arch = Arch {
    name: cowstr!("m65c02"),
    alias_names: slice![cowstr!("m6502")],
    machines: cow!(*M6502_MACHINES),
    raw_width: 8,
    features: cow!(*M6502_FEATURES),
    default_machine: cow!(M6502_MACHINES[2]),
    call_tags: slice![cowstr!("C"), cowstr!("interrupt")],
    arch_extended_properties: slice![],
    asm_spec: Some(cow!(M6502_ASM)),
};

/// w65 - CMOS Western Design Center 65816
pub static W65: Arch = Arch {
    name: cowstr!("w65"),
    alias_names: slice![],
    machines: cow!(*W65_MACHINES),
    raw_width: 16,
    features: cow!([]),
    default_machine: cow!(W65_MACHINES[0]),
    call_tags: slice![cowstr!("C"), cowstr!("interrupt")],
    arch_extended_properties: slice![],
    asm_spec: Some(cow!(W65_ASM)),
};

/// Primitives for CMOS Western Design Center 65816
pub static W65_PRIMITIVES: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_LP32_NEAR_FAR,
    max_int_align: 4,
    max_bit_int_align: 4,
    max_simd_align: 4,
    ldouble_align: 4,
    ldouble_format: IEEE754_DOUBLE,
};

/// Primitives for MOS 6502 and derivatives
pub static M6502_PRIMITIVES: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_IP16,
    max_int_align: 2,
    max_bit_int_align: 2,
    max_simd_align: 2,
    ldouble_align: 2,
    ldouble_format: IEEE754_DOUBLE,
};
