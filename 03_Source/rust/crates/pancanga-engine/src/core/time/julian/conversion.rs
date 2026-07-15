//! Public Julian Day conversion API.

use crate::core::math::approximately_equal;
use crate::core::time::{CivilDate, CivilDateTime, CivilTime, TimeScale};
use crate::core::types::JulianDate;

use super::algorithms::{gregorian_calendar_to_julian_day, julian_day_to_gregorian_calendar};

const SECONDS_PER_DAY: f64 = 86_400.0;
const SECONDS_PER_HOUR: f64 = 3_600.0;
const SECONDS_PER_MINUTE: f64 = 60.0;

/// Represents a weekday in the Gregorian/Julian Day cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    /// Sunday.
    Sunday,
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
}

/// Converts a Gregorian civil date-time to Julian Day.
///
/// The implementation follows the Gregorian-calendar branch of the algorithm
/// in Jean Meeus, *Astronomical Algorithms*, 2nd Edition, Chapter 7.
///
/// No time-scale conversion is performed. The input [`TimeScale`] remains a
/// label on the civil value and does not alter the Julian Day arithmetic.
pub fn gregorian_to_jd(date_time: CivilDateTime) -> JulianDate {
    let day = day_with_fraction(date_time.date(), date_time.time());
    let jd =
        gregorian_calendar_to_julian_day(date_time.date().year(), date_time.date().month(), day)
            .expect("CivilDateTime contains validated Gregorian date and time components");

    JulianDate::new(jd)
}

/// Converts a Julian Day to a Gregorian civil date-time.
///
/// The conversion uses the inverse Gregorian-calendar branch of the Meeus
/// Chapter 7 algorithm. The supplied [`TimeScale`] is attached to the returned
/// civil value; no conversion between time scales is performed.
pub fn jd_to_gregorian(julian_date: JulianDate, time_scale: TimeScale) -> Option<CivilDateTime> {
    let components = julian_day_to_gregorian_calendar(julian_date.value())?;
    let date = CivilDate::new(components.year, components.month, components.day)?;
    let time = civil_time_from_day_fraction(components.day_fraction)?;

    Some(CivilDateTime::new(date, time, time_scale))
}

/// Returns the weekday for a Julian Day.
///
/// The calculation follows the Julian Day weekday cycle where Sunday is zero:
/// `floor(JD + 1.5) mod 7`.
pub fn weekday(julian_date: JulianDate) -> Weekday {
    match ((julian_date.value() + 1.5).floor() as i64).rem_euclid(7) {
        0 => Weekday::Sunday,
        1 => Weekday::Monday,
        2 => Weekday::Tuesday,
        3 => Weekday::Wednesday,
        4 => Weekday::Thursday,
        5 => Weekday::Friday,
        _ => Weekday::Saturday,
    }
}

fn day_with_fraction(date: CivilDate, time: CivilTime) -> f64 {
    f64::from(date.day()) + day_fraction_from_time(time)
}

fn day_fraction_from_time(time: CivilTime) -> f64 {
    let seconds = f64::from(time.hour()) * SECONDS_PER_HOUR
        + f64::from(time.minute()) * SECONDS_PER_MINUTE
        + time.second();

    seconds / SECONDS_PER_DAY
}

fn civil_time_from_day_fraction(day_fraction: f64) -> Option<CivilTime> {
    let total_seconds = normalize_seconds(day_fraction * SECONDS_PER_DAY);
    let hour = (total_seconds / SECONDS_PER_HOUR).floor() as u8;
    let remaining_seconds = total_seconds - f64::from(hour) * SECONDS_PER_HOUR;
    let minute = (remaining_seconds / SECONDS_PER_MINUTE).floor() as u8;
    let second = remaining_seconds - f64::from(minute) * SECONDS_PER_MINUTE;

    CivilTime::new(hour, minute, second)
}

fn normalize_seconds(seconds: f64) -> f64 {
    if approximately_equal(seconds, SECONDS_PER_DAY) {
        0.0
    } else {
        seconds
    }
}

#[cfg(test)]
mod tests {
    use crate::core::math::approximately_equal;
    use crate::core::time::julian::Weekday;
    use crate::core::time::{
        gregorian_to_jd, jd_to_gregorian, weekday, CivilDate, CivilDateTime, CivilTime, TimeScale,
    };
    use crate::core::types::JulianDate;

