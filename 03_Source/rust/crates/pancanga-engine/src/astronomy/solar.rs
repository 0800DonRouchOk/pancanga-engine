//! Stable Solar Engine API.
//!
//! This module exposes the public interface that other subsystems should use
//! when they need solar quantities. The catalogued algorithms remain available
//! for audit and testing, but downstream code should prefer this module.

use crate::astronomy::algorithms::{
    apparent_solar_longitude, earth_heliocentric_longitude, earth_radius_vector,
    geocentric_solar_longitude, nutation_in_longitude, solar_aberration_correction,
};
use crate::core::{Degrees, JulianDate};

/// Returns the apparent ecliptic longitude of the Sun.
///
/// This is the stable Solar Engine v1.0 entry point for consumers such as the
/// future lunar and tithi engines.
///
/// Internally it composes the frozen Solar Pipeline v1.0:
///
/// ```text
/// Earth heliocentric longitude
///     → geocentric solar longitude
///     → Earth radius vector
///     → solar aberration
///     → nutation in longitude
///     → apparent solar longitude
/// ```
///
/// The result is normalized to `[0°, 360°)`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::solar;
/// use pancanga_engine::core::JulianDate;
///
/// let longitude = solar::apparent_longitude(JulianDate::new(2_451_545.0));
///
/// assert!(longitude.value() >= 0.0);
/// assert!(longitude.value() < 360.0);
/// ```
pub fn apparent_longitude(jd: JulianDate) -> Degrees {
    let earth_longitude = earth_heliocentric_longitude(jd);
    let geocentric_longitude = geocentric_solar_longitude(earth_longitude);
    let radius_vector = earth_radius_vector(jd);
    let aberration = solar_aberration_correction(radius_vector);
    let nutation = nutation_in_longitude(jd);

    apparent_solar_longitude(geocentric_longitude, aberration, nutation)
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::{
        apparent_solar_longitude, earth_heliocentric_longitude, earth_radius_vector,
        geocentric_solar_longitude, nutation_in_longitude, solar_aberration_correction,
    };
    use crate::astronomy::solar;
    use crate::core::math::approximately_equal;
    use crate::core::JulianDate;

    #[test]
    fn returns_normalized_apparent_longitude() {
        let longitude = solar::apparent_longitude(JulianDate::new(2_451_545.0));

        assert!(longitude.value() >= 0.0);
        assert!(longitude.value() < 360.0);
    }

    #[test]
    fn matches_manual_solar_pipeline_composition() {
        let jd = JulianDate::new(2_451_545.0);

        let earth_longitude = earth_heliocentric_longitude(jd);
        let geocentric_longitude = geocentric_solar_longitude(earth_longitude);
        let radius_vector = earth_radius_vector(jd);
        let aberration = solar_aberration_correction(radius_vector);
        let nutation = nutation_in_longitude(jd);
        let expected = apparent_solar_longitude(geocentric_longitude, aberration, nutation);

        let actual = solar::apparent_longitude(jd);

        assert!(approximately_equal(actual.value(), expected.value()));
    }
}
