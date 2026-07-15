use pancanga_engine::calendar::{sunrise, tithi_at_sunrise};
use pancanga_engine::core::time::{jd_to_gregorian, TimeScale};
use pancanga_engine::core::{
    CivilDate, DurationDays, GeoLocation, JulianDate, Latitude, Longitude,
};
use pancanga_engine::vaishnava::{
    arunodaya_start, ekadasi_candidate_at_sunrise, invalidate_viddha_candidate,
    observance_displacement, parana_window, EkadasiObservanceDisposition, ViddhaCandidateStatus,
};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const DEFAULT_INPUT: &str = "../../04_Tests/SCSMath/scsmath-fixtures.csv";
const DEFAULT_OUTPUT: &str = "../../04_Tests/SCSMath/scsmath-validation.csv";
const SCSMATH_UTC_OFFSET_HOURS: f64 = 5.5;
const SCAN_STEP_DAYS: f64 = 1.0 / 48.0;
const REFINEMENT_STEPS: usize = 48;

const OUTPUT_HEADERS: &str = "case_id,fixture_date,fixture_name,fixture_mahadvadasi_type,\
fixture_parana_start,fixture_parana_end,engine_path,engine_tithi_at_sunrise,\
engine_tithi_at_arunodaya,engine_observance_date,engine_parana_start,engine_parana_end,\
observance_status,parana_status,difference_classification,status,notes\n";

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args(env::args().skip(1).collect())?;

    if !config.input.is_file() {
        fs::write(
            &config.output,
            pending_csv("SCS Math fixtures are not available"),
        )?;
        println!("External Oracle Pending: SCS Math fixtures not found.");
        println!("Expected CSV: {}", config.input.display());
        return Ok(());
    }

    let fixtures = read_fixtures(&config.input)?;
    if fixtures.is_empty() {
        fs::write(
            &config.output,
            pending_csv("SCS Math fixture CSV has no rows"),
        )?;
        println!("External Oracle Pending: SCS Math fixture CSV has no rows.");
        return Ok(());
    }

    let mut csv = String::from(OUTPUT_HEADERS);
    let mut pass_count = 0usize;
    let mut difference_count = 0usize;

    for fixture in &fixtures {
        let row = validate_fixture(fixture);
        if row.status == "PASS" {
            pass_count += 1;
        } else {
            difference_count += 1;
        }
        csv.push_str(&row.to_csv());
    }

    fs::write(&config.output, csv)?;
    println!("SCS Math validation rows: {}", fixtures.len());
    println!("PASS: {pass_count}");
    println!("Differences: {difference_count}");
    println!("CSV: {}", config.output.display());

    Ok(())
}

struct Config {
    input: PathBuf,
    output: PathBuf,
}

impl Config {
    fn from_args(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut input = PathBuf::from(DEFAULT_INPUT);
        let mut output = PathBuf::from(DEFAULT_OUTPUT);

        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--input" => {
                    index += 1;
                    input = PathBuf::from(required_value(&args, index, "--input")?);
                }
                "--output" => {
                    index += 1;
                    output = PathBuf::from(required_value(&args, index, "--output")?);
                }
                other => return Err(format!("unknown argument: {other}").into()),
            }
            index += 1;
        }

        Ok(Self { input, output })
    }
}

fn required_value<'a>(
    args: &'a [String],
    index: usize,
    flag: &str,
) -> Result<&'a str, Box<dyn Error>> {
    args.get(index)
        .map(|value| value.as_str())
        .ok_or_else(|| format!("missing value for {flag}").into())
}

#[derive(Debug)]
struct Fixture {
    case_id: String,
    date: CivilDate,
    latitude: f64,
    longitude: f64,
    name: String,
    mahadvadasi_type: String,
    parana_start: String,
    parana_end: String,
}

#[derive(Clone)]
struct DayEvaluation {
    date: CivilDate,
    tithi_at_sunrise: u8,
    tithi_at_arunodaya: u8,
    disposition: EkadasiObservanceDisposition,
}

struct ValidationRow {
    case_id: String,
    fixture_date: String,
    fixture_name: String,
    fixture_mahadvadasi_type: String,
    fixture_parana_start: String,
    fixture_parana_end: String,
    engine_path: String,
    engine_tithi_at_sunrise: String,
    engine_tithi_at_arunodaya: String,
    engine_observance_date: String,
    engine_parana_start: String,
    engine_parana_end: String,
    observance_status: String,
    parana_status: String,
    difference_classification: String,
    status: String,
    notes: String,
}

