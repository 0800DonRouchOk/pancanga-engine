//! Instant representation.

use crate::core::time::TimeScale;
use crate::core::types::JulianDate;

/// Represents a point in time as a Julian date plus a time scale.
///
/// `Instant` does not convert between time scales. It only records which scale
/// the contained [`JulianDate`] belongs to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instant {
    julian_date: JulianDate,
    time_scale: TimeScale,
}

impl Instant {
    /// Creates an instant from a Julian date and a time scale.
    pub fn new(julian_date: JulianDate, time_scale: TimeScale) -> Self {
        Self {
            julian_date,
            time_scale,
        }
    }

    /// Returns the Julian date.
    pub fn julian_date(self) -> JulianDate {
        self.julian_date
    }

    /// Returns the associated time scale.
    pub fn time_scale(self) -> TimeScale {
        self.time_scale
    }
}

#[cfg(test)]
mod tests {
    use crate::core::time::{Instant, TimeScale};
    use crate::core::types::JulianDate;

    #[test]
    fn creates_instant() {
        let julian_date = JulianDate::new(2451545.0);

        let instant = Instant::new(julian_date, TimeScale::Tt);

        assert_eq!(instant.julian_date(), julian_date);
        assert_eq!(instant.time_scale(), TimeScale::Tt);
    }
}
