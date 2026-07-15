use std::collections::HashMap;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use pancanga_engine::astronomy::{
    lunar_solar_elongation, moon, nakshatra, solar, AstronomicalTithi, Paksha,
};
use pancanga_engine::calendar::{sunrise, tithi_at_sunrise};
use pancanga_engine::core::math::{
    acos_deg, asin_deg, atan_deg, cos_deg, normalize_360, normalize_hours, sin_deg, tan_deg,
};
use pancanga_engine::core::time::{
    gregorian_to_jd, jd_to_gregorian, weekday, CivilDateTime, CivilTime, TimeScale, Weekday,
};
use pancanga_engine::core::{
    CivilDate, Degrees, DurationDays, GeoLocation, JulianDate, Latitude, Longitude,
};
use pancanga_engine::vaishnava::{
    arunodaya_start, ekadasi_candidate_at_sunrise, hari_vasara_end, invalidate_viddha_candidate,
    observance_displacement, parana_window, EkadasiObservanceDisposition, ParanaMode,
};

const DEFAULT_ADDRESS: &str = "127.0.0.1:7878";
const HTML: &str = include_str!("../../../../../08_Examples/RC1-Experience/index.html");
const OFFICIAL_ZENITH_DEGREES: f64 = 90.833;
const SCAN_STEP_DAYS: f64 = 1.0 / 96.0;
const REFINEMENT_STEPS: usize = 48;

#[derive(Clone, Copy)]
struct City {
    id: &'static str,
    name: &'static str,
    timezone: &'static str,
    latitude: f64,
    longitude: f64,
}

#[derive(Clone, Copy)]
struct DayEvaluation {
    sunrise: JulianDate,
    arunodaya: JulianDate,
    tithi_at_sunrise: AstronomicalTithi,
    disposition: EkadasiObservanceDisposition,
}

#[derive(Clone, Copy)]
struct DvadasiInterval {
    start: JulianDate,
    end: JulianDate,
}

struct ParanaPresentation {
    recommended: Option<String>,
    normative: String,
    normative_limit: String,
    note: String,
}

fn main() -> std::io::Result<()> {
    let address = rc1_listen_address();
    let listener = TcpListener::bind(&address)?;
    println!("Pancanga Engine RC1 Experience");
    println!("Open http://{address}");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream),
            Err(error) => eprintln!("Connection error: {error}"),
        }
    }

    Ok(())
}

fn rc1_listen_address() -> String {
    if let Ok(address) = env::var("RC1_EXPERIENCE_ADDRESS") {
        return address;
    }

    if let Ok(port) = env::var("PORT") {
        return format!("0.0.0.0:{port}");
    }

    DEFAULT_ADDRESS.to_string()
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 8192];
    let Ok(bytes_read) = stream.read(&mut buffer) else {
        respond(stream, "400 Bad Request", "text/plain", "Bad request");
        return;
    };

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let Some(first_line) = request.lines().next() else {
        respond(stream, "400 Bad Request", "text/plain", "Bad request");
        return;
    };

    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");

    if method != "GET" {
        respond(
            stream,
            "405 Method Not Allowed",
            "text/plain",
            "Only GET is supported",
        );
        return;
    }

    if target == "/" || target == "/index.html" {
        respond(stream, "200 OK", "text/html", HTML);
        return;
    }

    if let Some(query) = target.strip_prefix("/api/calculate?") {
        match calculate_response(query) {
            Ok(body) => respond(stream, "200 OK", "application/json", &body),
            Err(message) => respond(
                stream,
                "400 Bad Request",
                "application/json",
                &json_error(&message),
            ),
        }
        return;
    }

    respond(stream, "404 Not Found", "text/plain", "Not found");
}

