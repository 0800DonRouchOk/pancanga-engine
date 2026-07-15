use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_INPUT: &str = "../../02_Research/PureBhakti/Calendario-Vaishnava-2026.html";
const DEFAULT_OUTPUT: &str = "../../04_Tests/PureBhakti/purebhakti-fixtures.csv";

const HEADERS: [&str; 15] = [
    "case_id",
    "date",
    "location",
    "latitude",
    "longitude",
    "timezone",
    "name",
    "is_mahadvadasi",
    "mahadvadasi_type",
    "parana_start",
    "parana_end",
    "masa",
    "paksha",
    "source_file",
    "status",
];

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args(env::args().skip(1).collect())?;

    if !config.input.is_file() {
        write_pending_template(&config.output, &config.input)?;
        println!("External Oracle Pending: PureBhakti HTML not found.");
        println!("Expected HTML: {}", config.input.display());
        println!("CSV: {}", config.output.display());
        return Ok(());
    }

    let html = fs::read_to_string(&config.input)?;
    let fixtures = extract_fixtures(&config.input, &html);

    if fixtures.is_empty() {
        write_pending_template(&config.output, &config.input)?;
        println!("External Oracle Pending: no Ekadasi rows were extracted.");
        println!("CSV: {}", config.output.display());
        return Ok(());
    }

    write_fixtures(&config.output, &fixtures)?;
    println!("PureBhakti fixtures extracted: {}", fixtures.len());
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

fn extract_fixtures(source: &Path, html: &str) -> Vec<Fixture> {
    let text = html_to_text(html);
    let metadata = Metadata::from_text(&text);
    let mut fixtures = extract_month_fixtures(source, html, &metadata);

    if fixtures.is_empty() {
        fixtures = extract_table_fixtures(source, html, &metadata);
    }

    if fixtures.is_empty() {
        fixtures = extract_line_fixtures(source, &text, &metadata);
    }

    fixtures
}

fn extract_month_fixtures(source: &Path, html: &str, metadata: &Metadata) -> Vec<Fixture> {
    month_page_chunks(html)
        .into_iter()
        .flat_map(|page| fixtures_from_month_page(source, page, metadata))
        .enumerate()
        .map(|(index, mut fixture)| {
            fixture.case_id = format!("PB-{id:04}", id = index + 1);
            fixture
        })
        .collect()
}

fn month_page_chunks(html: &str) -> Vec<&str> {
    let marker = "<div class=\"month-page page\">";
    let mut starts = Vec::new();
    let mut search_start = 0;

    while let Some(offset) = find_case_insensitive(&html[search_start..], marker) {
        starts.push(search_start + offset);
        search_start += offset + marker.len();
    }

    starts
        .iter()
        .enumerate()
        .map(|(index, start)| {
            let end = starts.get(index + 1).copied().unwrap_or_else(|| {
                find_case_insensitive(&html[*start..], "<!-- SUMMARY")
                    .map(|offset| *start + offset)
                    .unwrap_or(html.len())
            });
            &html[*start..end]
        })
        .collect()
}

fn fixtures_from_month_page(source: &Path, page: &str, metadata: &Metadata) -> Vec<Fixture> {
    let month_title = extract_first_tag_text(page, "h2");
    let month = month_number(&month_title).unwrap_or_default();
    let masa = extract_class_text(page, "masa-name");
    let cells = extract_day_cells(page);
    let mut fixtures = Vec::new();

    for (index, cell) in cells.iter().enumerate() {
        let cell_text = strip_tags(cell);
        if !cell.to_ascii_lowercase().contains("ekadasi-fast")
            || !normalized(&cell_text).contains("ayuno")
            || !is_ekadasi_text(&cell_text)
        {
            continue;
        }

        let day = extract_class_text(cell, "day-num");
        if day.is_empty() || month.is_empty() {
            continue;
        }

        let fast_text = extract_event_text(cell, "ev-fast").unwrap_or(cell_text);
        if !is_ekadasi_text(&fast_text) {
            continue;
        }
        let clean_fast = clean_fast_name(&fast_text);
        let (parana_start, parana_end) = parana_after_cell(index, &cells);
        let tithi_badge = extract_class_text(cell, "tithi-badge");

        fixtures.push(Fixture {
            case_id: String::new(),
            date: format!("2026-{month}-{day:0>2}"),
            location: metadata.location.clone(),
            latitude: metadata.latitude.clone(),
            longitude: metadata.longitude.clone(),
            timezone: metadata.timezone.clone(),
            name: clean_fast,
            is_mahadvadasi: bool_value(is_mahadvadasi_text(&fast_text)),
            mahadvadasi_type: mahadvadasi_type_from_text(&fast_text)
                .unwrap_or_else(|| String::from("None")),
            parana_start,
            parana_end,
            masa: masa_from_text(&format!("{masa} {fast_text}")),
            paksha: paksha_from_text(&tithi_badge),
            source_file: source.display().to_string(),
            status: String::from("IMPORTED_REQUIRES_REVIEW"),
        });
    }

    fixtures
}

