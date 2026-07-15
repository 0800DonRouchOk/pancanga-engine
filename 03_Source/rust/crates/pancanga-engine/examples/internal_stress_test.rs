use std::env;
use std::fs;
use std::panic;
use std::path::Path;
use std::time::Instant;

use pancanga_engine::astronomy::{
    astronomical_tithi, lunar_solar_elongation, moon, nakshatra, solar, AstronomicalTithi,
};
use pancanga_engine::calendar::sunrise;
use pancanga_engine::core::time::{jd_to_gregorian, TimeScale};
use pancanga_engine::core::{
    CivilDate, DurationDays, GeoLocation, JulianDate, Latitude, Longitude,
};
use pancanga_engine::vaishnava::{
    arunodaya_start, classify_mahadvadasi, ekadasi_candidate_at_sunrise, hari_vasara_end,
    invalidate_viddha_candidate, observance_displacement, parana_window, EkadasiCandidate,
    EkadasiObservanceDisposition, MahadvadasiClassification, ParanaMode, ViddhaCandidateStatus,
};

const START_YEAR: i32 = 1900;
const END_YEAR: i32 = 2100;
const CITY_NAME: &str = "Valencia, España";
const LATITUDE: f64 = 39.4699;
const LONGITUDE: f64 = -0.3763;
const OUTPUT_DIR: &str = "../../04_Tests/Stress";
const ISSUE_CSV: &str = "../../04_Tests/Stress/internal-stress-issues.csv";
const EXTREMES_CSV: &str = "../../04_Tests/Stress/internal-stress-extremes.csv";
const REPORT_MD: &str = "../../04_Tests/Stress/Internal-Stress-Test.md";
const SCAN_STEP_DAYS: f64 = 1.0 / 8.0;
const REFINEMENT_STEPS: usize = 32;

#[derive(Debug, Clone, Copy)]
struct DayEvaluation {
    sunrise: JulianDate,
    arunodaya: JulianDate,
    tithi_at_sunrise: AstronomicalTithi,
    tithi_at_arunodaya: AstronomicalTithi,
    candidate: EkadasiCandidate,
    viddha: ViddhaCandidateStatus,
    disposition: EkadasiObservanceDisposition,
    nakshatra_index: u8,
}

#[derive(Debug, Clone, Copy)]
struct DvadasiInterval {
    start: JulianDate,
    end: JulianDate,
}

#[derive(Debug, Default)]
struct Stats {
    total_days: u64,
    completed_days: u64,
    panics: u64,
    crashes: u64,
    nan_or_infinite: u64,
    invalid_dates: u64,
    negative_intervals: u64,
    overflow_suspicions: u64,
    contradictions: u64,
    missing_sunrise: u64,
    observance_days: u64,
    standard_parana_windows: u64,
    short_dvadasi_contingencies: u64,
    engine_bugs: u64,
}

#[derive(Debug)]
struct Issue {
    date: CivilDate,
    class: &'static str,
    details: String,
}

#[derive(Debug, Clone)]
struct Extreme {
    metric: &'static str,
    date: CivilDate,
    value_days: f64,
    details: String,
}

#[derive(Debug, Default)]
struct Extremes {
    shortest_parana: Option<Extreme>,
    longest_parana: Option<Extreme>,
    shortest_hari_vasara: Option<Extreme>,
    longest_hari_vasara: Option<Extreme>,
    shortest_dvadasi: Option<Extreme>,
    longest_dvadasi: Option<Extreme>,
}

