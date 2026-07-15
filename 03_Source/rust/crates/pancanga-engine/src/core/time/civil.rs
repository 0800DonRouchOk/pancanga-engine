//! Civil date and time representations.

use crate::core::time::TimeScale;

/// Represents a civil calendar date.
///
/// The date is validated as a proleptic Gregorian date. No astronomical or
/// liturgical calendar logic is implemented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CivilDate {
    year: i32,
    month: u8,
    day: u8,
}

impl CivilDate {
    /// Creates a civil date when the year, month, and day form a valid
    /// proleptic Gregorian date.
    pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
        if !(1..=12).contains(&month) {
            return None;
        }

        let max_day = days_in_month(year, month);

        if (1..=max_day).contains(&day) {
            Some(Self { year, month, day })
        } else {
            None
        }
    }

    /// Returns the year.
    pub fn year(self) -> i32 {
        self.year
    }

    /// Returns the month in the range `1..=12`.
    pub fn month(self) -> u8 {
        self.month
    }

    /// Returns the day of month.
    pub fn day(self) -> u8 {
        self.day
    }
}

/// Represents a civil time of day.
///
/// Seconds may contain a fractional part. Leap seconds are not represented in
/// M3.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CivilTime {
    hour: u8,
    minute: u8,
    second: f64,
}

impl CivilTime {
    /// Creates a civil time when the components are in range.
    ///
    /// Valid values are:
    ///
    /// - hour: `0..=23`
    /// - minute: `0..=59`
    /// - second: `0.0 <= second < 60.0`
    pub fn new(hour: u8, minute: u8, second: f64) -> Option<Self> {
        if hour <= 23 && minute <= 59 && second.is_finite() && (0.0..60.0).contains(&second) {
            Some(Self {
                hour,
                minute,
                second,
            })
        } else {
            None
        }
    }

    /// Returns the hour in the range `0..=23`.
    pub fn hour(self) -> u8 {
        self.hour
    }

    /// Returns the minute in the range `0..=59`.
    pub fn minute(self) -> u8 {
        self.minute
    }

    /// Returns the second in the range `0.0 <= second < 60.0`.
    pub fn second(self) -> f64 {
        self.second
    }
}

/// Represents a civil date and time associated with a time scale.
///
/// This type only groups validated civil components. It does not convert to or
/// from Julian dates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CivilDateTime {
    date: CivilDate,
    time: CivilTime,
    time_scale: TimeScale,
}

impl CivilDateTime {
    /// Creates a civil date-time from validated components and a time scale.
    pub fn new(date: CivilDate, time: CivilTime, time_scale: TimeScale) -> Self {
        Self {
            date,
            time,
            time_scale,
        }
    }

    /// Returns the civil date.
    pub fn date(self) -> CivilDate {
        self.date
    }

    /// Returns the civil time.
    pub fn time(self) -> CivilTime {
        self.time
    }

    /// Returns the associated time scale.
    pub fn time_scale(self) -> TimeScale {
        self.time_scale
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use crate::core::time::{CivilDate, CivilDateTime, CivilTime, TimeScale};

    #[test]
    fn creates_valid_civil_date() {
        let date = CivilDate::new(2026, 7, 11).expect("date should be valid");

        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 11);
    }

    #[test]
    fn rejects_invalid_civil_date() {
        assert_eq!(CivilDate::new(2026, 13, 1), None);
        assert_eq!(CivilDate::new(2026, 2, 29), None);
    }

    #[test]
    fn accepts_leap_day_in_leap_year() {
        let date = CivilDate::new(2024, 2, 29).expect("date should be valid");

        assert_eq!(date.day(), 29);
    }

    #[test]
    fn creates_valid_civil_time() {
        let time = CivilTime::new(23, 59, 59.5).expect("time should be valid");

        assert_eq!(time.hour(), 23);
        assert_eq!(time.minute(), 59);
        assert_eq!(time.second(), 59.5);
    }

    #[test]
    fn rejects_invalid_civil_time() {
        assert_eq!(CivilTime::new(24, 0, 0.0), None);
        assert_eq!(CivilTime::new(0, 60, 0.0), None);
        assert_eq!(CivilTime::new(0, 0, 60.0), None);
        assert_eq!(CivilTime::new(0, 0, f64::NAN), None);
    }

    #[test]
    fn creates_civil_date_time() {
        let date = CivilDate::new(2026, 7, 11).expect("date should be valid");
        let time = CivilTime::new(12, 30, 0.0).expect("time should be valid");

        let date_time = CivilDateTime::new(date, time, TimeScale::Utc);

        assert_eq!(date_time.date(), date);
        assert_eq!(date_time.time(), time);
        assert_eq!(date_time.time_scale(), TimeScale::Utc);
    }
}
