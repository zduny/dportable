mod immutable;
pub use immutable::*;

pub mod mutable;

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadySet {}

impl Display for AlreadySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value already set")
    }
}

impl std::error::Error for AlreadySet {}