impl ValidationRow {
    fn to_csv(&self) -> String {
        [
            self.case_id.as_str(),
            self.fixture_date.as_str(),
            self.fixture_name.as_str(),
            self.fixture_mahadvadasi_type.as_str(),
            self.fixture_parana_start.as_str(),
            self.fixture_parana_end.as_str(),
            self.engine_path.as_str(),
            self.engine_tithi_at_sunrise.as_str(),
            self.engine_tithi_at_arunodaya.as_str(),
            self.engine_observance_date.as_str(),
            self.engine_parana_start.as_str(),
            self.engine_parana_end.as_str(),
            self.observance_status.as_str(),
            self.parana_status.as_str(),
            self.difference_classification.as_str(),
            self.status.as_str(),
            self.notes.as_str(),
        ]
        .into_iter()
        .map(csv_escape)
        .collect::<Vec<_>>()
        .join(",")
            + "\n"
    }
}

fn read_fixtures(path: &PathBuf) -> Result<Vec<Fixture>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let mut lines = content.lines();
    let Some(header_line) = lines.next() else {
        return Ok(Vec::new());
    };
    let headers = parse_csv_line(header_line);
    let mut fixtures = Vec::new();

    for line in lines.filter(|line| !line.trim().is_empty()) {
        let columns = parse_csv_line(line);
        let fixture = Fixture {
            case_id: column(&headers, &columns, "case_id"),
            date: parse_date(&column(&headers, &columns, "date"))?,
            latitude: column(&headers, &columns, "latitude").parse()?,
            longitude: column(&headers, &columns, "longitude").parse()?,
            name: column(&headers, &columns, "name"),
            mahadvadasi_type: column(&headers, &columns, "mahadvadasi_type"),
            parana_start: column(&headers, &columns, "parana_start"),
            parana_end: column(&headers, &columns, "parana_end"),
        };
        fixtures.push(fixture);
    }

    Ok(fixtures)
}

fn validate_fixture(fixture: &Fixture) -> ValidationRow {
    let location = match location(fixture.latitude, fixture.longitude) {
        Some(location) => location,
        None => return error_row(fixture, "UNKNOWN", "Invalid fixture coordinates"),
    };
    let current = match evaluate_day(fixture.date, location) {
        Some(day) => day,
        None => return error_row(fixture, "CALENDAR", "Sunrise unavailable for fixture date"),
    };
    let previous_date = previous_day(fixture.date);
    let previous = match evaluate_day(previous_date, location) {
        Some(day) => day,
        None => return error_row(fixture, "CALENDAR", "Sunrise unavailable for previous date"),
    };

    let (engine_path, engine_observance_date, observance_status, observance_notes) =
        observance_match(fixture.date, &current, &previous);
    let (engine_parana_start, engine_parana_end, parana_status, parana_notes) =
        validate_parana(fixture, location, engine_observance_date);
    let mahadvadasi_note = mahadvadasi_note(fixture);

    let difference_classification = if observance_status != "PASS" {
        "CALENDAR DIFFERENCE"
    } else if parana_status != "PASS" {
        "CONFIGURATION DIFFERENCE"
    } else if mahadvadasi_note.is_empty() {
        "PASS"
    } else {
        "CONFIGURATION DIFFERENCE"
    };
    let status = if difference_classification == "PASS" {
        "PASS"
    } else {
        "DIFFERENCE"
    };

    ValidationRow {
        case_id: fixture.case_id.clone(),
        fixture_date: date_string(fixture.date),
        fixture_name: fixture.name.clone(),
        fixture_mahadvadasi_type: fixture.mahadvadasi_type.clone(),
        fixture_parana_start: fixture.parana_start.clone(),
        fixture_parana_end: fixture.parana_end.clone(),
        engine_path,
        engine_tithi_at_sunrise: current.tithi_at_sunrise.to_string(),
        engine_tithi_at_arunodaya: current.tithi_at_arunodaya.to_string(),
        engine_observance_date: engine_observance_date
            .map(date_string)
            .unwrap_or_else(String::new),
        engine_parana_start,
        engine_parana_end,
        observance_status,
        parana_status,
        difference_classification: difference_classification.to_string(),
        status: status.to_string(),
        notes: join_notes([observance_notes, parana_notes, mahadvadasi_note]),
    }
}

fn evaluate_day(date: CivilDate, location: GeoLocation) -> Option<DayEvaluation> {
    let sunrise = sunrise(date, location)?;
    let sunrise_tithi = tithi_at_sunrise(sunrise);
    let arunodaya_tithi = tithi_at_sunrise(arunodaya_start(sunrise));
    let candidate = ekadasi_candidate_at_sunrise(sunrise_tithi);
    let viddha = invalidate_viddha_candidate(candidate, arunodaya_tithi);
    let disposition = observance_displacement(viddha);

    Some(DayEvaluation {
        date,
        tithi_at_sunrise: sunrise_tithi.traditional_number(),
        tithi_at_arunodaya: arunodaya_tithi.traditional_number(),
        disposition,
    })
}

