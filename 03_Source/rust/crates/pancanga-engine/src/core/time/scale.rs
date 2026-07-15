//! Time scale definitions.

/// Identifies the time scale associated with a temporal value.
///
/// The enum is descriptive only. M3 does not implement conversions between time
/// scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeScale {
    /// Coordinated Universal Time.
    Utc,
    /// Universal Time 1.
    Ut1,
    /// International Atomic Time.
    Tai,
    /// Terrestrial Time.
    Tt,
}

#[cfg(test)]
mod tests {
    use super::TimeScale;

    #[test]
    fn creates_time_scale_value() {
        let scale = TimeScale::Utc;

        assert_eq!(scale, TimeScale::Utc);
    }

    #[test]
    fn time_scale_is_copy() {
        let scale = TimeScale::Tt;
        let copied = scale;

        assert_eq!(scale, copied);
    }
}
