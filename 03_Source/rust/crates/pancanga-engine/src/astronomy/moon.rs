//! Stable Lunar Engine API.
//!
//! This module exposes the public interface that other subsystems should use
//! when they need lunar quantities. The catalogued algorithms remain available
//! for audit and testing, but downstream code should prefer this module.

use crate::astronomy::algorithms::{
    apparent_lunar_longitude, apply_elp2000_frame_precession, geocentric_lunar_longitude,
    longitude_periodic_correction, lunar_mean_longitude_w1, nutation_in_longitude,
};
use crate::core::{Degrees, JulianDate};

/// Returns the apparent ecliptic longitude of the Moon.
///
/// This is the stable Lunar Engine entry point for consumers such as the
/// future elongation and tithi engines.
///
/// Internally it composes the validated lunar longitude pipeline:
///
/// ```text
/// W1(t)
///     → periodic longitude correction
///     → geocentric lunar longitude
///     → ELP2000 frame precession
///     → nutation in longitude
///     → apparent lunar longitude
/// ```
///
/// The result is normalized to `[0°, 360°)`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::moon;
/// use pancanga_engine::core::JulianDate;
///
/// let longitude = moon::apparent_longitude(JulianDate::new(2_451_545.0));
///
/// assert!(longitude.value() >= 0.0);
/// assert!(longitude.value() < 360.0);
/// ```
pub fn apparent_longitude(jd: JulianDate) -> Degrees {
    let w1 = lunar_mean_longitude_w1(jd);
    let periodic_correction = longitude_periodic_correction(jd);
    let geocentric_longitude = geocentric_lunar_longitude(w1, periodic_correction);
    let mean_of_date_longitude = apply_elp2000_frame_precession(geocentric_longitude, jd);
    let nutation = nutation_in_longitude(jd);

    apparent_lunar_longitude(mean_of_date_longitude, nutation)
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::{
        apparent_lunar_longitude, apply_elp2000_frame_precession, geocentric_lunar_longitude,
        longitude_periodic_correction, lunar_mean_longitude_w1, nutation_in_longitude,
    };
    use crate::astronomy::moon;
    use crate::core::math::approximately_equal;
    use crate::core::JulianDate;

    #[test]
    fn returns_normalized_apparent_longitude() {
        let longitude = moon::apparent_longitude(JulianDate::new(2_451_545.0));

        assert!(longitude.value() >= 0.0);
        assert!(longitude.value() < 360.0);
    }

    #[test]
    fn matches_manual_lunar_pipeline_composition() {
        let jd = JulianDate::new(2_451_545.0);

        let w1 = lunar_mean_longitude_w1(jd);
        let periodic_correction = longitude_periodic_correction(jd);
        let geocentric_longitude = geocentric_lunar_longitude(w1, periodic_correction);
        let mean_of_date_longitude = apply_elp2000_frame_precession(geocentric_longitude, jd);
        let nutation = nutation_in_longitude(jd);
        let expected = apparent_lunar_longitude(mean_of_date_longitude, nutation);

        let actual = moon::apparent_longitude(jd);

        assert!(approximately_equal(actual.value(), expected.value()));
    }

    #[test]
    fn applies_elp2000_frame_precession_before_nutation() {
        let jd = JulianDate::new(2_488_070.0);
        let longitude = moon::apparent_longitude(jd);

        assert!(approximately_equal(
            longitude.value(),
            164.412_873_007_571_05
        ));
    }
}
