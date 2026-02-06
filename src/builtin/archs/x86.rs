use crate::{
    helpers::{CowPtr, CowSlice, CowStr},
    properties::{
        abi::{
            IEEE754_DOUBLE, IntLayouts, LE_ILP32, LE_IP16, LE_IP16_NEAR_FAR, LE_LP32_NEAR_FAR,
            LE_LP64, PrimitiveLayouts, X87_DOUBLE_EXTENDED,
        },
        arch::{Arch, Asm, Machine},
    },
};

/// All x86 feature sets
pub static X86_FEATURES: &[CowStr] = &[
    cowstr!("x87"),
    cowstr!("mmx"),
    cowstr!("sse"),
    cowstr!("sse2"),
    cowstr!("sse3"),
    cowstr!("ssse3"),
    cowstr!("sse4.1"),
    cowstr!("sse4.2"),
    cowstr!("sse4"),
    cowstr!("sse4a"),
    cowstr!("avx"),
    cowstr!("avx2"),
    cowstr!("avx512f"),
    cowstr!("avx512cd"),
    cowstr!("avx512vl"),
    cowstr!("avx512cw"),
    cowstr!("avx512dq"),
    cowstr!("avx512ifma"),
    cowstr!("mavx512vbmi"),
    cowstr!("avx512vpopcntdq"),
    cowstr!("avx512vp2intersect"),
    cowstr!("avx512vnni"),
    cowstr!("avx512vbmi2"),
    cowstr!("avx512bf16"),
    cowstr!("avx512fp16"),
    cowstr!("avx512bitalg"),
    cowstr!("avx512bmm"),
    cowstr!("avxvnni"),
    cowstr!("avxifma"),
    cowstr!("avxvnniint8"),
    cowstr!("avxneconvert"),
    cowstr!("avxvnniint16"),
    cowstr!("avx10.1"),
    cowstr!("avx10.2"),
    cowstr!("sha"),
    cowstr!("aes"),
    cowstr!("pclmul"),
    cowstr!("clflushopt"),
    cowstr!("clwb"),
    cowstr!("fsgsbase"),
    cowstr!("ptwrite"),
    cowstr!("rdrnd"),
    cowstr!("f16c"),
    cowstr!("fma"),
    cowstr!("fma4"),
    cowstr!("pconfig"),
    cowstr!("wbnoinvd"),
    cowstr!("prfchw"),
    cowstr!("rdpid"),
    cowstr!("rdseed"),
    cowstr!("sgx"),
    cowstr!("xop"),
    cowstr!("3dnow"),
    cowstr!("3dnowa"),
    cowstr!("abm"),
    cowstr!("adx"),
    cowstr!("adx"),
    cowstr!("bmi"),
    cowstr!("bmi2"),
    cowstr!("lzcnt"),
    cowstr!("fxsr"),
    cowstr!("xsave"),
    cowstr!("xsaveopt"),
    cowstr!("xsavec"),
    cowstr!("xsaves"),
    cowstr!("rtm"),
    cowstr!("hle"),
    cowstr!("tbm"),
    cowstr!("mwaitx"),
    cowstr!("clzero"),
    cowstr!("pku"),
    cowstr!("gfni"),
    cowstr!("vaes"),
    cowstr!("waitpkg"),
    cowstr!("vpclmulqdq"),
    cowstr!("movdiri"),
    cowstr!("movdir64b"),
    cowstr!("uintr"),
    cowstr!("tsxldtrk"),
    cowstr!("cldemote"),
    cowstr!("serialize"),
    cowstr!("amx-tile"),
    cowstr!("amx-int8"),
    cowstr!("amx-bf16"),
    cowstr!("hreset"),
    cowstr!("kl"),
    cowstr!("widekl"),
    cowstr!("cmpccxadd"),
    cowstr!("amx-fp16"),
    cowstr!("pretetchi"),
    cowstr!("raoint"),
    cowstr!("amx-complex"),
    cowstr!("sm3"),
    cowstr!("sm4"),
    cowstr!("sha512"),
    cowstr!("apxf"),
    cowstr!("usermsr"),
    cowstr!("amx-avx512"),
    cowstr!("amx-tf32"),
    cowstr!("amx-fp8"),
    cowstr!("movrs"),
    cowstr!("amx-movrs"),
    cowstr!("cx16"),
    cowstr!("cx8"),
    cowstr!("sahf"),
    cowstr!("movbe"),
    cowstr!("shstk"),
    cowstr!("crc32"),
    cowstr!("mwait"),
];

