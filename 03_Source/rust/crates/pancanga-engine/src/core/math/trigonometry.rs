//! Trigonometric helpers using strong angle types.

use crate::core::types::{Degrees, Radians};

use super::compare::clamp_unit;
use super::constants::{DEG_TO_RAD, RAD_TO_DEG};

/// Converts degrees to radians.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{
///     math::{approximately_equal, deg_to_rad},
///     Degrees,
/// };
///
/// assert!(approximately_equal(
///     deg_to_rad(Degrees::new(180.0)).value(),
///     std::f64::consts::PI,
/// ));
/// ```
pub fn deg_to_rad(d: Degrees) -> Radians {
    Radians::new(d.value() * DEG_TO_RAD)
}

/// Converts radians to degrees.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{
///     math::{approximately_equal, rad_to_deg},
///     Radians,
/// };
///
/// assert!(approximately_equal(
///     rad_to_deg(Radians::new(std::f64::consts::PI)).value(),
///     180.0,
/// ));
/// ```
pub fn rad_to_deg(r: Radians) -> Degrees {
    Degrees::new(r.value() * RAD_TO_DEG)
}

/// Returns the sine of an angle expressed in degrees.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{math::sin_deg, Degrees};
///
/// assert!((sin_deg(Degrees::new(90.0)) - 1.0).abs() < 1.0e-12);
/// ```
pub fn sin_deg(angle: Degrees) -> f64 {
    deg_to_rad(angle).value().sin()
}

/// Returns the cosine of an angle expressed in degrees.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{
///     math::{approximately_equal, cos_deg},
///     Degrees,
/// };
///
/// assert!(approximately_equal(cos_deg(Degrees::new(180.0)), -1.0));
/// ```
pub fn cos_deg(angle: Degrees) -> f64 {
    deg_to_rad(angle).value().cos()
}

/// Returns the tangent of an angle expressed in degrees.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::{
///     math::{approximately_equal, tan_deg},
///     Degrees,
/// };
///
/// assert!(approximately_equal(tan_deg(Degrees::new(45.0)), 1.0));
/// ```
pub fn tan_deg(angle: Degrees) -> f64 {
    deg_to_rad(angle).value().tan()
}

/// Returns the arcsine in degrees.
///
/// Inputs are clamped to the unit interval to protect against tiny
/// floating-point boundary drift.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::{approximately_equal, asin_deg};
///
/// assert!(approximately_equal(asin_deg(1.0).value(), 90.0));
/// ```
pub fn asin_deg(x: f64) -> Degrees {
    rad_to_deg(Radians::new(clamp_unit(x).asin()))
}

/// Returns the arccosine in degrees.
///
/// Inputs are clamped to the unit interval to protect against tiny
/// floating-point boundary drift.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::{approximately_equal, acos_deg};
///
/// assert!(approximately_equal(acos_deg(-1.0).value(), 180.0));
/// ```
pub fn acos_deg(x: f64) -> Degrees {
    rad_to_deg(Radians::new(clamp_unit(x).acos()))
}

/// Returns the arctangent in degrees.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::{approximately_equal, atan_deg};
///
/// assert!(approximately_equal(atan_deg(1.0).value(), 45.0));
/// ```
pub fn atan_deg(x: f64) -> Degrees {
    rad_to_deg(Radians::new(x.atan()))
}

/// Returns the quadrant-aware arctangent in degrees.
///
/// # Examples
///
/// ```
/// use pancanga_engine::core::math::{approximately_equal, atan2_deg};
///
/// assert!(approximately_equal(atan2_deg(1.0, 0.0).value(), 90.0));
/// ```
pub fn atan2_deg(y: f64, x: f64) -> Degrees {
    rad_to_deg(Radians::new(y.atan2(x)))
}

#[cfg(test)]
mod tests {
    use crate::core::math::{approximately_equal, constants::PI};
    use crate::core::types::{Degrees, Radians};

    use super::{
        acos_deg, asin_deg, atan2_deg, atan_deg, cos_deg, deg_to_rad, rad_to_deg, sin_deg, tan_deg,
    };

    #[test]
    fn converts_degrees_to_radians() {
        assert!(approximately_equal(
            deg_to_rad(Degrees::new(180.0)).value(),
            PI,
        ));
    }

    #[test]
    fn converts_radians_to_degrees() {
        assert!(approximately_equal(
            rad_to_deg(Radians::new(PI)).value(),
            180.0,
        ));
    }

    #[test]
    fn calculates_sine_from_degrees() {
        assert!(approximately_equal(sin_deg(Degrees::new(90.0)), 1.0));
        assert!(approximately_equal(sin_deg(Degrees::new(0.0)), 0.0));
    }

    #[test]
    fn calculates_cosine_from_degrees() {
        assert!(approximately_equal(cos_deg(Degrees::new(180.0)), -1.0));
        assert!(approximately_equal(cos_deg(Degrees::new(0.0)), 1.0));
    }

    #[test]
    fn calculates_tangent_from_degrees() {
        assert!(approximately_equal(tan_deg(Degrees::new(45.0)), 1.0));
        assert!(approximately_equal(tan_deg(Degrees::new(0.0)), 0.0));
    }

    #[test]
    fn calculates_inverse_sine_in_degrees() {
        assert!(approximately_equal(asin_deg(1.0).value(), 90.0));
        assert!(approximately_equal(asin_deg(-1.0).value(), -90.0));
    }

    #[test]
    fn clamps_inverse_sine_input() {
        assert!(approximately_equal(asin_deg(1.0 + 1.0e-13).value(), 90.0));
        assert!(approximately_equal(asin_deg(-1.0 - 1.0e-13).value(), -90.0,));
    }

    #[test]
    fn calculates_inverse_cosine_in_degrees() {
        assert!(approximately_equal(acos_deg(1.0).value(), 0.0));
        assert!(approximately_equal(acos_deg(-1.0).value(), 180.0));
    }

    #[test]
    fn clamps_inverse_cosine_input() {
        assert!(approximately_equal(acos_deg(1.0 + 1.0e-13).value(), 0.0));
        assert!(approximately_equal(acos_deg(-1.0 - 1.0e-13).value(), 180.0,));
    }

    #[test]
    fn calculates_inverse_tangent_in_degrees() {
        assert!(approximately_equal(atan_deg(1.0).value(), 45.0));
        assert!(approximately_equal(atan_deg(0.0).value(), 0.0));
    }

    #[test]
    fn calculates_quadrant_aware_inverse_tangent_in_degrees() {
        assert!(approximately_equal(atan2_deg(1.0, 0.0).value(), 90.0));
        assert!(approximately_equal(atan2_deg(0.0, -1.0).value(), 180.0));
        assert!(approximately_equal(atan2_deg(-1.0, 0.0).value(), -90.0));
    }
}
