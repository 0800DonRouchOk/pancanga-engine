use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const REQUIRED_HEADERS: [&str; 13] = [
    "case_id",
    "category",
    "civil_date",
    "location",
    "timezone",
    "sunrise",
    "ekadasi",
    "viddha",
    "mahadvadasi",
    "parana",
    "gcal_source",
    "status",
    "notes",
];

const DEFAULT_OUTPUT: &str = "../../04_Tests/Vaishnava/GCal/gcal-fixtures.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args(env::args().skip(1).collect())?;
    let input = config.input.or_else(find_default_source);

    let rows = match input {
        Some(path) => import_source(&path)?,
        None => {
            write_pending_template(&config.output)?;
            println!("External Oracle Pending: no local GCal source found.");
            println!("Expected source directory: ../../02_Research/GCal");
            println!("CSV: {}", config.output.display());
            return Ok(());
        }
    };

    if rows.is_empty() {
        write_pending_template(&config.output)?;
        println!("External Oracle Pending: source contained no importable fixtures.");
        println!("CSV: {}", config.output.display());
        return Ok(());
    }

    write_fixture_rows(&config.output, &rows)?;
    println!("GCal fixtures imported: {}", rows.len());
    println!("CSV: {}", config.output.display());

    Ok(())
}

struct Config {
    input: Option<PathBuf>,
    output: PathBuf,
}

impl Config {
    fn from_args(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut input = None;
        let mut output = PathBuf::from(DEFAULT_OUTPUT);

        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--input" => {
                    index += 1;
                    input = Some(PathBuf::from(required_value(&args, index, "--input")?));
                }
                "--output" => {
                    index += 1;
                    output = PathBuf::from(required_value(&args, index, "--output")?);
                }
                other => {
                    return Err(format!("unknown argument: {other}").into());
                }
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

fn find_default_source() -> Option<PathBuf> {
    ["../../02_Research/GCal", "02_Research/GCal"]
        .into_iter()
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
        .flat_map(|path| fs::read_dir(path).into_iter().flatten().flatten())
        .map(|entry| entry.path())
        .find(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .map(|extension| {
                    extension.eq_ignore_ascii_case("ics") || extension.eq_ignore_ascii_case("csv")
                })
                .unwrap_or(false)
        })
}

fn import_source(path: &Path) -> Result<Vec<FixtureRow>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    match path.extension().and_then(|extension| extension.to_str()) {
        Some(extension) if extension.eq_ignore_ascii_case("ics") => Ok(import_ics(path, &content)),
        Some(extension) if extension.eq_ignore_ascii_case("csv") => import_csv(path, &content),
        _ => Err(format!("unsupported GCal source format: {}", path.display()).into()),
    }
}

fn import_csv(path: &Path, content: &str) -> Result<Vec<FixtureRow>, Box<dyn Error>> {
    let mut lines = content.lines().filter(|line| !line.trim().is_empty());
    let Some(header_line) = lines.next() else {
        return Ok(Vec::new());
    };
    let headers = parse_csv_record(header_line);
    let normalized_headers: Vec<String> = headers
        .iter()
        .map(|header| normalize_header(header))
        .collect();

    if REQUIRED_HEADERS
        .iter()
        .all(|required| normalized_headers.iter().any(|header| header == required))
    {
        return lines
            .enumerate()
            .map(|(index, line)| fixture_from_fixture_csv(index + 1, &headers, line))
            .collect::<Result<Vec<_>, _>>();
    }

    if has_event_csv_headers(&normalized_headers) {
        return lines
            .enumerate()
            .map(|(index, line)| fixture_from_event_csv(index + 1, path, &headers, line))
            .collect::<Result<Vec<_>, _>>();
    }

    Err(format!(
        "unsupported GCal CSV schema in {}. Expected fixture columns or event columns.",
        path.display()
    )
    .into())
}

fn has_event_csv_headers(headers: &[String]) -> bool {
    headers
        .iter()
        .any(|header| header == "summary" || header == "title")
        && headers
            .iter()
            .any(|header| header == "date" || header == "civil_date")
}

fn fixture_from_fixture_csv(
    row_number: usize,
    headers: &[String],
    line: &str,
) -> Result<FixtureRow, Box<dyn Error>> {
    let values = parse_csv_record(line);
    let row = indexed_row(headers, &values);
    Ok(FixtureRow {
        case_id: field(&row, "case_id").unwrap_or_else(|| format!("GCAL-{row_number:04}")),
        category: field(&row, "category").unwrap_or_default(),
        civil_date: field(&row, "civil_date").unwrap_or_default(),
        location: field(&row, "location").unwrap_or_default(),
        timezone: field(&row, "timezone").unwrap_or_default(),
        sunrise: field(&row, "sunrise").unwrap_or_default(),
        ekadasi: field(&row, "ekadasi").unwrap_or_default(),
        viddha: field(&row, "viddha").unwrap_or_default(),
        mahadvadasi: field(&row, "mahadvadasi").unwrap_or_default(),
        parana: field(&row, "parana").unwrap_or_default(),
        gcal_source: field(&row, "gcal_source").unwrap_or_default(),
        status: field(&row, "status").unwrap_or_else(|| String::from("IMPORTED_REQUIRES_REVIEW")),
        notes: field(&row, "notes").unwrap_or_default(),
    })
}

