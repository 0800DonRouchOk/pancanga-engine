# Sunrise Ephemeris Validation

Order: ORDEN-5 (sunrise validation against an astronomical ephemeris)

Status: MEASURED — no pass threshold applied; findings require owner review

## Objective

Measure `calendar::sunrise` against an independent astronomical reference
without modifying Pancanga Engine.

Before this validation, sunrise — the primitive every Ekādaśī date depends
on — was validated only against itself: the five unit tests in
`src/calendar/sunrise.rs` compare against engine-derived constants, the Swiss
validation (`swiss_validation.rs`) covers longitudes, elongation and tithi
index only, and the Tier 5 fixtures (PureBhakti, SCS Math) carry no sunrise
column.

This validation certifies the engine. It does not redefine the engine.

## Reference Convention (approved: Option A)

`OFFICIAL_ZENITH_DEGREES = 90.833` in `src/calendar/sunrise.rs` encodes the
geometric centre of the solar disc at a true altitude of `-50'` (34' standard
refraction + 16' semidiameter, fixed; no elevation, no atmosphere, no
variable semidiameter). No `swetest -rise` flag combination reproduces that
convention exactly, so the reference is computed directly:

```text
Reference sunrise = first upward crossing of true altitude -0.833333°
                    by the Sun's centre inside the local-mean-time day,
                    from swetest -fTh altitude series
                    (60-second coarse scan, 1-second fine scan,
                    linear interpolation).
```

Verified: the `-fTh` 0° crossing coincides with `swetest -rise -norefrac
-disccenter` (06:09:29.6 UTC, Valencia 2026-03-20), confirming `h` is the
true altitude of the disc centre.

The local-mean-time day is `[0h UT − longitude/15h, +24h)`. Mean solar time
is purely astronomical; no civil offset or DST rule is involved anywhere in
the comparison.

Two further instants are recorded as data only — never as reference, never as
pass/fail:

```text
swetest -rise          upper limb, modelled refraction (1013.25 hPa, 15 °C)
swetest -rise -hindu   disc centre on the geometric horizon, no refraction
```

The `-hindu` column exists so the data is available when the normative
question of which sunrise convention HBV's *udaya* denotes is resolved. That
question has no repository anchor today and is not resolved here.

## Runner

```text
03_Source/rust/crates/pancanga-engine/examples/sunrise_validation.rs
```

Usage from `03_Source/rust`:

```text
cargo run --release -p pancanga-engine --example sunrise_validation -- \
  --output ../../04_Tests/Astronomy/sunrise-ephemeris-validation.csv \
  --samples 1000
```

Options: `--output`, `--samples` (uniform civil dates, default 1000),
`--start-jd` (default 2415020.5), `--end-jd` (default 2488434.5),
`--cardinal-year-step` (year stride for non-shipped sites, default 3),
`--tolerance-seconds` (optional; when absent no classification is applied and
`difference_classification` reads `UNCLASSIFIED`). Environment: `SWETEST`,
`SWISSEPH_EPHE_PATH`, same discovery as `swiss_validation.rs`.

If `swetest` is not available the runner writes `EXTERNAL_ORACLE_PENDING`
and invents nothing.

## Sample

```text
Range:        1900-01-01 → 2100-12-31 (46STRESS certified range)
              (one uniform sample lands on 2101-01-01, the default end JD)
Ephemeris:    sepl_18.se1 / semo_18.se1 (1800–2399), swetest 2.10.03
Sites:        Valencia 39.4699 / -0.3763        (shipped, every year)
              Buenos Aires -34.6037 / -58.3816  (shipped, every year)
              Nabadwip 23.4071 / 88.3679        (shipped, every year)
              Equator 0 / 0, 55N, 60N, 65N, 66.5N, 70N on 0E (every 3rd year)
Cardinal:     20 Mar, 21 Jun, 22 Sep, 21 Dec per sampled year   4020 rows
Uniform:      1000 evenly spaced dates, round-robin over sites   1000 rows
Total:        5020 rows
```

Every year of the range is sampled for the shipped cities, so the leap cycle
is covered explicitly (see year-independence below).

## CSV

```text
04_Tests/Astronomy/sunrise-ephemeris-validation.csv   (5020 data rows)
```

Columns:

```text
case_id, sample_kind, location, latitude, longitude, civil_date, day_of_year,
leap_year, engine_sunrise_utc, engine_jd, reference_sunrise_utc, reference_jd,
error_seconds (engine − reference), day_shift (round(error / 86400)),
swetest_rise_default_utc, default_minus_reference_seconds,
swetest_hindu_utc, hindu_minus_reference_seconds,
engine_tithi_index (engine astronomy at engine instant),
reference_tithi_index (engine astronomy at reference instant),
tithi_label_differs, nearest_boundary_utc (±3 h, engine astronomy),
boundary_minus_reference_seconds, difference_classification, status, notes
```

