//! AST-L002: ELP2000 periodic longitude correction.
//!
//! This module exposes the algorithm-level API for the first ELP2000 lunar
//! longitude step. It delegates theory interpretation to the ELP2000
//! interpreter.

use crate::core::{ArcSeconds, JulianDate};

use super::elp2000_evaluator::Elp2000Evaluator;

/// Returns the ELP2000 periodic longitude correction `Δλ`.
///
/// The output is a signed angular correction in arcseconds. This function does
/// not add the lunar mean longitude `W1`, does not compute final lunar
/// longitude, and does not apply calendar or Gaudiya rules.
///
/// This is the public AST-L002 entry point.
pub fn longitude_periodic_correction(jd: JulianDate) -> ArcSeconds {
    Elp2000Evaluator::new().evaluate_longitude_correction(jd)
}