fn fixture_from_event_csv(
    row_number: usize,
    path: &Path,
    headers: &[String],
    line: &str,
) -> Result<FixtureRow, Box<dyn Error>> {
    let values = parse_csv_record(line);
    let row = indexed_row(headers, &values);
    let summary = field(&row, "summary")
        .or_else(|| field(&row, "title"))
        .unwrap_or_default();
    let description = field(&row, "description").unwrap_or_default();
    let text = format!("{summary} {description}");

    Ok(FixtureRow {
        case_id: format!("GCAL-IMPORT-{row_number:04}"),
        category: category_from_text(&text),
        civil_date: field(&row, "civil_date")
            .or_else(|| field(&row, "date"))
            .unwrap_or_default(),
        location: field(&row, "location").unwrap_or_default(),
        timezone: field(&row, "timezone").unwrap_or_default(),
        sunrise: field(&row, "sunrise").unwrap_or_default(),
        ekadasi: bool_field(contains_normalized(&text, "ekadasi")),
        viddha: bool_field(contains_normalized(&text, "viddha")),
        mahadvadasi: mahadvadasi_from_text(&text).unwrap_or_default(),
        parana: parana_from_text(&text),
        gcal_source: field(&row, "source").unwrap_or_else(|| path.display().to_string()),
        status: String::from("IMPORTED_REQUIRES_REVIEW"),
        notes: summary,
    })
}

fn import_ics(path: &Path, content: &str) -> Vec<FixtureRow> {
    let unfolded = unfold_ics_lines(content);
    let mut rows = Vec::new();
    let mut current = HashMap::new();
    let mut in_event = false;

    for line in unfolded {
        match line.as_str() {
            "BEGIN:VEVENT" => {
                current.clear();
                in_event = true;
            }
            "END:VEVENT" if in_event => {
                let row_number = rows.len() + 1;
                rows.push(fixture_from_ics_event(row_number, path, &current));
                current.clear();
                in_event = false;
            }
            _ if in_event => {
                for (key, value) in parse_ics_property(&line) {
                    current.insert(key, value);
                }
            }
            _ => {}
        }
    }

    rows
}

fn fixture_from_ics_event(
    row_number: usize,
    path: &Path,
    event: &HashMap<String, String>,
) -> FixtureRow {
    let summary = event.get("SUMMARY").cloned().unwrap_or_default();
    let description = event.get("DESCRIPTION").cloned().unwrap_or_default();
    let text = format!("{summary} {description}");
    let source = event
        .get("URL")
        .or_else(|| event.get("UID"))
        .cloned()
        .unwrap_or_else(|| path.display().to_string());

    FixtureRow {
        case_id: format!("GCAL-ICS-{row_number:04}"),
        category: category_from_text(&text),
        civil_date: event
            .get("DTSTART")
            .map(|value| format_ics_date(value))
            .unwrap_or_default(),
        location: event.get("LOCATION").cloned().unwrap_or_default(),
        timezone: event.get("TZID").cloned().unwrap_or_default(),
        sunrise: String::new(),
        ekadasi: bool_field(contains_normalized(&text, "ekadasi")),
        viddha: bool_field(contains_normalized(&text, "viddha")),
        mahadvadasi: mahadvadasi_from_text(&text).unwrap_or_default(),
        parana: parana_from_text(&text),
        gcal_source: source,
        status: String::from("IMPORTED_REQUIRES_REVIEW"),
        notes: summary,
    }
}

fn indexed_row(headers: &[String], values: &[String]) -> HashMap<String, String> {
    headers
        .iter()
        .enumerate()
        .map(|(index, header)| {
            (
                normalize_header(header),
                values.get(index).cloned().unwrap_or_default(),
            )
        })
        .collect()
}

fn field(row: &HashMap<String, String>, key: &str) -> Option<String> {
    row.get(key)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_csv_record(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;

    while let Some(ch) = chars.next() {
        match ch {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(field.trim().to_string());
                field.clear();
            }
            _ => field.push(ch),
        }
    }

    fields.push(field.trim().to_string());
    fields
}

fn normalize_header(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace([' ', '-'], "_")
}

fn unfold_ics_lines(content: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for raw_line in content.lines() {
        let line = raw_line.trim_end_matches('\r');
        if line.starts_with(' ') || line.starts_with('\t') {
            if let Some(previous) = lines.last_mut() {
                previous.push_str(line.trim_start());
            }
        } else {
            lines.push(line.to_string());
        }
    }
    lines
}

