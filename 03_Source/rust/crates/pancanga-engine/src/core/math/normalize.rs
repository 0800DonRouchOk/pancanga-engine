//! Normalization helpers for angular and cyclic values.

use crate::core::types::{Degrees, Radians};

use super::constants::TAU;

/// Normalizes an angle in degrees to the range `0.0..360.0`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{
///     math::{approximately_equal, normalize_360},
///     Degrees,
/// };
///
/// assert!(approximately_equal(normalize_360(Degrees::new(370.0)).value(), 10.0));
/// assert!(approximately_equal(normalize_360(Degrees::new(-20.0)).value(), 340.0));
/// ```
pub fn normalize_360(angle: Degrees) -> Degrees {
    Degrees::new(angle.value().rem_euclid(360.0))
}

/// Normalizes an angle in degrees to the range `-180.0..180.0`.
///
/// The upper boundary is exclusive: `180.0` normalizes to `-180.0`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{
///     math::{approximately_equal, normalize_180},
///     Degrees,
/// };
///
/// assert!(approximately_equal(normalize_180(Degrees::new(270.0)).value(), -90.0));
/// ```
pub fn normalize_180(angle: Degrees) -> Degrees {
    let normalized = normalize_360(angle).value();

    if normalized >= 180.0 {
        Degrees::new(normalized - 360.0)
    } else {
        Degrees::new(normalized)
    }
}

/// Normalizes an angle in radians to the range `0.0..τ`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{
///     math::{approximately_equal, normalize_radians},
///     Radians,
/// };
///
/// assert!(approximately_equal(
///     normalize_radians(Radians::new(std::f64::consts::TAU)).value(),
///     0.0,
/// ));
/// ```
pub fn normalize_radians(angle: Radians) -> Radians {
    Radians::new(angle.value().rem_euclid(TAU))
}

/// Normalizes an hour value to the range `0.0..24.0`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::{approximately_equal, normalize_hours};
///
/// assert!(approximately_equal(normalize_hours(25.0), 1.0));
/// assert!(approximately_equal(normalize_hours(-1.0), 23.0));
/// ```
pub fn normalize_hours(hours: f64) -> f64 {
    hours.rem_euclid(24.0)
}

#[cfg(test)]
mod tests {
    use crate::core::math::{approximately_equal, constants::TAU};
    use crate::core::types::{Degrees, Radians};

    use super::{normalize_180, normalize_360, normalize_hours, normalize_radians};

    #[test]
    fn normalizes_positive_degrees_to_360_range() {
        assert!(approximately_equal(
            normalize_360(Degrees::new(370.0)).value(),
            10.0,
        ));
    }

    #[test]
    fn normalizes_negative_degrees_to_360_range() {
        assert!(approximately_equal(
            normalize_360(Degrees::new(-10.0)).value(),
            350.0,
        ));
    }

    #[test]
    fn normalizes_full_turns_to_zero() {
        assert!(approximately_equal(
            normalize_360(Degrees::new(720.0)).value(),
            0.0,
        ));
        assert!(approximately_equal(
            normalize_360(Degrees::new(-720.0)).value(),
            0.0,
        ));
    }

    #[test]
    fn normalizes_degrees_to_180_range() {
        assert!(approximately_equal(
            normalize_180(Degrees::new(270.0)).value(),
            -90.0,
        ));
        assert!(approximately_equal(
            normalize_180(Degrees::new(-270.0)).value(),
            90.0,
        ));
    }

    #[test]
    fn maps_180_boundary_to_negative_180() {
        assert!(approximately_equal(
            normalize_180(Degrees::new(180.0)).value(),
            -180.0,
        ));
        assert!(approximately_equal(
            normalize_180(Degrees::new(540.0)).value(),
            -180.0,
        ));
    }

    #[test]
    fn normalizes_radians_to_tau_range() {
        assert!(approximately_equal(
            normalize_radians(Radians::new(TAU + 0.5)).value(),
            0.5,
        ));
        assert!(approximately_equal(
            normalize_radians(Radians::new(-0.5)).value(),
            TAU - 0.5,
        ));
    }

    #[test]
    fn normalizes_hours_to_day_range() {
        assert!(approximately_equal(normalize_hours(25.0), 1.0));
        assert!(approximately_equal(normalize_hours(-1.0), 23.0));
        assert!(approximately_equal(normalize_hours(48.0), 0.0));
    }
}
