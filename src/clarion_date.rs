//! The [`ClarionDate`] struct and associated `impl`s.

use time::{macros::date, Date, Duration};

use crate::ClarionErr;

/// The Clarion date epoch, represented by the 28th of December, 1800.
pub const CLARION_EPOCH: Date = date!(1800 - 12 - 28);

/// Defines a calendar date in the ClarionDate format - the number of days
/// between the date and the 28th of December, 1800.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ClarionDate {
    /// The number of days between this date and the 28th of December, 1800.
    date: i32,
}

impl ClarionDate {
    /// Creates a new `ClarionDate` with a specified number of days between the date
    /// and the 28th of December, 1800.
    ///
    /// # Examples
    /// ```
    /// let c_date = clarion::ClarionDate::new(80727);
    /// ```
    pub fn new(date: i32) -> Self {
        ClarionDate { date }
    }

    /// Get the integral value representing this ClarionDate value.
    ///
    /// Specified as the number of days between this date and the 28th of
    /// December, 1800.
    /// # Examples
    /// ```
    /// let c_date = clarion::ClarionDate::new(80727);
    /// assert_eq!(c_date.date(), 80727);
    /// ```
    pub fn date(&self) -> i32 {
        self.date
    }
}

impl From<Date> for ClarionDate {
    /// Convert a `time::Date` value into a ClarionDate value.
    ///
    /// # Examples
    /// Using `from()`
    /// ```
    /// let date = time::macros::date!(2022-01-05);
    /// let c_date = clarion::ClarionDate::from(date);
    /// assert_eq!(c_date.date(), 80727);
    /// ```
    /// Using `into()`:
    /// ```
    /// let date = time::macros::date!(2022-01-05);
    /// let c_date:clarion::ClarionDate = date.into();
    /// assert_eq!(c_date.date(), 80727);
    /// ```
    fn from(date: Date) -> Self {
        ClarionDate {
            date: (date - CLARION_EPOCH).whole_days() as i32,
        }
    }
}

impl TryFrom<ClarionDate> for Date {
    type Error = ClarionErr;
    /// Convert a `ClarionDate` into a `time::Date` value.
    ///
    /// # Examples
    /// Using `try_from()`
    /// ```
    /// use time::Date;
    /// use clarion::{ClarionDate, ClarionErr};
    /// let c_date = ClarionDate::new(80727);
    /// let date: Result<Date, ClarionErr> = Date::try_from(c_date);
    /// assert_eq!(date, Ok(time::macros::date!(2022-01-05)));
    /// ```
    /// Using `try_into()`
    /// ```
    /// use clarion::ClarionDate;
    /// let c_date = ClarionDate::new(80727);
    /// let date: time::Date = c_date.try_into().unwrap();
    /// let cmp_date = time::macros::date!(2022-01-05);
    /// assert_eq!(date, cmp_date);
    /// ```
    /// Using `try_from()` with `expect()` clause.
    /// ```
    /// let c_date = clarion::ClarionDate::new(80727);
    /// let date = time::Date::try_from(c_date)
    ///     .expect("The input value produce a valid date between Date::MAX and Date::MIN.");
    /// assert_eq!(date, time::macros::date!(2022-01-05));
    /// ```
    fn try_from(value: ClarionDate) -> Result<Self, Self::Error> {
        let date = CLARION_EPOCH.checked_add(Duration::days(value.date as i64));
        match date {
            None => Err(ClarionErr::ConversionOverflowed),
            Some(date) => Ok(date),
        }
    }
}

#[cfg(test)]
mod tests {
    use time::{macros::date, Date};

    use crate::{ClarionDate, ClarionErr};

    #[test]
    fn date_i32_max() {
        let c_date = ClarionDate { date: i32::MAX };
        let date: Result<Date, ClarionErr> = c_date.try_into();
        assert_eq!(date, Err(ClarionErr::ConversionOverflowed));
    }

    #[test]
    fn date_i32_min() {
        let c_date = ClarionDate::new(i32::MIN);
        let date: Result<Date, ClarionErr> = c_date.try_into();
        assert_eq!(date, Err(ClarionErr::ConversionOverflowed));
    }

    #[test]
    fn cdate_to_date_zero() {
        let date = ClarionDate::new(0);
        let result: Date = date.try_into().unwrap();
        assert_eq!(result, date!(1800 - 12 - 28));
    }

    #[test]
    fn cdate_to_date_one() {
        let date = ClarionDate::new(1);
        let result: Date = date.try_into().unwrap();
        assert_eq!(result, date!(1800 - 12 - 29));
    }

    #[test]
    fn cdate_to_date_negative_one() {
        let date = ClarionDate::new(-1);
        let result: Date = date.try_into().unwrap();
        assert_eq!(result, date!(1800 - 12 - 27));
    }

    #[test]
    fn date_to_cdate_date_min() {
        let date = Date::MIN;
        let result: ClarionDate = date.into();
        assert_eq!(result.date(), -4309857);
    }

    #[test]
    fn date_to_cdate_date_max() {
        let date = Date::MAX;
        let result: ClarionDate = date.into();
        assert_eq!(result.date(), 2994626);
    }

    #[test]
    fn date_to_cdate_millennium() {
        let date = date!(2000 - 01 - 01);
        let result: ClarionDate = date.into();
        assert_eq!(result.date(), 72687);
    }

    #[test]
    fn date_to_cdate_one() {
        let date = date!(0000 - 01 - 02);
        let result: ClarionDate = date.into();
        assert_eq!(result.date(), -657797);
    }

    #[test]
    fn date_to_cdate_zero() {
        let date = date!(0000 - 01 - 01);
        let result: ClarionDate = date.into();
        assert_eq!(result.date(), -657798);
    }

    #[test]
    fn date_to_cdate_negative_one() {
        let date = date!(-0001 - 12 - 31);
        let result: ClarionDate = date.into();
        assert_eq!(result.date(), -657799);
    }

    #[test]
    fn date_to_cdate_reversibility() {
        let date = date!(2020 - 06 - 30);
        let result: ClarionDate = date.into();
        let date2: Date = result.try_into().unwrap();
        assert_eq!(date, date2);
    }
}