fn parse_ics_property(line: &str) -> Vec<(String, String)> {
    let Some((left, value)) = line.split_once(':') else {
        return Vec::new();
    };
    let mut parts = left.split(';');
    let Some(key) = parts.next().map(|key| key.to_ascii_uppercase()) else {
        return Vec::new();
    };

    let property_value = unescape_ics_value(value);
    let mut properties = vec![(key.clone(), property_value)];
    for part in parts {
        if let Some(tzid) = part.strip_prefix("TZID=") {
            properties.push((String::from("TZID"), tzid.to_string()));
        }
    }

    properties
}

fn unescape_ics_value(value: &str) -> String {
    value
        .replace("\\n", " ")
        .replace("\\N", " ")
        .replace("\\,", ",")
        .replace("\\;", ";")
        .replace("\\\\", "\\")
}

fn format_ics_date(value: &str) -> String {
    let date = value.split('T').next().unwrap_or(value);
    if date.len() >= 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        value.to_string()
    }
}

fn category_from_text(text: &str) -> String {
    mahadvadasi_from_text(text).unwrap_or_else(|| {
        if contains_normalized(text, "viddha") {
            String::from("Ekadasi viddha")
        } else if contains_normalized(text, "ekadasi") {
            String::from("Ekadasi normal")
        } else if contains_normalized(text, "dvadasi") {
            String::from("Observancia desplazada")
        } else {
            String::from("Unclassified")
        }
    })
}

fn mahadvadasi_from_text(text: &str) -> Option<String> {
    [
        ("unmilani", "Unmilani"),
        ("vyanjuli", "Vyanjuli"),
        ("trisprsa", "Trisprsa"),
        ("trispṛsa", "Trisprsa"),
        ("paksavardhini", "Paksavardhini"),
        ("pakshavardhini", "Paksavardhini"),
        ("jaya", "Jaya"),
        ("vijaya", "Vijaya"),
        ("jayanti", "Jayanti"),
        ("papanasini", "Papanasini"),
    ]
    .into_iter()
    .find(|(needle, _)| contains_normalized(text, needle))
    .map(|(_, value)| value.to_string())
}

fn parana_from_text(text: &str) -> String {
    if contains_normalized(text, "parana") {
        String::from("Present in source; requires review")
    } else {
        String::new()
    }
}

fn contains_normalized(text: &str, needle: &str) -> bool {
    normalize_text(text).contains(&normalize_text(needle))
}

fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .replace('ā', "a")
        .replace('ī', "i")
        .replace('ū', "u")
        .replace(['ṛ', 'ṝ'], "r")
        .replace(['ṅ', 'ñ'], "n")
        .replace('ṭ', "t")
        .replace('ḍ', "d")
        .replace('ṇ', "n")
        .replace(['ś', 'ṣ'], "s")
        .replace('ḷ', "l")
        .replace(['ṁ', 'ṃ'], "m")
}

fn bool_field(value: bool) -> String {
    if value {
        String::from("TRUE")
    } else {
        String::new()
    }
}

#[derive(Debug)]
struct FixtureRow {
    case_id: String,
    category: String,
    civil_date: String,
    location: String,
    timezone: String,
    sunrise: String,
    ekadasi: String,
    viddha: String,
    mahadvadasi: String,
    parana: String,
    gcal_source: String,
    status: String,
    notes: String,
}

impl FixtureRow {
    fn to_csv(&self) -> String {
        [
            &self.case_id,
            &self.category,
            &self.civil_date,
            &self.location,
            &self.timezone,
            &self.sunrise,
            &self.ekadasi,
            &self.viddha,
            &self.mahadvadasi,
            &self.parana,
            &self.gcal_source,
            &self.status,
            &self.notes,
        ]
        .into_iter()
        .map(|value| csv_escape(value))
        .collect::<Vec<_>>()
        .join(",")
            + "\n"
    }
}

fn write_fixture_rows(path: &Path, rows: &[FixtureRow]) -> Result<(), Box<dyn Error>> {
    let mut csv = REQUIRED_HEADERS.join(",");
    csv.push('\n');
    for row in rows {
        csv.push_str(&row.to_csv());
    }
    fs::write(path, csv)?;
    Ok(())
}

fn write_pending_template(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::write(
        path,
        "\
case_id,category,civil_date,location,timezone,sunrise,ekadasi,viddha,mahadvadasi,parana,gcal_source,status,notes
GCAL-001,Ekadasi normal,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-002,Ekadasi viddha,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-003,Observancia desplazada,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-004,Unmilani,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-005,Vyanjuli,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture and ISSUE-VAI-001 tracking if difference appears
GCAL-006,Trisprsa,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-007,Paksavardhini,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-008,Jaya,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-009,Vijaya,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-010,Jayanti,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-011,Papanasini,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
GCAL-012,Dvadasi corta,,,,,,,,,,EXTERNAL_ORACLE_PENDING,Requires real GCal fixture
",
    )?;
    Ok(())
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
