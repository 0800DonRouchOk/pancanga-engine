//! AST-0003: Geocentric solar longitude.
//!
//! This algorithm converts Earth's heliocentric longitude into the Sun's
//! geocentric longitude by reversing the line of sight.

use crate::core::math::normalize_360;
use crate::core::Degrees;

const OPPOSITION_DEGREES: f64 = 180.0;

/// Returns the geocentric longitude of the Sun.
///
/// If `earth_heliocentric_longitude` is Earth's heliocentric longitude, the
/// geocentric longitude of the Sun is the opposite direction on the ecliptic:
///
/// ```text
/// Lg = normalize_360(L + 180°)
/// ```
///
/// The result is normalized to `0.0..360.0` degrees.
///
/// This function does not apply aberration, nutation, or apparent-place
/// corrections.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::geocentric_solar_longitude;
/// use pancanga_engine::core::Degrees;
///
/// let longitude = geocentric_solar_longitude(Degrees::new(90.0));
///
/// assert_eq!(longitude.value(), 270.0);
/// ```
pub fn geocentric_solar_longitude(earth_heliocentric_longitude: Degrees) -> Degrees {
    normalize_360(Degrees::new(
        earth_heliocentric_longitude.value() + OPPOSITION_DEGREES,
    ))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::geocentric_solar_longitude;
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
    fn reverses_zero_degrees_to_opposite_point() {
        assert_degrees(geocentric_solar_longitude(Degrees::new(0.0)), 180.0);
    }

    #[test]
    fn reverses_quadrant_angles() {
        assert_degrees(geocentric_solar_longitude(Degrees::new(90.0)), 270.0);
        assert_degrees(geocentric_solar_longitude(Degrees::new(270.0)), 90.0);
    }

    #[test]
    fn normalizes_full_turn_boundary() {
        assert_degrees(geocentric_solar_longitude(Degrees::new(180.0)), 0.0);
        assert_degrees(geocentric_solar_longitude(Degrees::new(540.0)), 0.0);
    }

    #[test]
    fn normalizes_near_end_of_circle() {
        assert_degrees(geocentric_solar_longitude(Degrees::new(359.0)), 179.0);
    }

    #[test]
    fn normalizes_negative_inputs() {
        assert_degrees(geocentric_solar_longitude(Degrees::new(-10.0)), 170.0);
    }
}