fn main() -> std::io::Result<()> {
    let started_at = Instant::now();
    let location = location();
    if let Ok(value) = env::var("STRESS_DIAGNOSTIC_DATE") {
        let date = parse_iso_date(&value).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "STRESS_DIAGNOSTIC_DATE must use YYYY-MM-DD",
            )
        })?;

        diagnose_date(date, location)?;
        return Ok(());
    }

    let start = date(START_YEAR, 1, 1);
    let end = date(END_YEAR, 12, 31);
    let mut current = start;
    let mut stats = Stats::default();
    let mut issues = Vec::new();
    let mut extremes = Extremes::default();

    run_static_invariants(&mut stats, &mut issues, start);
    let mut previous = evaluate_day(previous_day(start), location).map_err(|issue| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Unable to evaluate day before stress range: {} {}",
                issue.class, issue.details
            ),
        )
    })?;

    while current <= end {
        stats.total_days += 1;
        if stats.total_days % 10_000 == 0 {
            eprintln!(
                "stress progress: {} days processed through {}",
                stats.total_days,
                iso_date(current)
            );
        }

        let result = panic::catch_unwind(|| {
            let today = evaluate_day(current, location)?;
            let stress = stress_day(current, location, today, previous)?;
            Ok::<(DayEvaluation, DayStressResult), Issue>((today, stress))
        });

        match result {
            Ok(Ok((today, day_result))) => {
                stats.completed_days += 1;
                stats.observance_days += u64::from(day_result.observance);
                stats.standard_parana_windows += u64::from(day_result.standard_parana);
                stats.short_dvadasi_contingencies += u64::from(day_result.short_dvadasi);
                extremes.record(day_result.extremes);
                previous = today;
            }
            Ok(Err(issue)) => {
                classify_issue(&mut stats, issue.class);
                issues.push(issue);
            }
            Err(_) => {
                stats.panics += 1;
                stats.engine_bugs += 1;
                issues.push(Issue {
                    date: current,
                    class: "PANIC",
                    details: "panic captured while evaluating day".to_string(),
                });
            }
        }

        current = next_day(current);
    }

    write_outputs(
        &stats,
        &issues,
        &extremes,
        started_at.elapsed().as_secs_f64(),
    )
}

#[derive(Debug, Default)]
struct DayStressResult {
    observance: bool,
    standard_parana: bool,
    short_dvadasi: bool,
    extremes: Vec<Extreme>,
}

fn stress_day(
    date: CivilDate,
    location: GeoLocation,
    today: DayEvaluation,
    yesterday: DayEvaluation,
) -> Result<DayStressResult, Issue> {
    ensure_day_invariants(date, today)?;

    if today.candidate == EkadasiCandidate::NotCandidate
        && (today.viddha != ViddhaCandidateStatus::NotCandidate
            || today.disposition != EkadasiObservanceDisposition::NoObservanceCandidate)
    {
        return Err(issue(
            date,
            "CONTRADICTION",
            "NotCandidate produced downstream observance state",
        ));
    }

    let observes_today = today.disposition == EkadasiObservanceDisposition::ObserveOnEkadasi
        || yesterday.disposition == EkadasiObservanceDisposition::ObserveOnDvadasi;

    if !observes_today {
        return Ok(DayStressResult::default());
    }

    let parana_date = next_day(date);
    let parana_sunrise = sunrise(parana_date, location).ok_or_else(|| {
        issue(
            parana_date,
            "MISSING_SUNRISE",
            "No sunrise available for Parāṇa date",
        )
    })?;
    ensure_finite(parana_date, "parana_sunrise", parana_sunrise.value())?;

    let interval = dvadasi_interval_near(parana_sunrise).ok_or_else(|| {
        issue(
            parana_date,
            "CONTRADICTION",
            "No Dvādaśī interval found around Parāṇa sunrise",
        )
    })?;

    if interval.end <= interval.start {
        return Err(issue(
            parana_date,
            "NEGATIVE_INTERVAL",
            "Dvādaśī interval end is not after start",
        ));
    }

    let hari_vasara = hari_vasara_end(interval.start, interval.end).map_err(|error| {
        issue(
            parana_date,
            "NEGATIVE_INTERVAL",
            &format!("Hari-vāsara calculation rejected Dvādaśī interval: {error:?}"),
        )
    })?;

    if hari_vasara < interval.start || hari_vasara > interval.end {
        return Err(issue(
            parana_date,
            "CONTRADICTION",
            "Hari-vāsara end falls outside Dvādaśī interval",
        ));
    }

    let window = parana_window(parana_sunrise, interval.start, interval.end).map_err(|error| {
        issue(
            parana_date,
            "NEGATIVE_INTERVAL",
            &format!("Parāṇa window rejected Dvādaśī interval: {error:?}"),
        )
    })?;

    if window.starts_at < parana_sunrise || window.starts_at < hari_vasara {
        return Err(issue(
            parana_date,
            "CONTRADICTION",
            "Parāṇa starts before sunrise or before Hari-vāsara end",
        ));
    }

    if window.ends_at != interval.end {
        return Err(issue(
            parana_date,
            "CONTRADICTION",
            "Parāṇa end is not equal to Dvādaśī end",
        ));
    }

    if window.mode == ParanaMode::Standard && window.starts_at > window.ends_at {
        return Err(issue(
            parana_date,
            "NEGATIVE_INTERVAL",
            "Standard Parāṇa window has start after end",
        ));
    }

    let mut result = DayStressResult {
        observance: true,
        standard_parana: window.mode == ParanaMode::Standard,
        short_dvadasi: window.mode == ParanaMode::ShortDvadasi,
        extremes: Vec::new(),
    };

    let dvadasi_duration = interval.end.days_since(interval.start).value();
    let hari_vasara_duration = hari_vasara.days_since(interval.start).value();
    result.extremes.push(Extreme {
        metric: "Dvādaśī duration",
        date: parana_date,
        value_days: dvadasi_duration,
        details: format!(
            "start_jd={:.12}; end_jd={:.12}",
            interval.start.value(),
            interval.end.value()
        ),
    });
    result.extremes.push(Extreme {
        metric: "Hari-vāsara duration",
        date: parana_date,
        value_days: hari_vasara_duration,
        details: format!(
            "dvadasi_start_jd={:.12}; hari_vasara_end_jd={:.12}",
            interval.start.value(),
            hari_vasara.value()
        ),
    });

    if window.mode == ParanaMode::Standard {
        result.extremes.push(Extreme {
            metric: "Parāṇa duration",
            date: parana_date,
            value_days: window.ends_at.days_since(window.starts_at).value(),
            details: format!(
                "start_jd={:.12}; end_jd={:.12}",
                window.starts_at.value(),
                window.ends_at.value()
            ),
        });
    }

    Ok(result)
}

