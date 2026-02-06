//! Stores information about target properties
//!
//! Every target has a collection of properties, that describes the behaviour of the target,
//!  such as the layout of primitive types, how to search for libraries, or what artifacts to emit

use crate::helpers::{CowPtr, CowSlice, CowStr};

/// Extended Properties are a structured way of representing data.
/// These are string keys (identifiers separated by dots), with string, boolean, or integer values.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ExtPropertyValue {
    /// A String
    String(CowStr),
    /// A boolean
    Bool(bool),
    /// An Integer
    Int(i64),
}

pub mod arch;

pub mod os;

pub mod abi;

pub mod link;

pub mod target;