fn extract_day_cells(page: &str) -> Vec<&str> {
    let marker = "<div class=\"day-cell";
    let mut starts = Vec::new();
    let mut search_start = 0;

    while let Some(offset) = find_case_insensitive(&page[search_start..], marker) {
        starts.push(search_start + offset);
        search_start += offset + marker.len();
    }

    starts
        .iter()
        .enumerate()
        .map(|(index, start)| {
            let end = starts.get(index + 1).copied().unwrap_or(page.len());
            &page[*start..end]
        })
        .collect()
}

fn parana_after_cell(index: usize, cells: &[&str]) -> (String, String) {
    cells
        .iter()
        .skip(index + 1)
        .take(3)
        .map(|cell| strip_tags(cell))
        .find(|text| normalized(text).contains("romper ayuno"))
        .map(|text| parana_times_from_text(&text))
        .unwrap_or_default()
}

fn clean_fast_name(text: &str) -> String {
    let mut value = text
        .replace("🔴", "")
        .replace("⭐", "")
        .replace("AYUNO:", "")
        .replace("Ayuno:", "");
    if let Some((before, _)) = value.split_once('(') {
        if normalized(before).contains("ekadasi") {
            value = before.to_string();
        }
    }
    collapse_whitespace(&value)
}

fn extract_table_fixtures(source: &Path, html: &str, metadata: &Metadata) -> Vec<Fixture> {
    let rows = extract_table_rows(html);
    let mut fixtures = Vec::new();
    let mut header_map = HashMap::new();

    for row in rows {
        if row.is_empty() {
            continue;
        }

        if row.iter().any(|cell| normalized(cell).contains("date")) {
            header_map = build_header_map(&row);
            continue;
        }

        let joined = row.join(" ");
        if !is_ekadasi_text(&joined) {
            continue;
        }

        fixtures.push(fixture_from_cells(
            fixtures.len() + 1,
            source,
            metadata,
            &header_map,
            &row,
        ));
    }

    fixtures
}

fn fixture_from_cells(
    index: usize,
    source: &Path,
    metadata: &Metadata,
    header_map: &HashMap<String, usize>,
    cells: &[String],
) -> Fixture {
    let text = cells.join(" ");
    Fixture {
        case_id: format!("PB-{index:04}"),
        date: cell_value(header_map, cells, "date").unwrap_or_else(|| date_from_text(&text)),
        location: cell_value(header_map, cells, "location")
            .unwrap_or_else(|| metadata.location.clone()),
        latitude: cell_value(header_map, cells, "latitude")
            .unwrap_or_else(|| metadata.latitude.clone()),
        longitude: cell_value(header_map, cells, "longitude")
            .unwrap_or_else(|| metadata.longitude.clone()),
        timezone: cell_value(header_map, cells, "timezone")
            .unwrap_or_else(|| metadata.timezone.clone()),
        name: cell_value(header_map, cells, "name").unwrap_or_else(|| name_from_text(&text)),
        is_mahadvadasi: bool_value(is_mahadvadasi_text(&text)),
        mahadvadasi_type: mahadvadasi_type_from_text(&text).unwrap_or_else(|| String::from("None")),
        parana_start: parana_times_from_text(&text).0,
        parana_end: parana_times_from_text(&text).1,
        masa: masa_from_text(&text),
        paksha: paksha_from_text(&text),
        source_file: source.display().to_string(),
        status: String::from("IMPORTED_REQUIRES_REVIEW"),
    }
}

