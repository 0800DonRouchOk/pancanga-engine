//! Delta T representation.

/// Represents Delta T in seconds.
///
/// Delta T is the difference between Terrestrial Time and Universal Time
/// (`TT - UT`). M3 stores this value but does not estimate or calculate it.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DeltaT {
    seconds: f64,
}

impl DeltaT {
    /// Creates a Delta T value from seconds.
    ///
    /// Returns `None` when the value is not finite.
    pub fn from_seconds(seconds: f64) -> Option<Self> {
        if seconds.is_finite() {
            Some(Self { seconds })
        } else {
            None
        }
    }

    /// Returns the Delta T value in seconds.
    pub fn seconds(self) -> f64 {
        self.seconds
    }
}

#[cfg(test)]
mod tests {
    use crate::core::time::DeltaT;

    #[test]
    fn creates_delta_t() {
        let delta_t = DeltaT::from_seconds(69.184).expect("delta t should be valid");

        assert_eq!(delta_t.seconds(), 69.184);
    }

    #[test]
    fn rejects_non_finite_delta_t() {
        assert_eq!(DeltaT::from_seconds(f64::NAN), None);
        assert_eq!(DeltaT::from_seconds(f64::INFINITY), None);
    }
}
