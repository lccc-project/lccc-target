use crate::properties::{abi::{ByteOrder, FloatFormat, IEEE754_DOUBLE, IntLayouts, PointerKind, PrimitiveLayouts}, arch::{Arch, DEFAULT_ATOMICS_LOADSTORE_ONLY_WORD32, Machine, TargetFeature}};

/// Features for skyarch
/// Coprocessors use the format `cpi-<name>` (for official coprocessors)
pub const SKYARCH_FEATURES: &[TargetFeature] = &[];


/// Machines for skyarch. Currently only the default machine is defined.
pub static SKYARCH_MACHINES: &[Machine] = &[
    Machine {
        name: cowstr!("skyarch"),
        features: slice![],
        mach_extended_properties: slice![],
    }
];

/// the LC Skyarch Achitecture
pub static SKYARCH: Arch = Arch {
    name: cowstr!("skyarch"),
    alias_names: slice![],
    machines: cow!(*SKYARCH_MACHINES),
    raw_width: 32,
    features: cow!(*SKYARCH_FEATURES),
    default_machine: cow!(SKYARCH_MACHINES[0]),
    call_tags: slice![cowstr!("C")],
    arch_extended_properties: slice![],
    asm_spec: None, // todo: asm
    atomics: DEFAULT_ATOMICS_LOADSTORE_ONLY_WORD32,
};

/// Int layouts for the default psabi
pub const SKYARCH_INT_LAYOUT: IntLayouts = IntLayouts {
    int_width: 32,
    long_width: 32,
    llong_width: 64,
    size_width: 32,
    short_pointer_width: 32,
    long_pointer_width: 32,
    data_pointer_kind: PointerKind::Near,
    fn_pointer_kind: PointerKind::Near,
    byte_order: ByteOrder::Little,
    intmax_width: 64,
};

/// Primitive layouts for default psabi
pub static SKYARCH_PRIMITIVES: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: SKYARCH_INT_LAYOUT,
    max_int_align: 4,
    max_bit_int_align: 4,
    max_simd_align: 4,
    ldouble_align: 4,
    ldouble_format: IEEE754_DOUBLE,
};