fn evaluate_day(date: CivilDate, location: GeoLocation) -> Result<DayEvaluation, Issue> {
    let sunrise_time = sunrise(date, location)
        .ok_or_else(|| issue(date, "MISSING_SUNRISE", "No sunrise available"))?;
    let arunodaya_time = arunodaya_start(sunrise_time);
    let (tithi_at_sunrise_value, moon_longitude) = astronomy_at(sunrise_time);
    let (tithi_at_arunodaya_value, _) = astronomy_at(arunodaya_time);
    let nakshatra_index = nakshatra::index(moon_longitude).value();
    let candidate = ekadasi_candidate_at_sunrise(tithi_at_sunrise_value);
    let viddha = invalidate_viddha_candidate(candidate, tithi_at_arunodaya_value);
    let disposition = observance_displacement(viddha);

    Ok(DayEvaluation {
        sunrise: sunrise_time,
        arunodaya: arunodaya_time,
        tithi_at_sunrise: tithi_at_sunrise_value,
        tithi_at_arunodaya: tithi_at_arunodaya_value,
        candidate,
        viddha,
        disposition,
        nakshatra_index,
    })
}

fn diagnose_date(date: CivilDate, location: GeoLocation) -> std::io::Result<()> {
    println!("Campaign 46STRESS diagnostic");
    println!("Location: {CITY_NAME}");
    println!("Date under review: {}", iso_date(date));
    println!();

    for offset in -1..=3 {
        let current = add_days(date, offset);
        match evaluate_day(current, location) {
            Ok(day) => {
                println!("{}:", iso_date(current));
                println!(
                    "  sunrise={} jd={:.12}",
                    utc_datetime(day.sunrise),
                    day.sunrise.value()
                );
                println!(
                    "  arunodaya={} jd={:.12}",
                    utc_datetime(day.arunodaya),
                    day.arunodaya.value()
                );
                println!("  tithi_at_sunrise={}", tithi_summary(day.tithi_at_sunrise));
                println!(
                    "  tithi_at_arunodaya={}",
                    tithi_summary(day.tithi_at_arunodaya)
                );
                println!("  nakshatra_index={}", day.nakshatra_index);
                println!("  candidate={:?}", day.candidate);
                println!("  viddha={:?}", day.viddha);
                println!("  disposition={:?}", day.disposition);
            }
            Err(issue) => {
                println!("{}: {} {}", iso_date(current), issue.class, issue.details);
            }
        }
        println!();
    }

    let tomorrow = next_day(date);
    let tomorrow_sunrise = sunrise(tomorrow, location).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "No sunrise for diagnostic tomorrow",
        )
    })?;
    println!(
        "Dvādaśī interval near following sunrise ({}):",
        iso_date(tomorrow)
    );
    if let Some(interval) = dvadasi_interval_near(tomorrow_sunrise) {
        let contains_sunrise =
            interval.start <= tomorrow_sunrise && tomorrow_sunrise <= interval.end;
        println!(
            "  start={} jd={:.12}",
            utc_datetime(interval.start),
            interval.start.value()
        );
        println!(
            "  end={} jd={:.12}",
            utc_datetime(interval.end),
            interval.end.value()
        );
        println!("  contains_following_sunrise={contains_sunrise}");
        println!(
            "  duration_hours={:.6}",
            interval.end.days_since(interval.start).value() * 24.0
        );
    } else {
        println!("  no Dvādaśī interval located");
    }

    Ok(())
}

