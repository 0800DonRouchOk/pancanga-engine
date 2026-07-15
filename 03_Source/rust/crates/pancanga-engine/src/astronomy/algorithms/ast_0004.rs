//! AST-0004: Earth radius vector from VSOP87D.
//!
//! Reference:
//! - Bretagnon, P.; Francou, G. "Planetary theories in rectangular and
//!   spherical variables: VSOP87 solution", *Astronomy and Astrophysics*,
//!   202, 309, 1988.
//! - IMCCE / Observatoire de Paris, `VSOP87D.ear`.

use crate::core::{AstronomicalUnits, JulianDate};

use super::vsop87d_earth_radius::{Vsop87Term, RADIUS_SERIES};

const J2000_JULIAN_DAY: f64 = 2_451_545.0;
const DAYS_PER_JULIAN_MILLENNIUM: f64 = 365_250.0;

/// Returns Earth's heliocentric radius vector using VSOP87D.
///
/// This is AST-0004 in the solar pipeline. It evaluates variable 3 (`R`) from
/// the official `VSOP87D.ear` file:
///
/// ```text
/// R(tau) = Σ tau^n * Σ A cos(B + C tau)
/// ```
///
/// where `tau` is measured in thousands of Julian years from J2000.0:
///
/// ```text
/// tau = (JD - 2451545.0) / 365250.0
/// ```
///
/// The returned value is measured in astronomical units. This function does not
/// calculate solar aberration; it only provides the physical radius vector that
/// later aberration algorithms will need.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::earth_radius_vector;
/// use pancanga_engine::core::JulianDate;
///
/// let radius = earth_radius_vector(JulianDate::new(2_451_545.0));
///
/// assert!(radius.value() > 0.0);
/// ```
pub fn earth_radius_vector(jd: JulianDate) -> AstronomicalUnits {
    AstronomicalUnits::new(evaluate_radius(vsop87_tau(jd)))
}

fn vsop87_tau(jd: JulianDate) -> f64 {
    (jd.value() - J2000_JULIAN_DAY) / DAYS_PER_JULIAN_MILLENNIUM
}

fn evaluate_radius(tau: f64) -> f64 {
    let mut radius = 0.0;
    let mut tau_power = 1.0;

    for terms in RADIUS_SERIES {
        radius += tau_power * evaluate_series(terms, tau);
        tau_power *= tau;
    }

    radius
}

fn evaluate_series(terms: &[Vsop87Term], tau: f64) -> f64 {
    terms
        .iter()
        .map(|term| term.amplitude * (term.phase + term.frequency * tau).cos())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::earth_radius_vector;
    use crate::core::JulianDate;

    use super::vsop87_tau;

    // `vsop87.chk` prints radius vector values to 10 decimal places in AU.
    // The tolerance validates the implementation against the printed reference
    // without claiming more precision than the table publishes.
    const IMCCE_CHECK_TOLERANCE_AU: f64 = 1.0e-9;
    const IMCCE_EXPECTED_MAX_ERROR_AU: f64 = 5.0e-11;
    const IMCCE_CHECK_CASES: &[(f64, f64)] = &[
        (2_451_545.0, 0.983_327_681_9),
        (2_415_020.0, 0.983_268_977_8),
        (2_378_495.0, 0.983_227_432_1),
        (2_341_970.0, 0.983_149_844_1),
        (2_305_445.0, 0.983_125_437_6),
        (2_268_920.0, 0.983_081_675_6),
        (2_232_395.0, 0.983_075_440_9),
        (2_195_870.0, 0.983_094_238_5),
        (2_159_345.0, 0.983_044_039_7),
        (2_122_820.0, 0.983_033_181_5),
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
    fn returns_positive_radius_vector() {
        let radius = earth_radius_vector(JulianDate::new(2_451_545.0));

        assert!(radius.value() > 0.0);
    }

    #[test]
    fn matches_imcce_vsop87d_earth_radius_check_values() {
        for &(jd, expected) in IMCCE_CHECK_CASES {
            let radius = earth_radius_vector(JulianDate::new(jd));

            assert_close(radius.value(), expected, IMCCE_CHECK_TOLERANCE_AU);
        }
    }

    #[test]
    fn maximum_error_against_imcce_check_values_stays_below_reference_rounding() {
        let max_error = IMCCE_CHECK_CASES
            .iter()
            .map(|&(jd, expected)| {
                let radius = earth_radius_vector(JulianDate::new(jd));
                (radius.value() - expected).abs()
            })
            .fold(0.0, f64::max);

        assert!(
            max_error <= IMCCE_EXPECTED_MAX_ERROR_AU,
            "maximum error {max_error} exceeded {IMCCE_EXPECTED_MAX_ERROR_AU}",
        );
    }

    #[test]
    fn j2000_radius_matches_reference_with_project_comparison() {
        let radius = earth_radius_vector(JulianDate::new(2_451_545.0));

        assert_close(radius.value(), 0.983_327_681_9, IMCCE_CHECK_TOLERANCE_AU);
    }
}
