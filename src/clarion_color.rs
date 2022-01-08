//! The [`ClarionColor`] struct and associated `impl`s.
use crate::{ClarionErr, RgbColor};

/// Defines a color in the Clarion color format - an integer between 0
/// and 16,777,215.
pub struct ClarionColor {
    /// The integral representation of this color.
    color: i32,
}

impl ClarionColor {
    /// The maximum representation of a ClarionColor, representing white.
    pub const MAX: i32 = 16_777_215;

    /// The miniumum possible value of a ClarionColor, representing black.
    pub const MIN: i32 = 0;

    /// Creates a new `ClarionColor` with the specified color as a `i32`.
    ///
    /// The specified color must be between `ClarionColor::MIN` and
    /// `ClarionColor::MAX`. Colors outside of these bounds will return
    /// an `Err` result with `ClarionErr::OutOfRange`.
    ///
    /// # Examples
    ///
    /// Valid color returns `Ok`:
    /// ```
    /// let c_color = clarion::ClarionColor::new(4259584);
    /// assert!(c_color.is_ok())
    /// ```
    /// 
    /// Invalid color returns `Err`:
    /// ```
    /// let c_color = clarion::ClarionColor::new(i32::MAX);
    /// assert!(c_color.is_err());
    /// ```
    pub fn new(color: i32) -> Result<ClarionColor, ClarionErr> {
        if color <= ClarionColor::MAX && color >= ClarionColor::MIN {
            Ok(ClarionColor { color })
        } else {
            Err(ClarionErr::OutOfRange)
        }
    }

    /// Get the integral representation of this `ClarionColor` value.
    ///
    /// The integral representation of a `ClarionColor` is a i32 between
    /// `ClarionColor::MIN`, which represents black, and
    /// `ClarionColor::MAX`, which represents white.
    ///
    /// # Examples
    ///
    /// ```
    /// let c_color = clarion::ClarionColor::new(4259584).unwrap();
    /// assert_eq!(c_color.color(), 4259584);
    /// ```
    pub fn color(&self) -> i32 {
        self.color
    }
}

impl From<RgbColor> for ClarionColor {
    /// Convert an `RgbColor` value into a `ClarionColor` value.
    ///
    /// # Examples
    ///
    /// ```
    /// let color = clarion::RgbColor {red: 255, green: 128, blue: 64};
    /// let c_color = clarion::ClarionColor::from(color);
    /// assert_eq!(c_color.color(), 4227327);
    /// ```
    fn from(color: RgbColor) -> Self {
        let red = color.red as i32;
        let green = color.green as i32 * 256;
        let blue = color.blue as i32 * 65536;
        let color = red + green + blue;
        ClarionColor { color }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClarionColor, RgbColor};

    #[test]
    fn from_rgb_red() {
        let rgb_color = RgbColor {
            red: 255,
            green: 0,
            blue: 0,
        };
        let c_color = ClarionColor::from(rgb_color);
        assert_eq!(c_color.color(), 255);
    }

    #[test]
    fn from_rgb_greencyan() {
        let rgb_color = RgbColor {
            red: 0,
            green: 255,
            blue: 64,
        };
        let c_color = ClarionColor::from(rgb_color);
        assert_eq!(c_color.color(), 4259584);
    }

    #[test]
    fn from_rgb_blue() {
        let rgb_color = RgbColor {
            red: 0,
            green: 0,
            blue: 255,
        };
        let c_color = ClarionColor::from(rgb_color);
        assert_eq!(c_color.color(), 16711680);
    }
}
