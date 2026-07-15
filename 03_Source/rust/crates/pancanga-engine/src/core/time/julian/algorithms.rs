//! Meeus Chapter 7 Julian Day algorithms.
//!
//! References:
//! - Jean Meeus, *Astronomical Algorithms*, 2nd Edition, Chapter 7.
//! - PES-0002, Sistema Temporal de Pancanga Engine.

use super::validation::is_valid_gregorian_date;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct GregorianComponents {
    pub(crate) year: i32,
    pub(crate) month: u8,
    pub(crate) day: u8,
    pub(crate) day_fraction: f64,
}

/// Converts a proleptic Gregorian date with fractional day to Julian Day.
///
/// This is the Gregorian-calendar branch of the algorithm described by Meeus
/// in Chapter 7. The `day` argument may contain the fractional part of the day.
pub(crate) fn gregorian_calendar_to_julian_day(year: i32, month: u8, day: f64) -> Option<f64> {
    if !day.is_finite() || day < 1.0 {
        return None;
    }

    let whole_day = day.floor() as u8;
    let next_day = f64::from(whole_day) + 1.0;

    if !is_valid_gregorian_date(year, month, whole_day) || day >= next_day {
        return None;
    }

    let (adjusted_year, adjusted_month) = adjust_january_and_february(year, month);
    let century = floor_div_100(adjusted_year);
    let gregorian_correction = 2 - century + floor_div_4(century);

    Some(
        (365.25 * f64::from(adjusted_year + 4716)).floor()
            + (30.6001 * f64::from(adjusted_month + 1)).floor()
            + day
            + f64::from(gregorian_correction)
            - 1524.5,
    )
}

/// Converts a Julian Day to proleptic Gregorian calendar components.
///
/// This is the inverse Gregorian-calendar branch of the Chapter 7 algorithm.
pub(crate) fn julian_day_to_gregorian_calendar(julian_day: f64) -> Option<GregorianComponents> {
    if !julian_day.is_finite() {
        return None;
    }

    let shifted = julian_day + 0.5;
    let z = shifted.floor();
    let f = shifted - z;
    let alpha = ((z - 1_867_216.25) / 36_524.25).floor();
    let a = z + 1.0 + alpha - (alpha / 4.0).floor();
    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();

    let day_with_fraction = b - d - (30.6001 * e).floor() + f;
    let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
    let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };

    split_day_fraction(year as i32, month as u8, day_with_fraction)
}

fn adjust_january_and_february(year: i32, month: u8) -> (i32, u8) {
    if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    }
}

fn floor_div_100(value: i32) -> i32 {
    (f64::from(value) / 100.0).floor() as i32
}

fn floor_div_4(value: i32) -> i32 {
    (f64::from(value) / 4.0).floor() as i32
}

fn split_day_fraction(year: i32, month: u8, day_with_fraction: f64) -> Option<GregorianComponents> {
    let day = day_with_fraction.floor() as u8;
    let day_fraction = day_with_fraction - f64::from(day);

    if is_valid_gregorian_date(year, month, day) {
        Some(GregorianComponents {
            year,
            month,
            day,
            day_fraction,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::core::math::approximately_equal;

    use super::{gregorian_calendar_to_julian_day, julian_day_to_gregorian_calendar};

    #[test]
    fn converts_meeus_chapter_7_example_to_julian_day() {
        let jd = gregorian_calendar_to_julian_day(1957, 10, 4.81)
            .expect("Meeus example should be valid");

        assert!(approximately_equal(jd, 2_436_116.31));
    }

    #[test]
    fn converts_j2000_to_julian_day() {
        let jd = gregorian_calendar_to_julian_day(2000, 1, 1.5).expect("J2000 should be valid");

        assert!(approximately_equal(jd, 2_451_545.0));
    }

    #[test]
    fn converts_julian_day_to_gregorian_components() {
        let components = julian_day_to_gregorian_calendar(2_451_545.0)
            .expect("J2000 should convert to Gregorian components");

        assert_eq!(components.year, 2000);
        assert_eq!(components.month, 1);
        assert_eq!(components.day, 1);
        assert!(approximately_equal(components.day_fraction, 0.5));
    }

    #[test]
    fn rejects_non_finite_julian_day() {
        assert_eq!(julian_day_to_gregorian_calendar(f64::NAN), None);
    }
}
