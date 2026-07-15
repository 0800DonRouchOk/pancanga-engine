//! Geographic value types.

use super::Degrees;

/// Represents a geographic latitude in degrees.
///
/// Valid latitude values are in the inclusive range `-90.0..=90.0`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Latitude {
    degrees: Degrees,
}

impl Latitude {
    /// Creates a latitude from degrees.
    ///
    /// Returns `None` when the value is outside `-90.0..=90.0`.
    pub fn from_degrees(value: f64) -> Option<Self> {
        if (-90.0..=90.0).contains(&value) {
            Some(Self {
                degrees: Degrees::new(value),
            })
        } else {
            None
        }
    }

    /// Returns this latitude as degrees.
    pub fn degrees(self) -> Degrees {
        self.degrees
    }
}

/// Represents a geographic longitude in degrees.
///
/// Valid longitude values are in the inclusive range `-180.0..=180.0`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Longitude {
    degrees: Degrees,
}

impl Longitude {
    /// Creates a longitude from degrees.
    ///
    /// Returns `None` when the value is outside `-180.0..=180.0`.
    pub fn from_degrees(value: f64) -> Option<Self> {
        if (-180.0..=180.0).contains(&value) {
            Some(Self {
                degrees: Degrees::new(value),
            })
        } else {
            None
        }
    }

    /// Returns this longitude as degrees.
    pub fn degrees(self) -> Degrees {
        self.degrees
    }
}

/// Represents a geographic location.
///
/// This type groups latitude and longitude without adding astronomical or
/// calendar-specific behavior.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoLocation {
    latitude: Latitude,
    longitude: Longitude,
}

impl GeoLocation {
    /// Creates a geographic location from latitude and longitude.
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Returns the latitude.
    pub fn latitude(self) -> Latitude {
        self.latitude
    }

    /// Returns the longitude.
    pub fn longitude(self) -> Longitude {
        self.longitude
    }
}

#[cfg(test)]
mod tests {
    use super::{GeoLocation, Latitude, Longitude};

    #[test]
    fn creates_latitude_in_range() {
        let latitude = Latitude::from_degrees(-34.6037).expect("latitude should be valid");

        assert_eq!(latitude.degrees().value(), -34.6037);
    }

    #[test]
    fn rejects_latitude_out_of_range() {
        assert_eq!(Latitude::from_degrees(90.1), None);
        assert_eq!(Latitude::from_degrees(-90.1), None);
    }

    #[test]
    fn creates_longitude_in_range() {
        let longitude = Longitude::from_degrees(-58.3816).expect("longitude should be valid");

        assert_eq!(longitude.degrees().value(), -58.3816);
    }

    #[test]
    fn rejects_longitude_out_of_range() {
        assert_eq!(Longitude::from_degrees(180.1), None);
        assert_eq!(Longitude::from_degrees(-180.1), None);
    }

    #[test]
    fn creates_geo_location() {
        let latitude = Latitude::from_degrees(-34.6037).expect("latitude should be valid");
        let longitude = Longitude::from_degrees(-58.3816).expect("longitude should be valid");

        let location = GeoLocation::new(latitude, longitude);

        assert_eq!(location.latitude(), latitude);
        assert_eq!(location.longitude(), longitude);
    }
}