fn calculate_response(query: &str) -> Result<String, String> {
    let params = parse_query(query);
    let city_id = params.get("city").map(String::as_str).unwrap_or("valencia");
    let date_text = params
        .get("date")
        .map(String::as_str)
        .unwrap_or("2026-07-14");

    let city = city_by_id(city_id).ok_or_else(|| format!("Unknown city: {city_id}"))?;
    let date = parse_date(date_text)?;
    let location = city_location(city)?;
    let offset_hours = timezone_offset_hours(city, date);
    let today = evaluate_day(date, location)?;
    let yesterday = evaluate_day(previous_day(date), location)?;

    let observance = if today.disposition == EkadasiObservanceDisposition::ObserveOnEkadasi {
        Some((today, ObservanceKind::Today))
    } else if yesterday.disposition == EkadasiObservanceDisposition::ObserveOnDvadasi {
        Some((yesterday, ObservanceKind::DisplacedFromYesterday))
    } else {
        None
    };

    let decision = match observance {
        Some((_, ObservanceKind::Today)) => "✓ Observá Ekādaśī hoy".to_string(),
        Some((_, ObservanceKind::DisplacedFromYesterday)) => "✓ Observá Ekādaśī hoy".to_string(),
        None => "No se observa Ekādaśī hoy".to_string(),
    };

    let parana = observance.map(|_| {
        parana_presentation(next_day(date), location, offset_hours).unwrap_or_else(|error| {
            ParanaPresentation {
                recommended: None,
                normative: "Parāṇa pendiente".to_string(),
                normative_limit: "Pendiente".to_string(),
                note: format!(
                    "La decisión del día fue calculada, pero la beta no pudo ubicar la ventana de Parāṇa: {error}."
                ),
            }
        })
    });

    let astronomy = astronomy_summary(today, offset_hours);
    let why = why_summary(observance, today, yesterday);

    Ok(format!(
        "{{\
            \"weekday\":\"{}\",\
            \"display_date\":\"{}\",\
            \"input_date\":\"{}\",\
            \"processed_date\":\"{}\",\
            \"location\":\"{}\",\
            \"decision\":\"{}\",\
            \"parana_recommended\":\"{}\",\
            \"parana_normative\":\"{}\",\
            \"parana_normative_limit\":\"{}\",\
            \"parana_note\":\"{}\",\
            \"why\":[{}],\
            \"astronomy\":{{{}}},\
            \"date_debug\":{{\"input\":\"{}\",\"processed\":\"{}\",\"rendered\":\"{}\"}},\
            \"certification\":[{}]\
        }}",
        json_escape(&weekday_name(date)),
        json_escape(&display_date(date)),
        json_escape(date_text),
        json_escape(&iso_date(date)),
        json_escape(city.name),
        json_escape(&decision),
        json_escape(
            &parana
                .as_ref()
                .and_then(|presentation| presentation.recommended.clone())
                .unwrap_or_default()
        ),
        json_escape(
            &parana
                .as_ref()
                .map(|presentation| presentation.normative.clone())
                .unwrap_or_else(|| "Sin ventana de Parāṇa".to_string())
        ),
        json_escape(
            &parana
                .as_ref()
                .map(|presentation| presentation.normative_limit.clone())
                .unwrap_or_default()
        ),
        json_escape(
            &parana
                .as_ref()
                .map(|presentation| presentation.note.clone())
                .unwrap_or_else(|| {
                    "No hay observancia de Ekādaśī para esta fecha.".to_string()
                })
        ),
        why,
        astronomy,
        json_escape(date_text),
        json_escape(&iso_date(date)),
        json_escape(&display_date(date)),
        certification_json()
    ))
}

#[derive(Clone, Copy)]
enum ObservanceKind {
    Today,
    DisplacedFromYesterday,
}

fn evaluate_day(date: CivilDate, location: GeoLocation) -> Result<DayEvaluation, String> {
    let sunrise_time = sunrise(date, location)
        .ok_or_else(|| "No hay amanecer disponible para esta fecha".to_string())?;
    let arunodaya_time = arunodaya_start(sunrise_time);
    let tithi_at_sunrise_value = tithi_at_sunrise(sunrise_time);
    let tithi_at_arunodaya_value = tithi_at_sunrise(arunodaya_time);
    let candidate = ekadasi_candidate_at_sunrise(tithi_at_sunrise_value);
    let viddha = invalidate_viddha_candidate(candidate, tithi_at_arunodaya_value);
    let disposition = observance_displacement(viddha);

    Ok(DayEvaluation {
        sunrise: sunrise_time,
        arunodaya: arunodaya_time,
        tithi_at_sunrise: tithi_at_sunrise_value,
        disposition,
    })
}

