//! Distance value types.

/// Represents a distance measured in astronomical units.
///
/// This type stores the raw astronomical unit value without applying any
/// conversion to kilometers or other distance units.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AstronomicalUnits {
    value: f64,
}

impl AstronomicalUnits {
    /// Creates a distance from a raw astronomical-unit value.
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    /// Returns the raw astronomical-unit value.
    pub fn value(self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::AstronomicalUnits;

    #[test]
    fn creates_astronomical_units() {
        let distance = AstronomicalUnits::new(1.0);

        assert_eq!(distance.value(), 1.0);
    }
}
