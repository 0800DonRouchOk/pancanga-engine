//! Mathematical constants used by Pancanga Engine.

/// The mathematical constant π.
pub const PI: f64 = std::f64::consts::PI;

/// The mathematical constant τ, equal to `2π`.
pub const TAU: f64 = std::f64::consts::TAU;

/// Half of π.
pub const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;

/// Multiplicative factor for converting degrees to radians.
pub const DEG_TO_RAD: f64 = PI / 180.0;

/// Multiplicative factor for converting radians to degrees.
pub const RAD_TO_DEG: f64 = 180.0 / PI;

/// Default tolerance for floating-point comparisons.
pub const EPSILON: f64 = 1.0e-12;

#[cfg(test)]
mod tests {
    use super::{DEG_TO_RAD, EPSILON, HALF_PI, PI, RAD_TO_DEG, TAU};

    #[test]
    fn constants_match_standard_library() {
        assert!((PI - std::f64::consts::PI).abs() <= EPSILON);
        assert!((TAU - std::f64::consts::TAU).abs() <= EPSILON);
        assert!((HALF_PI - std::f64::consts::FRAC_PI_2).abs() <= EPSILON);
    }

    #[test]
    fn conversion_constants_are_reciprocal() {
        assert!(((DEG_TO_RAD * RAD_TO_DEG) - 1.0).abs() <= EPSILON);
    }
}
