//! Sunrise validation against the Swiss Ephemeris (`swetest`).
//!
//! Reference convention (approved, Option A): the instant at which the true
//! (unrefracted) altitude of the Sun's centre crosses `-50'`, found by
//! root-finding over `swetest -fTh` altitude series. This is the exact
//! definition encoded by `OFFICIAL_ZENITH_DEGREES = 90.833` in
//! `calendar::sunrise`, so the comparison measures the engine's algorithm
//! against its own definition.
//!
//! Two further instants are recorded as data only, never as reference and
//! never as pass/fail: `swetest -rise` (upper limb with modelled refraction)
//! and `swetest -rise -hindu` (disc centre on the geometric horizon).
//!
//! No pass threshold is applied unless `--tolerance-seconds` is given
//! explicitly. The distribution of differences is the result.

use pancanga_engine::calendar::{sunrise, tithi_at_sunrise, tithi_transition_between_sunrises};
use pancanga_engine::core::time::{gregorian_to_jd, jd_to_gregorian, TimeScale};
use pancanga_engine::core::{
    CivilDate, CivilDateTime, CivilTime, DurationDays, GeoLocation, JulianDate, Latitude, Longitude,
};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_SAMPLE_COUNT: usize = 1_000;
const DEFAULT_START_JD: f64 = 2_415_020.5;
const DEFAULT_END_JD: f64 = 2_488_434.5;
const DEFAULT_CARDINAL_YEAR_STEP_OTHER: i32 = 3;
const REFERENCE_ALTITUDE_DEGREES: f64 = -50.0 / 60.0;
const COARSE_STEP_SECONDS: usize = 60;
const SECONDS_PER_DAY: f64 = 86_400.0;
const BOUNDARY_SEARCH_HOURS: f64 = 3.0;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args(env::args().skip(1).collect())?;
    let mut csv = String::from(
        "case_id,sample_kind,location,latitude,longitude,civil_date,day_of_year,leap_year,\
         engine_sunrise_utc,engine_jd,reference_sunrise_utc,reference_jd,\
         error_seconds,day_shift,swetest_rise_default_utc,default_minus_reference_seconds,\
         swetest_hindu_utc,hindu_minus_reference_seconds,engine_tithi_index,\
         reference_tithi_index,tithi_label_differs,nearest_boundary_utc,\
         boundary_minus_reference_seconds,difference_classification,status,notes\n",
    );

    let Some(swetest) = find_swetest() else {
        csv.push_str(
            "SUNRISE-0000,,,,,,,,,,,,,,,,,,,,,,,EXTERNAL_ORACLE_PENDING,,\
             swetest not found in PATH\n",
        );
        fs::write(&config.output, csv)?;
        println!("External Oracle Pending: swetest not found.");
        println!("CSV: {}", config.output.display());
        return Ok(());
    };
    let ephemeris_path = find_swiss_ephemeris_path();
    let oracle = Oracle {
        swetest,
        ephemeris_path,
    };

    let cases = generate_cases(&config);
    let mut rows = Vec::with_capacity(cases.len());
    for case in cases {
        let row = validate_case(&oracle, case, config.tolerance_seconds);
        csv.push_str(&row.to_csv());
        rows.push(row);
    }

    fs::write(&config.output, csv)?;
    println!("Sunrise validation CSV: {}", config.output.display());
    print_summary(&rows);

    Ok(())
}

struct Config {
    output: PathBuf,
    samples: usize,
    start_jd: f64,
    end_jd: f64,
    tolerance_seconds: Option<f64>,
    cardinal_year_step_other: i32,
}

