#![warn(missing_docs)]
//! A very simple library for working with CSTime time/date formats.
//!
//! CSTime dates are specified by the integral number of days between a given
//! date and the 28th of December, 1800.
//!
//! CSTime times are specified by the number of centiseconds between a given
//! time and midnight. (A centisecond is 10 milliseconds.)
use time::{macros::date, Date, Duration, Time};

/// The maximum allowed number of days for a valid CSDate.
pub const MIN_DAYS: i32 = -4309857;

/// The minimum allowed number of days for a valid CSDate.
pub const MAX_DAYS: i32 = 2994626;

/// The CSTime date epoch, represented by the 28th of December, 1800.
pub const CSTIME_EPOCH: Date = date!(1800 - 12 - 28);

/// Defines a moment in time in the CSTime time format, the number of between
/// the time and midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSTime {
    time: i32,
}

/// Defines a calendar date in the CSTime date format - the number of days
/// between the date and the 28th of December, 1800.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSDate {
    date: i32,
}

impl CSDate {
    /// Creates a new `CSDate` with a specified number of days between the date
    /// and the 28th of December, 1800. The input number of days must be
    /// between `cstime::MIN_DAYS` and `cstime::MAX_DAYS`.
    ///
    /// Will return `Err` if the specified input date is out of range.
    pub fn new(date: i32) -> Result<Self, String> {
        if date > MAX_DAYS || date < MIN_DAYS {
            return Err("Date was out of range".to_string());
        }
        Ok(CSDate { date })
    }

    /// Get the integral value representing this CSTime date value.
    ///
    /// Specified as the number of days between this date and the 28th of
    /// December, 1800.
    pub fn date(&self) -> i32 {
        self.date
    }
}

impl CSTime {
    /// Creates a new `CSTime` with a specified number of centiseconds between
    /// the time and midnight.
    pub fn new(time: i32) -> Self {
        CSTime { time }
    }

    /// Get the integral value representing this CSTime time value.
    ///
    /// Specified as the number of centiseconds between this time and midnight.
    pub fn time(&self) -> i32 {
        self.time
    }
}

impl From<time::Date> for CSDate {
    /// Convert a `time::Date` value into a CSTime date value.
    ///
    /// # Examples
    /// ```
    /// use cstime::CSDate;
    /// use time::macros::date;
    /// let date = CSDate::from(date!(2022-01-05));
    /// assert_eq!(date.date(), 80727);
    /// ```
    fn from(date: time::Date) -> Self {
        CSDate {
            date: (date - CSTIME_EPOCH).whole_days() as i32,
        }
    }
}

impl From<CSDate> for time::Date {
    /// Convert a `CSDate` into a `time::Date` value.
    ///
    /// # Examples
    /// ```
    /// use cstime::CSDate;
    /// use time::macros::date;
    /// let date = time::Date::from(CSDate::new(80727).unwrap());
    /// assert_eq!(date!(2022-01-05), date);
    /// ```
    fn from(csdate: CSDate) -> Self {
        CSTIME_EPOCH
            .checked_add(Duration::days(csdate.date as i64))
            .unwrap()
    }
}

impl From<time::Time> for CSTime {
    /// Convert a `time::Time` value into a CSTime time value.
    ///
    /// # Examples
    /// ```
    /// use cstime::CSTime;
    /// use time::macros::time;
    /// let time = CSTime::from(time!(16:34:00));
    /// assert_eq!(time.time(), 5964000);
    /// ```
    fn from(time: time::Time) -> Self {
        CSTime {
            time: ((time - Time::MIDNIGHT).whole_milliseconds() / 10) as i32,
        }
    }
}

impl From<CSTime> for time::Time {
    /// Convert a `CSTime` time value into a `time::Time` value.
    ///
    /// # Examples
    /// ```
    /// use cstime::CSTime;
    /// use time::macros::time;
    /// let time = time::Time::from(CSTime::new(5964000));
    /// assert_eq!(time, time!(16:34:00));
    /// ```
    fn from(cstime: CSTime) -> Self {
        let time = Time::MIDNIGHT;
        time + Duration::milliseconds(cstime.time as i64 * 10)
    }
}

#[cfg(test)]
mod tests {
    use time::{macros::date, macros::time};

    use crate::*;

    #[test]
    fn date_i32_max() {
        let date = CSDate::new(i32::MAX);
        assert!(date.is_err());
    }

    #[test]
    fn date_i32_min() {
        let date = CSDate::new(i32::MIN);
        assert!(date.is_err());
    }

    #[test]
    fn csdate_to_date_zero() {
        let date = CSDate::new(0).unwrap();
        let result: Date = date.into();
        assert_eq!(result, date!(1800 - 12 - 28));
    }

    #[test]
    fn csdate_to_date_one() {
        let date = CSDate::new(1).unwrap();
        let result: Date = date.into();
        assert_eq!(result, date!(1800 - 12 - 29));
    }

    #[test]
    fn csdate_to_date_negative_one() {
        let date = CSDate::new(-1).unwrap();
        let result: Date = date.into();
        assert_eq!(result, date!(1800 - 12 - 27));
    }

    #[test]
    fn date_to_csdate_date_min() {
        let date = Date::MIN;
        let result: CSDate = date.into();
        assert_eq!(result.date(), -4309857);
    }

    #[test]
    fn date_to_csdate_date_max() {
        let date = Date::MAX;
        let result: CSDate = date.into();
        assert_eq!(result.date(), 2994626);
    }

    #[test]
    fn date_to_csdate_millennium() {
        let date = date!(2000 - 01 - 01);
        let result: CSDate = date.into();
        assert_eq!(result.date(), 72687);
    }

    #[test]
    fn date_to_csdate_one() {
        let date = date!(0000 - 01 - 02);
        let result: CSDate = date.into();
        assert_eq!(result.date(), -657797);
    }

    #[test]
    fn date_to_csdate_zero() {
        let date = date!(0000 - 01 - 01);
        let result: CSDate = date.into();
        assert_eq!(result.date(), -657798);
    }

    #[test]
    fn date_to_csdate_negative_one() {
        let date = date!(-0001 - 12 - 31);
        let result: CSDate = date.into();
        assert_eq!(result.date(), -657799);
    }

    #[test]
    fn date_to_csdate_reversibility() {
        let date = date!(2020 - 06 - 30);
        let result: CSDate = date.into();
        let date2: Date = result.into();
        assert_eq!(date, date2);
    }

    #[test]
    fn cstime_to_time_i32_max() {
        let time = CSTime::new(i32::MAX);
        let result: Time = time.into();
        assert_eq!(result, time!(13:13:56.47));
    }

    #[test]
    fn cstime_to_time_i32_min() {
        let time = CSTime::new(i32::MIN);
        let result: Time = time.into();
        assert_eq!(result, time!(10:46:03.52));
    }

    #[test]
    fn cstime_to_time_zero() {
        let time = CSTime::new(0);
        let result: Time = time.into();
        assert_eq!(result, time!(00:00:00));
    }

    #[test]
    fn cstime_to_time_one() {
        let time = CSTime::new(1);
        let result: Time = time.into();
        assert_eq!(result, time!(00:00:00.01));
    }

    #[test]
    fn cstime_to_time_negative_one() {
        let time = CSTime::new(-1);
        let result: Time = time.into();
        assert_eq!(result, time!(23:59:59.99));
    }

    #[test]
    fn cstime_to_time_reversibility() {
        let time = time!(17:30:43);
        let result: CSTime = time.into();
        let time2 = result.into();
        assert_eq!(time, time2);
    }
}