fn observance_match(
    fixture_date: CivilDate,
    current: &DayEvaluation,
    previous: &DayEvaluation,
) -> (String, Option<CivilDate>, String, String) {
    if current.disposition == EkadasiObservanceDisposition::ObserveOnEkadasi {
        return (
            "current_valid_ekadasi".to_string(),
            Some(current.date),
            "PASS".to_string(),
            "SCS Math date matches current-day valid Ekadasi observance".to_string(),
        );
    }

    if previous.disposition == EkadasiObservanceDisposition::ObserveOnDvadasi {
        return (
            "previous_viddha_displacement".to_string(),
            Some(fixture_date),
            "PASS".to_string(),
            "SCS Math date matches Dvadasi observance displaced from previous-day Viddha candidate"
                .to_string(),
        );
    }

    (
        "no_matching_observance_path".to_string(),
        None,
        "DIFFERENCE".to_string(),
        format!(
            "No current Ekadasi observance or previous-day Viddha displacement; previous status was {}",
            disposition_name(previous.disposition)
        ),
    )
}

fn validate_parana(
    fixture: &Fixture,
    location: GeoLocation,
    engine_observance_date: Option<CivilDate>,
) -> (String, String, String, String) {
    let Some(observance_date) = engine_observance_date else {
        return (
            String::new(),
            String::new(),
            "NOT_COMPARED".to_string(),
            "Parana not compared because observance date did not match".to_string(),
        );
    };
    let parana_date = next_day(observance_date);
    let Some(parana_sunrise) = sunrise(parana_date, location) else {
        return (
            String::new(),
            String::new(),
            "DIFFERENCE".to_string(),
            "Sunrise unavailable for Parana date".to_string(),
        );
    };
    let Some((dvadasi_start, dvadasi_end)) = dvadasi_interval_around(parana_sunrise) else {
        return (
            String::new(),
            String::new(),
            "DIFFERENCE".to_string(),
            "Dvadasi interval could not be located around Parana date".to_string(),
        );
    };
    let Ok(window) = parana_window(parana_sunrise, dvadasi_start, dvadasi_end) else {
        return (
            String::new(),
            String::new(),
            "DIFFERENCE".to_string(),
            "Parana window inputs were rejected by HBV-EK-005".to_string(),
        );
    };

    let engine_start = local_hhmm(window.starts_at, SCSMATH_UTC_OFFSET_HOURS);
    let engine_end = local_hhmm(window.ends_at, SCSMATH_UTC_OFFSET_HOURS);
    let status = if engine_start == fixture.parana_start && engine_end == fixture.parana_end {
        "PASS"
    } else {
        "DIFFERENCE"
    };
    let notes = if status == "PASS" {
        "SCS Math Parana window matches engine-calculated HBV-EK-005 window".to_string()
    } else {
        format!(
            "SCS Math Parana {}-{} differs from engine {}-{}; classify before changing code",
            fixture.parana_start, fixture.parana_end, engine_start, engine_end
        )
    };

    (engine_start, engine_end, status.to_string(), notes)
}