Status values: `COMPARED`, `NO_SUNRISE_BOTH`, `ENGINE_SUNRISE_REFERENCE_NONE`,
`ENGINE_NONE_REFERENCE_SUNRISE`, `EXTERNAL_TOOL_ERROR`, `INVALID_SITE`,
`EXTERNAL_ORACLE_PENDING`.

## Result

| Metric | Count |
| --- | ---: |
| Rows | 5020 |
| COMPARED | 4771 |
| NO_SUNRISE_BOTH (polar, both agree) | 248 |
| ENGINE_SUNRISE_REFERENCE_NONE | 1 |
| ENGINE_NONE_REFERENCE_SUNRISE | 0 |
| EXTERNAL_TOOL_ERROR | 0 |
| COMPARED with `day_shift ≠ 0` | 459 (all Nabadwip, all +1 day) |
| COMPARED in-day (`day_shift = 0`) | 4312 |

### Finding 1 — Day shift at east longitude (Nabadwip)

For 459 of the 915 compared Nabadwip dates the engine's instant lies one day
**after** the local date's sunrise. The affected dates are day-of-year 97–276
(≈ 7 April – 3 October) in every sampled year.

Cause, from `src/calendar/sunrise.rs` `sunrise()`: the almanac universal time
is normalised into `[0, 24)` and then attached to the requested civil date by
`julian_date_at_universal_hours(date, universal_time)`. At Nabadwip in summer
local sunrise (~04:50 IST) is 23:20 UT of the **previous** UTC day; the
normalised 23.3 h is attached to the requested date, which is the sunrise of
the following local day. Example: `sunrise(2026-06-21)` returns
`2026-06-21 23:20:47 UTC` = 22 June 04:50 IST; the reference for 21 June is
`2026-06-20 23:20:56 UTC`.

In 453 of those 459 rows the tithi label at the engine's instant differs from
the label at the local date's sunrise. The doc comment states "The input date
is interpreted as the local civil date for the requested location"; the
returned instant is not that date's sunrise for these cases.

The same mechanism is visible in the frozen golden test
`calendar::tithi::tests::calculates_tithi_at_sydney_summer_sunrise` /
`sunrise::tests::calculates_mid_southern_latitude_summer_sunrise`
(`2026-12-21`, 151.2°E → JD 2461396.278 = 2026-12-21 18:40 UT = 22 Dec 05:40
local). Not modified.

Cross-reference for Campaign 46F.2 (measured correlation, not a resolution):
the SCS Math rows in `04_Tests/Vaishnava/normative-observance-certification.csv`
classified `DOC-001` ("no matching observance path") are SCS-0003 … SCS-0015,
SCS-0018 and SCS-0026. SCS-0003 … SCS-0014 (13 April – 22 September, doy
103–265) all fall inside the day-shift window and all carry
`engine_tithi_at_sunrise = 12` or `13` in `scsmath-validation.csv` — Dvādaśī on
an Ekādaśī date, i.e. the next day's sunrise. Every SCS row outside the window
carries 11 (or 10 for SCS-0018). SCS-0015 (6 October, doy 279) sits three
days past the sampled edge and also reads 12. The recorded classification
`CONFIGURATION DIFFERENCE` for those rows is contradicted by this
measurement; the classification decision belongs to 46F.2 and the owner.

### Finding 2 — Year-independence and secular drift

`sunrise()` uses the year only for the leap-day count. For a given
day-of-year the engine returns exactly two possible instants (leap / non-leap
year). Measured: 21 June 1900, 2026, 2050, 2100 → `04:34:23.565 UTC` at
Valencia; 2024, 2028 → `04:34:36.604 UTC`.

Consequence: the in-day error carries a secular trend plus a four-year
modulation. Valencia, 20 March, mean error by decade (seconds):

```text
1900s −60  1910s −62  1920s −55  1930s −50  1940s −39  1950s −31
1960s −24  1970s −16  1980s  −8  1990s   0  2000s  +7  2010s +15
2020s +23  2030s +31  2040s +39  2050s +46  2060s +54  2070s +62
2080s +70  2090s +77
```

Mean error by year mod 4 (Valencia, 20 March): 0 → −31 s, 1 → +39 s,
2 → +17 s, 3 → −7 s. The algorithm is centred on the 1990s and degrades
linearly away from it.

### In-day error distribution (engine − reference, seconds)

