#![warn(missing_docs)]
//! The [`CSDate`] struct and associated `impl`s.

use time::{macros::date, Date, Duration};

use crate::cserr::CSErr;

/// The CSTime date epoch, represented by the 28th of December, 1800.
pub const CSTIME_EPOCH: Date = date!(1800 - 12 - 28);

/// Defines a calendar date in the CSTime date format - the number of days
/// between the date and the 28th of December, 1800.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSDate {
    /// The number of days between this date and the 28th of December, 1800.
    pub date: i32,
}

impl CSDate {
    /// Creates a new `CSDate` with a specified number of days between the date
    /// and the 28th of December, 1800.
    /// 
    /// # Examples
    /// ```
    /// let cs_date = cstime::CSDate { date: 80727 };
    /// assert_eq!(cs_date.date, 80727);
    /// ```
    pub fn new(date: i32) -> Self {
        CSDate { date }
    }
}

impl From<Date> for CSDate {
    /// Convert a `time::Date` value into a CSTime date value.
    ///
    /// # Examples
    /// Using `from()`
    /// ```
    /// let date = time::macros::date!(2022-01-05);
    /// let cs_date = cstime::CSDate::from(date);
    /// assert_eq!(cs_date.date, 80727);
    /// ```
    /// Using `into()`:
    /// ```
    /// let date = time::macros::date!(2022-01-05);
    /// let cs_date:cstime::CSDate = date.into();
    /// assert_eq!(cs_date.date, 80727);
    /// ```
    fn from(date: Date) -> Self {
        CSDate {
            date: (date - CSTIME_EPOCH).whole_days() as i32,
        }
    }
}

impl TryFrom<CSDate> for Date {
    type Error = CSErr;
    /// Convert a `CSDate` into a `time::Date` value.
    ///
    /// # Examples
    /// Using `try_from()`
    /// ```
    /// use time::Date;
    /// use cstime::{CSDate, CSErr};
    /// let cs_date = CSDate::new(80727);
    /// let date: Result<Date, CSErr> = Date::try_from(cs_date);
    /// assert_eq!(date, Ok(time::macros::date!(2022-01-05)));
    /// ```
    /// Using `try_into()`
    /// ```
    /// use cstime::CSDate;
    /// let cs_date = CSDate::new(80727);
    /// let date: time::Date = cs_date.try_into().unwrap();
    /// let cmp_date = time::macros::date!(2022-01-05);
    /// assert_eq!(date, cmp_date);
    /// ```
    /// Using `try_from()` with `expect()` clause.
    /// ```
    /// let cs_date = cstime::CSDate::new(80727);
    /// let date = time::Date::try_from(cs_date)
    ///     .expect("The input value produce a valid date between Date::MAX and Date::MIN.");
    /// assert_eq!(date, time::macros::date!(2022-01-05));
    /// ```
    fn try_from(value: CSDate) -> Result<Self, Self::Error> {
        let date = CSTIME_EPOCH.checked_add(Duration::days(value.date as i64));
        match date {
            None => Err(CSErr::ConversionOverflowed),
            Some(date) => Ok(date),
        }
    }
}

#[cfg(test)]
mod tests {
    use time::{macros::date, Date};

    use crate::{csdate::CSDate, cserr::CSErr};

    #[test]
    fn date_i32_max() {
        let cs_date = CSDate { date: i32::MAX };
        let date: Result<Date, CSErr> = cs_date.try_into();
        assert_eq!(date, Err(CSErr::ConversionOverflowed));
    }

    #[test]
    fn date_i32_min() {
        let cs_date = CSDate::new(i32::MIN);
        let date: Result<Date, CSErr> = cs_date.try_into();
        assert_eq!(date, Err(CSErr::ConversionOverflowed));
    }

    #[test]
    fn csdate_to_date_zero() {
        let date = CSDate::new(0);
        let result: Date = date.try_into().unwrap();
        assert_eq!(result, date!(1800 - 12 - 28));
    }

    #[test]
    fn csdate_to_date_one() {
        let date = CSDate::new(1);
        let result: Date = date.try_into().unwrap();
        assert_eq!(result, date!(1800 - 12 - 29));
    }

    #[test]
    fn csdate_to_date_negative_one() {
        let date = CSDate::new(-1);
        let result: Date = date.try_into().unwrap();
        assert_eq!(result, date!(1800 - 12 - 27));
    }

    #[test]
    fn date_to_csdate_date_min() {
        let date = Date::MIN;
        let result: CSDate = date.into();
        assert_eq!(result.date, -4309857);
    }

    #[test]
    fn date_to_csdate_date_max() {
        let date = Date::MAX;
        let result: CSDate = date.into();
        assert_eq!(result.date, 2994626);
    }

    #[test]
    fn date_to_csdate_millennium() {
        let date = date!(2000 - 01 - 01);
        let result: CSDate = date.into();
        assert_eq!(result.date, 72687);
    }

    #[test]
    fn date_to_csdate_one() {
        let date = date!(0000 - 01 - 02);
        let result: CSDate = date.into();
        assert_eq!(result.date, -657797);
    }

    #[test]
    fn date_to_csdate_zero() {
        let date = date!(0000 - 01 - 01);
        let result: CSDate = date.into();
        assert_eq!(result.date, -657798);
    }

    #[test]
    fn date_to_csdate_negative_one() {
        let date = date!(-0001 - 12 - 31);
        let result: CSDate = date.into();
        assert_eq!(result.date, -657799);
    }

    #[test]
    fn date_to_csdate_reversibility() {
        let date = date!(2020 - 06 - 30);
        let result: CSDate = date.into();
        let date2: Date = result.try_into().unwrap();
        assert_eq!(date, date2);
    }
}