macro_rules! x86_machines {
    {
        $vis:vis static {
            $(#[$meta16:meta])*
            $x86_16_machines:ident,
            $(#[$meta32:meta])*
            $x86_32_machines:ident,
            $(#[$meta64:meta])*
            $x86_64_machines:ident
        $(,)?} = [

            $($(#[default] $(@ $_bit16_default_tt:tt)?)? $bit16_machine:literal [$($bit16_feature:literal),* $(,)?],)*
            #![x86_32]
            $($(#[default] $(@ $_bit32_default_tt:tt)?)? $bit32_machine:literal [$($bit32_feature:literal),* $(,)?],)+
            #![x86_64]
            $( $(#[default] $(@ $_bit64_default_tt:tt)?)? $bit64_machine:literal [$($bit64_feature:literal),* $(,)?]),+ $(,)?
        ];
    } => {

        mod mach_impl {
            use super::{Machine};
            static ALL_X86_MACHINES: &[Machine] = &[
                $(Machine {
                    name: cowstr!($bit16_machine),
                    features: slice![$(cowstr!($bit16_feature)),*],
                    mach_extended_properties: slice![],
                },)*
                $(Machine {
                    name: cowstr!($bit32_machine),
                    features: slice![$(cowstr!($bit32_feature)),*],
                    mach_extended_properties: slice![],
                },)*
                $(Machine {
                    name: cowstr!($bit64_machine),
                    features: slice![$(cowstr!($bit64_feature)),*],
                    mach_extended_properties: slice![],
                },)*
            ];

            const fn const_slice<T>(x: &[T], begin: usize) -> &[T] {
                if begin > x.len() {
                    panic!("invalid indexing")
                }

                let len = x.len() - begin;

                let x = x as *const [T] as *const T;



                let ptr = unsafe { x.add(begin) };

                unsafe {core::slice::from_raw_parts(ptr, len)}
            }


            $(#[$meta16])*
            pub static $x86_16_machines: &[Machine] = ALL_X86_MACHINES;
            $(#[$meta32])*
            pub static $x86_32_machines: &[Machine] = const_slice(ALL_X86_MACHINES, ${count($bit16_machine)});
            $(#[$meta64])*
            pub static $x86_64_machines: &[Machine] = const_slice(ALL_X86_MACHINES, ${count($bit16_machine)} + ${count($bit32_machine)});
        }

        $vis use mach_impl::*;
    }
}

/// x86 machines
pub mod machines {
    use super::Machine;
    use crate::helpers::{CowPtr, CowSlice};
    x86_machines! {
        pub static {
            /// 16-bit x86 machines
            X86_16,
            /// 32-bit x86 machines
            X86_32,
            /// 64-bit x86 machines
            X86_64
        } = [
            "8086" ["x87"],
            "80286" ["x87"],
            #![x86_32]
            "i386" ["x87"],
            "i486" ["x87"],
            "i586" ["x87"],
            "pentium" ["x87"],
            "lakemont" ["x87"],
            "pentium-mmx" ["x87", "mmx"],
            "pentiumpro" ["x87"],
            "i686" ["x87"],
            "pentium2" ["x87", "mmx", "fxsr"],
            "petnium3" ["x87", "mmx", "fxsr", "sse"],
            "pentium3m" ["x87", "mmx", "fxsr", "sse"],
            "pentium-m" ["x87", "mmx", "fxsr", "sse", "sse2"],
            "prescott" ["cx8", "cmov", "x87", "mmx", "sse", "sse2", "sse3", "fxsr"],
            "k6" ["x87", "mmx"],
            "k6-2" ["x87", "mmx", "3dnow"],
            "k6-3" ["x87", "mmx", "3dnow"],
            "athlon" ["x87", "mmx", "3dnow", "3dnowa"],
            "althon-third" ["x87", "mmx", "3dnow", "3dnowa"],
            "athlon-4" ["x87", "mmx", "3dnow", "3dnowa", "sse"],
            "athlon-xp" ["x87", "mmx", "3dnow", "3dnowa", "sse"],
            "athlon-mp" ["x87", "mmx", "3dnow", "3dnowa", "sse"],
            #![x86_64]
            "x86-64" ["cx8", "cmov", "x87", "mmx", "sse", "sse2", "fxsr"],
            "x86-64v2" ["cx8", "cmov", "x87", "mmx", "sse", "sse2", "fxsr", "cx16", "sahf", "sse", "popcnt", "ssse3", "sse4.1", "sse4.2"],
            "x86-64v3" ["cx8", "cmov", "x87", "mmx", "sse", "sse2", "fxsr", "cx16", "sahf", "sse", "popcnt", "ssse3", "sse4.1", "sse4.2",
                "avx", "avx2", "bmi1", "bmi2", "f16c", "abm", "movbe", "xsave"],
            "x86-64v4" ["cx8", "cmov", "x87", "mmx", "sse", "sse2", "fxsr", "cx16", "sahf", "sse", "popcnt", "ssse3", "sse4.1", "sse4.2",
                "avx", "avx2", "bmi1", "bmi2", "f16c", "abm", "movbe", "xsave", "avx512f", "avx512bw", "avx512cd", "avx512dq", "avx512vl"],
            "nocona" ["cx8", "cmov", "x87", "mmx", "sse", "sse2", "sse3", "fxsr"],
            "core2" ["cx8", "cmov", "x87", "mmx", "sse", "sse2", "sse3", "ssse3", "cx16", "sahf", "fxsr"],

        ];
    }
}

/// asm properties for x86 targets
pub mod asm {
    use crate::properties::arch::Asm;

    /// 16-bit x86 asm
    pub static X86_16: Asm = Asm {};

    /// 32-bit x86 asm
    pub static X86_32: Asm = Asm {};

    /// 64-bit x86 asm
    pub static X86_64: Asm = Asm {};
}

/// x86 call tags
pub mod tags {
    use crate::helpers::CowStr;

    /// x86-16 call tags
    pub static X86_16: &[CowStr] = &[
        cowstr!("cdecl"),
        cowstr!("pascal"),
        cowstr!("fastcall-ms"),
        cowstr!("fastcall-turbo"),
        cowstr!("watcall"),
    ];

    /// x86-32 call tags
    pub static X86_32: &[CowStr] = &[
        cowstr!("cdecl-ms"),
        cowstr!("cdecl-unix"),
        cowstr!("stdcall-ms"),
        cowstr!("stdcall-unix"),
        cowstr!("fastcall-ms"),
        cowstr!("fastcall-unix"),
        cowstr!("thiscall-ms"),
        cowstr!("thiscall-unix"),
        cowstr!("register"),
        cowstr!("vectorcall-ms"),
        cowstr!("vectorcall-unix"),
        cowstr!("watcall"),
    ];

    /// x86-64 call tags
    pub static X86_64: &[CowStr] = &[cowstr!("sysv64"), cowstr!("win64"), cowstr!("vectorcall")];
}

macro_rules! x86_archs {
    {
        $($(#[$meta:meta])* $vis:vis static $name:ident ($name_canon:literal) {
            $(alias_names: [$($name_alias_extra:literal),* $(,)?],)?
            width: $width:literal,
            base: $base:ident
            $(,default_machine: $default_machine:expr)?
            $(,)?
        })*
    } => {
        $(
            $(#[$meta])*
            $vis static $name: Arch = Arch {
                name: cowstr!($name_canon),
                alias_names: slice![cowstr!("x86"), $($(cowstr!($name_alias_extra)),*)?],
                machines: cow!(*machines:: $base),
                raw_width: $width,
                default_machine: cow!(((machines:: $base))[($($default_machine,)? 0, ).0]),
                call_tags: cow!(*tags:: $base),
                arch_extended_properties: slice![],
                asm_spec: Some(cow!(asm:: $base)),
                features: cow!(*X86_FEATURES),
            };
        )*
    };
}

x86_archs! {

    /// base x86-16
    pub static A8086 ("8086") {
        width: 16,
        base: X86_16,
    }

    /// x86-16 with protected mode
    pub static I286 ("i286") {
        width: 16,
        base: X86_16,
        default_machine: 1,
    }

    /// i386
    pub static I386 ("i386") {
        width: 32,
        base: X86_32,
    }

    /// i486
    pub static I486 ("i486") {
        width: 32,
        base: X86_32,
        default_machine: 1
    }

    /// i586
    pub static I586 ("i586") {
        width: 32,
        base: X86_32,
        default_machine: 2
    }

    /// i686
    pub static I686 ("i686") {
        width: 32,
        base: X86_32,
        default_machine: 7
    }

    /// i786 (pentium 4 or better cpu on 32-bit)
    pub static I786 ("i786") {
        width: 32,
        base: X86_32,
        default_machine: (machines::X86_32.len() - machines::X86_64.len())
    }

    /// Baseline x86_64
    pub static X86_64 ("x86-64") {
        alias_names: ["amd64", "x64_64", "intel64"],
        width: 64,
        base: X86_64
    }

    /// x86_64v2 (x86_64 microarchitecture levels)
    pub static X86_64_V2 ("x86-64v2") {
        alias_names: ["amd64", "x64_64", "intel64", "x86-64"],
        width: 64,
        base: X86_64,
        default_machine: 1
    }

    /// x86_64v3 (x86_64 microarchitecture levels)
    pub static X86_64V3 ("x86-64v3") {
        alias_names: ["amd64", "x64_64", "intel64", "x86-64"],
        width: 64,
        base: X86_64,
        default_machine: 2
    }

    /// x86_64v4 (x86_64 microarchitecture levels)
    pub static X86_64V4 ("x86-64v4") {
        alias_names: ["amd64", "x64_64", "intel64", "x86-64"],
        width: 64,
        base: X86_64,
        default_machine: 3
    }
}

/// x86-16 memory model with flat (unsegmented/transparently segmented) memory
pub static X86_16_FLAT: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_IP16,
    max_int_align: 2,
    max_bit_int_align: 2,
    max_simd_align: 16,
    ldouble_align: 2,
    ldouble_format: X87_DOUBLE_EXTENDED,
};

/// x86-16 memory model with segmentation, using near pointers by default
/// ss and ds must be the same. cs and es may be different
pub static X86_16_NEAR: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_IP16_NEAR_FAR,
    max_int_align: 2,
    max_bit_int_align: 2,
    max_simd_align: 16,
    ldouble_align: 2,
    ldouble_format: X87_DOUBLE_EXTENDED,
};

/// x86-16 memory model with segmentation, using far pointers by default
pub static X86_16_FAR: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_LP32_NEAR_FAR,
    max_int_align: 2,
    max_bit_int_align: 2,
    max_simd_align: 16,
    ldouble_align: 2,
    ldouble_format: X87_DOUBLE_EXTENDED,
};

/// Default Primitives for x86-32
pub static X86_32_PRIMITIVES: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_ILP32,
    max_int_align: 4,
    max_bit_int_align: 4,
    max_simd_align: 64,
    ldouble_align: 4,
    ldouble_format: X87_DOUBLE_EXTENDED,
};

/// Default Primitives for Sys-V x86-64
pub static X86_64_PRIMITIVES_SYSV: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_LP64,
    max_int_align: 16,
    max_bit_int_align: 8,
    max_simd_align: 64,
    ldouble_align: 16,
    ldouble_format: X87_DOUBLE_EXTENDED,
};

/// Default Primitives for Sys-V x86-64 ILP32/x32
pub static X32_PRIMITIVES: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_ILP32,
    max_int_align: 16,
    max_bit_int_align: 8,
    max_simd_align: 64,
    ldouble_align: 16,
    ldouble_format: X87_DOUBLE_EXTENDED,
};

/// Default Primitives for Sys-V x86-64 using binary64 long double
pub static X86_64_F64_LONG_DOUBLE: PrimitiveLayouts = PrimitiveLayouts {
    int_layout: LE_LP64,
    max_int_align: 16,
    max_bit_int_align: 8,
    max_simd_align: 64,
    ldouble_align: 8,
    ldouble_format: IEEE754_DOUBLE,
};
