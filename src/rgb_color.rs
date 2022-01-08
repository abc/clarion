//! The [`RgbColor`] struct and associated `impl`s.
use crate::ClarionColor;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// Defines a 24-bit color within the RGB color space, represented by
/// three 8-bit integers, one integer for each segment.
///
/// # Examples
///
/// ```
/// let color = clarion::RgbColor { red: 128, green: 64, blue: 0 };
/// let (red, green, blue) = (color.red, color.green, color.blue);
/// assert_eq!((red, green, blue), (128, 64, 0));
/// ```
pub struct RgbColor {
    /// The `red` part of the RGB color, represented by an integer
    /// between 0 and 255, where 0 is no red and 255 is fully red.
    pub red: u8,
    /// The `green` part of the RGB color, represented by an integer
    /// between 0 and 255, where 0 is no green and 255 is fully green.
    pub green: u8,
    /// The `blue` part of the RGB color, represented by an integer
    /// between 0 and 255, where 0 is no blue and 255 is fully blue.
    pub blue: u8,
}

impl From<ClarionColor> for RgbColor {
    /// Convert an `ClarionColor` value into a `RgbColor` value.
    ///
    /// # Examples
    ///
    /// ```
    /// let c_color = clarion::ClarionColor::new(4227327).unwrap();
    /// let color = clarion::RgbColor::from(c_color);
    /// let expected_color = clarion::RgbColor {red: 255, green: 128, blue: 64};
    /// assert_eq!(color, expected_color);
    /// ```
    fn from(color: ClarionColor) -> Self {
        let color = color.color();
        let blue = (color / 65536) as u8;
        let green = (color % 65536 / 256) as u8;
        let red = (color % 256) as u8;
        RgbColor { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClarionColor, RgbColor};
    #[test]
    fn from_clarion_white() {
        let c_color = ClarionColor::new(16777215).expect("An invalid color was specified.");
        let rgb_color = RgbColor::from(c_color);
        assert_eq!(
            rgb_color,
            RgbColor {
                red: 255,
                green: 255,
                blue: 255
            }
        );
    }

    #[test]
    fn from_clarion_olive() {
        let c_color = ClarionColor::new(32896).expect("An invalid color was specified.");
        let rgb_color = RgbColor::from(c_color);
        assert_eq!(
            rgb_color,
            RgbColor {
                red: 128,
                green: 128,
                blue: 0
            }
        );
    }

    #[test]
    fn from_clarion_yellow() {
        let c_color = ClarionColor::new(65535).expect("An invalid color was specified.");
        let rgb_color = RgbColor::from(c_color);
        assert_eq!(
            rgb_color,
            RgbColor {
                red: 255,
                green: 255,
                blue: 0
            }
        );
    }
}
