//! The [`ClarionErr`] enum and associated `impl`s.

use std::{error::Error, fmt::Display};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// Defines error states for the `clarion` library.
pub enum ClarionErr {
    /// An error state for when a conversion has failed due to an overflow error.
    ConversionOverflowed,
    /// An error state for when a constructor would create an out-of-range value.
    OutOfRange,
}

impl Display for ClarionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClarionErr::ConversionOverflowed => write!(
                f,
                "The ClarionDate value was out of range for the conversion and overflowed."
            ),
            ClarionErr::OutOfRange => {
                write!(f, "The parameter was out of range for the constructor.",)
            }
        }
    }
}

impl Error for ClarionErr {}
