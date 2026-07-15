use pancanga_engine::astronomy::{lunar_solar_elongation, moon, solar, tithi_index};
use pancanga_engine::core::{Degrees, JulianDate};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_SAMPLE_COUNT: usize = 1_000;
const DEFAULT_START_JD: f64 = 2_415_020.5;
const DEFAULT_END_JD: f64 = 2_488_434.5;
const DEFAULT_TOLERANCE_DEGREES: f64 = 0.01;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args(env::args().skip(1).collect())?;
    let mut csv = String::from(
        "case_id,julian_day,pancanga_sun_longitude,swiss_sun_longitude,\
         sun_error_degrees,pancanga_moon_longitude,swiss_moon_longitude,\
         moon_error_degrees,pancanga_elongation,swiss_elongation,\
         elongation_error_degrees,pancanga_tithi,swiss_tithi,\
         difference_classification,status,notes\n",
    );

    let Some(swetest) = find_swetest() else {
        csv.push_str(
            "SWISS-0000,,,,,,,,,,,,,,EXTERNAL_ORACLE_PENDING,\
             swetest not found in PATH\n",
        );
        fs::write(&config.output, csv)?;
        println!("External Oracle Pending: swetest not found.");
        println!("CSV: {}", config.output.display());
        return Ok(());
    };
    let ephemeris_path = find_swiss_ephemeris_path();

    for case in generate_cases(config.samples, config.start_jd, config.end_jd) {
        let row = validate_case(
            &swetest,
            ephemeris_path.as_deref(),
            case,
            config.tolerance_degrees,
        );
        csv.push_str(&row.to_csv());
    }

    fs::write(&config.output, csv)?;
    println!("Swiss validation CSV: {}", config.output.display());

    Ok(())
}

struct Config {
    output: PathBuf,
    samples: usize,
    start_jd: f64,
    end_jd: f64,
    tolerance_degrees: f64,
}

impl Config {
    fn from_args(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut output =
            PathBuf::from("../../04_Tests/Astronomy/SwissEphemeris/swiss-validation.csv");
        let mut samples = DEFAULT_SAMPLE_COUNT;
        let mut start_jd = DEFAULT_START_JD;
        let mut end_jd = DEFAULT_END_JD;
        let mut tolerance_degrees = DEFAULT_TOLERANCE_DEGREES;

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
                "--tolerance-degrees" => {
                    index += 1;
                    tolerance_degrees =
                        required_value(&args, index, "--tolerance-degrees")?.parse()?;
                }
                other => {
                    return Err(format!("unknown argument: {other}").into());
                }
            }
            index += 1;
        }

        if samples == 0 {
            return Err("--samples must be greater than zero".into());
        }
        if end_jd <= start_jd {
            return Err("--end-jd must be greater than --start-jd".into());
        }
        if !tolerance_degrees.is_finite() || tolerance_degrees < 0.0 {
            return Err("--tolerance-degrees must be a finite non-negative value".into());
        }

        Ok(Self {
            output,
            samples,
            start_jd,
            end_jd,
            tolerance_degrees,
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
struct Case {
    id: usize,
    jd: JulianDate,
}

fn generate_cases(samples: usize, start_jd: f64, end_jd: f64) -> Vec<Case> {
    if samples == 1 {
        return vec![Case {
            id: 1,
            jd: JulianDate::new(start_jd),
        }];
    }

    let step = (end_jd - start_jd) / (samples - 1) as f64;

    (0..samples)
        .map(|index| Case {
            id: index + 1,
            jd: JulianDate::new(start_jd + step * index as f64),
        })
        .collect()
}

struct ValidationRow {
    case_id: String,
    julian_day: f64,
    pancanga_sun: Option<f64>,
    swiss_sun: Option<f64>,
    sun_error: Option<f64>,
    pancanga_moon: Option<f64>,
    swiss_moon: Option<f64>,
    moon_error: Option<f64>,
    pancanga_elongation: Option<f64>,
    swiss_elongation: Option<f64>,
    elongation_error: Option<f64>,
    pancanga_tithi: Option<u8>,
    swiss_tithi: Option<u8>,
    classification: String,
    status: String,
    notes: String,
}

impl ValidationRow {
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            self.case_id,
            self.julian_day,
            optional_f64(self.pancanga_sun),
            optional_f64(self.swiss_sun),
            optional_f64(self.sun_error),
            optional_f64(self.pancanga_moon),
            optional_f64(self.swiss_moon),
            optional_f64(self.moon_error),
            optional_f64(self.pancanga_elongation),
            optional_f64(self.swiss_elongation),
            optional_f64(self.elongation_error),
            optional_u8(self.pancanga_tithi),
            optional_u8(self.swiss_tithi),
            self.classification,
            self.status,
            csv_escape(&self.notes),
        )
    }
}