fn dvadasi_interval_around(anchor: JulianDate) -> Option<(JulianDate, JulianDate)> {
    let scan_start = anchor.add_days(DurationDays::new(-2.5));
    let scan_end = anchor.add_days(DurationDays::new(2.5));
    let mut previous = scan_start;
    let mut previous_is_dvadasi = is_dvadasi(previous);
    let mut dvadasi_start = None;

    let mut value = scan_start.value() + SCAN_STEP_DAYS;
    while value <= scan_end.value() {
        let current = JulianDate::new(value);
        let current_is_dvadasi = is_dvadasi(current);

        if current_is_dvadasi != previous_is_dvadasi {
            let boundary = refine_tithi_predicate_boundary(previous, current, previous_is_dvadasi);
            if !previous_is_dvadasi && current_is_dvadasi {
                dvadasi_start = Some(boundary);
            } else if previous_is_dvadasi && !current_is_dvadasi {
                if let Some(start) = dvadasi_start {
                    return Some((start, boundary));
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

fn mahadvadasi_note(fixture: &Fixture) -> String {
    if fixture.mahadvadasi_type == "None" {
        String::new()
    } else {
        format!(
            "SCS Math marks Mahadvadasi type {}; automatic Mahadvadasi fact derivation is not part of this validator",
            fixture.mahadvadasi_type
        )
    }
}

fn error_row(fixture: &Fixture, classification: &str, notes: &str) -> ValidationRow {
    ValidationRow {
        case_id: fixture.case_id.clone(),
        fixture_date: date_string(fixture.date),
        fixture_name: fixture.name.clone(),
        fixture_mahadvadasi_type: fixture.mahadvadasi_type.clone(),
        fixture_parana_start: fixture.parana_start.clone(),
        fixture_parana_end: fixture.parana_end.clone(),
        engine_path: String::new(),
        engine_tithi_at_sunrise: String::new(),
        engine_tithi_at_arunodaya: String::new(),
        engine_observance_date: String::new(),
        engine_parana_start: String::new(),
        engine_parana_end: String::new(),
        observance_status: "DIFFERENCE".to_string(),
        parana_status: "NOT_COMPARED".to_string(),
        difference_classification: classification.to_string(),
        status: "DIFFERENCE".to_string(),
        notes: notes.to_string(),
    }
}

fn pending_csv(notes: &str) -> String {
    format!(
        "{OUTPUT_HEADERS}SCS-0000,,,,,,,,,,,,,,EXTERNAL_ORACLE_PENDING,EXTERNAL_ORACLE_PENDING,{notes}\n"
    )
}

fn location(latitude: f64, longitude: f64) -> Option<GeoLocation> {
    Some(GeoLocation::new(
        Latitude::from_degrees(latitude)?,
        Longitude::from_degrees(longitude)?,
    ))
}

fn local_hhmm(jd: JulianDate, utc_offset_hours: f64) -> String {
    let local_jd = jd.add_days(DurationDays::new(utc_offset_hours / 24.0));
    let Some(date_time) = jd_to_gregorian(local_jd, TimeScale::Utc) else {
        return String::new();
    };
    let time = date_time.time();
    let mut total_minutes = u16::from(time.hour()) * 60 + u16::from(time.minute());
    if time.second() >= 30.0 {
        total_minutes += 1;
    }
    total_minutes %= 24 * 60;

    format!("{:02}:{:02}", total_minutes / 60, total_minutes % 60)
}

fn parse_date(value: &str) -> Result<CivilDate, Box<dyn Error>> {
    let parts = value
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<u16>, _>>()?;
    if parts.len() != 3 {
        return Err(format!("invalid date: {value}").into());
    }

    CivilDate::new(parts[0] as i32, parts[1] as u8, parts[2] as u8)
        .ok_or_else(|| format!("invalid date: {value}").into())
}

fn date_string(date: CivilDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
}

fn next_day(date: CivilDate) -> CivilDate {
    let mut year = date.year();
    let mut month = date.month();
    let mut day = date.day() + 1;
    if CivilDate::new(year, month, day).is_none() {
        day = 1;
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
    }

    CivilDate::new(year, month, day).expect("next day should be a valid Gregorian date")
}

fn previous_day(date: CivilDate) -> CivilDate {
    if date.day() > 1 {
        return CivilDate::new(date.year(), date.month(), date.day() - 1)
            .expect("previous day should be valid");
    }

    let mut year = date.year();
    let mut month = date.month();
    if month == 1 {
        year -= 1;
        month = 12;
    } else {
        month -= 1;
    }
    let day = days_in_month(year, month);

    CivilDate::new(year, month, day).expect("previous day should be valid")
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

fn disposition_name(disposition: EkadasiObservanceDisposition) -> &'static str {
    match disposition {
        EkadasiObservanceDisposition::ObserveOnEkadasi => "ObserveOnEkadasi",
        EkadasiObservanceDisposition::ObserveOnDvadasi => "ObserveOnDvadasi",
        EkadasiObservanceDisposition::NoObservanceCandidate => "NoObservanceCandidate",
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut value = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                value.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                values.push(value);
                value = String::new();
            }
            _ => value.push(ch),
        }
    }

    values.push(value);
    values
}

fn column(headers: &[String], columns: &[String], name: &str) -> String {
    headers
        .iter()
        .position(|header| header == name)
        .and_then(|index| columns.get(index))
        .cloned()
        .unwrap_or_default()
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn join_notes(notes: [String; 3]) -> String {
    notes
        .into_iter()
        .filter(|note| !note.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" | ")
}

#[allow(dead_code)]
fn _viddha_name(status: ViddhaCandidateStatus) -> &'static str {
    match status {
        ViddhaCandidateStatus::NotCandidate => "NotCandidate",
        ViddhaCandidateStatus::ValidCandidate => "ValidCandidate",
        ViddhaCandidateStatus::InvalidViddha => "InvalidViddha",
    }
}