fn parana_presentation(
    parana_date: CivilDate,
    location: GeoLocation,
    offset_hours: f64,
) -> Result<ParanaPresentation, String> {
    let parana_sunrise = sunrise(parana_date, location)
        .ok_or_else(|| "No hay amanecer disponible para la fecha de Parāṇa".to_string())?;
    let parana_sunset = solar_event(parana_date, location, SolarEvent::Sunset);
    let interval = dvadasi_interval_around(parana_sunrise)
        .ok_or_else(|| "No se pudo ubicar el intervalo de Dvādaśī para Parāṇa".to_string())?;
    let window = parana_window(parana_sunrise, interval.start, interval.end)
        .map_err(|error| format!("No se pudo calcular Parāṇa: {error:?}"))?;
    if window.mode == ParanaMode::ShortDvadasi {
        return Ok(ParanaPresentation {
            recommended: None,
            normative: format!("{} · Dvādaśī corta", display_date(parana_date)),
            normative_limit: "Contingencia".to_string(),
            note: format!(
                "Dvādaśī termina a las {}, antes de una ventana ordinaria de Parāṇa. La beta lo muestra como contingencia sin inventar una ventana práctica.",
                local_hhmm(window.ends_at, offset_hours)
            ),
        });
    }

    let normative = format!(
        "{} · {} – {}",
        display_date(parana_date),
        local_hhmm(window.starts_at, offset_hours),
        local_hhmm(window.ends_at, offset_hours)
    );

    let recommended = parana_sunset.and_then(|sunset_time| {
        let daylight_duration = sunset_time.days_since(parana_sunrise).value();
        let first_third_end = parana_sunrise.add_days(DurationDays::new(daylight_duration / 3.0));
        let recommended_end = if first_third_end < window.ends_at {
            first_third_end
        } else {
            window.ends_at
        };

        if recommended_end <= window.starts_at {
            return None;
        }

        Some(format!(
            "{} · {} – {}",
            display_date(parana_date),
            local_hhmm(window.starts_at, offset_hours),
            local_hhmm(recommended_end, offset_hours)
        ))
    });

    let note = if recommended.is_some() {
        "La ventana recomendada usa un límite práctico de presentación. En el caso PureBhakti estudiado, ese límite coincide con el primer tercio del día civil; la ventana normativa permanece abierta hasta el final de Dvādaśī."
    } else {
        "La ventana normativa se calcula desde el final de Hari-vāsara hasta el final de Dvādaśī."
    };

    Ok(ParanaPresentation {
        recommended,
        normative,
        normative_limit: format!("Hasta {}", local_hhmm(window.ends_at, offset_hours)),
        note: note.to_string(),
    })
}