fn ensure_day_invariants(date: CivilDate, day: DayEvaluation) -> Result<(), Issue> {
    ensure_finite(date, "sunrise", day.sunrise.value())?;
    ensure_finite(date, "arunodaya", day.arunodaya.value())?;

    if day.arunodaya >= day.sunrise {
        return Err(issue(
            date,
            "CONTRADICTION",
            "Aruṇodaya is not before sunrise",
        ));
    }

    ensure_tithi_valid(date, "sunrise", day.tithi_at_sunrise)?;
    ensure_tithi_valid(date, "arunodaya", day.tithi_at_arunodaya)?;

    if day.nakshatra_index > 26 {
        return Err(issue(
            date,
            "CONTRADICTION",
            "Nakṣatra index is outside 0..26",
        ));
    }

    Ok(())
}

fn run_static_invariants(stats: &mut Stats, issues: &mut Vec<Issue>, date: CivilDate) {
    let no_observance_with_facts = classify_mahadvadasi(
        MahadvadasiClassification::new(EkadasiObservanceDisposition::NoObservanceCandidate)
            .with_tithi_condition(pancanga_engine::vaishnava::TithiMahadvadasiCondition::Unmilani),
    );

    if no_observance_with_facts.is_some() {
        stats.contradictions += 1;
        stats.engine_bugs += 1;
        issues.push(issue(
            date,
            "CONTRADICTION",
            "Mahādvādaśī classified without observance",
        ));
    }
}

fn ensure_tithi_valid(
    date: CivilDate,
    label: &'static str,
    tithi: AstronomicalTithi,
) -> Result<(), Issue> {
    if tithi.index() > 29 {
        return Err(issue(
            date,
            "CONTRADICTION",
            &format!("{label} tithi index is outside 0..29"),
        ));
    }

    if !(1..=15).contains(&tithi.traditional_number()) {
        return Err(issue(
            date,
            "CONTRADICTION",
            &format!("{label} traditional tithi number is outside 1..15"),
        ));
    }

    Ok(())
}

fn ensure_finite(date: CivilDate, label: &'static str, value: f64) -> Result<(), Issue> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(issue(
            date,
            "NAN_OR_INFINITE",
            &format!("{label} is not finite"),
        ))
    }
}

fn tithi_at(jd: JulianDate) -> AstronomicalTithi {
    astronomy_at(jd).0
}

fn astronomy_at(jd: JulianDate) -> (AstronomicalTithi, pancanga_engine::core::Degrees) {
    let sun_longitude = solar::apparent_longitude(jd);
    let moon_longitude = moon::apparent_longitude(jd);
    let elongation = lunar_solar_elongation(moon_longitude, sun_longitude);

    (astronomical_tithi(elongation), moon_longitude)
}

fn is_dvadasi(jd: JulianDate) -> bool {
    tithi_at(jd).traditional_number() == 12
}

