//! AST-0006: Nutation in longitude.
//!
//! Reference:
//! - Jean Meeus, *Astronomical Algorithms*, 2nd Edition.
//!
//! This module implements only the truncated Meeus series for nutation in
//! longitude (`Δψ`). It does not calculate nutation in obliquity or apply the
//! result to a solar longitude.

use crate::core::math::{normalize_360, sin_deg};
use crate::core::{Degrees, JulianDate};

use super::julian_centuries;

const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;

/// Returns nutation in longitude using the truncated Meeus series.
///
/// The implemented formula is:
///
/// ```text
/// Δψ = -17.20 sin Ω
///      - 1.32 sin 2L
///      - 0.23 sin 2L'
///      + 0.21 sin 2Ω
/// ```
///
/// where:
///
/// - `L` is the mean longitude of the Sun;
/// - `L'` is the mean longitude of the Moon;
/// - `Ω` is the longitude of the Moon's ascending node.
///
/// The returned value is expressed in degrees and is not normalized. It is a
/// small signed correction intended to be added later to ecliptic longitude.
///
/// This function does not calculate obliquity or apparent solar longitude.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::nutation_in_longitude;
/// use pancanga_engine::core::JulianDate;
///
/// let delta_psi = nutation_in_longitude(JulianDate::new(2_451_545.0));
///
/// assert!(delta_psi.value().abs() < 0.01);
/// ```
pub fn nutation_in_longitude(jd: JulianDate) -> Degrees {
    let t = julian_centuries(jd);
    let arguments = NutationArguments::from_julian_centuries(t);
    let delta_psi_arcseconds = -17.20 * sin_deg(arguments.lunar_ascending_node)
        - 1.32 * sin_deg(arguments.double_mean_solar_longitude())
        - 0.23 * sin_deg(arguments.double_mean_lunar_longitude())
        + 0.21 * sin_deg(arguments.double_lunar_ascending_node());

    Degrees::new(delta_psi_arcseconds / ARCSECONDS_PER_DEGREE)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NutationArguments {
    mean_solar_longitude: Degrees,
    mean_lunar_longitude: Degrees,
    lunar_ascending_node: Degrees,
}

impl NutationArguments {
    fn from_julian_centuries(t: f64) -> Self {
        Self {
            mean_solar_longitude: mean_solar_longitude(t),
            mean_lunar_longitude: mean_lunar_longitude(t),
            lunar_ascending_node: lunar_ascending_node_longitude(t),
        }
    }

    fn double_mean_solar_longitude(self) -> Degrees {
        Degrees::new(2.0 * self.mean_solar_longitude.value())
    }

    fn double_mean_lunar_longitude(self) -> Degrees {
        Degrees::new(2.0 * self.mean_lunar_longitude.value())
    }

    fn double_lunar_ascending_node(self) -> Degrees {
        Degrees::new(2.0 * self.lunar_ascending_node.value())
    }
}

fn mean_solar_longitude(t: f64) -> Degrees {
    normalize_360(Degrees::new(280.4665 + 36_000.769_8 * t))
}

fn mean_lunar_longitude(t: f64) -> Degrees {
    normalize_360(Degrees::new(218.3165 + 481_267.881_3 * t))
}

fn lunar_ascending_node_longitude(t: f64) -> Degrees {
    normalize_360(Degrees::new(125.04452 - 1_934.136_261 * t))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::nutation_in_longitude;
    use crate::core::{Degrees, JulianDate};

    use super::{
        lunar_ascending_node_longitude, mean_lunar_longitude, mean_solar_longitude,
        NutationArguments,
    };

    const TOLERANCE_DEGREES: f64 = 1.0e-12;
    const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;

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

    fn assert_arcseconds(actual: Degrees, expected_arcseconds: f64) {
        let actual_arcseconds = actual.value() * ARCSECONDS_PER_DEGREE;
        let difference = (actual_arcseconds - expected_arcseconds).abs();

        assert!(
            difference <= 1.0e-9,
            "expected {expected_arcseconds}\" got {actual_arcseconds}\"; difference {difference}\"",
        );
    }

    #[test]
    fn calculates_mean_solar_longitude_argument_at_j2000() {
        assert_degrees(mean_solar_longitude(0.0), 280.4665);
    }

    #[test]
    fn calculates_mean_lunar_longitude_argument_at_j2000() {
        assert_degrees(mean_lunar_longitude(0.0), 218.3165);
    }

    #[test]
    fn calculates_lunar_ascending_node_argument_at_j2000() {
        assert_degrees(lunar_ascending_node_longitude(0.0), 125.04452);
    }

    #[test]
    fn normalizes_arguments_to_degrees() {
        let arguments = NutationArguments::from_julian_centuries(1.0);

        assert!(arguments.mean_solar_longitude.value() >= 0.0);
        assert!(arguments.mean_solar_longitude.value() < 360.0);
        assert!(arguments.mean_lunar_longitude.value() >= 0.0);
        assert!(arguments.mean_lunar_longitude.value() < 360.0);
        assert!(arguments.lunar_ascending_node.value() >= 0.0);
        assert!(arguments.lunar_ascending_node.value() < 360.0);
    }

    #[test]
    fn returns_truncated_meeus_value_at_j2000() {
        let delta_psi = nutation_in_longitude(JulianDate::new(2_451_545.0));

        assert_arcseconds(delta_psi, -14.031_356_821_395_894);
    }

    #[test]
    fn returns_truncated_meeus_value_for_chapter_22_example_date() {
        let delta_psi = nutation_in_longitude(JulianDate::new(2_446_895.5));

        assert_arcseconds(delta_psi, -3.862_757_835_101_797_3);
    }

    #[test]
    fn produces_signed_small_angular_correction() {
        let delta_psi = nutation_in_longitude(JulianDate::new(2_451_545.0));

        assert!(delta_psi.value() < 0.0);
        assert!(delta_psi.value().abs() < 0.01);
    }
}
