#![warn(missing_docs)]
//! A very simple library for working with CSTime time/date formats.
//!
//! CSTime dates are specified by the integral number of days between a given
//! date and the 28th of December, 1800.
//!
//! CSTime times are specified by the number of centiseconds between a given
//! time and midnight. (A centisecond is 10 milliseconds.)

use time::{macros::date, Date, Duration, Time};

/// The CSTime date epoch, represented by the 28th of December, 1800.
pub const CSTIME_EPOCH: Date = date!(1800 - 12 - 28);

/// Convert an integral CSTime date value into a `Date` struct.
///
/// Add the number of days specified by `cstime_date` to the CSTime epoch to
/// get the date represented by the `cstime_date`.
///
/// # Examples
/// ```
/// use cstime::cstime_to_date;
/// use time::macros::date;
/// let date = cstime_to_date(80727).unwrap();
/// assert_eq!(date, date!(2022-01-05));
/// ```
pub fn cstime_to_date(cstime_date: i32) -> Option<Date> {
    CSTIME_EPOCH.checked_add(Duration::days(cstime_date as i64))
}

/// Convert a `Date` struct into an integral CSTime date value.
///
/// Subtract the date specified by `date` from the CSTime epoch to get
/// the number of days.
///
/// # Examples
/// ```
/// use cstime::date_to_cstime;
/// use time::macros::date;
/// let date = date!(2021-10-07);
/// assert_eq!(date_to_cstime(date), 80637);
/// ```
pub fn date_to_cstime(date: Date) -> i32 {
    (date - CSTIME_EPOCH).whole_days() as i32
}

/// Convert a `Time` struct into an integral CSTime time value.
///
/// Subtract the time specified by `time` from midnight to get the number of
/// centiseconds passed since midnight.
///
/// # Examples
/// ```
/// use cstime::time_to_cstime;
/// use time::macros::time;
/// let time = time!(16:01:54.21);
/// assert_eq!(time_to_cstime(time), 5771421);
/// ```
pub fn time_to_cstime(time: Time) -> i32 {
    ((time - Time::MIDNIGHT).whole_milliseconds() / 10) as i32
}

/// Convert an integral CSTime time value into a `Time` struct.
///
/// Add the number of centiseconds specified by `cstime_time` to midnight.
///
/// # Examples
/// ```
/// use cstime::cstime_to_time;
/// use time::macros::time;
/// assert_eq!(cstime_to_time(5964000), time!(16:34:00));
/// ```
pub fn cstime_to_time(cstime_time: i32) -> Time {
    let time = Time::MIDNIGHT;
    time + Duration::milliseconds(cstime_time as i64 * 10)
}

#[cfg(test)]
mod tests {
    use time::{macros::date, macros::time};

    use crate::*;

    #[test]
    fn cstime_to_date_i32_max() {
        let result = cstime_to_date(i32::MAX);
        assert_eq!(result, None);
    }

    #[test]
    fn cstime_to_date_i32_min() {
        let result = cstime_to_date(i32::MIN);
        assert_eq!(result, None);
    }

    #[test]
    fn cstime_to_date_zero() {
        let result = cstime_to_date(0).unwrap();
        assert_eq!(result, date!(1800 - 12 - 28));
    }

    #[test]
    fn cstime_to_date_one() {
        let result = cstime_to_date(1).unwrap();
        assert_eq!(result, date!(1800 - 12 - 29));
    }

    #[test]
    fn cstime_to_date_negative_one() {
        let result = cstime_to_date(-1).unwrap();
        assert_eq!(result, date!(1800 - 12 - 27));
    }

    #[test]
    fn date_to_cstime_date_min() {
        let date = Date::MIN;
        let result = date_to_cstime(date);
        assert_eq!(result, -4309857);
    }

    #[test]
    fn date_to_cstime_date_max() {
        let date = Date::MAX;
        let result = date_to_cstime(date);
        assert_eq!(result, 2994626);
    }

    #[test]
    fn date_to_cstime_millennium() {
        let date = date!(2000 - 01 - 01);
        let result = date_to_cstime(date);
        assert_eq!(result, 72687);
    }

    #[test]
    fn date_to_cstime_one() {
        let date = date!(0000 - 01 - 02);
        let result = date_to_cstime(date);
        assert_eq!(result, -657797);
    }

    #[test]
    fn date_to_cstime_zero() {
        let date = date!(0000 - 01 - 01);
        let result = date_to_cstime(date);
        assert_eq!(result, -657798);
    }

    #[test]
    fn date_to_cstime_negative_one() {
        let date = date!(-0001 - 12 - 31);
        let result = date_to_cstime(date);
        assert_eq!(result, -657799);
    }

    #[test]
    fn date_to_cstime_reversibility() {
        let date = date!(2020 - 06 - 30);
        let result = date_to_cstime(date);
        let date2 = cstime_to_date(result).unwrap();
        assert_eq!(date, date2);
    }

    #[test]
    fn time_to_cstime_i32_max() {
        let result = cstime_to_time(i32::MAX);
        assert_eq!(result, time!(13:13:56.47));
    }

    #[test]
    fn time_to_cstime_i32_min() {
        let result = cstime_to_time(i32::MIN);
        assert_eq!(result, time!(10:46:03.52));
    }

    #[test]
    fn time_to_cstime_zero() {
        let result = cstime_to_time(0);
        assert_eq!(result, time!(00:00:00));
    }

    #[test]
    fn time_to_cstime_one() {
        let result = cstime_to_time(1);
        assert_eq!(result, time!(00:00:00.01));
    }

    #[test]
    fn time_to_cstime_negative_one() {
        let result = cstime_to_time(-1);
        assert_eq!(result, time!(23:59:59.99));
    }

    #[test]
    fn time_to_cstime_reversibility() {
        let time = time!(17:30:43);
        let result = time_to_cstime(time);
        let time2 = cstime_to_time(result);
        assert_eq!(time, time2);
    }
}
