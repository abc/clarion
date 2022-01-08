#![warn(missing_docs)]
use time::Duration;
use time::Time;

/// Defines a moment in time in the Clarion time format, the number of
/// centiseconds between the time and midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ClarionTime {
    /// The number of centiseconds between this time and midnight.
    time: i32,
}

impl ClarionTime {
    /// Creates a new `ClarionTime` with a specified number of centiseconds between
    /// the time and midnight.
    ///
    /// # Examples
    /// ```
    /// let c_time = clarion::ClarionTime::new(5964000);
    /// ```
    pub fn new(time: i32) -> Self {
        // The time cannot exceed the total number of centiseconds in 24 hours.
        let time = time % 8640000;
        ClarionTime { time }
    }

    /// Get the integral value representing this ClarionTime value.
    ///
    /// Specified as the number of centiseconds between this time and midnight.
    ///
    /// # Examples
    /// ```
    /// let date = time::macros::date!(2020-06-30);
    /// let c_date = clarion::ClarionDate::from(date);
    /// assert_eq!(c_date.date(), 80173);
    /// ```
    pub fn time(&self) -> i32 {
        self.time
    }
}

impl From<time::Time> for ClarionTime {
    /// Convert a `time::Time` value into a ClarionTime time value.
    ///
    /// # Examples
    /// Using `from()`
    /// ```
    /// use clarion::ClarionTime;
    /// use time::macros::time;
    /// let time = ClarionTime::from(time!(16:34:00));
    /// assert_eq!(time.time(), 5964000);
    /// ```
    /// Using `into()`
    /// ```
    /// let c_time = clarion::ClarionTime::new(5964000);
    /// let time: time::Time = c_time.into();
    /// assert_eq!(time, time::macros::time!(16:34:00))
    /// ```
    fn from(time: time::Time) -> Self {
        ClarionTime {
            time: ((time - Time::MIDNIGHT).whole_milliseconds() / 10) as i32,
        }
    }
}

impl From<ClarionTime> for time::Time {
    /// Convert a `ClarionTime` time value into a `time::Time` value.
    ///
    /// # Examples
    /// ```
    /// use clarion::ClarionTime;
    /// use time::macros::time;
    /// let time = time::Time::from(ClarionTime::new(5964000));
    /// assert_eq!(time, time!(16:34:00));
    /// ```
    fn from(value: ClarionTime) -> Self {
        let time = Time::MIDNIGHT;
        time + Duration::milliseconds(value.time as i64 * 10)
    }
}

#[cfg(test)]
mod tests {
    use time::{macros::time, Time};

    use crate::ClarionTime;

    #[test]
    fn ctime_to_time_i32_max() {
        let time = ClarionTime::new(i32::MAX);
        let result: Time = time.into();
        assert_eq!(result, time!(13:13:56.47));
    }

    #[test]
    fn ctime_to_time_i32_min() {
        let time = ClarionTime::new(i32::MIN);
        let result: Time = time.into();
        assert_eq!(result, time!(10:46:03.52));
    }

    #[test]
    fn ctime_to_time_zero() {
        let time = ClarionTime::new(0);
        let result: Time = time.into();
        assert_eq!(result, time!(00:00:00));
    }

    #[test]
    fn ctime_to_time_one() {
        let time = ClarionTime::new(1);
        let result: Time = time.into();
        assert_eq!(result, time!(00:00:00.01));
    }

    #[test]
    fn ctime_to_time_negative_one() {
        let time = ClarionTime::new(-1);
        let result: Time = time.into();
        assert_eq!(result, time!(23:59:59.99));
    }

    #[test]
    fn ctime_to_time_reversibility() {
        let time = time!(17:30:43);
        let result: ClarionTime = time.into();
        let time2 = result.into();
        assert_eq!(time, time2);
    }
}
