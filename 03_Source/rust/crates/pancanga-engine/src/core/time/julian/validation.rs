//! Validation helpers for Gregorian calendar dates.

/// Returns `true` when a Gregorian year is a leap year.
///
/// This is the standard Gregorian leap-year rule:
///
/// - years divisible by 4 are leap years;
/// - years divisible by 100 are not leap years;
/// - years divisible by 400 are leap years.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub(crate) fn days_in_month(year: i32, month: u8) -> Option<u8> {
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return None,
    };

    Some(days)
}

pub(crate) fn is_valid_gregorian_date(year: i32, month: u8, day: u8) -> bool {
    days_in_month(year, month).is_some_and(|max_day| (1..=max_day).contains(&day))
}

#[cfg(test)]
mod tests {
    use super::{days_in_month, is_leap_year, is_valid_gregorian_date};

    #[test]
    fn detects_gregorian_leap_years() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2026));
    }

    #[test]
    fn returns_days_in_month() {
        assert_eq!(days_in_month(2024, 2), Some(29));
        assert_eq!(days_in_month(2026, 2), Some(28));
        assert_eq!(days_in_month(2026, 4), Some(30));
        assert_eq!(days_in_month(2026, 12), Some(31));
        assert_eq!(days_in_month(2026, 13), None);
    }

    #[test]
    fn validates_gregorian_dates() {
        assert!(is_valid_gregorian_date(2024, 2, 29));
        assert!(!is_valid_gregorian_date(2026, 2, 29));
        assert!(!is_valid_gregorian_date(2026, 0, 1));
        assert!(!is_valid_gregorian_date(2026, 1, 0));
    }
}
