//! Properties Related to ABI, such as primitive layout, and passing modes

use core::num::{NonZeroU8, NonZeroU16};

/// Specifies the layout of primitive types
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PrimitiveLayouts {
    /// Specifies the wdiths of integer types, and also byte order
    pub int_layout: IntLayouts,
    /// Specifies the maximum fundamental alignment for integer types and normal floating-point types
    pub max_int_align: u16,
    /// Specifies the maximum alignment for C `_BitInt`.
    pub max_bit_int_align: u16,
    /// Specifies the maximum alignment for vector types
    pub max_simd_align: u16,
    /// Specifies the alignment of C `long double`
    pub ldouble_align: u16,
    /// Specifies the format used by C `long double`. `double` and `float` are assumed to be ieee754-binary64 and ieee754-binary32 respectively
    pub ldouble_format: FloatFormat,
}

/// Describes a floating point format
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum FloatFormat {
    /// IBM's double-double extended precision format, used by legacy powerpc.
    /// Represented by a pair of binary64 values `(a, b)` (such that `lg|b| <= lg|a|`) that are summed to produce the complete value
    Ibm128,
    /// A format that is like IEEE754's binary format, with a specified number of exponential bits and mantissa bits
    Ieee754Like {
        /// The number of bits used for the exponent
        exp_bits: NonZeroU8,
        /// Whether or not an explicit integer bit is used. This is false for IEEE754 binary formats, and true for x87 double extended precision.
        /// Regardless of whether this is set, values produced by lxca will always have the integer bit set "correctly" (1 for normals and infinities, 0 for zeroes and subnormals).
        repr_int_bit: bool,
        /// The number of bits used for the mantissa, not including the integer bit.
        mant_bits: NonZeroU16,
    },
}

/// The default format that describes IEEE754's binary64 format
pub const IEEE754_DOUBLE: FloatFormat = FloatFormat::Ieee754Like {
    exp_bits: nzlit!(11),
    repr_int_bit: false,
    mant_bits: nzlit!(52),
};
/// The default format that describes the x87 double extended format
pub const X87_DOUBLE_EXTENDED: FloatFormat = FloatFormat::Ieee754Like {
    exp_bits: nzlit!(15),
    repr_int_bit: true,
    mant_bits: nzlit!(63),
};
/// The default format that describes IEEE754's binary128 format
pub const IEEE764_QUAD: FloatFormat = FloatFormat::Ieee754Like {
    exp_bits: nzlit!(15),
    repr_int_bit: false,
    mant_bits: nzlit!(112),
};

/// A type that stores the representations of various types
///
/// Note that certain types have assumed widths:
/// * `short` is assumed to always be 16 bit.
/// * `char` (and thus a byte) is assumed to always be 8 bit,
/// * `float` and `double` are assumed to be 32 and 64 bits respectively.`
///
/// All integer in this type must be powers of two that are at least 8.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct IntLayouts {
    /// The width of `int`, in bits. This must be at least 16
    pub int_width: u16,
    /// The width of `long`, in bits. THis must be at least 32 and at least [`IntLayouts::int_width`]
    pub long_width: u16,
    /// The width of `long long`, in bits. This must be at least 64 and at least [`IntLayouts::long_width`]. This is almost always `64`
    pub llong_width: u16,
    /// The width of `size_t` and `ptrdiff_t,` in bits. This also controls the maximum size of an object. This must be at least 16.
    /// This is almost always the same as one of [`IntLayouts::short_pointer_width`] and [`IntLayouts::long_pointer_width`].
    pub size_width: u16,
    /// The width of a short/near pointer, in bits. Must be at least 16.
    /// This is almost always the same as [`IntLayouts::long_pointer_width`]. Targets with different pointer widths are known as split-pointer targets
    pub short_pointer_width: u16,
    /// The width of a long/far pointer, in bits. Must be at least 16.
    /// This is almost always the same as [`IntLayouts::short_pointer_width`]. Targets with different pointer widths are known as split-pointer targets
    pub long_pointer_width: u16,
    /// The pointer kind of a data pointer. For most targets this is ignored.
    ///
    /// Picks whether or not pointers to objects and pointers to void default to using [`IntLayouts::short_pointer_width`] or [`IntLayouts::long_pointer_width`].
    /// This also controls the width of `intptr_t`.
    pub data_pointer_kind: PointerKind,
    /// The pointer kind of a function pointer. For most targets this is ignored.
    ///
    /// Picks whether or not pointers to functions default to using [`IntLayouts::short_pointer_width`] or [`IntLayouts::long_pointer_width`].
    /// This also controls the width of `intfnptr_t` where defined.
    pub fn_pointer_kind: PointerKind,
    /// The byte order of integers, pointers, and most primitives on the target.
    pub byte_order: ByteOrder,
    /// The width of `intmax_t`. For legacy reasons, this is usually 64. It must be at least [`IntLayouts::llong_width`]
    /// (as intmax_t is defined to be at least as wide as the widest standard integer type).
    ///
    /// This also controls whether or not `__int128` is considered an integer type for C18 and earlier, and for C++20 and earlier.
    pub intmax_width: u16,
}