fn extract_line_fixtures(source: &Path, text: &str, metadata: &Metadata) -> Vec<Fixture> {
    text.lines()
        .filter(|line| is_ekadasi_text(line))
        .enumerate()
        .map(|(index, line)| {
            let (parana_start, parana_end) = parana_times_from_text(line);
            Fixture {
                case_id: format!("PB-{id:04}", id = index + 1),
                date: date_from_text(line),
                location: metadata.location.clone(),
                latitude: metadata.latitude.clone(),
                longitude: metadata.longitude.clone(),
                timezone: metadata.timezone.clone(),
                name: name_from_text(line),
                is_mahadvadasi: bool_value(is_mahadvadasi_text(line)),
                mahadvadasi_type: mahadvadasi_type_from_text(line)
                    .unwrap_or_else(|| String::from("None")),
                parana_start,
                parana_end,
                masa: masa_from_text(line),
                paksha: paksha_from_text(line),
                source_file: source.display().to_string(),
                status: String::from("IMPORTED_REQUIRES_REVIEW"),
            }
        })
        .collect()
}

fn extract_table_rows(html: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut start = 0;

    while let Some(row_start) = find_case_insensitive(&html[start..], "<tr") {
        let absolute_start = start + row_start;
        let Some(open_end) = html[absolute_start..].find('>') else {
            break;
        };
        let content_start = absolute_start + open_end + 1;
        let Some(row_end) = find_case_insensitive(&html[content_start..], "</tr>") else {
            break;
        };
        let content_end = content_start + row_end;
        let row_html = &html[content_start..content_end];
        let cells = extract_cells(row_html);
        if !cells.is_empty() {
            rows.push(cells);
        }
        start = content_end + "</tr>".len();
    }

    rows
}

fn extract_cells(row_html: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut start = 0;

    while let Some(cell_start) = find_next_cell(&row_html[start..]) {
        let absolute_start = start + cell_start;
        let tag = if row_html[absolute_start..]
            .to_ascii_lowercase()
            .starts_with("<th")
        {
            "th"
        } else {
            "td"
        };
        let Some(open_end) = row_html[absolute_start..].find('>') else {
            break;
        };
        let content_start = absolute_start + open_end + 1;
        let close_tag = format!("</{tag}>");
        let Some(cell_end) = find_case_insensitive(&row_html[content_start..], &close_tag) else {
            break;
        };
        let content_end = content_start + cell_end;
        cells.push(collapse_whitespace(&strip_tags(
            &row_html[content_start..content_end],
        )));
        start = content_end + close_tag.len();
    }

    cells
}

fn extract_first_tag_text(html: &str, tag: &str) -> String {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let Some(start) = find_case_insensitive(html, &open) else {
        return String::new();
    };
    let Some(open_end) = html[start..].find('>') else {
        return String::new();
    };
    let content_start = start + open_end + 1;
    let Some(end) = find_case_insensitive(&html[content_start..], &close) else {
        return String::new();
    };
    collapse_whitespace(&strip_tags(&html[content_start..content_start + end]))
}

fn extract_class_text(html: &str, class_name: &str) -> String {
    let Some(start) = find_class_start(html, class_name) else {
        return String::new();
    };
    let Some(open_end) = html[start..].find('>') else {
        return String::new();
    };
    let content_start = start + open_end + 1;
    let Some(end) = find_case_insensitive(&html[content_start..], "</") else {
        return String::new();
    };
    collapse_whitespace(&strip_tags(&html[content_start..content_start + end]))
}

fn extract_event_text(html: &str, class_name: &str) -> Option<String> {
    let start = find_class_start(html, class_name)?;
    let open_end = html[start..].find('>')?;
    let content_start = start + open_end + 1;
    let end = find_case_insensitive(&html[content_start..], "</span>")?;
    Some(collapse_whitespace(&strip_tags(
        &html[content_start..content_start + end],
    )))
}

