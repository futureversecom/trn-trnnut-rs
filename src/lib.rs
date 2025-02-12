// Copyright 2022-2023 Futureverse Corporation Limited
//!
//! # TRNNut
//!
//! Delegated authority nut for TRN
//!

#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

extern crate alloc;

use alloc::fmt::{self, Display, Formatter};
use codec::Input;
pub use core::convert::TryFrom;

pub mod trnnut;
pub mod validation;
pub mod method;
pub mod module;

pub use crate::{
    trnnut::TRNNutV0,
    validation::ValidationErr,
};

#[cfg(test)]
mod tests;

pub const WILDCARD: &str = "*";

/// A TRN module permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum RuntimeDomain {
    Method,
    MethodArguments,
    Module,
}

impl Display for RuntimeDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Method => write!(f, "method"),
            Self::MethodArguments => write!(f, "method arguments"),
            Self::Module => write!(f, "module"),
        }
    }
}

pub trait PartialDecode: Sized {
    /// decode an input which is not including the version as the up front two bytes
    ///
    /// # Errors
    ///
    /// On failure, returns a `codec::Error`
    fn partial_decode<I: Input>(input: &mut I) -> Result<Self, codec::Error>;
}
