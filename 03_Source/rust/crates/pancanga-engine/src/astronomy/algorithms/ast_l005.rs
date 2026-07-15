//! AST-L005: Apparent lunar longitude.
//!
//! This module applies the already computed nutation in longitude to the
//! geocentric lunar longitude.
//!
//! It does not calculate nutation, latitude, distance, tithi, or calendar
//! rules.

use crate::core::math::normalize_360;
use crate::core::Degrees;

/// Returns the apparent lunar longitude.
///
/// The result is normalized to the interval `[0°, 360°)`.
///
/// ```text
/// lambda_apparent = lambda_geo + Delta psi
/// ```
///
/// This function does not calculate `Delta psi`. The nutation value must come
/// from `AST-0006` (`nutation_in_longitude`), so the lunar and solar pipelines
/// reuse the same nutation algorithm.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::apparent_lunar_longitude;
/// use pancanga_engine::core::Degrees;
///
/// let longitude = apparent_lunar_longitude(
///     Degrees::new(223.31896007340734),
///     Degrees::new(-0.003897599117054415),
/// );
///
/// assert!(longitude.value() >= 0.0);
/// assert!(longitude.value() < 360.0);
/// ```
pub fn apparent_lunar_longitude(
    geocentric_lunar_longitude: Degrees,
    nutation_in_longitude: Degrees,
) -> Degrees {
    normalize_360(Degrees::new(
        geocentric_lunar_longitude.value() + nutation_in_longitude.value(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::{apparent_lunar_longitude, nutation_in_longitude};
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
    fn applies_existing_nutation_to_j2000_geocentric_lunar_longitude() {
        let longitude = apparent_lunar_longitude(
            Degrees::new(223.318_960_073_407_34),
            nutation_in_longitude(JulianDate::new(2_451_545.0)),
        );

        assert_degrees(longitude, 223.315_062_474_290_27);
    }

    #[test]
    fn applies_existing_nutation_one_century_after_j2000() {
        let longitude = apparent_lunar_longitude(
            Degrees::new(163.014_995_210_235_95),
            nutation_in_longitude(JulianDate::new(2_488_070.0)),
        );

        assert_degrees(longitude, 163.015_901_729_793_28);
    }

    #[test]
    fn normalizes_positive_overflow() {
        let longitude = apparent_lunar_longitude(Degrees::new(359.9), Degrees::new(0.2));

        assert_degrees(longitude, 0.1);
    }

    #[test]
    fn normalizes_negative_result() {
        let longitude = apparent_lunar_longitude(Degrees::new(0.1), Degrees::new(-0.2));

        assert_degrees(longitude, 359.9);
    }
}