impl Config {
    fn from_args(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut output = PathBuf::from("../../04_Tests/Astronomy/sunrise-ephemeris-validation.csv");
        let mut samples = DEFAULT_SAMPLE_COUNT;
        let mut start_jd = DEFAULT_START_JD;
        let mut end_jd = DEFAULT_END_JD;
        let mut tolerance_seconds: Option<f64> = None;
        let mut cardinal_year_step_other = DEFAULT_CARDINAL_YEAR_STEP_OTHER;

        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--output" => {
                    index += 1;
                    output = PathBuf::from(required_value(&args, index, "--output")?);
                }
                "--samples" => {
                    index += 1;
                    samples = required_value(&args, index, "--samples")?.parse()?;
                }
                "--start-jd" => {
                    index += 1;
                    start_jd = required_value(&args, index, "--start-jd")?.parse()?;
                }
                "--end-jd" => {
                    index += 1;
                    end_jd = required_value(&args, index, "--end-jd")?.parse()?;
                }
                "--tolerance-seconds" => {
                    index += 1;
                    tolerance_seconds =
                        Some(required_value(&args, index, "--tolerance-seconds")?.parse()?);
                }
                "--cardinal-year-step" => {
                    index += 1;
                    cardinal_year_step_other =
                        required_value(&args, index, "--cardinal-year-step")?.parse()?;
                }
                other => {
                    return Err(format!("unknown argument: {other}").into());
                }
            }
            index += 1;
        }

        if end_jd <= start_jd {
            return Err("--end-jd must be greater than --start-jd".into());
        }
        if let Some(tolerance) = tolerance_seconds {
            if !tolerance.is_finite() || tolerance < 0.0 {
                return Err("--tolerance-seconds must be a finite non-negative value".into());
            }
        }
        if cardinal_year_step_other <= 0 {
            return Err("--cardinal-year-step must be greater than zero".into());
        }

        Ok(Self {
            output,
            samples,
            start_jd,
            end_jd,
            tolerance_seconds,
            cardinal_year_step_other,
        })
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

fn find_swetest() -> Option<String> {
    env::var("SWETEST")
        .ok()
        .filter(|path| !path.trim().is_empty())
        .or_else(|| {
            Command::new("swetest")
                .arg("-h")
                .output()
                .ok()
                .map(|_| String::from("swetest"))
        })
        .or_else(|| {
            [
                "../../04_Tests/Astronomy/SwissEphemeris/bin/swetest",
                "04_Tests/Astronomy/SwissEphemeris/bin/swetest",
            ]
            .into_iter()
            .find(|path| {
                Command::new(path)
                    .arg("-h")
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            })
            .map(String::from)
        })
}

fn find_swiss_ephemeris_path() -> Option<PathBuf> {
    env::var("SWISSEPH_EPHE_PATH")
        .ok()
        .map(PathBuf::from)
        .filter(|path| has_required_ephemeris_files(path))
        .or_else(|| {
            [
                "../../04_Tests/Astronomy/SwissEphemeris/ephe",
                "04_Tests/Astronomy/SwissEphemeris/ephe",
            ]
            .into_iter()
            .map(PathBuf::from)
            .find(|path| has_required_ephemeris_files(path))
        })
}

fn has_required_ephemeris_files(path: &Path) -> bool {
    path.join("sepl_18.se1").is_file() && path.join("semo_18.se1").is_file()
}

#[derive(Clone, Copy)]
struct Site {
    name: &'static str,
    latitude: f64,
    longitude: f64,
    /// Shipped cities sample every year; other sites use the configured step.
    shipped: bool,
}

/// Sample sites. The three shipped cities use the coordinates of `cities()`
/// in `rc1_experience.rs`; the remaining sites are abstract latitude probes on
/// the Greenwich meridian.
fn sites() -> [Site; 9] {
    [
        Site {
            name: "Valencia",
            latitude: 39.4699,
            longitude: -0.3763,
            shipped: true,
        },
        Site {
            name: "Buenos Aires",
            latitude: -34.6037,
            longitude: -58.3816,
            shipped: true,
        },
        Site {
            name: "Nabadwip",
            latitude: 23.4071,
            longitude: 88.3679,
            shipped: true,
        },
        Site {
            name: "Equator 0E",
            latitude: 0.0,
            longitude: 0.0,
            shipped: false,
        },
        Site {
            name: "Lat 55N 0E",
            latitude: 55.0,
            longitude: 0.0,
            shipped: false,
        },
        Site {
            name: "Lat 60N 0E",
            latitude: 60.0,
            longitude: 0.0,
            shipped: false,
        },
        Site {
            name: "Lat 65N 0E",
            latitude: 65.0,
            longitude: 0.0,
            shipped: false,
        },
        Site {
            name: "Lat 66.5N 0E",
            latitude: 66.5,
            longitude: 0.0,
            shipped: false,
        },
        Site {
            name: "Lat 70N 0E",
            latitude: 70.0,
            longitude: 0.0,
            shipped: false,
        },
    ]
}

