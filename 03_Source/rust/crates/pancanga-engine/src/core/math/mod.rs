//! Generic mathematical utilities for Pancanga Engine.
//!
//! This module contains only project-wide mathematical primitives. It has no
//! knowledge of astronomy, calendars, or Gaudiya rules.

pub mod compare;
pub mod constants;
pub mod normalize;
pub mod trigonometry;

pub use compare::{approximately_equal, clamp_unit, nearly_zero};
pub use normalize::{normalize_180, normalize_360, normalize_hours, normalize_radians};
pub use trigonometry::{
    acos_deg, asin_deg, atan2_deg, atan_deg, cos_deg, deg_to_rad, rad_to_deg, sin_deg, tan_deg,
};
