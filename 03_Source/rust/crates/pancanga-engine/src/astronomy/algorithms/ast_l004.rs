//! AST-L004: Geocentric lunar longitude.
//!
//! This module combines the ELP2000 mean lunar longitude `W1` with the
//! periodic longitude correction `Delta lambda`.
//!
//! It does not calculate nutation, apparent longitude, latitude, distance,
//! tithi, or calendar rules.

use crate::core::math::normalize_360;
use crate::core::{ArcSeconds, Degrees};

const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;

/// Returns the geocentric lunar longitude.
///
/// The result is normalized to the interval `[0°, 360°)`.
///
/// ```text
/// lambda_geo = W1 + Delta lambda periodic / 3600
/// ```
///
/// This function only combines two already computed physical magnitudes. It
/// does not calculate `W1`, does not evaluate ELP2000 terms, and does not apply
/// nutation or apparent corrections.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::geocentric_lunar_longitude;
/// use pancanga_engine::core::{ArcSeconds, Degrees};
///
/// let longitude = geocentric_lunar_longitude(
///     Degrees::new(218.3166543638889),
///     ArcSeconds::new(18_008.30055426643),
/// );
///
/// assert!(longitude.value() >= 0.0);
/// assert!(longitude.value() < 360.0);
/// ```
pub fn geocentric_lunar_longitude(w1: Degrees, delta_lambda_periodic: ArcSeconds) -> Degrees {
    normalize_360(Degrees::new(
        w1.value() + delta_lambda_periodic.value() / ARCSECONDS_PER_DEGREE,
    ))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::geocentric_lunar_longitude;
    use crate::core::{ArcSeconds, Degrees};

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
    fn combines_w1_and_periodic_correction_at_j2000() {
        let longitude = geocentric_lunar_longitude(
            Degrees::new(218.316_654_363_888_9),
            ArcSeconds::new(18_008.300_554_266_43),
        );

        assert_degrees(longitude, 223.318_960_073_407_34);
    }

    #[test]
    fn combines_w1_and_periodic_correction_one_century_after_j2000() {
        let longitude = geocentric_lunar_longitude(
            Degrees::new(164.799_391_672_946_5),
            ArcSeconds::new(-6_423.827_265_758_044),
        );

        assert_degrees(longitude, 163.014_995_210_235_95);
    }

    #[test]
    fn normalizes_positive_overflow() {
        let longitude = geocentric_lunar_longitude(Degrees::new(359.0), ArcSeconds::new(7_200.0));

        assert_degrees(longitude, 1.0);
    }

    #[test]
    fn normalizes_negative_result() {
        let longitude = geocentric_lunar_longitude(Degrees::new(1.0), ArcSeconds::new(-7_200.0));

        assert_degrees(longitude, 359.0);
    }
}
