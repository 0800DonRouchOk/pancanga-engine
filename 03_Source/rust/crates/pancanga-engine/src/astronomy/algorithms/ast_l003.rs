//! AST-L003: Lunar mean longitude W1.
//!
//! Reference:
//! - Chapront-Touze & Chapront, ELP2000-82B.
//!
//! This module calculates only the ELP2000 mean lunar longitude `W1(t)`.
//! It does not add periodic longitude corrections, nutation, apparent
//! longitude, tithi, or calendar rules.

use crate::core::math::normalize_360;
use crate::core::{Degrees, JulianDate};

use super::julian_centuries;

const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;

/// Returns the ELP2000 mean lunar longitude `W1(t)`.
///
/// The result is normalized to the interval `[0°, 360°)`.
///
/// This is the mean lunar longitude term used later with `AST-L002`:
///
/// ```text
/// lambda = W1 + Delta lambda periodic
/// ```
///
/// This function returns only `W1`; it does not add the periodic correction.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::lunar_mean_longitude_w1;
/// use pancanga_engine::core::JulianDate;
///
/// let w1 = lunar_mean_longitude_w1(JulianDate::new(2_451_545.0));
///
/// assert!(w1.value() >= 0.0);
/// assert!(w1.value() < 360.0);
/// ```
pub fn lunar_mean_longitude_w1(jd: JulianDate) -> Degrees {
    let t = julian_centuries(jd);

    normalize_360(Degrees::new(w1_degrees(t)))
}

fn w1_degrees(t: f64) -> f64 {
    218.0
        + 18.0 / 60.0
        + 59.955_71 / ARCSECONDS_PER_DEGREE
        + 1_732_559_343.736_04 / ARCSECONDS_PER_DEGREE * t
        - 5.888_3 / ARCSECONDS_PER_DEGREE * t.powi(2)
        + 0.660_4e-2 / ARCSECONDS_PER_DEGREE * t.powi(3)
        - 0.316_9e-4 / ARCSECONDS_PER_DEGREE * t.powi(4)
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::lunar_mean_longitude_w1;
    use crate::core::{Degrees, JulianDate};

    const TOLERANCE_DEGREES: f64 = 1.0e-9;

    fn assert_degrees(actual: Degrees, expected: f64) {
        let difference = (actual.value() - expected).abs();

        assert!(
            difference <= TOLERANCE_DEGREES,
            "expected {}°, got {}°, difference {}°",
            expected,
            actual.value(),
            difference,
        );
    }

    #[test]
    fn returns_elp2000_w1_at_j2000() {
        let w1 = lunar_mean_longitude_w1(JulianDate::new(2_451_545.0));

        assert_degrees(w1, 218.316_654_363_888_9);
    }

    #[test]
    fn returns_elp2000_w1_one_century_after_j2000() {
        let w1 = lunar_mean_longitude_w1(JulianDate::new(2_488_070.0));

        assert_degrees(w1, 164.799_391_672_946_5);
    }

    #[test]
    fn returns_elp2000_w1_one_century_before_j2000() {
        let w1 = lunar_mean_longitude_w1(JulianDate::new(2_415_020.0));

        assert_degrees(w1, 271.830_645_759_473_55);
    }

    #[test]
    fn returns_normalized_longitude() {
        let w1 = lunar_mean_longitude_w1(JulianDate::new(2_488_070.0));

        assert!(w1.value() >= 0.0);
        assert!(w1.value() < 360.0);
    }
}
