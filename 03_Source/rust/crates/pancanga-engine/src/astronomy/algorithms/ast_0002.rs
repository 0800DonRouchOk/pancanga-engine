//! AST-0002: Earth heliocentric longitude from VSOP87D.
//!
//! Reference:
//! - Bretagnon, P.; Francou, G. "Planetary theories in rectangular and
//!   spherical variables: VSOP87 solution", *Astronomy and Astrophysics*,
//!   202, 309, 1988.
//! - IMCCE / Observatoire de Paris, `VSOP87D.ear`.
//! - Jean Meeus, *Astronomical Algorithms*, 2nd Edition.

use crate::core::math::{normalize_radians, rad_to_deg};
use crate::core::{Degrees, JulianDate, Radians};

use super::vsop87d_earth_longitude::{Vsop87Term, LONGITUDE_SERIES};

const J2000_JULIAN_DAY: f64 = 2_451_545.0;
const DAYS_PER_JULIAN_MILLENNIUM: f64 = 365_250.0;

/// Returns the heliocentric longitude of Earth using VSOP87D.
///
/// This is the first implementation stage of AST-0002. It evaluates only
/// variable 1 (`L`) from the official `VSOP87D.ear` file:
///
/// ```text
/// L(tau) = Σ tau^n * Σ A cos(B + C tau)
/// ```
///
/// where `tau` is measured in thousands of Julian years from J2000.0:
///
/// ```text
/// tau = (JD - 2451545.0) / 365250.0
/// ```
///
/// The returned longitude is normalized to `0.0..360.0` degrees. The input
/// Julian Day is expected to be expressed on the same dynamical time basis used
/// by VSOP87, normally TDB for high-precision work.
/// The IMCCE coefficient file already expresses angular amplitudes in radians,
/// so no `1e8` scale factor is applied here.
///
/// This function does not compute the apparent longitude of the Sun. That is a
/// later algorithm in the solar chain.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::earth_heliocentric_longitude;
/// use pancanga_engine::core::JulianDate;
///
/// let longitude = earth_heliocentric_longitude(JulianDate::new(2_451_545.0));
///
/// assert!(longitude.value() >= 0.0);
/// assert!(longitude.value() < 360.0);
/// ```
pub fn earth_heliocentric_longitude(jd: JulianDate) -> Degrees {
    rad_to_deg(earth_heliocentric_longitude_radians(jd))
}

fn earth_heliocentric_longitude_radians(jd: JulianDate) -> Radians {
    normalize_radians(Radians::new(evaluate_longitude(vsop87_tau(jd))))
}

fn vsop87_tau(jd: JulianDate) -> f64 {
    (jd.value() - J2000_JULIAN_DAY) / DAYS_PER_JULIAN_MILLENNIUM
}

fn evaluate_longitude(tau: f64) -> f64 {
    let mut longitude = 0.0;
    let mut tau_power = 1.0;

    for terms in LONGITUDE_SERIES {
        longitude += tau_power * evaluate_series(terms, tau);
        tau_power *= tau;
    }

    longitude
}

fn evaluate_series(terms: &[Vsop87Term], tau: f64) -> f64 {
    terms
        .iter()
        .map(|term| term.amplitude * (term.phase + term.frequency * tau).cos())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::earth_heliocentric_longitude;
    use crate::core::math::deg_to_rad;
    use crate::core::{Degrees, JulianDate};

    use super::{earth_heliocentric_longitude_radians, vsop87_tau};

    // `vsop87.chk` prints longitude to 10 decimal places in radians. The
    // tolerance is intentionally looser than the last printed digit so the test
    // validates the implementation without pretending the reference table
    // carries more precision than it publishes.
    const IMCCE_CHECK_TOLERANCE_RAD: f64 = 1.0e-9;
    const IMCCE_EXPECTED_MAX_ERROR_RAD: f64 = 5.0e-11;
    const IMCCE_CHECK_CASES: &[(f64, f64)] = &[
        (2_451_545.0, 1.751_923_868_1),
        (2_415_020.0, 1.739_122_556_3),
        (2_378_495.0, 1.726_263_891_6),
        (2_341_970.0, 1.713_441_910_5),
        (2_305_445.0, 1.700_606_593_8),
        (2_268_920.0, 1.687_762_496_0),
        (2_232_395.0, 1.675_011_096_1),
        (2_195_870.0, 1.662_204_865_7),
        (2_159_345.0, 1.649_514_319_7),
        (2_122_820.0, 1.636_719_362_3),
    ];

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        let difference = (actual - expected).abs();

        assert!(
            difference <= tolerance,
            "expected {actual} to be within {tolerance} of {expected}; difference {difference}",
        );
    }

    #[test]
    fn calculates_vsop87_time_in_julian_millennia() {
        assert_close(vsop87_tau(JulianDate::new(2_451_545.0)), 0.0, 1.0e-15);
        assert_close(vsop87_tau(JulianDate::new(2_816_795.0)), 1.0, 1.0e-15);
        assert_close(vsop87_tau(JulianDate::new(2_086_295.0)), -1.0, 1.0e-15);
    }

    #[test]
    fn matches_imcce_vsop87d_earth_longitude_check_values() {
        for &(jd, expected) in IMCCE_CHECK_CASES {
            let longitude = earth_heliocentric_longitude_radians(JulianDate::new(jd));

            assert_close(longitude.value(), expected, IMCCE_CHECK_TOLERANCE_RAD);
        }
    }

    #[test]
    fn maximum_error_against_imcce_check_values_stays_below_reference_rounding() {
        let max_error = IMCCE_CHECK_CASES
            .iter()
            .map(|&(jd, expected)| {
                let longitude = earth_heliocentric_longitude_radians(JulianDate::new(jd));
                (longitude.value() - expected).abs()
            })
            .fold(0.0, f64::max);

        assert!(
            max_error <= IMCCE_EXPECTED_MAX_ERROR_RAD,
            "maximum error {max_error} exceeded {IMCCE_EXPECTED_MAX_ERROR_RAD}",
        );
    }

    #[test]
    fn public_api_returns_normalized_degrees() {
        let longitude = earth_heliocentric_longitude(JulianDate::new(2_451_545.0));

        assert!(longitude.value() >= 0.0);
        assert!(longitude.value() < 360.0);
    }

    #[test]
    fn public_degrees_match_internal_radians() {
        let jd = JulianDate::new(2_451_545.0);
        let public_longitude = earth_heliocentric_longitude(jd);
        let internal_longitude = earth_heliocentric_longitude_radians(jd);

        assert_close(
            deg_to_rad(Degrees::new(public_longitude.value())).value(),
            internal_longitude.value(),
            1.0e-12,
        );
    }
}
