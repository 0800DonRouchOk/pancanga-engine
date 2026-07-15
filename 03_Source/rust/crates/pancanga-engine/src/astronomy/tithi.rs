//! Astronomical tithi index.
//!
//! This module contains only the instantaneous astronomical index derived from
//! lunar-solar elongation. It does not contain calendar or Vaishnava rules.

use crate::core::Degrees;

/// The lunar fortnight of an astronomical tithi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paksha {
    /// Waxing fortnight, traditionally Sukla paksha.
    Sukla,

    /// Waning fortnight, traditionally Krsna paksha.
    Krsna,
}

/// Instantaneous astronomical tithi.
///
/// This type is derived only from the zero-based astronomical tithi index.
/// It does not know civil dates, sunrise, time zones, Ekadasi, parana, or
/// Vaishnava rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstronomicalTithi {
    index: u8,
}

impl AstronomicalTithi {
    /// Creates an astronomical tithi from a zero-based index in `0..29`.
    ///
    /// Returns `None` when the index is outside the thirty astronomical tithi
    /// sectors.
    pub fn from_index(index: u8) -> Option<Self> {
        (index < 30).then_some(Self { index })
    }

    /// Returns the zero-based astronomical index in `0..29`.
    pub fn index(self) -> u8 {
        self.index
    }

    /// Returns the lunar fortnight.
    pub fn paksha(self) -> Paksha {
        if self.index < 15 {
            Paksha::Sukla
        } else {
            Paksha::Krsna
        }
    }

    /// Returns the traditional tithi number in `1..15`.
    pub fn traditional_number(self) -> u8 {
        (self.index % 15) + 1
    }
}

/// Returns the zero-based astronomical tithi index.
///
/// The index divides the lunar-solar elongation into thirty sectors of
/// `12°` each:
///
/// ```text
/// tithi_index = floor(elongation / 12°)
/// ```
///
/// The result is an internal astronomical index in the range `0..29`.
///
/// # Examples
///
/// ```
/// use pancanga_engine::astronomy::tithi_index;
/// use pancanga_engine::core::Degrees;
///
/// assert_eq!(tithi_index(Degrees::new(12.0)), 1);
/// ```
pub fn tithi_index(elongation: Degrees) -> u8 {
    (elongation.value() / 12.0).floor() as u8
}

/// Returns the instantaneous astronomical tithi from lunar-solar elongation.
///
/// The result is derived only from the astronomical elongation and contains no
/// calendar or Vaishnava rule interpretation.
pub fn astronomical_tithi(elongation: Degrees) -> AstronomicalTithi {
    AstronomicalTithi {
        index: tithi_index(elongation),
    }
}

#[cfg(test)]
mod tests {
    use crate::astronomy::{astronomical_tithi, tithi_index, AstronomicalTithi, Paksha};
    use crate::core::Degrees;

    #[test]
    fn returns_first_index_at_zero_degrees() {
        assert_eq!(tithi_index(Degrees::new(0.0)), 0);
    }

    #[test]
    fn stays_in_first_index_before_twelve_degrees() {
        assert_eq!(tithi_index(Degrees::new(11.999_999)), 0);
    }

    #[test]
    fn advances_at_twelve_degrees() {
        assert_eq!(tithi_index(Degrees::new(12.0)), 1);
    }

    #[test]
    fn returns_middle_index() {
        assert_eq!(tithi_index(Degrees::new(180.0)), 15);
    }

    #[test]
    fn returns_last_index_before_full_circle() {
        assert_eq!(tithi_index(Degrees::new(359.999_999)), 29);
    }

    #[test]
    fn creates_tithi_from_valid_index() {
        let tithi = AstronomicalTithi::from_index(0).expect("valid tithi index");

        assert_eq!(tithi.index(), 0);
    }

    #[test]
    fn rejects_invalid_index() {
        assert_eq!(AstronomicalTithi::from_index(30), None);
    }

    #[test]
    fn index_zero_is_sukla_one() {
        let tithi = AstronomicalTithi::from_index(0).expect("valid tithi index");

        assert_eq!(tithi.paksha(), Paksha::Sukla);
        assert_eq!(tithi.traditional_number(), 1);
    }

    #[test]
    fn index_fourteen_is_sukla_fifteen() {
        let tithi = AstronomicalTithi::from_index(14).expect("valid tithi index");

        assert_eq!(tithi.paksha(), Paksha::Sukla);
        assert_eq!(tithi.traditional_number(), 15);
    }

    #[test]
    fn index_fifteen_is_krsna_one() {
        let tithi = AstronomicalTithi::from_index(15).expect("valid tithi index");

        assert_eq!(tithi.paksha(), Paksha::Krsna);
        assert_eq!(tithi.traditional_number(), 1);
    }

    #[test]
    fn index_twenty_nine_is_krsna_fifteen() {
        let tithi = AstronomicalTithi::from_index(29).expect("valid tithi index");

        assert_eq!(tithi.paksha(), Paksha::Krsna);
        assert_eq!(tithi.traditional_number(), 15);
    }

    #[test]
    fn derives_astronomical_tithi_from_elongation() {
        let tithi = astronomical_tithi(Degrees::new(180.0));

        assert_eq!(tithi.index(), 15);
        assert_eq!(tithi.paksha(), Paksha::Krsna);
        assert_eq!(tithi.traditional_number(), 1);
    }
}
