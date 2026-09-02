//! Properties about the environment

use crate::helpers::CowSlice;

/// Environment definition
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Env {
    /// Builtin symbols known to be linkable on the target (unless builtin replacement is disabled)
    pub supported_builtins: CowSlice<BuiltinSymbol>
}

/// An enum of well known symbol functions
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum BuiltinSymbol {
    /// `void* memmove(void* dest, const void* src, size_t len);`
    /// A well-known symbol that copies `len` bytes from `src` to `dest.`
    Memmove,
    /// `void* memcpy(void* restrict dest, const void* restrict src, size_t len);`
    /// A well-known symbol that copies `len` bytes from `src` to `dest`, assuming that `[dest, dest+len)` does not overlap with `[src, src+len)`.
    Memcpy,
    /// `int memcmp(const void* src1, const void* src2, size_t len);`
    /// A well-known symbol that compares `len` bytes from `src1`, and `src2` lexicographically, and returns 0 if they compare equal, -1 if the comparison returns `src1 < src2`, and 1 if `src1 > src2`
    Memcmp,
    /// `int __memcmpeq(const void* src1, const void* src2, size_t len);`
    /// A well-known symbol that is equivalent to [`BuiltinSymbol::Memcmp`], except that it assumes that only equality or inequality matters. The return value for unequal sequences is an arbitrary non-zero value
    MemcmpEq,
    /// `int bcmp(const void* src1, const void* src2, size_t len);`
    /// A well-known symbol that is a legacy version of [`BuiltinSymbol::MemcmpEq`]
    Bcmp,
    /// `void* memset(void* dest, int src, size_t len);`
    /// A well-known symbol that writes `src & 0xFF` to each byte in `[dest, dest+len)`
    Memset,
    /// `const void* memchr(const void* src, int ch, size_t len);`
    /// A well-knwon symbol that finds `ch` in `[dest, dest+len)` and returns a pointer to it if present, or a null pointer otherwise
    Memchr,
    /// `size_t strlen(const char* src);`
    /// A well-known symbol that determines the length of `src`
    Strlen,
    /// `size_t strnlen(const char* src, size_t maxlen);`
    /// A well-known symbol that determines the length of `src`, up to `maxlen`
    Strnlen,
    /// `char* strcpy(char* restrict dest, const char* src);`
    /// A well-known symbol that copies a null-terminated multibyte string from `src` to `dest`, assuming that they do not overlap
    Strcpy,
    /// `int strcmp(const char* src1, const char* src2);`
    /// A well-known symbol that compares null-terminated multibyte strings, and returns 0 if they compare euqal, -1 if the comparison returns `src1 < src2`, and 1 if `src1 > src2`
    Strcmp,

    /// `errno_t memcpy_s(void* restrict dest, rsize_t destsz, const void* restrict src, size_t len);`
    /// A well-known symbol that copies `len` bytes from `src` to `dest`, assuming that `[dest, dest+len)` does not overlap with `[src, src+len)`.
    /// Errors (returns non-zero) if undefined behaviour for `memcpy` is detected according to the `destsz`. In this case, `[dest, dest+destsz)` is zeroed out.`
    MemcpyS,
    /// `errnot_t memset(void* dest, rsize_t destsz, int src, size_t len);`
    /// A well-known symbol that writes `src & 0xFF` to each byte in `[dest, dest+len)`
    /// Errors (returns non-zero) if undefined behaviour for `memset` is detected according to the `destsz`. In this case, `[dest, dest+destsz)` is zeroed out.`
    MemsetS,
}


/// The default supported functions for standard builtins.
/// These are functions that other C compilers generally rely on.
pub const DEFAULT_C_ENV: Env = Env {
    supported_builtins: slice![BuiltinSymbol::Memcpy, BuiltinSymbol::Memcmp, BuiltinSymbol::Memset, BuiltinSymbol::Memchr],
};

/// The default supported functions for standard builtins.
/// These are functions that other C compilers generally rely on.
pub const DEFAULT_LEGACY_POSIX: Env = Env {
    supported_builtins: slice![BuiltinSymbol::Memcpy, BuiltinSymbol::Memcmp, BuiltinSymbol::Memset, BuiltinSymbol::Memchr, BuiltinSymbol::Bcmp],
};


/// The default supported functions for standard builtins.
/// These are functions that other C compilers generally rely on.
pub const DEFAULT_EXTENDED_ENV: Env = Env {
    supported_builtins: slice![BuiltinSymbol::Memcpy, BuiltinSymbol::Memcmp, BuiltinSymbol::Memset, BuiltinSymbol::Memchr, BuiltinSymbol::MemcmpEq],
};