fn find_class_start(html: &str, class_name: &str) -> Option<usize> {
    let needle = class_name.to_ascii_lowercase();
    let lower = html.to_ascii_lowercase();
    let mut search_start = 0;

    while let Some(offset) = lower[search_start..].find("class=\"") {
        let class_attr_start = search_start + offset;
        let class_value_start = class_attr_start + "class=\"".len();
        let class_value_end = lower[class_value_start..].find('"')?;
        let class_value = &lower[class_value_start..class_value_start + class_value_end];
        if class_value.split_whitespace().any(|class| class == needle) {
            return html[..class_attr_start].rfind('<');
        }
        search_start = class_value_start + class_value_end + 1;
    }

    None
}

fn find_next_cell(html: &str) -> Option<usize> {
    [
        find_case_insensitive(html, "<td"),
        find_case_insensitive(html, "<th"),
    ]
    .into_iter()
    .flatten()
    .min()
}

fn build_header_map(headers: &[String]) -> HashMap<String, usize> {
    headers
        .iter()
        .enumerate()
        .map(|(index, header)| (header_key(header), index))
        .collect()
}

fn header_key(value: &str) -> String {
    let normalized = normalized(value);
    if normalized.contains("date") || normalized.contains("fecha") {
        String::from("date")
    } else if normalized.contains("name") || normalized.contains("nombre") {
        String::from("name")
    } else if normalized.contains("location") || normalized.contains("lugar") {
        String::from("location")
    } else if normalized.contains("latitude") || normalized.contains("latitud") {
        String::from("latitude")
    } else if normalized.contains("longitude") || normalized.contains("longitud") {
        String::from("longitude")
    } else if normalized.contains("timezone") || normalized.contains("time zone") {
        String::from("timezone")
    } else {
        normalized
    }
}

fn cell_value(header_map: &HashMap<String, usize>, cells: &[String], key: &str) -> Option<String> {
    header_map
        .get(key)
        .and_then(|index| cells.get(*index))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn html_to_text(html: &str) -> String {
    let mut prepared = html
        .replace("</tr>", "\n")
        .replace("</p>", "\n")
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("</li>", "\n");
    prepared = strip_tags(&prepared);
    prepared
        .lines()
        .map(collapse_whitespace)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_tags(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(ch),
            _ => {}
        }
    }

    decode_html_entities(&text)
}

fn decode_html_entities(text: &str) -> String {
    text.replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&ndash;", "-")
        .replace("&mdash;", "-")
}

fn find_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .to_ascii_lowercase()
        .find(&needle.to_ascii_lowercase())
}

fn is_ekadasi_text(text: &str) -> bool {
    let normalized = normalized(text);
    normalized.contains("ekadasi") || normalized.contains("mahadvadasi")
}

fn is_mahadvadasi_text(text: &str) -> bool {
    normalized(text).contains("mahadvadasi") || mahadvadasi_type_from_text(text).is_some()
}

fn mahadvadasi_type_from_text(text: &str) -> Option<String> {
    [
        ("unmilani", "Unmilani"),
        ("vyanjuli", "Vyanjuli"),
        ("trisprsa", "Trisprsa"),
        ("paksavardhini", "Paksavardhini"),
        ("pakshavardhini", "Paksavardhini"),
        ("jaya", "Jaya"),
        ("vijaya", "Vijaya"),
        ("jayanti", "Jayanti"),
        ("papanasini", "Papanasini"),
    ]
    .into_iter()
    .find(|(needle, _)| normalized(text).contains(needle))
    .map(|(_, value)| value.to_string())
}

fn name_from_text(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let Some(index) = words
        .iter()
        .position(|word| normalized(word).contains("ekadasi"))
    else {
        return String::new();
    };

    let start = index.saturating_sub(2);
    words[start..index].join(" ")
}

fn date_from_text(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    for (index, word) in words.iter().enumerate() {
        if month_number(word).is_some() {
            let previous = index.checked_sub(1).and_then(|i| words.get(i)).copied();
            let next = words.get(index + 1).copied();
            let year = words.iter().find_map(numeric_year).unwrap_or_default();
            let day = previous
                .and_then(numeric_day)
                .or_else(|| next.and_then(numeric_day))
                .unwrap_or_default();
            let month = month_number(word).unwrap_or_default();

            if !day.is_empty() && !month.is_empty() && !year.is_empty() {
                return format!("{year}-{month}-{day:0>2}");
            }
        }
    }

    String::new()
}