    const ROUND_TRIP_SECOND_TOLERANCE: f64 = 1.0e-4;

    #[test]
    fn converts_meeus_example_to_julian_day() {
        let date_time = civil_date_time(1957, 10, 4, 19, 26, 24.0);

        let jd = gregorian_to_jd(date_time);

        assert!(approximately_equal(jd.value(), 2_436_116.31));
    }

    #[test]
    fn converts_meeus_j2000_example_to_julian_day() {
        let date_time = civil_date_time(2000, 1, 1, 12, 0, 0.0);

        let jd = gregorian_to_jd(date_time);

        assert!(approximately_equal(jd.value(), 2_451_545.0));
    }

    #[test]
    fn converts_other_meeus_published_examples_to_julian_day() {
        let examples = [
            (civil_date_time(1987, 1, 27, 0, 0, 0.0), 2_446_822.5),
            (civil_date_time(1987, 6, 19, 12, 0, 0.0), 2_446_966.0),
            (civil_date_time(1988, 1, 27, 0, 0, 0.0), 2_447_187.5),
            (civil_date_time(1988, 6, 19, 12, 0, 0.0), 2_447_332.0),
            (civil_date_time(1900, 1, 1, 0, 0, 0.0), 2_415_020.5),
            (civil_date_time(1600, 1, 1, 0, 0, 0.0), 2_305_447.5),
            (civil_date_time(1600, 12, 31, 0, 0, 0.0), 2_305_812.5),
        ];

        for (date_time, expected_jd) in examples {
            assert!(approximately_equal(
                gregorian_to_jd(date_time).value(),
                expected_jd
            ));
        }
    }

    #[test]
    fn converts_julian_day_to_gregorian_date_time() {
        let date_time = jd_to_gregorian(JulianDate::new(2_451_545.0), TimeScale::Tt)
            .expect("Julian Day should convert");

        assert_eq!(date_time.date().year(), 2000);
        assert_eq!(date_time.date().month(), 1);
        assert_eq!(date_time.date().day(), 1);
        assert_eq!(date_time.time().hour(), 12);
        assert_eq!(date_time.time().minute(), 0);
        assert!(approximately_equal(date_time.time().second(), 0.0));
        assert_eq!(date_time.time_scale(), TimeScale::Tt);
    }

    #[test]
    fn round_trip_gregorian_date_times() {
        let examples = [
            civil_date_time(1957, 10, 4, 19, 26, 24.0),
            civil_date_time(2000, 1, 1, 12, 0, 0.0),
            civil_date_time(2024, 2, 29, 6, 30, 15.5),
            civil_date_time(1900, 1, 1, 0, 0, 0.0),
        ];

        for original in examples {
            let jd = gregorian_to_jd(original);
            let converted =
                jd_to_gregorian(jd, original.time_scale()).expect("Julian Day should convert");

            assert_same_civil_date_time(original, converted);
        }
    }

    #[test]
    fn returns_weekday_for_j2000() {
        assert_eq!(weekday(JulianDate::new(2_451_545.0)), Weekday::Saturday);
    }

    #[test]
    fn returns_weekday_for_meeus_example() {
        assert_eq!(weekday(JulianDate::new(2_436_116.31)), Weekday::Friday);
    }

    #[test]
    fn rejects_non_finite_julian_day() {
        assert_eq!(
            jd_to_gregorian(JulianDate::new(f64::NAN), TimeScale::Utc),
            None
        );
    }

    fn civil_date_time(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f64,
    ) -> CivilDateTime {
        let date = CivilDate::new(year, month, day).expect("date should be valid");
        let time = CivilTime::new(hour, minute, second).expect("time should be valid");

        CivilDateTime::new(date, time, TimeScale::Utc)
    }

    fn assert_same_civil_date_time(expected: CivilDateTime, actual: CivilDateTime) {
        assert_eq!(actual.date(), expected.date());
        assert_eq!(actual.time().hour(), expected.time().hour());
        assert_eq!(actual.time().minute(), expected.time().minute());
        assert!(
            (actual.time().second() - expected.time().second()).abs()
                <= ROUND_TRIP_SECOND_TOLERANCE
        );
        assert_eq!(actual.time_scale(), expected.time_scale());
    }
}