fn validate_case(
    swetest: &str,
    ephemeris_path: Option<&Path>,
    case: Case,
    tolerance_degrees: f64,
) -> ValidationRow {
    let case_id = format!("SWISS-{id:04}", id = case.id);
    let jd = case.jd;
    let pancanga_sun = solar::apparent_longitude(jd).value();
    let pancanga_moon = moon::apparent_longitude(jd).value();
    let pancanga_elongation =
        lunar_solar_elongation(Degrees::new(pancanga_moon), Degrees::new(pancanga_sun)).value();
    let pancanga_tithi = tithi_index(Degrees::new(pancanga_elongation));

    let swiss_sun = match swetest_longitude(swetest, ephemeris_path, jd, "0") {
        Ok(value) => value,
        Err(error) => {
            return external_tool_error(case_id, jd.value(), error);
        }
    };
    let swiss_moon = match swetest_longitude(swetest, ephemeris_path, jd, "1") {
        Ok(value) => value,
        Err(error) => {
            return external_tool_error(case_id, jd.value(), error);
        }
    };

    let swiss_elongation = normalize_360(swiss_moon - swiss_sun);
    let swiss_tithi = tithi_index(Degrees::new(swiss_elongation));
    let sun_error = circular_error(pancanga_sun, swiss_sun);
    let moon_error = circular_error(pancanga_moon, swiss_moon);
    let elongation_error = circular_error(pancanga_elongation, swiss_elongation);

    let classification = if sun_error <= tolerance_degrees
        && moon_error <= tolerance_degrees
        && elongation_error <= tolerance_degrees
        && pancanga_tithi == swiss_tithi
    {
        "PASS"
    } else {
        "ASTRONOMY DIFFERENCE"
    };

    ValidationRow {
        case_id,
        julian_day: jd.value(),
        pancanga_sun: Some(pancanga_sun),
        swiss_sun: Some(swiss_sun),
        sun_error: Some(sun_error),
        pancanga_moon: Some(pancanga_moon),
        swiss_moon: Some(swiss_moon),
        moon_error: Some(moon_error),
        pancanga_elongation: Some(pancanga_elongation),
        swiss_elongation: Some(swiss_elongation),
        elongation_error: Some(elongation_error),
        pancanga_tithi: Some(pancanga_tithi),
        swiss_tithi: Some(swiss_tithi),
        classification: String::from(classification),
        status: String::from("COMPARED"),
        notes: String::from(if ephemeris_path.is_some() {
            "automatic Swiss Ephemeris comparison with explicit ephemeris path"
        } else {
            "automatic Swiss Ephemeris comparison without explicit ephemeris path"
        }),
    }
}

fn swetest_longitude(
    swetest: &str,
    ephemeris_path: Option<&Path>,
    jd: JulianDate,
    planet: &str,
) -> Result<f64, String> {
    let mut args = vec![
        format!("-bj{:.9}", jd.value()),
        format!("-p{planet}"),
        String::from("-fPl"),
        String::from("-g,"),
        String::from("-head"),
        String::from("-eswe"),
    ];
    if let Some(path) = ephemeris_path {
        args.push(format!("-edir{}", path.display()));
    }

    let output = Command::new(swetest)
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

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter_map(|token| token.parse::<f64>().ok())
        .last()
        .ok_or_else(|| format!("could not parse longitude from swetest output: {stdout:?}"))
}

fn external_tool_error(case_id: String, julian_day: f64, notes: String) -> ValidationRow {
    ValidationRow {
        case_id,
        julian_day,
        pancanga_sun: None,
        swiss_sun: None,
        sun_error: None,
        pancanga_moon: None,
        swiss_moon: None,
        moon_error: None,
        pancanga_elongation: None,
        swiss_elongation: None,
        elongation_error: None,
        pancanga_tithi: None,
        swiss_tithi: None,
        classification: String::from("EXTERNAL TOOL ERROR"),
        status: String::from("FAILED"),
        notes,
    }
}

fn circular_error(left: f64, right: f64) -> f64 {
    let difference = (normalize_360(left) - normalize_360(right)).abs();
    difference.min(360.0 - difference)
}

fn normalize_360(value: f64) -> f64 {
    value.rem_euclid(360.0)
}

fn optional_f64(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.12}"))
        .unwrap_or_default()
}

fn optional_u8(value: Option<u8>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