fn parana_times_from_text(text: &str) -> (String, String) {
    let normalized = normalized(text);
    if !(normalized.contains("parana")
        || normalized.contains("break")
        || normalized.contains("breakfast")
        || normalized.contains("romper ayuno"))
    {
        return (String::new(), String::new());
    }

    let times: Vec<String> = text
        .split(|ch: char| ch.is_whitespace() || ch == '-' || ch == '–' || ch == '—')
        .filter_map(clean_time)
        .collect();

    (
        times.first().cloned().unwrap_or_default(),
        times.get(1).cloned().unwrap_or_default(),
    )
}

fn clean_time(value: &str) -> Option<String> {
    let trimmed = value.trim_matches(|ch: char| !ch.is_ascii_digit() && ch != ':');
    let mut parts = trimmed.split(':');
    let hour = parts.next()?;
    let minute = parts.next()?;
    if parts.next().is_some()
        || hour.is_empty()
        || minute.len() != 2
        || !hour.chars().all(|ch| ch.is_ascii_digit())
        || !minute.chars().all(|ch| ch.is_ascii_digit())
    {
        return None;
    }
    Some(format!("{hour:0>2}:{minute}"))
}

fn masa_from_text(text: &str) -> String {
    [
        "Madhusudana",
        "Trivikrama",
        "Vamana",
        "Sridhara",
        "Hrsikesa",
        "Padmanabha",
        "Damodara",
        "Kesava",
        "Narayana",
        "Madhava",
        "Govinda",
        "Visnu",
        "Purusottama",
    ]
    .into_iter()
    .find(|name| normalized(text).contains(&normalized(name)))
    .unwrap_or("")
    .to_string()
}

fn paksha_from_text(text: &str) -> String {
    let normalized = normalized(text);
    if normalized.contains("krsna")
        || normalized.contains("krishna")
        || normalized.contains("· k")
        || normalized.ends_with(" k")
    {
        String::from("Krsna")
    } else if normalized.contains("sukla")
        || normalized.contains("gaura")
        || normalized.contains("· g")
        || normalized.ends_with(" g")
    {
        String::from("Sukla")
    } else {
        String::new()
    }
}

fn numeric_day(value: &str) -> Option<String> {
    let digits: String = value.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.len() <= 2 && !digits.is_empty() {
        Some(digits)
    } else {
        None
    }
}

fn numeric_year(value: &&str) -> Option<String> {
    let digits: String = value.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.len() == 4 {
        Some(digits)
    } else {
        None
    }
}

fn month_number(value: &str) -> Option<String> {
    normalized(value)
        .split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_ascii_alphabetic()))
        .find_map(|token| match token {
            "january" | "jan" | "enero" => Some(String::from("01")),
            "february" | "feb" | "febrero" => Some(String::from("02")),
            "march" | "mar" | "marzo" => Some(String::from("03")),
            "april" | "apr" | "abril" => Some(String::from("04")),
            "may" | "mayo" => Some(String::from("05")),
            "june" | "jun" | "junio" => Some(String::from("06")),
            "july" | "jul" | "julio" => Some(String::from("07")),
            "august" | "aug" | "agosto" => Some(String::from("08")),
            "september" | "sep" | "septiembre" => Some(String::from("09")),
            "october" | "oct" | "octubre" => Some(String::from("10")),
            "november" | "nov" | "noviembre" => Some(String::from("11")),
            "december" | "dec" | "diciembre" => Some(String::from("12")),
            _ => None,
        })
}

