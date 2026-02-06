#![deny(missing_docs)]
#![feature(macro_metavar_expr)]
//! Target Property Library for lccc

/// Helper macro for producing a literal of type [`core::num::NonZero`]. Errors at compile time if the value `0` is passed
#[macro_export]
macro_rules! nzlit {
    ($expr:expr) => {
        const {
            let __val = $expr;
            $crate::helpers::__core::assert!(
                __val != 0,
                $crate::helpers::__core::concat!(::core::stringify!($expr), " is zero")
            );

            unsafe { $crate::helpers::__core::num::NonZero::new_unchecked(__val) }
        }
    };
}

/// Helper macro for producing a compile-time constant [`CowPtr`][helpers::CowPtr] of a slice.
#[macro_export]
macro_rules! slice {
    [$($expr:expr),+ $(,)?] => {
        const { $crate::helpers::CowPtr::<[_]>::Borrowed(&[$($expr),+]) }
    };
    [] => {
        const { $crate::helpers::CowPtr::<[_]>::Borrowed(&[]) }
    }
}

/// Helper macro for producing a compile-time constant [`CowPtr`][helpers::CowPtr] of a string.
#[macro_export]
macro_rules! cowstr {
    ($lit:expr) => {
        const { $crate::helpers::CowPtr::<$crate::helpers::__core::primitive::str>::Borrowed($lit) }
    };
}

/// Helper macro for producing a compile-time constant [`CowPtr`][helpers::CowPtr] from a constant expression
#[macro_export]
macro_rules! cow {
    ($expr:expr) => {
        const { $crate::helpers::CowPtr::Borrowed(&$expr) }
    };
}

pub mod helpers;

pub mod properties;

pub mod builtin;
