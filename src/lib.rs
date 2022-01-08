#![warn(missing_docs)]
//! A very simple library for working with CSTime time/date formats.
//!
//! CSTime dates are specified by the integral number of days between a given
//! date and the 28th of December, 1800.
//!
//! CSTime times are specified by the number of centiseconds between a given
//! time and midnight. (A centisecond is 10 milliseconds.)
mod csdate;
mod cserr;
mod cstime;

pub use crate::csdate::CSDate;
pub use crate::cserr::CSErr;
pub use crate::cstime::CSTime;
