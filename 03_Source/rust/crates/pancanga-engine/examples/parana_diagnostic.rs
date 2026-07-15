use pancanga_engine::calendar::{sunrise, tithi_at_sunrise};
use pancanga_engine::core::math::{
    acos_deg, asin_deg, atan_deg, cos_deg, normalize_360, normalize_hours, sin_deg, tan_deg,
};
use pancanga_engine::core::time::{
    gregorian_to_jd, jd_to_gregorian, CivilDateTime, CivilTime, TimeScale,
};
use pancanga_engine::core::{
    CivilDate, Degrees, DurationDays, GeoLocation, JulianDate, Latitude, Longitude,
};
use pancanga_engine::vaishnava::{hari_vasara_end, parana_window, ParanaMode};

const BUENOS_AIRES_UTC_OFFSET_HOURS: f64 = -3.0;
const OFFICIAL_ZENITH_DEGREES: f64 = 90.833;
const SCAN_STEP_DAYS: f64 = 1.0 / 96.0;
const REFINEMENT_STEPS: usize = 48;

fn main() {
    let location = GeoLocation::new(
        Latitude::from_degrees(-34.616_667).expect("valid latitude"),
        Longitude::from_degrees(-58.383_333).expect("valid longitude"),
    );
    let observance_date = CivilDate::new(2026, 8, 8).expect("valid date");
    let parana_date = CivilDate::new(2026, 8, 9).expect("valid date");
    let sunrise_jd = sunrise(parana_date, location).expect("sunrise");
    let sunset_jd = solar_event(parana_date, location, SolarEvent::Sunset).expect("sunset");
    let daylight_duration = sunset_jd.days_since(sunrise_jd).value();
    let first_third_end = sunrise_jd.add_days(DurationDays::new(daylight_duration / 3.0));
    let (dvadasi_start, dvadasi_end) =
        dvadasi_interval_around(sunrise_jd).expect("dvadasi interval");
    let harivasara_end = hari_vasara_end(dvadasi_start, dvadasi_end).expect("hari-vasara end");
    let window = parana_window(sunrise_jd, dvadasi_start, dvadasi_end).expect("parana window");
    let dvadasi_duration_hours = dvadasi_end.days_since(dvadasi_start).value() * 24.0;

    println!("Campaign 46BETA.2 - Parana Diagnostic");
    println!("Case: PB-0007 / Kamika Ekadasi");
    println!("Location: Buenos Aires, Argentina");
    println!("Observance date: {}", date_string(observance_date));
    println!("Parana date: {}", date_string(parana_date));
    println!("PureBhakti Parana: 07:51-11:13 UTC-3");
    println!();
    println!("sunrise: {}", local_hhmm(sunrise_jd));
    println!("sunset: {}", local_hhmm(sunset_jd));
    println!("first_third_of_day_end: {}", local_hhmm(first_third_end));
    println!("hari_vasara_end: {}", local_hhmm(harivasara_end));
    println!("dvadasi_start: {}", local_datetime(dvadasi_start));
    println!("dvadasi_end: {}", local_datetime(dvadasi_end));
    println!("dvadasi_duration_hours: {:.6}", dvadasi_duration_hours);
    println!("parana_start: {}", local_hhmm(window.starts_at));
    println!("parana_end: {}", local_hhmm(window.ends_at));
    println!(
        "parana_mode: {}",
        match window.mode {
            ParanaMode::Standard => "Standard",
            ParanaMode::ShortDvadasi => "ShortDvadasi",
        }
    );
    println!();
    println!("classification: EDITORIAL_POLICY");
    println!("probable_published_limit: first_third_of_day_end");
    println!("engine_bug: 0");
}

#[derive(Clone, Copy)]
enum SolarEvent {
    Sunset,
}

