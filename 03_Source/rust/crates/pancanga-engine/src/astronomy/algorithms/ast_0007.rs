//! AST-0007: Apparent solar longitude.
//!
//! This module integrates the already isolated solar-pipeline corrections. It
//! does not introduce new astronomical formulae.

use crate::core::math::normalize_360;
use crate::core::Degrees;

/// Returns the apparent ecliptic longitude of the Sun.
///
/// This integration step combines three physical inputs:
///
/// ```text
/// λ☉ = normalize_360(λg + aberration + Δψ)
/// ```
///
/// where:
///
/// - `λg` is the geocentric solar longitude;
/// - `aberration` is the solar aberration correction;
/// - `Δψ` is nutation in longitude.
///
/// This function does not calculate any of those inputs. They must be supplied
/// by the preceding algorithms in the solar pipeline.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::apparent_solar_longitude;
/// use pancanga_engine::core::Degrees;
///
/// let longitude = apparent_solar_longitude(
///     Degrees::new(100.0),
///     Degrees::new(-0.005),
///     Degrees::new(-0.004),
/// );
///
/// assert_eq!(longitude.value(), 99.991);
/// ```
pub fn apparent_solar_longitude(
    geocentric_solar_longitude: Degrees,
    solar_aberration: Degrees,
    nutation_in_longitude: Degrees,
) -> Degrees {
    normalize_360(Degrees::new(
        geocentric_solar_longitude.value()
            + solar_aberration.value()
            + nutation_in_longitude.value(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::apparent_solar_longitude;
    use crate::core::math::approximately_equal;
    use crate::core::Degrees;

    fn assert_degrees(actual: Degrees, expected: f64) {
        assert!(
            approximately_equal(actual.value(), expected),
            "expected {}°, got {}°",
            expected,
            actual.value(),
        );
    }

    #[test]
    fn combines_geocentric_longitude_aberration_and_nutation() {
        let longitude = apparent_solar_longitude(
            Degrees::new(100.0),
            Degrees::new(-0.005),
            Degrees::new(-0.004),
        );

        assert_degrees(longitude, 99.991);
    }

    #[test]
    fn preserves_zero_corrections() {
        let longitude =
            apparent_solar_longitude(Degrees::new(123.456), Degrees::new(0.0), Degrees::new(0.0));

        assert_degrees(longitude, 123.456);
    }

    #[test]
    fn normalizes_positive_overflow() {
        let longitude =
            apparent_solar_longitude(Degrees::new(359.9), Degrees::new(0.2), Degrees::new(0.1));

        assert_degrees(longitude, 0.2);
    }

    #[test]
    fn normalizes_negative_result() {
        let longitude =
            apparent_solar_longitude(Degrees::new(0.1), Degrees::new(-0.2), Degrees::new(-0.1));

        assert_degrees(longitude, 359.8);
    }

    #[test]
    fn accepts_signed_nutation_and_aberration_independently() {
        let longitude =
            apparent_solar_longitude(Degrees::new(250.0), Degrees::new(-0.01), Degrees::new(0.02));

        assert_degrees(longitude, 250.01);
    }
}
