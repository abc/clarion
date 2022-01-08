#![warn(missing_docs)]
use time::Duration;
use time::Time;

/// Defines a moment in time in the CSTime time format, the number of between
/// the time and midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CSTime {
    /// The number of centiseconds between this time and midnight.
    pub time: i32,
}

impl CSTime {
    /// Creates a new `CSTime` with a specified number of centiseconds between
    /// the time and midnight.
    /// 
    /// # Examples
    /// ```
    /// let cs_time = cstime::CSTime::new(5964000);
    /// ```
    pub fn new(time: i32) -> Self {
        CSTime { time }
    }
}

impl From<time::Time> for CSTime {
    /// Convert a `time::Time` value into a CSTime time value.
    ///
    /// # Examples
    /// Using `from()`
    /// ```
    /// use cstime::CSTime;
    /// use time::macros::time;
    /// let time = CSTime::from(time!(16:34:00));
    /// assert_eq!(time.time, 5964000);
    /// ```
    /// Using `into()`
    /// ```
    /// let cs_time = cstime::CSTime::new(5964000);
    /// let time: time::Time = cs_time.into();
    /// assert_eq!(time, time::macros::time!(16:34:00))
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
    use time::{macros::time, Time};

    use crate::cstime::CSTime;

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