#[derive(Clone, Copy)]
struct Case {
    id: usize,
    kind: &'static str,
    site: Site,
    date: CivilDate,
}

fn generate_cases(config: &Config) -> Vec<Case> {
    let mut cases = Vec::new();
    let mut id = 0;

    // Cardinal dates: equinoxes and solstices, every year of the range for the
    // shipped cities, every `cardinal_year_step_other` years elsewhere.
    let start_year = jd_to_gregorian(JulianDate::new(config.start_jd), TimeScale::Utc)
        .map(|dt| dt.date().year())
        .unwrap_or(1900);
    // The default end JD is 2101-01-01 00:00 UT (shared with swiss_validation);
    // cardinal sampling covers the years whose 31 December lies inside the range.
    let end_year = jd_to_gregorian(JulianDate::new(config.end_jd - 1.0), TimeScale::Utc)
        .map(|dt| dt.date().year())
        .unwrap_or(2100);
    for site in sites() {
        let step = if site.shipped {
            1
        } else {
            config.cardinal_year_step_other
        };
        let mut year = start_year;
        while year <= end_year {
            for (month, day) in [(3, 20), (6, 21), (9, 22), (12, 21)] {
                if let Some(date) = CivilDate::new(year, month, day) {
                    id += 1;
                    cases.push(Case {
                        id,
                        kind: "cardinal",
                        site,
                        date,
                    });
                }
            }
            year += step;
        }
    }

    // Uniformly spaced civil dates across the range, round-robin over sites.
    if config.samples > 0 {
        let sites = sites();
        let step = if config.samples == 1 {
            0.0
        } else {
            (config.end_jd - config.start_jd) / (config.samples - 1) as f64
        };
        for index in 0..config.samples {
            let jd = JulianDate::new(config.start_jd + step * index as f64);
            let Some(date_time) = jd_to_gregorian(jd, TimeScale::Utc) else {
                continue;
            };
            id += 1;
            cases.push(Case {
                id,
                kind: "uniform",
                site: sites[index % sites.len()],
                date: date_time.date(),
            });
        }
    }

    cases
}

struct Oracle {
    swetest: String,
    ephemeris_path: Option<PathBuf>,
}

impl Oracle {
    fn command(&self) -> Command {
        let mut command = Command::new(&self.swetest);
        command.arg("-eswe").arg("-p0").arg("-head");
        if let Some(path) = &self.ephemeris_path {
            command.arg(format!("-edir{}", path.display()));
        }
        command
    }

