use target_tuples::pieces::{Architecture, Environment, OS};

use crate::{
    builtin::archs::x86::{X32_PRIMITIVES, X86_64_F64_LONG_DOUBLE, X86_64_PRIMITIVES_SYSV},
    properties::abi::{Abi, PrimitiveLayouts},
};

/// Hardfloat ABI
pub static ABI_HARDFLOAT: Abi = Abi {
    float_pass_override: None,
    simd_pass_override: None,
};

/// Softfloat ABI
pub static ABI_SOFTFLOAT: Abi = Abi {
    float_pass_override: Some(crate::properties::abi::PassModeOverride::Int),
    simd_pass_override: Some(crate::properties::abi::PassModeOverride::Int),
};

/// Obtains the [`PrimitiveLayouts`] properties for a given arch, os, env triple
pub const fn primitives_from_target(
    arch: Architecture,
    os: OS,
    env: Option<Environment>,
) -> Option<&'static PrimitiveLayouts> {
    match (arch, os, env) {
        (Architecture::X86_64 { .. }, OS::Linux, Some(Environment::GNUX32)) => {
            Some(&X32_PRIMITIVES)
        }
        (
            Architecture::X86_64 { .. },
            OS::Linux | OS::FreeBSD | OS::OpenBSD | OS::NetBSD | OS::Fuchsia,
            _,
        ) => Some(&X86_64_PRIMITIVES_SYSV),
        (Architecture::X86_64 { .. }, OS::Lilium, _) => Some(&X86_64_F64_LONG_DOUBLE),
        _ => None,
    }
}

/// Obtains the [`Abi`] properties for a given arch, os, env triple
pub const fn abi_from_target(
    arch: Architecture,
    os: OS,
    env: Option<Environment>,
) -> Option<&'static Abi> {
    match (arch, os, env) {
        (Architecture::X86_64 { .. } | Architecture::Clever | Architecture::HoleyBytes, _, _) => {
            Some(&ABI_HARDFLOAT)
        }
        (
            Architecture::Aarch64
            | Architecture::Aarch64Be
            | Architecture::Aarch64_32
            | Architecture::Arm
            | Architecture::ArmBe,
            OS::Linux | OS::Lilium | OS::Win32 | OS::MacOSX,
            _,
        )
        | (
            Architecture::Aarch64
            | Architecture::Aarch64Be
            | Architecture::Aarch64_32
            | Architecture::Arm
            | Architecture::ArmBe,
            _,
            Some(Environment::GNUEABIHF | Environment::EABIHF),
        ) => Some(&ABI_HARDFLOAT),
        (
            Architecture::Aarch64
            | Architecture::Aarch64Be
            | Architecture::Aarch64_32
            | Architecture::Arm
            | Architecture::ArmBe,
            _,
            Some(Environment::GNUEABI | Environment::EABI),
        ) => Some(&ABI_SOFTFLOAT),
        _ => None,
    }
}
