use pancanga_engine::core::{DurationDays, JulianDate};

#[test]
fn create_julian_date() {
    let jd = JulianDate::new(2451545.0);

    assert_eq!(jd.value(), 2451545.0);
}

#[test]
fn add_days() {
    let jd = JulianDate::new(2451545.0);

    let next = jd.add_days(DurationDays::new(1.0));

    assert_eq!(next.value(), 2451546.0);
}

#[test]
fn difference() {
    let a = JulianDate::new(2451545.0);
    let b = JulianDate::new(2451547.5);

    assert_eq!(b.days_since(a), DurationDays::new(2.5));
}