    fn run(&self, args: &[String]) -> Result<String, String> {
        let output = self
            .command()
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute swetest: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "swetest exited with status {:?}: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// True altitude of the Sun's centre at `count` instants starting at
    /// `start`, spaced `step_seconds` apart.
    fn altitude_series(
        &self,
        site: Site,
        start: JulianDate,
        step_seconds: usize,
        count: usize,
    ) -> Result<(JulianDate, Vec<f64>), String> {
        let (anchor, date_arg, time_arg) = swetest_instant_args(start)?;
        let args = vec![
            format!("-geopos{},{},0", site.longitude, site.latitude),
            date_arg,
            time_arg,
            format!("-n{count}"),
            format!("-s{}", step_seconds as f64 / SECONDS_PER_DAY),
            String::from("-fTh"),
            String::from("-g,"),
        ];
        let output = self.run(&args)?;
        let mut values = Vec::with_capacity(count);
        for line in output.lines() {
            let Some(last) = line.rsplit(',').next() else {
                continue;
            };
            if let Ok(value) = last.trim().parse::<f64>() {
                values.push(value);
            }
        }
        if values.len() != count {
            return Err(format!(
                "altitude series returned {} values, expected {count}",
                values.len()
            ));
        }
        Ok((anchor, values))
    }

    /// First upward crossing of `REFERENCE_ALTITUDE_DEGREES` inside the
    /// local-mean-time day starting at `day_start`.
    fn reference_sunrise(
        &self,
        site: Site,
        day_start: JulianDate,
    ) -> Result<Option<JulianDate>, String> {
        let coarse_count = (SECONDS_PER_DAY as usize) / COARSE_STEP_SECONDS + 1;
        let (coarse_anchor, coarse) =
            self.altitude_series(site, day_start, COARSE_STEP_SECONDS, coarse_count)?;
        let Some(index) = first_upward_crossing(&coarse) else {
            return Ok(None);
        };
        // Start one second before the bracketing coarse sample and extend one
        // second past the next, so a crossing that sits within floating-point
        // residue of a coarse sample is still bracketed by the fine scan.
        let fine_request = coarse_anchor.add_days(DurationDays::new(
            (((index - 1) * COARSE_STEP_SECONDS) as f64 - 1.0) / SECONDS_PER_DAY,
        ));
        // `fine_start` is the whole-second instant swetest actually used.
        let (fine_start, fine) =
            self.altitude_series(site, fine_request, 1, COARSE_STEP_SECONDS + 3)?;
        let Some(fine_index) = first_upward_crossing(&fine) else {
            return Err("coarse crossing not confirmed by fine scan".to_string());
        };
        let before = fine[fine_index - 1];
        let after = fine[fine_index];
        let fraction = (REFERENCE_ALTITUDE_DEGREES - before) / (after - before);
        let seconds = (fine_index - 1) as f64 + fraction;
        Ok(Some(
            fine_start.add_days(DurationDays::new(seconds / SECONDS_PER_DAY)),
        ))
    }

    /// `swetest -rise` searched from `day_start`; `hindu` selects `-hindu`.
    fn rise(
        &self,
        site: Site,
        day_start: JulianDate,
        hindu: bool,
    ) -> Result<Option<JulianDate>, String> {
        let (_, date_arg, time_arg) = swetest_instant_args(day_start)?;
        let mut args = vec![
            String::from("-rise"),
            format!("-geopos{},{},0", site.longitude, site.latitude),
            date_arg,
            time_arg,
            String::from("-n1"),
        ];
        if hindu {
            args.push(String::from("-hindu"));
        }
        let output = self.run(&args)?;
        let tokens: Vec<&str> = output.split_whitespace().collect();
        let Some(position) = tokens.iter().position(|token| *token == "rise") else {
            return Ok(None);
        };
        let (Some(date), Some(time)) = (tokens.get(position + 1), tokens.get(position + 2)) else {
            return Ok(None);
        };
        if *date == "-" || *time == "-" {
            return Ok(None);
        }
        let jd = parse_swetest_instant(date, time)?;
        // Only accept a rise inside the requested local-mean-time day.
        if jd.days_since(day_start).value() >= 1.0 {
            return Ok(None);
        }
        Ok(Some(jd))
    }
}

fn first_upward_crossing(values: &[f64]) -> Option<usize> {
    (1..values.len()).find(|&index| {
        values[index - 1] <= REFERENCE_ALTITUDE_DEGREES
            && values[index] > REFERENCE_ALTITUDE_DEGREES
    })
}

/// `-b<d.m.y>` and `-ut<h:m:s>` for a UTC instant rounded down to the second,
/// together with the exact whole-second instant those arguments encode. All
/// offsets into a swetest series must be anchored to that instant, not to the
/// unrounded request.
fn swetest_instant_args(instant: JulianDate) -> Result<(JulianDate, String, String), String> {
    // Nudge by half a millisecond so that an instant sitting a hair below a
    // whole second (floating-point residue) floors to that second.
    let nudged = instant.add_days(DurationDays::new(0.000_5 / SECONDS_PER_DAY));
    let date_time = jd_to_gregorian(nudged, TimeScale::Utc)
        .ok_or_else(|| format!("cannot convert JD {} to a civil instant", instant.value()))?;
    let date = date_time.date();
    let time = date_time.time();
    let second = time.second().floor();
    let anchor_time = CivilTime::new(time.hour(), time.minute(), second)
        .ok_or_else(|| format!("invalid anchor time for JD {}", instant.value()))?;
    let anchor = gregorian_to_jd(CivilDateTime::new(date, anchor_time, TimeScale::Utc));
    Ok((
        anchor,
        format!("-b{}.{}.{}", date.day(), date.month(), date.year()),
        format!("-ut{}:{}:{}", time.hour(), time.minute(), second as u8),
    ))
}

fn parse_swetest_instant(date: &str, time: &str) -> Result<JulianDate, String> {
    let mut date_parts = date.split('.');
    let day: u8 = next_number(&mut date_parts, date)?;
    let month: u8 = next_number(&mut date_parts, date)?;
    let year: i32 = next_number(&mut date_parts, date)?;
    let mut time_parts = time.split(':');
    let hour: u8 = next_number(&mut time_parts, time)?;
    let minute: u8 = next_number(&mut time_parts, time)?;
    let second: f64 = next_number(&mut time_parts, time)?;
    let civil_date =
        CivilDate::new(year, month, day).ok_or_else(|| format!("invalid swetest date {date}"))?;
    let civil_time = CivilTime::new(hour, minute, second)
        .ok_or_else(|| format!("invalid swetest time {time}"))?;
    Ok(gregorian_to_jd(CivilDateTime::new(
        civil_date,
        civil_time,
        TimeScale::Utc,
    )))
}

fn next_number<'a, T: std::str::FromStr>(
    parts: &mut impl Iterator<Item = &'a str>,
    source: &str,
) -> Result<T, String> {
    parts
        .next()
        .and_then(|part| part.parse::<T>().ok())
        .ok_or_else(|| format!("cannot parse swetest value {source:?}"))
}

