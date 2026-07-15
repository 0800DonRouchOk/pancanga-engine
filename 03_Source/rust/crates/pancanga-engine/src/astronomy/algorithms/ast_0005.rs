//! AST-0005: Solar aberration.
//!
//! Reference:
//! - Jean Meeus, *Astronomical Algorithms*, 2nd Edition.
//!
//! This module applies only the solar aberration correction. It does not apply
//! nutation, obliquity, right ascension, or any calendar-specific rule.

use crate::core::math::normalize_360;
use crate::core::{AstronomicalUnits, Degrees};

const SOLAR_ABERRATION_ARCSECONDS: f64 = -20.4898;
const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;

/// Returns the solar aberration correction in ecliptic longitude.
///
/// Meeus gives the solar aberration correction as:
///
/// ```text
/// Δλ = -20.4898" / R
/// ```
///
/// where `R` is the Earth-Sun radius vector in astronomical units.
///
/// The returned value is expressed in degrees and is not normalized; it is a
/// small signed correction intended to be added to the geocentric solar
/// longitude.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::solar_aberration_correction;
/// use pancanga_engine::core::AstronomicalUnits;
///
/// let correction = solar_aberration_correction(AstronomicalUnits::new(1.0));
///
/// assert!(correction.value() < 0.0);
/// ```
pub fn solar_aberration_correction(earth_radius_vector: AstronomicalUnits) -> Degrees {
    Degrees::new(
        (SOLAR_ABERRATION_ARCSECONDS / earth_radius_vector.value()) / ARCSECONDS_PER_DEGREE,
    )
}

/// Applies the solar aberration correction to geocentric solar longitude.
///
/// This function performs only:
///
/// ```text
/// λ = normalize_360(λg + Δλ)
/// ```
///
/// It does not apply nutation. Nutation in longitude is intentionally reserved
/// for a later algorithm in the solar pipeline.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::apply_solar_aberration;
/// use pancanga_engine::core::{AstronomicalUnits, Degrees};
///
/// let longitude = apply_solar_aberration(
///     Degrees::new(100.0),
///     AstronomicalUnits::new(1.0),
/// );
///
/// assert!(longitude.value() < 100.0);
/// ```
pub fn apply_solar_aberration(
    geocentric_solar_longitude: Degrees,
    earth_radius_vector: AstronomicalUnits,
) -> Degrees {
    let correction = solar_aberration_correction(earth_radius_vector);

    normalize_360(Degrees::new(
        geocentric_solar_longitude.value() + correction.value(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::{apply_solar_aberration, solar_aberration_correction};
    use crate::core::math::approximately_equal;
    use crate::core::{AstronomicalUnits, Degrees};

    const ONE_AU_CORRECTION_DEGREES: f64 = -20.4898 / 3_600.0;

    fn assert_degrees(actual: Degrees, expected: f64) {
        assert!(
            approximately_equal(actual.value(), expected),
            "expected {}°, got {}°",
            expected,
            actual.value(),
        );
    }

    #[test]
    fn correction_at_one_au_matches_meeus_constant() {
        let correction = solar_aberration_correction(AstronomicalUnits::new(1.0));

        assert_degrees(correction, ONE_AU_CORRECTION_DEGREES);
    }

    #[test]
    fn correction_scales_with_radius_vector() {
        let correction_at_two_au = solar_aberration_correction(AstronomicalUnits::new(2.0));
        let correction_at_half_au = solar_aberration_correction(AstronomicalUnits::new(0.5));

        assert_degrees(correction_at_two_au, ONE_AU_CORRECTION_DEGREES / 2.0);
        assert_degrees(correction_at_half_au, ONE_AU_CORRECTION_DEGREES * 2.0);
    }

    #[test]
    fn applies_correction_to_geocentric_solar_longitude() {
        let longitude = apply_solar_aberration(Degrees::new(100.0), AstronomicalUnits::new(1.0));

        assert_degrees(longitude, 100.0 + ONE_AU_CORRECTION_DEGREES);
    }

    #[test]
    fn normalizes_after_applying_negative_correction() {
        let longitude = apply_solar_aberration(Degrees::new(0.0), AstronomicalUnits::new(1.0));

        assert_degrees(longitude, 360.0 + ONE_AU_CORRECTION_DEGREES);
    }

    #[test]
    fn does_not_apply_nutation_or_other_corrections() {
        let longitude = apply_solar_aberration(Degrees::new(180.0), AstronomicalUnits::new(1.0));

        assert_degrees(longitude, 180.0 + ONE_AU_CORRECTION_DEGREES);
    }
}
