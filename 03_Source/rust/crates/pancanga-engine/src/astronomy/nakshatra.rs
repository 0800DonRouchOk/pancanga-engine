//! Astronomical nakshatra index.
//!
//! This module contains only the ecliptic longitude division into 27
//! nakshatras. It does not contain calendar or Vaishnava rules.

use crate::core::math::normalize_360;
use crate::core::Degrees;

const NAKSHATRA_COUNT: u8 = 27;
const NAKSHATRA_SPAN_DEGREES: f64 = 360.0 / NAKSHATRA_COUNT as f64;

/// Zero-based nakshatra index in `0..26`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NakshatraIndex {
    index: u8,
}

impl NakshatraIndex {
    /// Rohini, needed by HBV-MD-007 Jayanti.
    pub const ROHINI: Self = Self { index: 3 };

    /// Punarvasu, needed by HBV-MD-005 Jaya.
    pub const PUNARVASU: Self = Self { index: 6 };

    /// Pushya, needed by HBV-MD-008 Papanasini.
    pub const PUSHYA: Self = Self { index: 7 };

    /// Shravana, needed by HBV-MD-006 Vijaya.
    pub const SHRAVANA: Self = Self { index: 21 };

    /// Creates a nakshatra index from a zero-based value in `0..26`.
    pub fn from_index(index: u8) -> Option<Self> {
        (index < NAKSHATRA_COUNT).then_some(Self { index })
    }

    /// Returns the zero-based index in `0..26`.
    pub fn value(self) -> u8 {
        self.index
    }
}

/// Returns the zero-based nakshatra index from apparent lunar longitude.
///
/// ```text
/// nakshatra_index = floor(normalize_360(lambda_moon) / 13°20')
/// ```
pub fn index(moon_longitude: Degrees) -> NakshatraIndex {
    let longitude = normalize_360(moon_longitude).value();
    let index = (longitude / NAKSHATRA_SPAN_DEGREES).floor() as u8;

    NakshatraIndex { index }
}

#[cfg(test)]
mod tests {
    use super::{index, NakshatraIndex};
    use crate::core::Degrees;

    #[test]
    fn returns_first_nakshatra_at_zero_degrees() {
        assert_eq!(index(Degrees::new(0.0)).value(), 0);
    }

    #[test]
    fn advances_at_thirteen_degrees_twenty_minutes() {
        assert_eq!(index(Degrees::new(13.0 + 20.0 / 60.0)).value(), 1);
    }

    #[test]
    fn normalizes_full_circle_to_first_nakshatra() {
        assert_eq!(index(Degrees::new(360.0)).value(), 0);
    }

    #[test]
    fn normalizes_negative_longitudes() {
        assert_eq!(index(Degrees::new(-1.0)).value(), 26);
    }

    #[test]
    fn exposes_required_hbv_ek_004_nakshatras() {
        assert_eq!(NakshatraIndex::PUNARVASU.value(), 6);
        assert_eq!(NakshatraIndex::SHRAVANA.value(), 21);
        assert_eq!(NakshatraIndex::ROHINI.value(), 3);
        assert_eq!(NakshatraIndex::PUSHYA.value(), 7);
    }
}
