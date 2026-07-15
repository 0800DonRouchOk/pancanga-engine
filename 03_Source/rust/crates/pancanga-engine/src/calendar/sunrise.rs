//! Sunrise calculation.
//!
//! This module calculates the astronomical instant of local sunrise for a civil
//! date and geographic location. It does not calculate tithi presence,
//! arunodaya, Ekadasi, or Vaishnava rules.

use crate::core::math::{
    acos_deg, asin_deg, atan_deg, cos_deg, normalize_360, normalize_hours, sin_deg, tan_deg,
};
use crate::core::time::{gregorian_to_jd, CivilDate, CivilDateTime, CivilTime, TimeScale};
use crate::core::{Degrees, GeoLocation, JulianDate};

const OFFICIAL_ZENITH_DEGREES: f64 = 90.833;

/// Returns the Julian Date of local sunrise for a civil date and location.
///
/// The calculation follows the standard solar-almanac sunrise approximation
/// using the official sunrise zenith of `90.833°`.
///
/// The input date is interpreted as the local civil date for the requested
/// location. The returned [`JulianDate`] is the astronomical instant expressed
/// on the UTC Julian Day scale used by the rest of the project.
///
/// Returns `None` for locations and dates where the Sun does not rise.
///
/// # Examples
///
/// ```
/// use pancanga_engine::calendar::sunrise;
/// use pancanga_engine::core::{CivilDate, GeoLocation, Latitude, Longitude};
///
/// let date = CivilDate::new(2026, 3, 20).expect("date should be valid");
/// let latitude = Latitude::from_degrees(0.0).expect("latitude should be valid");
/// let longitude = Longitude::from_degrees(0.0).expect("longitude should be valid");
/// let location = GeoLocation::new(latitude, longitude);
///
/// let jd = sunrise(date, location).expect("sunrise should exist");
///
/// assert!(jd.value().is_finite());
/// ```
pub fn sunrise(date: CivilDate, location: GeoLocation) -> Option<JulianDate> {
    let day = day_of_year(date);
    let longitude = location.longitude().degrees().value();
    let latitude = location.latitude().degrees().value();
    let longitude_hour = longitude / 15.0;
    let approximate_time = f64::from(day) + ((6.0 - longitude_hour) / 24.0);
    let mean_anomaly = solar_mean_anomaly(approximate_time);
    let true_longitude = solar_true_longitude(mean_anomaly);
    let right_ascension = solar_right_ascension_hours(true_longitude);
    let sin_declination = 0.39782 * sin_deg(true_longitude);
    let cos_declination = cos_deg(asin_deg(sin_declination));
    let local_hour_angle = sunrise_local_hour_angle(latitude, sin_declination, cos_declination)?;
    let local_mean_time = local_hour_angle + right_ascension - (0.06571 * approximate_time) - 6.622;
    let universal_time = normalize_hours(local_mean_time - longitude_hour);

    Some(julian_date_at_universal_hours(date, universal_time))
}

fn solar_mean_anomaly(approximate_time: f64) -> Degrees {
    Degrees::new((0.9856 * approximate_time) - 3.289)
}

fn solar_true_longitude(mean_anomaly: Degrees) -> Degrees {
    normalize_360(Degrees::new(
        mean_anomaly.value()
            + (1.916 * sin_deg(mean_anomaly))
            + (0.020 * sin_deg(Degrees::new(2.0 * mean_anomaly.value())))
            + 282.634,
    ))
}

fn solar_right_ascension_hours(true_longitude: Degrees) -> f64 {
    let raw_right_ascension = normalize_360(atan_deg(0.91764 * tan_deg(true_longitude)));
    let longitude_quadrant = (true_longitude.value() / 90.0).floor() * 90.0;
    let right_ascension_quadrant = (raw_right_ascension.value() / 90.0).floor() * 90.0;
    let adjusted_right_ascension =
        raw_right_ascension.value() + longitude_quadrant - right_ascension_quadrant;

    adjusted_right_ascension / 15.0
}

fn sunrise_local_hour_angle(
    latitude: f64,
    sin_declination: f64,
    cos_declination: f64,
) -> Option<f64> {
    let cos_hour_angle = (cos_deg(Degrees::new(OFFICIAL_ZENITH_DEGREES))
        - (sin_declination * sin_deg(Degrees::new(latitude))))
        / (cos_declination * cos_deg(Degrees::new(latitude)));

    if !(-1.0..=1.0).contains(&cos_hour_angle) {
        return None;
    }

    Some((360.0 - acos_deg(cos_hour_angle).value()) / 15.0)
}

fn julian_date_at_universal_hours(date: CivilDate, universal_time: f64) -> JulianDate {
    let hour = universal_time.floor() as u8;
    let minute_fraction = (universal_time - f64::from(hour)) * 60.0;
    let minute = minute_fraction.floor() as u8;
    let second = (minute_fraction - f64::from(minute)) * 60.0;
    let time = CivilTime::new(hour, minute, second).expect("normalized UTC time should be valid");

    gregorian_to_jd(CivilDateTime::new(date, time, TimeScale::Utc))
}

fn day_of_year(date: CivilDate) -> u16 {
    let mut day = u16::from(date.day());

    for month in 1..date.month() {
        day += u16::from(days_in_month(date.year(), month));
    }

    day
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::sunrise;
    use crate::core::{CivilDate, GeoLocation, Latitude, Longitude};

    const TOLERANCE_DAYS: f64 = 1.0e-8;

    fn assert_julian_date(actual: f64, expected: f64) {
        let difference = (actual - expected).abs();

        assert!(
            difference <= TOLERANCE_DAYS,
            "expected JD {}, got JD {}, difference {} days",
            expected,
            actual,
            difference,
        );
    }

    #[test]
    fn calculates_equatorial_sunrise_near_march_equinox() {
        let date = date(2026, 3, 20);
        let location = location(0.0, 0.0);

        let sunrise = sunrise(date, location).expect("sunrise should exist");

        assert_julian_date(sunrise.value(), 2_461_119.753_038_483);
    }

    #[test]
    fn calculates_mid_northern_latitude_summer_sunrise() {
        let date = date(2026, 6, 21);
        let location = location(40.7128, -74.0060);

        let sunrise = sunrise(date, location).expect("sunrise should exist");

        assert_julian_date(sunrise.value(), 2_461_212.892_268_492);
    }

    #[test]
    fn calculates_mid_southern_latitude_winter_sunrise() {
        let date = date(2026, 6, 21);
        let location = location(-34.6037, -58.3816);

        let sunrise = sunrise(date, location).expect("sunrise should exist");

        assert_julian_date(sunrise.value(), 2_461_212.958_473_451_4);
    }

    #[test]
    fn calculates_mid_southern_latitude_summer_sunrise() {
        let date = date(2026, 12, 21);
        let location = location(-33.8688, 151.2093);

        let sunrise = sunrise(date, location).expect("sunrise should exist");

        assert_julian_date(sunrise.value(), 2_461_396.278_178_977);
    }

    #[test]
    fn returns_none_when_sun_does_not_rise() {
        let date = date(2026, 12, 21);
        let location = location(80.0, 0.0);

        assert_eq!(sunrise(date, location), None);
    }

    fn date(year: i32, month: u8, day: u8) -> CivilDate {
        CivilDate::new(year, month, day).expect("date should be valid")
    }

    fn location(latitude: f64, longitude: f64) -> GeoLocation {
        let latitude = Latitude::from_degrees(latitude).expect("latitude should be valid");
        let longitude = Longitude::from_degrees(longitude).expect("longitude should be valid");

        GeoLocation::new(latitude, longitude)
    }
}