| Subset | n | min | p05 | median | p95 | max | \|e\| median | \|e\| p95 | \|e\| max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| All in-day | 4312 | −596.4 | −93.9 | +0.0 | +96.4 | +572.5 | 16.9 | 142.6 | 596.4 |
| Shipped cities | 2287 | −106.8 | −46.8 | +0.6 | +50.7 | +115.0 | 13.8 | 62.8 | 115.0 |
| Valencia | 916 | −106.8 | −54.3 | −0.2 | +59.4 | +115.0 | 15.7 | 69.7 | 115.0 |
| Buenos Aires | 915 | −93.2 | −41.7 | +0.8 | +45.8 | +93.9 | 13.1 | 58.9 | 93.9 |
| Nabadwip (in-day only) | 456 | −66.1 | −37.7 | +1.5 | +47.4 | +73.7 | 11.8 | 53.6 | 73.7 |
| Equator 0E | 379 | −28.0 | −17.4 | +0.7 | +18.2 | +26.0 | 7.6 | 20.3 | 28.0 |
| 55N | 379 | −160.7 | −94.5 | −0.8 | +100.0 | +169.3 | 23.0 | 116.2 | 169.3 |
| 60N | 379 | −199.1 | −115.0 | −2.0 | +117.5 | +199.7 | 27.5 | 144.6 | 199.7 |
| 65N | 379 | −237.7 | −150.4 | −5.4 | +153.2 | +256.3 | 38.5 | 183.1 | 256.3 |
| 66.5N | 303 | −596.4 | −185.4 | −4.1 | +175.1 | +572.5 | 62.2 | 213.3 | 596.4 |
| 70N | 206 | −329.1 | −239.4 | −5.7 | +249.7 | +333.6 | 106.1 | 274.0 | 333.6 |

By season (cardinal rows, all sites):

| Date | n | min | p05 | median | p95 | max | \|e\| median | \|e\| p95 | \|e\| max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 20 Mar | 1005 | −291.5 | −116.2 | +3.1 | +126.4 | +300.7 | 31.2 | 166.9 | 300.7 |
| 21 Jun | 670 | −70.6 | −30.6 | −2.4 | +22.8 | +46.3 | 13.3 | 31.8 | 70.6 |
| 22 Sep | 804 | −264.8 | −117.7 | −0.7 | +113.0 | +255.2 | 34.9 | 153.2 | 264.8 |
| 21 Dec | 938 | −41.8 | −16.7 | +0.2 | +15.7 | +34.6 | 7.9 | 18.2 | 41.8 |

Worst in-day case per site:

```text
Valencia      +115.0 s  2097-03-20
Buenos Aires   +93.9 s  2097-09-22
Nabadwip       +73.7 s  2097-03-20
Equator 0E     −28.0 s  2089-06-21
55N           +169.3 s  2089-03-20
60N           +199.7 s  2089-03-20
65N           +256.3 s  2093-02-24
66.5N         −596.4 s  2053-07-07
70N           +333.6 s  1930-08-02
```

Errors are largest at the equinoxes (where sunrise moves fastest day to day),
grow with latitude, and grow toward the ends of the range.

### Convention columns (data only)

| Column | All compared (n = 4771) | Shipped cities |
| --- | --- | --- |
| `-rise` default − reference | min +0.6 s, median +2.4 s, max +66.0 s | min +0.6 s, median +2.0 s, max +4.1 s |
| `-hindu` − reference | min +199.3 s, median +275.9 s, max +3059.5 s | min +217.1 s, median +257.9 s, max +302.9 s |

The `-hindu` convention is 3.6–5.0 minutes later than the engine's for the
shipped cities. Whether that convention is the normative one is a Knowledge
Base question (RESEARCH_REQUIRED); it is not decided here.

### Polar edge

- `sunrise()` returns `None` when `|cos H| > 1`
  (`sunrise_local_hour_angle`). Both engine and reference report no sunrise
  in 248 rows (70N: 172, 66.5N: 76).
- One boundary disagreement: `SUNRISE-04056`, 70N, 1907-01-17 — the engine
  returns `11:55:39 UTC`; the Sun's centre peaks at −0.994° true altitude that
  day (below −50'), `swetest -rise` also reports no rise, first rise
  18 January. The engine's polar boundary date is off by at least one day in
  this case. No case of the opposite kind (engine `None`, reference sunrise)
  occurred.

### Tithi-at-sunrise label impact

In-day rows (4312):

- Label flips (engine instant vs reference instant): **1** —
  `SUNRISE-04082`, 66.5N, 1912-04-10, error −261.3 s, boundary −163.1 s
  (engine index 22, reference 23). None at the shipped cities.
- Rows with a tithi boundary inside the shipped-city error window
  (±115.0 s of the reference): **3**, none flipped —
  `SUNRISE-00465` Valencia 2016-03-20 (error −20.0 s, boundary −63.1 s),
  `SUNRISE-04572` Nabadwip 2010-11-11 (error −6.4 s, boundary −21.2 s),
  `SUNRISE-04814` Buenos Aires 2059-07-21 (error +5.3 s, boundary +55.4 s).
- Rows with a boundary inside the global error window (±596.4 s): 48 (listed
  in the CSV via `boundary_minus_reference_seconds`).

Day-shifted rows (459, Nabadwip): **453** carry a different tithi label. This
is the dominant date-changing effect by three orders of magnitude.

## Methodological Rule

```text
External validation certifies the engine.
It does not redefine the engine.
Every discrepancy is documented before any line of code changes.
```

No engine code, golden test, Knowledge Base or normative document was
modified by this validation. The classification `ENGINE BUG` is reserved for
human review of these findings.
