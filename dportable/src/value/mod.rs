//! Asynchronous cloneable lazily-initialized value.
//!
//! Note that it uses locking internally - it is intended for cases where convenience of use
//! is more important than high performance - for those use naked async channels instead.

mod immutable;
pub use immutable::*;

pub mod mutable;

use std::fmt::Display;

/// Value was already set error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadySet {}

impl Display for AlreadySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value already set")
    }
}

impl std::error::Error for AlreadySet {}
