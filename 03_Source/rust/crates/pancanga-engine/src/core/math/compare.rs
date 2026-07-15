//! Floating-point comparison helpers.

use super::constants::EPSILON;

/// Returns `true` when two floating-point values differ by no more than
/// [`EPSILON`].
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::approximately_equal;
///
/// assert!(approximately_equal(1.0, 1.0 + 1.0e-13));
/// ```
pub fn approximately_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}

/// Returns `true` when a floating-point value is close enough to zero.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::nearly_zero;
///
/// assert!(nearly_zero(1.0e-13));
/// ```
pub fn nearly_zero(x: f64) -> bool {
    x.abs() <= EPSILON
}

/// Clamps a value to the inclusive range `-1.0..=1.0`.
///
/// This is useful at the boundary of inverse trigonometric functions, where
/// small floating-point drift can otherwise produce values just outside the
/// mathematical domain.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::{approximately_equal, clamp_unit};
///
/// assert!(approximately_equal(clamp_unit(1.000000000001), 1.0));
/// ```
pub fn clamp_unit(x: f64) -> f64 {
    x.clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::{approximately_equal, clamp_unit, nearly_zero};

    #[test]
    fn detects_approximately_equal_values() {
        assert!(approximately_equal(1.0, 1.0 + 1.0e-13));
    }

    #[test]
    fn rejects_values_outside_epsilon() {
        assert!(!approximately_equal(1.0, 1.0 + 1.0e-9));
    }

    #[test]
    fn detects_nearly_zero_values() {
        assert!(nearly_zero(-1.0e-13));
    }

    #[test]
    fn rejects_non_zero_values() {
        assert!(!nearly_zero(1.0e-9));
    }

    #[test]
    fn clamps_values_to_unit_interval() {
        assert!(approximately_equal(clamp_unit(1.25), 1.0));
        assert!(approximately_equal(clamp_unit(-1.25), -1.0));
        assert!(approximately_equal(clamp_unit(0.25), 0.25));
    }

    #[test]
    fn preserves_unit_boundaries() {
        assert!(approximately_equal(clamp_unit(1.0), 1.0));
        assert!(approximately_equal(clamp_unit(-1.0), -1.0));
    }
}
