//! Helper types for the crate

use std::{borrow::Borrow, hash::Hash, ops::Deref};

/// A [`std::borrow::Cow`] that is always pointer sized (plus discriminant).
/// This is similar to [`std::borrow::Cow`], except the owned branch is contained in a [`Box`] instead of [`ToOwned::Owned`].
///
/// Note that the above statement about being pointer-sized is not a layout guarantee, and is subject to support of the compiler.
/// In general, this is lighter to carry arround than a large `T`.
pub enum CowPtr<'a, T: ?Sized> {
    /// A boxed value
    Boxed(Box<T>),
    /// A borrowed value
    Borrowed(&'a T),
}

impl<'a, T: ?Sized> Clone for CowPtr<'a, T>
where
    Box<T>: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Boxed(v) => Self::Boxed(v.clone()),
            Self::Borrowed(v) => Self::Borrowed(*v),
        }
    }
}

impl<'a, T: ?Sized> CowPtr<'a, T> {
    /// Converts a [`CowPtr`] into a reference to the inner value, dereferencing the pointer if necessary.
    /// This is the same as [`Deref`], [`AsRef`], and [`Borrow`], but additionally can be used at compile time for a borrowed value
    pub const fn as_ref(&self) -> &T {
        match self {
            Self::Borrowed(v) => &**v,
            Self::Boxed(v) => &**v,
        }
    }

    /// Leaks the [`CowPtr`] to a `&T` that lasts for at least as long as `'a`. If this is a borrowed value, returns the inner reference.
    /// If this is a boxed value, leaks the box.
    pub fn leak(self) -> &'a T {
        match self {
            Self::Borrowed(v) => v,
            Self::Boxed(v) => Box::leak(v),
        }
    }

    /// Convience method to borrow out of [`CowPtr`] into another [`CowPtr`]
    pub const fn borrow(&self) -> CowPtr<'_, T> {
        CowPtr::Borrowed(self.as_ref())
    }
}

impl<'a, T: ?Sized + CloneToBox> CowPtr<'a, T> {
    /// Converts the [`CowPtr`] to an owned variant, cloning it if necessary.
    /// This lengthens the lifetime to an unbound lifetime (which may be `'static` if `T: 'static``)
    pub fn to_owned<'b>(self) -> CowPtr<'b, T>
    where
        T: 'b,
    {
        CowPtr::Boxed(self.to_box())
    }

    /// Obtains mutable access to the inside of the [`CowPtr`], cloning it into a new [`Box`] if it is borrowed.
    pub fn get_mut(&mut self) -> &mut T {
        match self {
            Self::Boxed(v) => v,
            Self::Borrowed(v) => {
                let v = (*v).clone_to_box();
                *self = Self::Boxed(v);

                let Self::Boxed(v) = self else { unreachable!() };

                v
            }
        }
    }

    /// Converts the [`CowPtr`] into a boxed value, cloning it if necessary
    pub fn to_box(self) -> Box<T> {
        match self {
            Self::Borrowed(v) => v.clone_to_box(),
            Self::Boxed(v) => v,
        }
    }
}

impl<'a, T: ?Sized> Deref for CowPtr<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: ?Sized> AsRef<T> for CowPtr<'a, T> {
    fn as_ref(&self) -> &T {
        self.as_ref()
    }
}

impl<'a, T: ?Sized> Borrow<T> for CowPtr<'a, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<'a, T: ?Sized + core::fmt::Debug> core::fmt::Debug for CowPtr<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, T: ?Sized + PartialEq> PartialEq for CowPtr<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<'a, T: ?Sized + Eq> Eq for CowPtr<'a, T> {}

impl<'a, T: ?Sized + PartialOrd> PartialOrd for CowPtr<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<'a, T: ?Sized + Ord> Ord for CowPtr<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<'a, T: ?Sized + PartialEq> PartialEq<T> for CowPtr<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other
    }
}

impl<'a, T: ?Sized + PartialOrd> PartialOrd<T> for CowPtr<'a, T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

impl<'a, T: ?Sized + Hash> Hash for CowPtr<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<'a, T: ?Sized> From<&'a T> for CowPtr<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T: ?Sized> From<Box<T>> for CowPtr<'a, T> {
    fn from(value: Box<T>) -> Self {
        Self::Boxed(value)
    }
}

/// Helper trait for [`CowPtr`]. Normally you don't need to implement this (but it can be useful for, e.g. an unsized type (such as a custom slice or a trait object),
///  or a type that is expensive to store on the stack)
///
/// # Implementors
/// Blanket implementations are provided for the following types:
/// * `T`, where `T: Clone`.
/// * `[T]`, where `T: Clone`,
/// * `str`.
pub trait CloneToBox {
    /// Constructs a new box that contains an equivalent value to `self`
    fn clone_to_box(&self) -> Box<Self>;
}

impl<T: Clone> CloneToBox for T {
    fn clone_to_box(&self) -> Box<Self> {
        Box::new(self.clone())
    }
}

impl<T: Clone> CloneToBox for [T] {
    fn clone_to_box(&self) -> Box<Self> {
        Box::from(self)
    }
}

impl CloneToBox for str {
    fn clone_to_box(&self) -> Box<Self> {
        Box::from(self)
    }
}

impl<'a, T> From<Vec<T>> for CowPtr<'a, [T]> {
    fn from(value: Vec<T>) -> Self {
        Self::Boxed(value.into_boxed_slice())
    }
}

impl<'a> From<String> for CowPtr<'a, str> {
    fn from(value: String) -> Self {
        Self::Boxed(value.into_boxed_str())
    }
}

/// Convience type alias for a [`CowPtr`] that contains a static string
pub type CowStr = CowPtr<'static, str>;

/// Convience type alias for a [`CowPtr`] that contains a static slice
pub type CowSlice<T> = CowPtr<'static, [T]>;

#[doc(hidden)]
pub use core as __core;

impl<'a, T> IntoIterator for &'a CowPtr<'_, [T]> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