/// Start of the local-mean-time day for a civil date: `0h UT - longitude/15h`.
/// Mean solar time is purely astronomical; no civil offset is involved.
fn local_mean_day_start(date: CivilDate, site: Site) -> JulianDate {
    let midnight = CivilTime::new(0, 0, 0.0).expect("midnight is valid");
    let utc_midnight = gregorian_to_jd(CivilDateTime::new(date, midnight, TimeScale::Utc));
    // Round to a whole second so swetest receives an exact instant.
    let raw = utc_midnight.value() - site.longitude / 360.0;
    let rounded = (raw * SECONDS_PER_DAY).round() / SECONDS_PER_DAY;
    JulianDate::new(rounded)
}

fn location(site: Site) -> Option<GeoLocation> {
    Some(GeoLocation::new(
        Latitude::from_degrees(site.latitude)?,
        Longitude::from_degrees(site.longitude)?,
    ))
}

struct ValidationRow {
    case_id: String,
    kind: &'static str,
    site: Site,
    date: CivilDate,
    day_of_year: u16,
    leap_year: bool,
    engine: Option<JulianDate>,
    reference: Option<JulianDate>,
    error_seconds: Option<f64>,
    day_shift: Option<i64>,
    rise_default: Option<JulianDate>,
    default_minus_reference: Option<f64>,
    rise_hindu: Option<JulianDate>,
    hindu_minus_reference: Option<f64>,
    engine_tithi: Option<u8>,
    reference_tithi: Option<u8>,
    tithi_label_differs: Option<bool>,
    nearest_boundary: Option<JulianDate>,
    boundary_minus_reference: Option<f64>,
    classification: String,
    status: String,
    notes: String,
}

