use target_tuples::pieces::{Environment, OS, System};

use crate::properties::env::{Env, BuiltinSymbol, DEFAULT_C_ENV};


/// lilium-std environment
pub static LILIUM_STD_ENV: Env = Env {
    supported_builtins: slice![BuiltinSymbol::Memcpy, BuiltinSymbol::Memset, BuiltinSymbol::Memchr, BuiltinSymbol::Memcmp, BuiltinSymbol::MemcmpEq, BuiltinSymbol::MemcpyS, BuiltinSymbol::MemsetS]
};

/// Generic elf environment
pub static FREESTANDING_ENV: Env = DEFAULT_C_ENV;

/// Determines the environment from the specified [`System`] string
pub fn from_target(sys: System) -> Option<&'static Env> {
    match (sys.os(), sys.env(), sys.object_format()) {
        (Some(OS::Lilium), Some(Environment::Standard), _) => Some(&LILIUM_STD_ENV),
        _ => Some(&FREESTANDING_ENV)
    }
}