fn bool_value(value: bool) -> String {
    if value {
        String::from("true")
    } else {
        String::from("false")
    }
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalized(value: &str) -> String {
    value
        .to_lowercase()
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

#[derive(Default)]
struct Metadata {
    location: String,
    latitude: String,
    longitude: String,
    timezone: String,
}

impl Metadata {
    fn from_text(text: &str) -> Self {
        let location_line = text
            .lines()
            .find(|line| normalized(line).contains("localizacion de referencia"))
            .unwrap_or("");
        let location_value = labeled_value(text, &["Localización de referencia", "Location"]);
        let (location, latitude, longitude) = parse_location_and_coordinates(&location_value);

        Self {
            location: if location.is_empty() {
                location_line.to_string()
            } else {
                location
            },
            latitude: if latitude.is_empty() {
                labeled_value(text, &["Latitude", "Latitud", "Lat"])
            } else {
                latitude
            },
            longitude: if longitude.is_empty() {
                labeled_value(text, &["Longitude", "Longitud", "Lon", "Lng"])
            } else {
                longitude
            },
            timezone: labeled_value(text, &["Timezone", "Time zone", "Zona horaria", "TZ"]),
        }
    }
}

fn parse_location_and_coordinates(value: &str) -> (String, String, String) {
    let Some((location, rest)) = value.split_once('(') else {
        return (value.to_string(), String::new(), String::new());
    };
    let coordinates = rest
        .split_once(')')
        .map(|(inside, _)| inside)
        .unwrap_or(rest);
    let parts: Vec<&str> = coordinates.split('·').collect();
    let latitude = parts
        .first()
        .map(|value| parse_coordinate(value))
        .unwrap_or_default();
    let longitude = parts
        .get(1)
        .map(|value| parse_coordinate(value))
        .unwrap_or_default();

    (collapse_whitespace(location.trim()), latitude, longitude)
}

fn parse_coordinate(value: &str) -> String {
    let direction = value
        .chars()
        .find(|ch| matches!(ch, 'N' | 'S' | 'E' | 'W' | 'n' | 's' | 'e' | 'w'))
        .map(|ch| ch.to_ascii_uppercase());
    let numbers: Vec<f64> = value
        .split(|ch: char| !ch.is_ascii_digit() && ch != '.')
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse().ok())
        .collect();

    let Some(degrees) = numbers.first().copied() else {
        return String::new();
    };
    let minutes = numbers.get(1).copied().unwrap_or(0.0);
    let mut decimal = degrees + minutes / 60.0;
    if matches!(direction, Some('S' | 'W')) {
        decimal = -decimal;
    }

    format!("{decimal:.6}")
}

fn labeled_value(text: &str, labels: &[&str]) -> String {
    for line in text.lines() {
        let normalized_line = normalized(line);
        for label in labels {
            let normalized_label = normalized(label);
            if normalized_line.starts_with(&normalized_label) {
                if let Some((_, value)) = line.split_once(':') {
                    return collapse_whitespace(value);
                }
            }
        }
    }
    String::new()
}

struct Fixture {
    case_id: String,
    date: String,
    location: String,
    latitude: String,
    longitude: String,
    timezone: String,
    name: String,
    is_mahadvadasi: String,
    mahadvadasi_type: String,
    parana_start: String,
    parana_end: String,
    masa: String,
    paksha: String,
    source_file: String,
    status: String,
}

impl Fixture {
    fn to_csv(&self) -> String {
        [
            &self.case_id,
            &self.date,
            &self.location,
            &self.latitude,
            &self.longitude,
            &self.timezone,
            &self.name,
            &self.is_mahadvadasi,
            &self.mahadvadasi_type,
            &self.parana_start,
            &self.parana_end,
            &self.masa,
            &self.paksha,
            &self.source_file,
            &self.status,
        ]
        .into_iter()
        .map(|value| csv_escape(value))
        .collect::<Vec<_>>()
        .join(",")
            + "\n"
    }
}

fn write_fixtures(path: &Path, fixtures: &[Fixture]) -> Result<(), Box<dyn Error>> {
    let mut csv = HEADERS.join(",");
    csv.push('\n');
    for fixture in fixtures {
        csv.push_str(&fixture.to_csv());
    }
    fs::write(path, csv)?;
    Ok(())
}

fn write_pending_template(output: &Path, input: &Path) -> Result<(), Box<dyn Error>> {
    let mut csv = HEADERS.join(",");
    csv.push('\n');
    csv.push_str(
        &Fixture {
            case_id: String::from("PB-0000"),
            date: String::new(),
            location: String::new(),
            latitude: String::new(),
            longitude: String::new(),
            timezone: String::new(),
            name: String::new(),
            is_mahadvadasi: String::new(),
            mahadvadasi_type: String::new(),
            parana_start: String::new(),
            parana_end: String::new(),
            masa: String::new(),
            paksha: String::new(),
            source_file: input.display().to_string(),
            status: String::from("EXTERNAL_ORACLE_PENDING"),
        }
        .to_csv(),
    );
    fs::write(output, csv)?;
    Ok(())
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
