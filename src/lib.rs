#![warn(missing_docs)]
//! A very simple library for working with Clarion data formats in Rust.
//!
//! Clarion dates are specified by the integral number of days between a
//! given date and the 28th of December, 1800.
//!
//! Clarion times are specified by the number of centiseconds between a
//! given time and midnight. (A centisecond is 10 milliseconds.)
//! 
//! Clarion colors are specified by a base-10 integral representation of
//! a 24-bit RGB color in 0xBBGGRR format, as opposed to the standard
//! 0xRRGGBB format, where RR represents red, GG represents green, and BB
//! represents the blue 8-bit components of the 24 bit color respectively.
mod clarion_color;
mod clarion_date;
mod clarion_err;
mod clarion_time;
mod rgb_color;

pub use crate::clarion_color::ClarionColor;
pub use crate::clarion_date::ClarionDate;
pub use crate::clarion_err::ClarionErr;
pub use crate::clarion_time::ClarionTime;
pub use crate::rgb_color::RgbColor;
