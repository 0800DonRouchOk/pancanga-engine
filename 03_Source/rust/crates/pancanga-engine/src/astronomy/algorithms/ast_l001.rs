//! AST-L001: Fundamental lunar arguments.
//!
//! Reference:
//! - Jean Meeus, *Astronomical Algorithms*, 2nd Edition, Chapter 47.
//!
//! This module calculates the fundamental lunar arguments used by the lunar
//! periodic terms. It does not evaluate any periodic series.

use crate::core::math::normalize_360;
use crate::core::{Degrees, JulianDate};

use super::julian_centuries;

/// Fundamental lunar arguments used by Meeus' lunar theory.
///
/// The values are normalized degrees in the interval `[0°, 360°)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FundamentalLunarArguments {
    mean_elongation: Degrees,
    solar_mean_anomaly: Degrees,
    lunar_mean_anomaly: Degrees,
    argument_of_latitude: Degrees,
}

impl FundamentalLunarArguments {
    /// Creates fundamental lunar arguments from explicit angular values.
    pub fn new(
        mean_elongation: Degrees,
        solar_mean_anomaly: Degrees,
        lunar_mean_anomaly: Degrees,
        argument_of_latitude: Degrees,
    ) -> Self {
        Self {
            mean_elongation,
            solar_mean_anomaly,
            lunar_mean_anomaly,
            argument_of_latitude,
        }
    }

    /// Returns the Moon's mean elongation from the Sun (`D`).
    pub fn mean_elongation(self) -> Degrees {
        self.mean_elongation
    }

    /// Returns the Sun's mean anomaly (`M`).
    pub fn solar_mean_anomaly(self) -> Degrees {
        self.solar_mean_anomaly
    }

    /// Returns the Moon's mean anomaly (`M'`).
    pub fn lunar_mean_anomaly(self) -> Degrees {
        self.lunar_mean_anomaly
    }

    /// Returns the Moon's argument of latitude (`F`).
    pub fn argument_of_latitude(self) -> Degrees {
        self.argument_of_latitude
    }
}

/// Returns the fundamental lunar arguments for a Julian Date.
///
/// Meeus defines the lunar periodic terms using four fundamental arguments:
///
/// ```text
/// D  = mean elongation of the Moon
/// M  = mean anomaly of the Sun
/// M' = mean anomaly of the Moon
/// F  = argument of latitude of the Moon
/// ```
///
/// This function returns only those arguments. It does not calculate lunar
/// longitude, latitude, distance, tithi, or any calendar rule.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::algorithms::fundamental_lunar_arguments;
/// use pancanga_engine::core::JulianDate;
///
/// let arguments = fundamental_lunar_arguments(JulianDate::new(2_451_545.0));
///
/// assert!(arguments.mean_elongation().value() >= 0.0);
/// assert!(arguments.mean_elongation().value() < 360.0);
/// ```
pub fn fundamental_lunar_arguments(jd: JulianDate) -> FundamentalLunarArguments {
    let t = julian_centuries(jd);

    FundamentalLunarArguments::new(
        mean_elongation(t),
        solar_mean_anomaly(t),
        lunar_mean_anomaly(t),
        argument_of_latitude(t),
    )
}

fn mean_elongation(t: f64) -> Degrees {
    normalize_360(Degrees::new(
        297.850_192_1 + 445_267.111_403_4 * t - 0.001_881_9 * t.powi(2) + t.powi(3) / 545_868.0
            - t.powi(4) / 113_065_000.0,
    ))
}

fn solar_mean_anomaly(t: f64) -> Degrees {
    normalize_360(Degrees::new(
        357.529_109_2 + 35_999.050_290_9 * t - 0.000_153_6 * t.powi(2) + t.powi(3) / 24_490_000.0,
    ))
}

fn lunar_mean_anomaly(t: f64) -> Degrees {
    normalize_360(Degrees::new(
        134.963_396_4 + 477_198.867_505_5 * t + 0.008_741_4 * t.powi(2) + t.powi(3) / 69_699.0
            - t.powi(4) / 14_712_000.0,
    ))
}

fn argument_of_latitude(t: f64) -> Degrees {
    normalize_360(Degrees::new(
        93.272_095 + 483_202.017_523_3 * t - 0.003_653_9 * t.powi(2) - t.powi(3) / 3_526_000.0
            + t.powi(4) / 863_310_000.0,
    ))
}

#[cfg(test)]
mod tests {
    use crate::astronomy::algorithms::{fundamental_lunar_arguments, FundamentalLunarArguments};
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
    fn creates_explicit_fundamental_lunar_arguments() {
        let arguments = FundamentalLunarArguments::new(
            Degrees::new(1.0),
            Degrees::new(2.0),
            Degrees::new(3.0),
            Degrees::new(4.0),
        );

        assert_eq!(arguments.mean_elongation().value(), 1.0);
        assert_eq!(arguments.solar_mean_anomaly().value(), 2.0);
        assert_eq!(arguments.lunar_mean_anomaly().value(), 3.0);
        assert_eq!(arguments.argument_of_latitude().value(), 4.0);
    }

    #[test]
    fn returns_j2000_arguments_from_meeus_polynomials() {
        let arguments = fundamental_lunar_arguments(JulianDate::new(2_451_545.0));

        assert_degrees(arguments.mean_elongation(), 297.850_192_1);
        assert_degrees(arguments.solar_mean_anomaly(), 357.529_109_2);
        assert_degrees(arguments.lunar_mean_anomaly(), 134.963_396_4);
        assert_degrees(arguments.argument_of_latitude(), 93.272_095);
    }

    #[test]
    fn returns_meeus_chapter_47_example_arguments() {
        let arguments = fundamental_lunar_arguments(JulianDate::new(2_448_724.5));

        assert_degrees(arguments.mean_elongation(), 113.842_303_716_635_8);
        assert_degrees(arguments.solar_mean_anomaly(), 97.643_513_609_622_46);
        assert_degrees(arguments.lunar_mean_anomaly(), 5.150_832_522_958_808);
        assert_degrees(arguments.argument_of_latitude(), 219.889_720_727_856_9);
    }

    #[test]
    fn normalizes_all_arguments() {
        let arguments = fundamental_lunar_arguments(JulianDate::new(2_488_070.0));

        assert!(arguments.mean_elongation().value() >= 0.0);
        assert!(arguments.mean_elongation().value() < 360.0);
        assert!(arguments.solar_mean_anomaly().value() >= 0.0);
        assert!(arguments.solar_mean_anomaly().value() < 360.0);
        assert!(arguments.lunar_mean_anomaly().value() >= 0.0);
        assert!(arguments.lunar_mean_anomaly().value() < 360.0);
        assert!(arguments.argument_of_latitude().value() >= 0.0);
        assert!(arguments.argument_of_latitude().value() < 360.0);
    }
}
