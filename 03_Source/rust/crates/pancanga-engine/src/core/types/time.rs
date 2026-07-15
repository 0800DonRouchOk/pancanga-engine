//! Time value types.
//!
//! References:
//! - Jean Meeus, Astronomical Algorithms, 2nd Edition
//! - PES-0002 (Time System)

/// Represents a duration measured in days.
///
/// This type stores a raw day count without calendar interpretation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DurationDays {
    days: f64,
}

impl DurationDays {
    /// Creates a duration from a raw number of days.
    pub fn new(days: f64) -> Self {
        Self { days }
    }

    /// Returns the raw number of days.
    pub fn value(self) -> f64 {
        self.days
    }
}

/// Represents a Julian Day.
///
/// Internally all astronomical calculations operate on Julian Days.
///
/// Example:
///
/// J2000.0 = 2451545.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct JulianDate {
    jd: f64,
}

impl JulianDate {
    /// Creates a Julian date from a raw Julian Day value.
    pub fn new(jd: f64) -> Self {
        Self { jd }
    }

    /// Returns the raw Julian Day value.
    pub fn value(self) -> f64 {
        self.jd
    }

    /// Adds a duration in days.
    pub fn add_days(self, days: DurationDays) -> Self {
        Self {
            jd: self.jd + days.value(),
        }
    }

    /// Returns the duration between two Julian dates.
    pub fn days_since(self, other: Self) -> DurationDays {
        DurationDays::new(self.jd - other.jd)
    }
}

#[cfg(test)]
mod tests {
    use super::{DurationDays, JulianDate};

    #[test]
    fn creates_duration_days() {
        let duration = DurationDays::new(2.5);

        assert_eq!(duration.value(), 2.5);
    }

    #[test]
    fn creates_julian_date() {
        let jd = JulianDate::new(2451545.0);

        assert_eq!(jd.value(), 2451545.0);
    }

    #[test]
    fn adds_duration_to_julian_date() {
        let jd = JulianDate::new(2451545.0);

        let next = jd.add_days(DurationDays::new(1.0));

        assert_eq!(next.value(), 2451546.0);
    }

    #[test]
    fn calculates_duration_between_julian_dates() {
        let a = JulianDate::new(2451545.0);
        let b = JulianDate::new(2451547.5);

        assert_eq!(b.days_since(a), DurationDays::new(2.5));
    }
}
