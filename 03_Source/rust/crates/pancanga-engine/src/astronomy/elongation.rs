//! Lunar-solar elongation.
//!
//! This module contains derived astronomical magnitudes built from already
//! computed apparent longitudes. It does not contain calendar or Vaishnava
//! rules.

use crate::core::math::normalize_360;
use crate::core::Degrees;

/// Returns the angular elongation from the Sun to the Moon.
///
/// The elongation is the normalized difference between apparent lunar and
/// solar ecliptic longitudes:
///
/// ```text
/// elongation = normalize_360(lambda_moon - lambda_sun)
/// ```
///
/// The result is normalized to `[0°, 360°)`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::lunar_solar_elongation;
/// use pancanga_engine::core::Degrees;
///
/// let elongation = lunar_solar_elongation(Degrees::new(10.0), Degrees::new(350.0));
///
/// assert_eq!(elongation.value(), 20.0);
/// ```
pub fn lunar_solar_elongation(moon_longitude: Degrees, sun_longitude: Degrees) -> Degrees {
    normalize_360(Degrees::new(moon_longitude.value() - sun_longitude.value()))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::{lunar_solar_elongation, moon, solar};
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
    fn returns_zero_when_longitudes_match() {
        let elongation = lunar_solar_elongation(Degrees::new(42.0), Degrees::new(42.0));

        assert_degrees(elongation, 0.0);
    }

    #[test]
    fn calculates_forward_elongation() {
        let elongation = lunar_solar_elongation(Degrees::new(90.0), Degrees::new(30.0));

        assert_degrees(elongation, 60.0);
    }

    #[test]
    fn normalizes_across_zero_degrees() {
        let elongation = lunar_solar_elongation(Degrees::new(10.0), Degrees::new(350.0));

        assert_degrees(elongation, 20.0);
    }

    #[test]
    fn normalizes_negative_difference() {
        let elongation = lunar_solar_elongation(Degrees::new(350.0), Degrees::new(10.0));

        assert_degrees(elongation, 340.0);
    }

    #[test]
    fn calculates_j2000_elongation_from_public_solar_and_lunar_apis() {
        let jd = JulianDate::new(2_451_545.0);
        let elongation =
            lunar_solar_elongation(moon::apparent_longitude(jd), solar::apparent_longitude(jd));

        assert_degrees(elongation, 302.946_904_514_570_5);
    }

    #[test]
    fn calculates_representative_elongation_from_public_solar_and_lunar_apis() {
        let jd = JulianDate::new(2_488_070.0);
        let elongation =
            lunar_solar_elongation(moon::apparent_longitude(jd), solar::apparent_longitude(jd));

        assert_degrees(elongation, 243.299_989_618_885_18);
    }
}
