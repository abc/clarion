#![warn(missing_docs)]
//! The [`CSErr`] enum

use std::{error::Error, fmt::Display};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// Defines error states for the `cstime` library.
pub enum CSErr {
    /// An error state for when a conversion has failed due to an overflow error.
    ConversionOverflowed,
}

impl Display for CSErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSErr::ConversionOverflowed => write!(
                f,
                "{}",
                "The CSDate value was out of range for the conversion and overflowed."
            ),
        }
    }
}

impl Error for CSErr {}