impl ValidationRow {
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            self.case_id,
            self.kind,
            csv_escape(self.site.name),
            self.site.latitude,
            self.site.longitude,
            iso_date(self.date),
            self.day_of_year,
            self.leap_year,
            optional_instant(self.engine),
            optional_jd(self.engine),
            optional_instant(self.reference),
            optional_jd(self.reference),
            optional_f64(self.error_seconds),
            self.day_shift.map(|v| v.to_string()).unwrap_or_default(),
            optional_instant(self.rise_default),
            optional_f64(self.default_minus_reference),
            optional_instant(self.rise_hindu),
            optional_f64(self.hindu_minus_reference),
            self.engine_tithi.map(|v| v.to_string()).unwrap_or_default(),
            self.reference_tithi
                .map(|v| v.to_string())
                .unwrap_or_default(),
            self.tithi_label_differs
                .map(|v| v.to_string())
                .unwrap_or_default(),
            optional_instant(self.nearest_boundary),
            optional_f64(self.boundary_minus_reference),
            self.classification,
            self.status,
            csv_escape(&self.notes),
        )
    }
}

fn validate_case(oracle: &Oracle, case: Case, tolerance_seconds: Option<f64>) -> ValidationRow {
    let case_id = format!("SUNRISE-{id:05}", id = case.id);
    let day_of_year = day_of_year(case.date);
    let leap_year = is_leap_year(case.date.year());
    let mut row = ValidationRow {
        case_id,
        kind: case.kind,
        site: case.site,
        date: case.date,
        day_of_year,
        leap_year,
        engine: None,
        reference: None,
        error_seconds: None,
        day_shift: None,
        rise_default: None,
        default_minus_reference: None,
        rise_hindu: None,
        hindu_minus_reference: None,
        engine_tithi: None,
        reference_tithi: None,
        tithi_label_differs: None,
        nearest_boundary: None,
        boundary_minus_reference: None,
        classification: String::from("UNCLASSIFIED"),
        status: String::new(),
        notes: String::new(),
    };

    let Some(geo) = location(case.site) else {
        row.status = String::from("INVALID_SITE");
        row.notes = String::from("site coordinates rejected by GeoLocation");
        return row;
    };

    row.engine = sunrise(case.date, geo);
    let day_start = local_mean_day_start(case.date, case.site);

    let reference = match oracle.reference_sunrise(case.site, day_start) {
        Ok(value) => value,
        Err(error) => {
            row.status = String::from("EXTERNAL_TOOL_ERROR");
            row.notes = error;
            return row;
        }
    };
    row.reference = reference;

    match oracle.rise(case.site, day_start, false) {
        Ok(value) => row.rise_default = value,
        Err(error) => {
            row.status = String::from("EXTERNAL_TOOL_ERROR");
            row.notes = error;
            return row;
        }
    }
    match oracle.rise(case.site, day_start, true) {
        Ok(value) => row.rise_hindu = value,
        Err(error) => {
            row.status = String::from("EXTERNAL_TOOL_ERROR");
            row.notes = error;
            return row;
        }
    }

    match (row.engine, row.reference) {
        (Some(engine), Some(reference)) => {
            let error = engine.days_since(reference).value() * SECONDS_PER_DAY;
            row.error_seconds = Some(error);
            let shift = (error / SECONDS_PER_DAY).round() as i64;
            row.day_shift = Some(shift);
            row.engine_tithi = Some(tithi_at_sunrise(engine).index());
            row.reference_tithi = Some(tithi_at_sunrise(reference).index());
            row.tithi_label_differs = Some(row.engine_tithi != row.reference_tithi);
            if let Some((boundary, offset)) = nearest_tithi_boundary(reference) {
                row.nearest_boundary = Some(boundary);
                row.boundary_minus_reference = Some(offset);
            }
            row.default_minus_reference = row
                .rise_default
                .map(|value| value.days_since(reference).value() * SECONDS_PER_DAY);
            row.hindu_minus_reference = row
                .rise_hindu
                .map(|value| value.days_since(reference).value() * SECONDS_PER_DAY);
            row.status = String::from("COMPARED");
            if shift != 0 {
                row.notes = format!(
                    "engine instant lies outside the local mean-time day of the civil date (shift {shift} day)"
                );
            }
            if let Some(tolerance) = tolerance_seconds {
                row.classification = if error.abs() <= tolerance {
                    String::from("PASS")
                } else {
                    String::from("SUNRISE DIFFERENCE")
                };
            }
        }
        (None, None) => {
            row.status = String::from("NO_SUNRISE_BOTH");
            row.notes = String::from("neither engine nor reference finds a sunrise");
        }
        (Some(_), None) => {
            row.status = String::from("ENGINE_SUNRISE_REFERENCE_NONE");
            row.notes = String::from("engine returns a sunrise; reference finds no -50' crossing");
        }
        (None, Some(_)) => {
            row.status = String::from("ENGINE_NONE_REFERENCE_SUNRISE");
            row.notes = String::from("engine returns None; reference finds a -50' crossing");
        }
    }

    row
}