fn solar_event(date: CivilDate, location: GeoLocation, event: SolarEvent) -> Option<JulianDate> {
    let day = day_of_year(date);
    let longitude = location.longitude().degrees().value();
    let latitude = location.latitude().degrees().value();
    let longitude_hour = longitude / 15.0;
    let approximate_hour = match event {
        SolarEvent::Sunset => 18.0,
    };
    let approximate_time = f64::from(day) + ((approximate_hour - longitude_hour) / 24.0);
    let mean_anomaly = Degrees::new((0.9856 * approximate_time) - 3.289);
    let true_longitude = normalize_360(Degrees::new(
        mean_anomaly.value()
            + (1.916 * sin_deg(mean_anomaly))
            + (0.020 * sin_deg(Degrees::new(2.0 * mean_anomaly.value())))
            + 282.634,
    ));
    let raw_right_ascension = normalize_360(atan_deg(0.91764 * tan_deg(true_longitude)));
    let longitude_quadrant = (true_longitude.value() / 90.0).floor() * 90.0;
    let right_ascension_quadrant = (raw_right_ascension.value() / 90.0).floor() * 90.0;
    let right_ascension_hours =
        (raw_right_ascension.value() + longitude_quadrant - right_ascension_quadrant) / 15.0;
    let sin_declination = 0.39782 * sin_deg(true_longitude);
    let cos_declination = cos_deg(asin_deg(sin_declination));
    let cos_hour_angle = (cos_deg(Degrees::new(OFFICIAL_ZENITH_DEGREES))
        - (sin_declination * sin_deg(Degrees::new(latitude))))
        / (cos_declination * cos_deg(Degrees::new(latitude)));

    if !(-1.0..=1.0).contains(&cos_hour_angle) {
        return None;
    }

    let local_hour_angle = match event {
        SolarEvent::Sunset => acos_deg(cos_hour_angle).value() / 15.0,
    };
    let local_mean_time =
        local_hour_angle + right_ascension_hours - (0.06571 * approximate_time) - 6.622;
    let universal_time = normalize_hours(local_mean_time - longitude_hour);

    Some(julian_date_at_universal_hours(date, universal_time))
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

fn dvadasi_interval_around(anchor: JulianDate) -> Option<(JulianDate, JulianDate)> {
    let scan_start = anchor.add_days(DurationDays::new(-2.5));
    let scan_end = anchor.add_days(DurationDays::new(2.5));
    let mut previous = scan_start;
    let mut previous_is_dvadasi = is_dvadasi(previous);
    let mut start = None;
    let mut value = scan_start.value() + SCAN_STEP_DAYS;

    while value <= scan_end.value() {
        let current = JulianDate::new(value);
        let current_is_dvadasi = is_dvadasi(current);

        if current_is_dvadasi != previous_is_dvadasi {
            let boundary = refine_tithi_predicate_boundary(previous, current, previous_is_dvadasi);
            if !previous_is_dvadasi && current_is_dvadasi {
                start = Some(boundary);
            } else if previous_is_dvadasi && !current_is_dvadasi {
                if let Some(interval_start) = start {
                    return Some((interval_start, boundary));
                }
            }
        }

        previous = current;
        previous_is_dvadasi = current_is_dvadasi;
        value += SCAN_STEP_DAYS;
    }

    None
}

fn refine_tithi_predicate_boundary(
    low: JulianDate,
    high: JulianDate,
    low_is_dvadasi: bool,
) -> JulianDate {
    let mut low_value = low.value();
    let mut high_value = high.value();

    for _ in 0..REFINEMENT_STEPS {
        let midpoint = JulianDate::new((low_value + high_value) / 2.0);
        if is_dvadasi(midpoint) == low_is_dvadasi {
            low_value = midpoint.value();
        } else {
            high_value = midpoint.value();
        }
    }

    JulianDate::new((low_value + high_value) / 2.0)
}

fn is_dvadasi(jd: JulianDate) -> bool {
    tithi_at_sunrise(jd).traditional_number() == 12
}

fn local_hhmm(jd: JulianDate) -> String {
    let date_time = local_datetime_components(jd);
    let time = date_time.time();
    let mut total_minutes = u16::from(time.hour()) * 60 + u16::from(time.minute());
    if time.second() >= 30.0 {
        total_minutes += 1;
    }
    total_minutes %= 24 * 60;

    format!("{:02}:{:02}", total_minutes / 60, total_minutes % 60)
}

fn local_datetime(jd: JulianDate) -> String {
    let date_time = local_datetime_components(jd);
    format!("{} {}", date_string(date_time.date()), local_hhmm(jd))
}

fn local_datetime_components(jd: JulianDate) -> pancanga_engine::core::time::CivilDateTime {
    jd_to_gregorian(
        jd.add_days(DurationDays::new(BUENOS_AIRES_UTC_OFFSET_HOURS / 24.0)),
        TimeScale::Utc,
    )
    .expect("Julian date should convert")
}

fn date_string(date: CivilDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
}