fn astronomy_summary(day: DayEvaluation, offset_hours: f64) -> String {
    let sun_longitude = solar::apparent_longitude(day.sunrise);
    let moon_longitude = moon::apparent_longitude(day.sunrise);
    let elongation = lunar_solar_elongation(moon_longitude, sun_longitude);
    let nakshatra_index = nakshatra::index(moon_longitude);
    let hv_end = if day.tithi_at_sunrise.traditional_number() == 12 {
        dvadasi_interval_around(day.sunrise)
            .and_then(|interval| hari_vasara_end(interval.start, interval.end).ok())
            .map(|end| local_hhmm(end, offset_hours))
            .unwrap_or_else(|| "Not available".to_string())
    } else {
        "Se calcula en la Dvādaśī de observancia".to_string()
    };

    let entries = [
        ("Sun", format!("{:.6}°", sun_longitude.value())),
        ("Moon", format!("{:.6}°", moon_longitude.value())),
        ("Elongation", format!("{:.6}°", elongation.value())),
        ("Tithi", tithi_name(day.tithi_at_sunrise)),
        (
            "Nakṣatra",
            nakshatra_name(nakshatra_index.value()).to_string(),
        ),
        ("Aruṇodaya", local_hhmm(day.arunodaya, offset_hours)),
        ("Sunrise", local_hhmm(day.sunrise, offset_hours)),
        ("Hari-vāsara end", hv_end),
    ];

    entries
        .iter()
        .map(|(key, value)| format!("\"{}\":\"{}\"", json_escape(key), json_escape(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn why_summary(
    observance: Option<(DayEvaluation, ObservanceKind)>,
    today: DayEvaluation,
    yesterday: DayEvaluation,
) -> String {
    let items: Vec<(&str, &str)> = match observance {
        Some((_, ObservanceKind::Today)) => vec![
            ("Ekādaśī estaba presente al amanecer local.", "HBV-EK-001"),
            (
                "Daśamī no estaba presente durante Aruṇodaya.",
                "HBV-EK-002 / HBV 12.315",
            ),
            (
                "La pantalla RC1 todavía no deriva automáticamente todos los hechos de Mahādvādaśī basados en tithi.",
                "HBV-EK-004",
            ),
            ("Resultado: observar Ekādaśī hoy.", "Vaiṣṇava Engine"),
        ],
        Some((_, ObservanceKind::DisplacedFromYesterday)) => vec![
            ("El candidato de Ekādaśī de ayer era Viddhā.", "HBV-EK-002"),
            ("La observancia se traslada a Dvādaśī.", "HBV-EK-003"),
            ("Resultado: observar hoy.", "Vaiṣṇava Engine"),
        ],
        None => vec![
            (
                "Ekādaśī no estaba presente al amanecer local de hoy.",
                "HBV-EK-001",
            ),
            (
                "No se encontró una observancia Viddhā trasladada desde ayer.",
                "HBV-EK-003",
            ),
            ("Resultado: hoy no se observa Ekādaśī.", "Vaiṣṇava Engine"),
        ],
    };

    let _ = (today, yesterday);
    items
        .iter()
        .map(|(text, reference)| {
            format!(
                "{{\"text\":\"{}\",\"ref\":\"{}\"}}",
                json_escape(text),
                json_escape(reference)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn certification_json() -> String {
    let entries = [
        ("Swiss Ephemeris", "PASS", "1000/1000"),
        ("PureBhakti", "PASS", "Observancias certificadas"),
        ("SCS Math", "CONFIGURATION", "Diferencias clasificadas"),
        ("ENGINE BUG", "PASS", "0 confirmados"),
    ];

    entries
        .iter()
        .map(|(name, status, label)| {
            format!(
                "{{\"name\":\"{}\",\"status\":\"{}\",\"label\":\"{}\"}}",
                json_escape(name),
                json_escape(status),
                json_escape(label)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
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

fn dvadasi_interval_around(anchor: JulianDate) -> Option<DvadasiInterval> {
    let start_scan = anchor.add_days(DurationDays::new(-2.5));
    let end_scan = anchor.add_days(DurationDays::new(2.5));
    let mut previous = start_scan;
    let mut previous_is_dvadasi = is_dvadasi(previous);
    let mut current = start_scan.add_days(DurationDays::new(SCAN_STEP_DAYS));
    let mut start = None;
    let mut best_interval: Option<(f64, DvadasiInterval)> = None;

    while current <= end_scan {
        let current_is_dvadasi = is_dvadasi(current);
        if !previous_is_dvadasi && current_is_dvadasi {
            start = Some(refine_boundary(previous, current, true));
        }
        if previous_is_dvadasi && !current_is_dvadasi {
            let interval_end = refine_boundary(previous, current, false);
            if let Some(interval_start) = start {
                let interval = DvadasiInterval {
                    start: interval_start,
                    end: interval_end,
                };
                if interval_start <= anchor && anchor <= interval_end {
                    return Some(interval);
                }

                let distance = if anchor < interval_start {
                    interval_start.days_since(anchor).value().abs()
                } else {
                    anchor.days_since(interval_end).value().abs()
                };

                if best_interval
                    .map(|(best_distance, _)| distance < best_distance)
                    .unwrap_or(true)
                {
                    best_interval = Some((distance, interval));
                }
            }
            start = None;
        }

        previous = current;
        previous_is_dvadasi = current_is_dvadasi;
        current = current.add_days(DurationDays::new(SCAN_STEP_DAYS));
    }

    best_interval.map(|(_, interval)| interval)
}

fn refine_boundary(
    mut low: JulianDate,
    mut high: JulianDate,
    target_is_dvadasi: bool,
) -> JulianDate {
    for _ in 0..REFINEMENT_STEPS {
        let mid = JulianDate::new((low.value() + high.value()) / 2.0);
        if is_dvadasi(mid) == target_is_dvadasi {
            high = mid;
        } else {
            low = mid;
        }
    }
    high
}

fn is_dvadasi(jd: JulianDate) -> bool {
    tithi_at_sunrise(jd).traditional_number() == 12
}

fn tithi_name(tithi: AstronomicalTithi) -> String {
    let number = tithi.traditional_number();
    let paksha = match tithi.paksha() {
        Paksha::Sukla => "Śukla",
        Paksha::Krsna => "Kṛṣṇa",
    };
    let name = match number {
        1 => "Pratipat",
        2 => "Dvitīyā",
        3 => "Tṛtīyā",
        4 => "Caturthī",
        5 => "Pañcamī",
        6 => "Ṣaṣṭhī",
        7 => "Saptamī",
        8 => "Aṣṭamī",
        9 => "Navamī",
        10 => "Daśamī",
        11 => "Ekādaśī",
        12 => "Dvādaśī",
        13 => "Trayodaśī",
        14 => "Caturdaśī",
        15 => "Pūrṇimā / Amāvasyā",
        _ => "Unknown",
    };

    format!("{paksha} {name}")
}

fn nakshatra_name(index: u8) -> &'static str {
    const NAMES: [&str; 27] = [
        "Aśvinī",
        "Bharaṇī",
        "Kṛttikā",
        "Rohiṇī",
        "Mṛgaśīrṣa",
        "Ārdrā",
        "Punarvasu",
        "Puṣya",
        "Āśleṣā",
        "Maghā",
        "Pūrva Phalgunī",
        "Uttara Phalgunī",
        "Hasta",
        "Citrā",
        "Svātī",
        "Viśākhā",
        "Anurādhā",
        "Jyeṣṭhā",
        "Mūla",
        "Pūrva Āṣāḍhā",
        "Uttara Āṣāḍhā",
        "Śravaṇa",
        "Dhaniṣṭhā",
        "Śatabhiṣā",
        "Pūrva Bhādrapadā",
        "Uttara Bhādrapadā",
        "Revatī",
    ];
    NAMES.get(index as usize).copied().unwrap_or("Unknown")
}

fn city_by_id(id: &str) -> Option<City> {
    cities().into_iter().find(|city| city.id == id)
}

fn cities() -> [City; 3] {
    [
        City {
            id: "valencia",
            name: "Valencia, España",
            timezone: "Europe/Madrid",
            latitude: 39.4699,
            longitude: -0.3763,
        },
        City {
            id: "buenos-aires",
            name: "Buenos Aires, Argentina",
            timezone: "America/Argentina/Buenos_Aires",
            latitude: -34.6037,
            longitude: -58.3816,
        },
        City {
            id: "nabadwip",
            name: "Navadvīpa, India",
            timezone: "Asia/Kolkata",
            latitude: 23.4071,
            longitude: 88.3679,
        },
    ]
}

fn city_location(city: City) -> Result<GeoLocation, String> {
    let latitude = Latitude::from_degrees(city.latitude)
        .ok_or_else(|| format!("Latitud inválida para {}", city.name))?;
    let longitude = Longitude::from_degrees(city.longitude)
        .ok_or_else(|| format!("Longitud inválida para {}", city.name))?;

    Ok(GeoLocation::new(latitude, longitude))
}

fn timezone_offset_hours(city: City, date: CivilDate) -> f64 {
    match city.timezone {
        "Europe/Madrid" => {
            if is_europe_dst(date) {
                2.0
            } else {
                1.0
            }
        }
        "America/Argentina/Buenos_Aires" => -3.0,
        "Asia/Kolkata" => 5.5,
        _ => 0.0,
    }
}

fn is_europe_dst(date: CivilDate) -> bool {
    let start = last_sunday(date.year(), 3);
    let end = last_sunday(date.year(), 10);
    date >= start && date < end
}

fn last_sunday(year: i32, month: u8) -> CivilDate {
    let mut day = days_in_month(year, month);
    loop {
        let date = civil_date(year, month, day);
        if weekday_name(date) == "Domingo" {
            return date;
        }
        day -= 1;
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((percent_decode(key), percent_decode(value)))
        })
        .collect()
}

fn percent_decode(value: &str) -> String {
    let mut output = String::new();
    let mut bytes = value.as_bytes().iter().copied();

    while let Some(byte) = bytes.next() {
        match byte {
            b'+' => output.push(' '),
            b'%' => {
                let high = bytes.next();
                let low = bytes.next();
                if let (Some(high), Some(low)) = (high, low) {
                    let hex = [high, low];
                    if let Ok(hex_text) = std::str::from_utf8(&hex) {
                        if let Ok(decoded) = u8::from_str_radix(hex_text, 16) {
                            output.push(decoded as char);
                            continue;
                        }
                    }
                }
                output.push('%');
            }
            _ => output.push(byte as char),
        }
    }

    output
}

fn parse_date(value: &str) -> Result<CivilDate, String> {
    let parts: Vec<_> = value.split('-').collect();
    if parts.len() != 3 {
        return Err("La fecha debe usar YYYY-MM-DD".to_string());
    }
    let year = parts[0]
        .parse::<i32>()
        .map_err(|_| "Año inválido".to_string())?;
    let month = parts[1]
        .parse::<u8>()
        .map_err(|_| "Mes inválido".to_string())?;
    let day = parts[2]
        .parse::<u8>()
        .map_err(|_| "Día inválido".to_string())?;
    CivilDate::new(year, month, day).ok_or_else(|| "Fecha inválida".to_string())
}

fn next_day(date: CivilDate) -> CivilDate {
    let days = days_in_month(date.year(), date.month());
    if date.day() < days {
        civil_date(date.year(), date.month(), date.day() + 1)
    } else if date.month() < 12 {
        civil_date(date.year(), date.month() + 1, 1)
    } else {
        civil_date(date.year() + 1, 1, 1)
    }
}

fn previous_day(date: CivilDate) -> CivilDate {
    if date.day() > 1 {
        civil_date(date.year(), date.month(), date.day() - 1)
    } else if date.month() > 1 {
        let month = date.month() - 1;
        civil_date(date.year(), month, days_in_month(date.year(), month))
    } else {
        civil_date(date.year() - 1, 12, 31)
    }
}

fn civil_date(year: i32, month: u8, day: u8) -> CivilDate {
    CivilDate::new(year, month, day).expect("internal date arithmetic should produce valid dates")
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn day_of_year(date: CivilDate) -> u16 {
    let mut day = u16::from(date.day());

    for month in 1..date.month() {
        day += u16::from(days_in_month(date.year(), month));
    }

    day
}

fn display_date(date: CivilDate) -> String {
    format!(
        "{} de {} de {}",
        date.day(),
        month_name(date.month()),
        date.year()
    )
}

fn iso_date(date: CivilDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
}

fn month_name(month: u8) -> &'static str {
    match month {
        1 => "enero",
        2 => "febrero",
        3 => "marzo",
        4 => "abril",
        5 => "mayo",
        6 => "junio",
        7 => "julio",
        8 => "agosto",
        9 => "septiembre",
        10 => "octubre",
        11 => "noviembre",
        12 => "diciembre",
        _ => "mes desconocido",
    }
}

fn weekday_name(date: CivilDate) -> String {
    let time = CivilTime::new(12, 0, 0.0).expect("valid noon time");
    let date_time = CivilDateTime::new(date, time, TimeScale::Utc);
    match weekday(gregorian_to_jd(date_time)) {
        Weekday::Sunday => "Domingo",
        Weekday::Monday => "Lunes",
        Weekday::Tuesday => "Martes",
        Weekday::Wednesday => "Miércoles",
        Weekday::Thursday => "Jueves",
        Weekday::Friday => "Viernes",
        Weekday::Saturday => "Sábado",
    }
    .to_string()
}

fn local_hhmm(jd: JulianDate, offset_hours: f64) -> String {
    let local_jd = jd.add_days(DurationDays::new(offset_hours / 24.0));
    let date_time =
        jd_to_gregorian(local_jd, TimeScale::Utc).expect("Julian date should convert to Gregorian");
    let mut hour = date_time.time().hour();
    let mut minute = date_time.time().minute();
    if date_time.time().second() >= 30.0 {
        minute += 1;
        if minute >= 60 {
            minute = 0;
            hour = (hour + 1) % 24;
        }
    }

    format!("{hour:02}:{minute:02}")
}

fn json_error(message: &str) -> String {
    format!("{{\"error\":\"{}\"}}", json_escape(message))
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) {
    let allowed_origin =
        env::var("RC1_EXPERIENCE_ALLOWED_ORIGIN").unwrap_or_else(|_| "*".to_string());
    let _ = write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}; charset=utf-8\r\nAccess-Control-Allow-Origin: {allowed_origin}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
}