/// Nearest tithi boundary within `BOUNDARY_SEARCH_HOURS` of the reference
/// instant, using the engine's own astronomy. Returns the boundary and its
/// offset in seconds relative to the reference (positive = after).
fn nearest_tithi_boundary(reference: JulianDate) -> Option<(JulianDate, f64)> {
    let window = DurationDays::new(BOUNDARY_SEARCH_HOURS / 24.0);
    let before = tithi_transition_between_sunrises(
        reference.add_days(DurationDays::new(-window.value())),
        reference,
    );
    let after = tithi_transition_between_sunrises(reference, reference.add_days(window));
    let offset = |transition: pancanga_engine::calendar::TithiTransition| {
        let instant = transition.instant();
        (
            instant,
            instant.days_since(reference).value() * SECONDS_PER_DAY,
        )
    };
    match (before.map(offset), after.map(offset)) {
        (Some(b), Some(a)) => Some(if b.1.abs() <= a.1.abs() { b } else { a }),
        (Some(b), None) => Some(b),
        (None, Some(a)) => Some(a),
        (None, None) => None,
    }
}

fn print_summary(rows: &[ValidationRow]) {
    let compared: Vec<&ValidationRow> = rows.iter().filter(|r| r.status == "COMPARED").collect();
    let in_day: Vec<&ValidationRow> = compared
        .iter()
        .copied()
        .filter(|r| r.day_shift == Some(0))
        .collect();
    let shifted: Vec<&ValidationRow> = compared
        .iter()
        .copied()
        .filter(|r| r.day_shift != Some(0))
        .collect();
    let mut errors: Vec<f64> = in_day.iter().filter_map(|r| r.error_seconds).collect();
    errors.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let mut abs_errors: Vec<f64> = errors.iter().map(|e| e.abs()).collect();
    abs_errors.sort_by(|a, b| a.partial_cmp(b).expect("finite"));

    println!("rows: {}", rows.len());
    println!("compared: {}", compared.len());
    for status in [
        "NO_SUNRISE_BOTH",
        "ENGINE_SUNRISE_REFERENCE_NONE",
        "ENGINE_NONE_REFERENCE_SUNRISE",
        "EXTERNAL_TOOL_ERROR",
        "INVALID_SITE",
    ] {
        let count = rows.iter().filter(|r| r.status == status).count();
        if count > 0 {
            println!("{status}: {count}");
        }
    }
    println!("compared with day shift != 0: {}", shifted.len());
    if !errors.is_empty() {
        let percentile =
            |values: &[f64], p: f64| values[((values.len() - 1) as f64 * p).round() as usize];
        println!("in-day error (engine - reference), seconds:");
        println!(
            "  min {:.1}  p05 {:.1}  median {:.1}  p95 {:.1}  max {:.1}",
            errors[0],
            percentile(&errors, 0.05),
            percentile(&errors, 0.5),
            percentile(&errors, 0.95),
            errors[errors.len() - 1]
        );
        println!(
            "  |error| median {:.1}  p95 {:.1}  max {:.1}",
            percentile(&abs_errors, 0.5),
            percentile(&abs_errors, 0.95),
            abs_errors[abs_errors.len() - 1]
        );
    }

    let max_abs = abs_errors.last().copied().unwrap_or(0.0);
    let flips: Vec<&ValidationRow> = in_day
        .iter()
        .copied()
        .filter(|r| r.tithi_label_differs == Some(true))
        .collect();
    let potential: Vec<&ValidationRow> = in_day
        .iter()
        .copied()
        .filter(|r| {
            r.boundary_minus_reference
                .is_some_and(|b| b.abs() <= max_abs)
        })
        .collect();
    println!(
        "in-day tithi label flips (engine instant vs reference instant): {}",
        flips.len()
    );
    for r in &flips {
        println!(
            "  {} {} {} error {:+.1}s boundary {:+.1}s engine {} ref {}",
            r.case_id,
            r.site.name,
            iso_date(r.date),
            r.error_seconds.unwrap_or(0.0),
            r.boundary_minus_reference.unwrap_or(f64::NAN),
            r.engine_tithi.unwrap_or(99),
            r.reference_tithi.unwrap_or(99)
        );
    }
    println!("in-day cases with a tithi boundary within +/-{max_abs:.1}s (max |error|) of the reference: {}", potential.len());
    for r in &potential {
        println!(
            "  {} {} {} error {:+.1}s boundary {:+.1}s flips={}",
            r.case_id,
            r.site.name,
            iso_date(r.date),
            r.error_seconds.unwrap_or(0.0),
            r.boundary_minus_reference.unwrap_or(f64::NAN),
            r.tithi_label_differs.unwrap_or(false)
        );
    }
    let shifted_flips = shifted
        .iter()
        .filter(|r| r.tithi_label_differs == Some(true))
        .count();
    println!(
        "day-shifted cases whose tithi label differs: {shifted_flips} of {}",
        shifted.len()
    );

    println!("worst in-day |error| by site:");
    for site in sites() {
        let mut worst: Option<&ValidationRow> = None;
        for r in in_day.iter().copied().filter(|r| r.site.name == site.name) {
            if worst.is_none_or(|w| {
                r.error_seconds.unwrap_or(0.0).abs() > w.error_seconds.unwrap_or(0.0).abs()
            }) {
                worst = Some(r);
            }
        }
        if let Some(w) = worst {
            println!(
                "  {:<14} {:+8.1}s on {} (doy {})",
                site.name,
                w.error_seconds.unwrap_or(0.0),
                iso_date(w.date),
                w.day_of_year
            );
        }
    }
}

