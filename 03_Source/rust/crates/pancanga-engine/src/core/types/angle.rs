//! Angle value types.

/// Represents an angle measured in degrees.
///
/// This type only stores the raw numeric value. It does not normalize the
/// angle or perform astronomical conversions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Degrees {
    value: f64,
}

impl Degrees {
    /// Creates a degree value from a raw `f64`.
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    /// Returns the raw degree value.
    pub fn value(self) -> f64 {
        self.value
    }
}

/// Represents an angle measured in radians.
///
/// This type only stores the raw numeric value. It does not normalize the
/// angle or perform astronomical conversions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radians {
    value: f64,
}

impl Radians {
    /// Creates a radian value from a raw `f64`.
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    /// Returns the raw radian value.
    pub fn value(self) -> f64 {
        self.value
    }
}

/// Represents an angular quantity measured in arcseconds.
///
/// This type is used for small angular corrections published in astronomical
/// theories. It stores the raw signed value without normalization.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ArcSeconds {
    value: f64,
}

impl ArcSeconds {
    /// Creates an arcsecond value from a raw `f64`.
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    /// Returns the raw arcsecond value.
    pub fn value(self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::{ArcSeconds, Degrees, Radians};

    #[test]
    fn creates_degrees() {
        let degrees = Degrees::new(180.0);

        assert_eq!(degrees.value(), 180.0);
    }

    #[test]
    fn creates_radians() {
        let radians = Radians::new(std::f64::consts::PI);

        assert_eq!(radians.value(), std::f64::consts::PI);
    }

    #[test]
    fn creates_arcseconds() {
        let arcseconds = ArcSeconds::new(-12.5);

        assert_eq!(arcseconds.value(), -12.5);
    }
}