fn dvadasi_interval_near(anchor: JulianDate) -> Option<DvadasiInterval> {
    let start_scan = anchor.add_days(DurationDays::new(-2.5));
    let end_scan = anchor.add_days(DurationDays::new(2.5));
    let mut previous = start_scan;
    let mut previous_is_dvadasi = is_dvadasi(previous);
    let mut current = start_scan.add_days(DurationDays::new(SCAN_STEP_DAYS));
    let mut start = if previous_is_dvadasi {
        Some(refine_backward_to_dvadasi_start(previous))
    } else {
        None
    };
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

                if interval.start <= anchor && anchor <= interval.end {
                    return Some(interval);
                }

                let distance = if anchor < interval.start {
                    interval.start.days_since(anchor).value().abs()
                } else {
                    anchor.days_since(interval.end).value().abs()
                };

                if best_interval
                    .as_ref()
                    .map(|(best_distance, _)| distance < *best_distance)
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

fn refine_backward_to_dvadasi_start(anchor_inside: JulianDate) -> JulianDate {
    let mut high = anchor_inside;
    let mut low = high.add_days(DurationDays::new(-SCAN_STEP_DAYS));

    while is_dvadasi(low) {
        high = low;
        low = low.add_days(DurationDays::new(-SCAN_STEP_DAYS));
    }

    refine_boundary(low, high, true)
}

fn refine_boundary(
    mut low: JulianDate,
    mut high: JulianDate,
    target_is_dvadasi: bool,
) -> JulianDate {
    for _ in 0..REFINEMENT_STEPS {
        let midpoint = JulianDate::new((low.value() + high.value()) / 2.0);
        if is_dvadasi(midpoint) == target_is_dvadasi {
            high = midpoint;
        } else {
            low = midpoint;
        }
    }

    high
}

impl Extremes {
    fn record(&mut self, extremes: Vec<Extreme>) {
        for extreme in extremes {
            match extreme.metric {
                "Parāṇa duration" => {
                    update_min(&mut self.shortest_parana, extreme.clone());
                    update_max(&mut self.longest_parana, extreme);
                }
                "Hari-vāsara duration" => {
                    update_min(&mut self.shortest_hari_vasara, extreme.clone());
                    update_max(&mut self.longest_hari_vasara, extreme);
                }
                "Dvādaśī duration" => {
                    update_min(&mut self.shortest_dvadasi, extreme.clone());
                    update_max(&mut self.longest_dvadasi, extreme);
                }
                _ => {}
            }
        }
    }

    fn all(&self) -> Vec<&Extreme> {
        [
            self.shortest_parana.as_ref(),
            self.longest_parana.as_ref(),
            self.shortest_hari_vasara.as_ref(),
            self.longest_hari_vasara.as_ref(),
            self.shortest_dvadasi.as_ref(),
            self.longest_dvadasi.as_ref(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn update_min(slot: &mut Option<Extreme>, candidate: Extreme) {
    if slot
        .as_ref()
        .map(|current| candidate.value_days < current.value_days)
        .unwrap_or(true)
    {
        *slot = Some(candidate);
    }
}

fn update_max(slot: &mut Option<Extreme>, candidate: Extreme) {
    if slot
        .as_ref()
        .map(|current| candidate.value_days > current.value_days)
        .unwrap_or(true)
    {
        *slot = Some(candidate);
    }
}

fn classify_issue(stats: &mut Stats, class: &str) {
    match class {
        "MISSING_SUNRISE" => stats.missing_sunrise += 1,
        "NAN_OR_INFINITE" => stats.nan_or_infinite += 1,
        "INVALID_DATE" => stats.invalid_dates += 1,
        "NEGATIVE_INTERVAL" => stats.negative_intervals += 1,
        "OVERFLOW" => stats.overflow_suspicions += 1,
        "CONTRADICTION" => stats.contradictions += 1,
        _ => stats.crashes += 1,
    }
    stats.engine_bugs += 1;
}

fn write_outputs(
    stats: &Stats,
    issues: &[Issue],
    extremes: &Extremes,
    elapsed_seconds: f64,
) -> std::io::Result<()> {
    fs::create_dir_all(OUTPUT_DIR)?;
    fs::write(ISSUE_CSV, issue_csv(issues))?;
    fs::write(EXTREMES_CSV, extremes_csv(extremes))?;
    fs::write(
        REPORT_MD,
        report_md(stats, issues, extremes, elapsed_seconds),
    )?;

    println!("Campaign 46STRESS");
    if stats.engine_bugs == 0 && stats.panics == 0 {
        println!("PASS");
    } else {
        println!("OPEN");
    }
    println!("Cases executed: {}", stats.total_days);
    println!("Engine bugs: {}", stats.engine_bugs);
    println!("Report: {REPORT_MD}");

    Ok(())
}

fn issue_csv(issues: &[Issue]) -> String {
    let mut output = String::from("date,class,details\n");
    for issue in issues {
        output.push_str(&format!(
            "{},{},\"{}\"\n",
            iso_date(issue.date),
            issue.class,
            issue.details.replace('"', "\"\"")
        ));
    }
    output
}

fn extremes_csv(extremes: &Extremes) -> String {
    let mut output = String::from("metric,date,value_days,value_hours,details\n");
    for extreme in extremes.all() {
        output.push_str(&format!(
            "\"{}\",{},{:.12},{:.6},\"{}\"\n",
            extreme.metric,
            iso_date(extreme.date),
            extreme.value_days,
            extreme.value_days * 24.0,
            extreme.details.replace('"', "\"\"")
        ));
    }
    output
}

fn report_md(stats: &Stats, issues: &[Issue], extremes: &Extremes, elapsed_seconds: f64) -> String {
    let status = if stats.engine_bugs == 0 && stats.panics == 0 {
        "✅ PASS"
    } else {
        "🟡 OPEN"
    };
    let mut output = format!(
        "# Internal Stress Test\n\n\
Campaign: 46STRESS\n\n\
Status: {status}\n\n\
## Scope\n\n\
This stress test attempts to break Pancanga Engine without modifying the motor,\n\
Knowledge Base, architecture, or supported-location scope.\n\n\
Configuration:\n\n\
```text\n\
Location: {CITY_NAME}\n\
Latitude: {LATITUDE}\n\
Longitude: {LONGITUDE}\n\
Range: {START_YEAR}-01-01 to {END_YEAR}-12-31 inclusive\n\
```\n\n\
## Summary\n\n\
| Metric | Count |\n\
|---|---:|\n\
| Cases executed | {} |\n\
| Completed days | {} |\n\
| Panics captured | {} |\n\
| Crashes | {} |\n\
| NaN / infinite values | {} |\n\
| Invalid dates | {} |\n\
| Negative intervals | {} |\n\
| Overflow suspicions | {} |\n\
| Contradictions | {} |\n\
| Missing sunrise | {} |\n\
| Observance days detected | {} |\n\
| Standard Parāṇa windows | {} |\n\
| Short Dvādaśī contingencies | {} |\n\
| Confirmed ENGINE BUG | {} |\n\
\n\
Elapsed seconds: {:.3}\n\n",
        stats.total_days,
        stats.completed_days,
        stats.panics,
        stats.crashes,
        stats.nan_or_infinite,
        stats.invalid_dates,
        stats.negative_intervals,
        stats.overflow_suspicions,
        stats.contradictions,
        stats.missing_sunrise,
        stats.observance_days,
        stats.standard_parana_windows,
        stats.short_dvadasi_contingencies,
        stats.engine_bugs,
        elapsed_seconds
    );

    output.push_str("## Invariants Checked\n\n");
    output.push_str(
        "- no panic per civil day;\n\
- sunrise and Aruṇodaya are finite;\n\
- Aruṇodaya precedes sunrise;\n\
- tithi index remains in `0..29`;\n\
- traditional tithi number remains in `1..15`;\n\
- nakṣatra index remains in `0..26`;\n\
- `NotCandidate` does not produce an observance;\n\
- Parāṇa is calculated only for observance days;\n\
- Dvādaśī interval is positive;\n\
- Hari-vāsara end is inside Dvādaśī;\n\
- standard Parāṇa windows do not have negative duration;\n\
- Mahādvādaśī is not classified without observance.\n\n",
    );

    output.push_str("## Extremes\n\n");
    output.push_str("| Metric | Date | Days | Hours | Details |\n|---|---|---:|---:|---|\n");
    for extreme in extremes.all() {
        output.push_str(&format!(
            "| {} | {} | {:.12} | {:.6} | {} |\n",
            extreme.metric,
            iso_date(extreme.date),
            extreme.value_days,
            extreme.value_days * 24.0,
            extreme.details
        ));
    }

    output.push_str("\n## Issues\n\n");
    if issues.is_empty() {
        output.push_str("No issues recorded.\n\n");
    } else {
        output.push_str("| Date | Class | Details |\n|---|---|---|\n");
        for issue in issues {
            output.push_str(&format!(
                "| {} | {} | {} |\n",
                iso_date(issue.date),
                issue.class,
                issue.details
            ));
        }
        output.push('\n');
    }

    output.push_str("## Artifacts\n\n");
    output.push_str(
        "```text\n\
04_Tests/Stress/Internal-Stress-Test.md\n\
04_Tests/Stress/internal-stress-issues.csv\n\
04_Tests/Stress/internal-stress-extremes.csv\n\
```\n\n",
    );

    output.push_str("## Diagnostic Note\n\n");
    output.push_str(
        "An initial stress pass used a stricter sunrise-only invariant for displaced\n\
observance days: `ObserveOnDvadasi` was expected to imply Dvādaśī at the\n\
following sunrise. The first flagged case, 1904-09-19, showed that this was a\n\
validation-tool assumption rather than an engine result: Dvādaśī began after\n\
the following sunrise and continued into the next civil day. The stress test now\n\
validates the actual Dvādaśī interval used for Parāṇa instead of requiring\n\
Dvādaśī at a specific sunrise.\n\n",
    );

    output.push_str("## Methodological Note\n\n");
    output.push_str(
        "This campaign is internal certification. It does not redefine the engine. If a\n\
difference or impossible state appears, it must be documented before any code is\n\
changed.\n",
    );

    output
}

fn issue(date: CivilDate, class: &'static str, details: &str) -> Issue {
    Issue {
        date,
        class,
        details: details.to_string(),
    }
}

fn location() -> GeoLocation {
    let latitude = Latitude::from_degrees(LATITUDE).expect("stress latitude should be valid");
    let longitude = Longitude::from_degrees(LONGITUDE).expect("stress longitude should be valid");
    GeoLocation::new(latitude, longitude)
}

fn next_day(date: CivilDate) -> CivilDate {
    let days = days_in_month(date.year(), date.month());
    if date.day() < days {
        return civil_date(date.year(), date.month(), date.day() + 1);
    }

    if date.month() < 12 {
        civil_date(date.year(), date.month() + 1, 1)
    } else {
        civil_date(date.year() + 1, 1, 1)
    }
}

fn previous_day(date: CivilDate) -> CivilDate {
    if date.day() > 1 {
        return civil_date(date.year(), date.month(), date.day() - 1);
    }

    if date.month() > 1 {
        let month = date.month() - 1;
        civil_date(date.year(), month, days_in_month(date.year(), month))
    } else {
        civil_date(date.year() - 1, 12, 31)
    }
}

fn add_days(mut date: CivilDate, offset: i32) -> CivilDate {
    if offset >= 0 {
        for _ in 0..offset {
            date = next_day(date);
        }
    } else {
        for _ in 0..offset.abs() {
            date = previous_day(date);
        }
    }

    date
}

fn date(year: i32, month: u8, day: u8) -> CivilDate {
    CivilDate::new(year, month, day).expect("stress date should be valid")
}

fn civil_date(year: i32, month: u8, day: u8) -> CivilDate {
    CivilDate::new(year, month, day).expect("internal date arithmetic should be valid")
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

fn iso_date(date: CivilDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
}

fn parse_iso_date(value: &str) -> Option<CivilDate> {
    let mut parts = value.split('-');
    let year = parts.next()?.parse().ok()?;
    let month = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;

    if parts.next().is_some() {
        return None;
    }

    CivilDate::new(year, month, day)
}

fn tithi_summary(tithi: AstronomicalTithi) -> String {
    format!(
        "index={} paksha={:?} number={}",
        tithi.index(),
        tithi.paksha(),
        tithi.traditional_number()
    )
}

fn utc_datetime(jd: JulianDate) -> String {
    let Some(date_time) = jd_to_gregorian(jd, TimeScale::Utc) else {
        return "invalid-utc".to_string();
    };
    let date = date_time.date();
    let time = date_time.time();

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:06.3} UTC",
        date.year(),
        date.month(),
        date.day(),
        time.hour(),
        time.minute(),
        time.second()
    )
}

#[allow(dead_code)]
fn ensure_output_dir_exists() -> bool {
    Path::new(OUTPUT_DIR).exists()
}