fn day_of_year(date: CivilDate) -> u16 {
    let midnight = CivilTime::new(0, 0, 0.0).expect("midnight is valid");
    let jd = gregorian_to_jd(CivilDateTime::new(date, midnight, TimeScale::Utc));
    let first = CivilDate::new(date.year(), 1, 1).expect("January 1 is valid");
    let jd_first = gregorian_to_jd(CivilDateTime::new(first, midnight, TimeScale::Utc));
    (jd.days_since(jd_first).value().round() as u16) + 1
}

fn is_leap_year(year: i32) -> bool {
    pancanga_engine::core::time::is_leap_year(year)
}

fn iso_date(date: CivilDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
}

fn optional_instant(value: Option<JulianDate>) -> String {
    value
        .and_then(|jd| jd_to_gregorian(jd, TimeScale::Utc))
        .map(|dt| {
            format!(
                "{} {:02}:{:02}:{:04.1}",
                iso_date(dt.date()),
                dt.time().hour(),
                dt.time().minute(),
                dt.time().second()
            )
        })
        .unwrap_or_default()
}

fn optional_jd(value: Option<JulianDate>) -> String {
    value
        .map(|jd| format!("{:.9}", jd.value()))
        .unwrap_or_default()
}

fn optional_f64(value: Option<f64>) -> String {
    value.map(|v| format!("{v:.1}")).unwrap_or_default()
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
