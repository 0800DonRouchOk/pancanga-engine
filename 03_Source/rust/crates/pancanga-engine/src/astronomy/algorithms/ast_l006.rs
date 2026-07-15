//! AST-L006: ELP2000 frame transformation.
//!
//! This module applies the scalar ELP2000 precession needed to express the
//! lunar longitude in the same date frame as the apparent solar longitude used
//! by the calendar engine.

use crate::core::math::normalize_360;
use crate::core::{Degrees, JulianDate};

use super::julian_centuries;

const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;
const ELP2000_PRECESSION_ARCSECONDS_PER_CENTURY: f64 = 5_029.096_6;

/// Returns the ELP2000 scalar frame precession for `jd`.
///
/// The value is positive after J2000 and negative before J2000.
///
/// ```text
/// precession = 5029.0966" * t
/// t = (JD - 2451545.0) / 36525
/// ```
pub fn elp2000_frame_precession(jd: JulianDate) -> Degrees {
    Degrees::new(
        ELP2000_PRECESSION_ARCSECONDS_PER_CENTURY / ARCSECONDS_PER_DEGREE * julian_centuries(jd),
    )
}

/// Applies the ELP2000 scalar frame precession to a lunar longitude.
///
/// This converts the scalar ELP2000 longitude used by the lunar pipeline into
/// the mean-of-date frame before nutation is applied.
pub fn apply_elp2000_frame_precession(lunar_longitude: Degrees, jd: JulianDate) -> Degrees {
    normalize_360(Degrees::new(
        lunar_longitude.value() + elp2000_frame_precession(jd).value(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::{apply_elp2000_frame_precession, elp2000_frame_precession};
    use crate::core::{Degrees, JulianDate};

    const TOLERANCE_DEGREES: f64 = 1.0e-12;

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
    fn precession_is_zero_at_j2000() {
        assert_degrees(elp2000_frame_precession(JulianDate::new(2_451_545.0)), 0.0);
    }

    #[test]
    fn precession_matches_elp2000_constant_one_century_after_j2000() {
        assert_degrees(
            elp2000_frame_precession(JulianDate::new(2_488_070.0)),
            1.396_971_277_777_777_7,
        );
    }

    #[test]
    fn precession_is_negative_before_j2000() {
        assert_degrees(
            elp2000_frame_precession(JulianDate::new(2_415_020.0)),
            -1.396_971_277_777_777_7,
        );
    }

    #[test]
    fn applies_frame_precession_and_normalizes() {
        let longitude =
            apply_elp2000_frame_precession(Degrees::new(359.5), JulianDate::new(2_488_070.0));

        assert_degrees(longitude, 0.896_971_277_777_777_7);
    }
}