/// The byte order for multi-byte primitive types stored in memory
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ByteOrder {
    /// Little-endian (LSB First) representation
    Little,
    /// Big-endian (MSB First) representation
    Big,
}

/// The kind of a pointer, determining the width.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PointerKind {
    /// A Near or Short pointer. Uses [`IntLayouts::short_pointer_width`]
    Near,
    /// A Far or Long pointer. Uses [`IntLayouts::long_pointer_width`]
    Far,
}

/// The IP16 (int/pointer 16-bit) default layout for a little-endian target.
pub const LE_IP16: IntLayouts = IntLayouts {
    int_width: 16,
    long_width: 32,
    llong_width: 64,
    size_width: 16,
    short_pointer_width: 16,
    long_pointer_width: 16,
    byte_order: ByteOrder::Little,
    intmax_width: 64,
    data_pointer_kind: PointerKind::Near,
    fn_pointer_kind: PointerKind::Near,
};

/// The LP32 (long/pointer 32-bit) default layout for a little-endian target.
/// Int and size_t are both still 16-bit on this target.
pub const LE_LP32: IntLayouts = IntLayouts {
    short_pointer_width: 32,
    long_pointer_width: 32,
    ..LE_IP16
};

/// The ILP32 (int/long/pointer 32-bit) default layout for a little-endian target.
/// This is the default for most 32-bit platforms.
pub const LE_ILP32: IntLayouts = IntLayouts {
    size_width: 32,
    int_width: 32,
    ..LE_LP32
};

/// The LLP64 (long long/pointer 64-bit) default layout for a little-endian target.
/// This is the layout for 64-bit windows
pub const LE_LLP64: IntLayouts = IntLayouts {
    size_width: 64,
    short_pointer_width: 64,
    long_pointer_width: 64,
    ..LE_ILP32
};

/// The LP64 (long/pointer 64-bit) default layout for a little-endian target.
/// This is the default for most 64-bit platforms
pub const LE_LP64: IntLayouts = IntLayouts {
    long_width: 64,
    ..LE_LLP64
};

/// The IP16 (int/pointer 16-bit) default layout for a big-endian target.
pub const BE_IP16: IntLayouts = IntLayouts {
    byte_order: ByteOrder::Big,
    ..LE_IP16
};

/// The LP32 (long/pointer 32-bit) default layout for a big-endian target.
pub const BE_LP32: IntLayouts = IntLayouts {
    byte_order: ByteOrder::Big,
    ..LE_LP32
};

/// The ILP32 (int/long/pointer 32-bit) default layout for a big-endian target.
pub const BE_ILP32: IntLayouts = IntLayouts {
    byte_order: ByteOrder::Big,
    ..LE_ILP32
};

/// The LLP64 (long long/pointer 64-bit) default layout for a big-endian target.
pub const BE_LLP64: IntLayouts = IntLayouts {
    byte_order: ByteOrder::Big,
    ..LE_LLP64
};

/// The LP64 (long/pointer 64-bit) default layout for a big-endian target.
pub const BE_LP64: IntLayouts = IntLayouts {
    byte_order: ByteOrder::Big,
    ..LE_LP64
};

/// THE [`LE_IP16`] layout with 32-bit far pointers.
/// This is suitable, e.g. for segmented 16-bit x86 with near pointers being the default
pub const LE_IP16_NEAR_FAR: IntLayouts = IntLayouts {
    long_pointer_width: 32,
    ..LE_IP16
};

/// THE [`LE_LP32`] layout with 16-bit near pointers (pointers default to far).
/// This is suitable, e.g. for segmented 16-bit x86 with far pointers being the default
pub const LE_LP32_NEAR_FAR: IntLayouts = IntLayouts {
    short_pointer_width: 16,
    data_pointer_kind: PointerKind::Far,
    fn_pointer_kind: PointerKind::Far,
    ..LE_LP32
};

/// THE [`BE_IP16`] layout with 32-bit far pointers.
pub const BE_IP16_NEAR_FAR: IntLayouts = IntLayouts {
    byte_order: ByteOrder::Big,
    ..LE_IP16_NEAR_FAR
};

/// THE [`BE_LP32`] layout with 16-bit near pointers (pointers default to far).
pub const BE_LP32_NEAR_FAR: IntLayouts = IntLayouts {
    byte_order: ByteOrder::Big,
    ..LE_LP32_NEAR_FAR
};

/// Override modes for ABI types
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Abi {
    /// Describes how to override the lowering of floating-point types to lxca at ABI boundaries
    pub float_pass_override: Option<PassModeOverride>,
    /// Describes how to override the lowering of vector types to lxca at ABI boundaries
    pub simd_pass_override: Option<PassModeOverride>,
}

/// Overrides the lowering of certain types to change the ABI the type exhibits
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PassModeOverride {
    /// Lowers the type to integers at the abi boundary
    Int,
    /// Lowers the type to a pointer at the abi boundary
    Memory,
}
