//! AST-0001: Julian Centuries from J2000.0.
//!
//! Reference:
//! - Jean Meeus, *Astronomical Algorithms*, 2nd Edition, Chapter 25.

use crate::core::JulianDate;

const J2000_JULIAN_DAY: f64 = 2_451_545.0;
const DAYS_PER_JULIAN_CENTURY: f64 = 36_525.0;

/// Returns Julian centuries elapsed since J2000.0.
///
/// This is AST-0001, the first official astronomical algorithm in Pancanga
/// Engine.
///
/// The formula is:
///
/// ```text
/// T = (JD - 2451545.0) / 36525.0
/// ```
///
/// where `JD` is the Julian Day and `T` is the number of Julian centuries since
/// J2000.0.
///
/// # Reference
///
/// Jean Meeus, *Astronomical Algorithms*, 2nd Edition, Chapter 25.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::julian_centuries;
/// use pancanga_engine::core::math::approximately_equal;
/// use pancanga_engine::core::JulianDate;
///
/// assert!(approximately_equal(
///     julian_centuries(JulianDate::new(2_451_545.0)),
///     0.0,
/// ));
/// ```
pub fn julian_centuries(jd: JulianDate) -> f64 {
    (jd.value() - J2000_JULIAN_DAY) / DAYS_PER_JULIAN_CENTURY
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::julian_centuries;
    use crate::core::math::approximately_equal;
    use crate::core::JulianDate;

    #[test]
    fn returns_zero_at_j2000() {
        let t = julian_centuries(JulianDate::new(2_451_545.0));

        assert!(approximately_equal(t, 0.0));
    }

    #[test]
    fn returns_one_century_after_j2000() {
        let t = julian_centuries(JulianDate::new(2_488_070.0));

        assert!(approximately_equal(t, 1.0));
    }

    #[test]
    fn returns_one_century_before_j2000() {
        let t = julian_centuries(JulianDate::new(2_415_020.0));

        assert!(approximately_equal(t, -1.0));
    }

    #[test]
    fn returns_half_century_after_j2000() {
        let t = julian_centuries(JulianDate::new(2_469_807.5));

        assert!(approximately_equal(t, 0.5));
    }

    #[test]
    fn preserves_fractional_julian_days() {
        let t = julian_centuries(JulianDate::new(2_451_545.25));

        assert!(approximately_equal(t, 0.25 / 36_525.0));
    }

    #[test]
    fn is_linear_over_equal_intervals() {
        let earlier = julian_centuries(JulianDate::new(2_451_545.0));
        let later = julian_centuries(JulianDate::new(2_451_555.0));

        assert!(approximately_equal(later - earlier, 10.0 / 36_525.0));
    }
}
