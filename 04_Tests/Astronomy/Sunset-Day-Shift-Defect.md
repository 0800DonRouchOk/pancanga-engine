# Sunset Day-Shift Defect

Found under: ORDEN-3A (Campaign 46F.2 - Civil Configuration Audit), Phase 1

Status: **NOT FIXED. NOT AUTHORIZED FOR REMEDIATION.** This document records
a finding only. Fixing it is out of scope for the campaign that found it and
requires its own campaign with its own Phase 1-4 discipline, matching the
one `Sunrise-Day-Shift-Fix.md` already went through for the sunrise case.

## Defect

Two example binaries, `03_Source/rust/crates/pancanga-engine/examples/
parana_diagnostic.rs` and `examples/rc1_experience.rs`, each define an
`enum SolarEvent` and `fn solar_event(date, location, event)` that computes
sunset using the single-pass almanac formula
(`OFFICIAL_ZENITH_DEGREES = 90.833`, same constant `src/calendar/sunrise.rs`
uses). Both files carry a private helper of the same name,
`julian_date_at_universal_hours(date, universal_time)`.

The two implementations are byte-for-byte identical (diffed directly from
source, `enum SolarEvent` through the closing brace of `solar_event`:
zero-line diff). They exist as two copies, not one shared function, in
`examples/`, which is outside `src/` and therefore outside the frozen,
audited surface - not by design, but by the accident CLAUDE.md § 3.3 already
names: "Calendrical or civil-time computation that lives in `examples/`
rather than `src/` is outside the freeze by accident, not by design."

This shared implementation carries the identical day-carry pattern that
`Sunrise-Day-Shift-Fix.md` already diagnosed and fixed for `sunrise()` -
for sunset, unfixed:

```text
src/calendar/sunrise.rs (fixed under ORDEN-6, current):
  local_mean_time = normalize_hours(A)               // normalise FIRST
  universal_time  = local_mean_time - longitude_hour  // NOT renormalised:
                                                       // the carry survives
  attach: day_start(date) + universal_time / 24        // carry preserved,
                                                       // may land on the
                                                       // adjacent UTC date

examples/{parana_diagnostic,rc1_experience}.rs solar_event(Sunset) (unfixed):
  universal_time = normalize_hours(A - longitude_hour) // normalises the
                                                        // FINAL UT
  attach: CivilTime(universal_time) on `date` directly  // carry silently
                                                        // dropped, always
                                                        // attached to the
                                                        // requested date
```

`A` is the same raw pre-normalisation quantity in both formulas (local hour
angle + right ascension - equation-of-time terms), computed identically
apart from the sunrise/sunset branch inside the hour-angle step. The only
structural difference between the fixed sunrise path and the unfixed sunset
path is the order of normalisation relative to the longitude subtraction -
exactly the defect ORDEN-6 removed, reintroduced here because this code
lives in a second, separate implementation that ORDEN-6 never touched.

## ORDEN-6 did not cover this

`Sunrise-Day-Shift-Fix.md`'s stated scope was
"`03_Source/rust/crates/pancanga-engine/src/calendar/sunrise.rs` only." That
order fixed `sunrise()` inside `src/`. It never inspected, ran, or modified
`solar_event`/`SolarEvent` in `examples/`, and its Phase 3 impact
measurement (5,020-row ORDEN-5 CSV rerun, the 73,414-day Nabadwip scan, the
Ekādaśī observance-date impact table) covers sunrise instants only. None of
that measurement bears on sunset. This defect was not latent in the sense of
"already checked and found not to matter" - it was simply out of that
order's scope.

## Measured: where it is live

Re-implemented the shipped formula exactly (verified line-for-line against
the extracted Rust source) and evaluated it outside the repository for both
orderings - the shipped (buggy) order and the carry-preserving order that
mirrors the fixed `sunrise()` - to find where the two orderings disagree on
the calendar date.

**The three shipped cities show zero day-shift for sunset, at every sampled
date.** Checked Valencia, Buenos Aires, and Nabadwip at four seasonal points
each (15 January, both equinoxes, both solstices, 2026): the buggy ordering
and the carry-preserving ordering produce the identical calendar date in
every one of the twelve samples. The defect is latent for the shipped app,
but as a numeric coincidence of these three specific longitudes and typical
evening local-mean-times, not because the mechanism is absent.

**It is not latent for arbitrary longitude.** A longitude/date/latitude
scan (721 longitude samples per row, five seasonal dates, latitudes 0°,
±30°, 60°N) found the defect triggers for **west longitudes**, starting
around -74° to -140° depending on latitude and season and extending to
-180° - i.e., most of the Americas and the central/east Pacific. Example
counts (of 721 longitude samples, -180° to +180° in 0.5° steps):

| Latitude | Date | Samples shifted | Longitude range affected |
| ---: | --- | ---: | --- |
| 0° | 2026-01-15 | 187 | -180.0° to -87.0° |
| 0° | 2026-06-21 (solstice) | 183 | -180.0° to -89.0° |
| 30°N | 2026-06-21 (solstice) | 213 | -180.0° to -74.0° |
| 30°N | 2026-12-21 (solstice) | 153 | -180.0° to -104.0° |
| 60°N | 2026-06-21 (solstice) | 284 | -180.0° to -38.5° |
| 60°N | 2026-12-21 (solstice) | 88 | -180.0° to -136.5° |
| -30° | 2026-01-15 | 213 | -180.0° to -74.0° |

East longitudes did not trigger the shift in any sampled case. The
mechanism is asymmetric between morning and evening events: evening
local-mean-time runs roughly 15-23h, and subtracting a longitude offset
bounded to ±12h keeps the result inside `[0,24)` on the east side but can
push it past 24h for large negative (west) longitude offsets - the mirror
image of the sunrise case, where the affected side was east.

Where it triggers, the shift is always exactly one calendar day (the sunset
equivalent of ORDEN-6's measured ±86,400.0 s for sunrise - not independently
reconfirmed to nine decimal places here, but bounded by construction: the
only two states are "no carry" and "one calendar day of carry", there is no
intermediate value), and always in the same direction as the sunrise
defect: the buggy code reports the sunset of the **following** local day
under the label of the requested date.

## Path to the user-facing pāraṇa recommendation

```text
examples/rc1_experience.rs:
  calculate_response(query)
    -> parana_date = next_day(date)
    -> parana_sunset = solar_event(parana_date, location, SolarEvent::Sunset)   [rc1_experience.rs:526]
    -> daylight_duration = sunset_time.days_since(parana_sunrise).value()       [rc1_experience.rs:551]
    -> first_third_end = parana_sunrise.add_days(daylight_duration / 3.0)       [rc1_experience.rs:552]
    -> feeds ParanaPresentation.recommended, rendered to the user

examples/parana_diagnostic.rs:
  main()
    -> sunset_jd = solar_event(parana_date, location, SolarEvent::Sunset)      [parana_diagnostic.rs:26]
    -> daylight_duration = sunset_jd.days_since(sunrise_jd).value()            [parana_diagnostic.rs:27]
    -> first_third_end = sunrise_jd.add_days(daylight_duration / 3.0)          [parana_diagnostic.rs:28]
```

In `rc1_experience.rs` this is the RC1 desktop/web experience's actual
pāraṇa `recommended` field - the "first third of daylight" recommendation a
devotee reading the app would see. For any location west of roughly -74°
(more, at higher latitude, in summer), the daylight-duration computation
feeding that recommendation would use a sunset instant silently shifted one
day out of true, propagating a wrong `recommended` time to the display.

The validation harnesses (`scsmath_validation.rs`, `purebhakti_validation.rs`,
`swiss_validation.rs`, `sunrise_validation.rs`) do not call `solar_event` or
reference `SolarEvent` (confirmed by `grep`, zero matches) - they do not
compute sunset and are not exposed to this defect.

## Why this was not fixed here

ORDEN-3A (Campaign 46F.2) Phase 1 is an audit; it explicitly forbids
remediation without separate authorization, and this order's own Phase 2
gate remains unauthorized. Fixing `sunset()` properly requires the same
discipline `Sunrise-Day-Shift-Fix.md` applied to `sunrise()`:

- an owner decision on the day definition to use (ORDEN-6's Phase 1 - the
  local-mean-time boundary that decision reached should presumably apply
  symmetrically to sunset, but that is the owner's call, not an inference
  to make here);
- a Phase 3 impact measurement of its own: which dates, which locations,
  which pāraṇa `recommended` values change, once a real location is
  affected;
- explicit authorization before touching any test or expected value.

None of that has been done for sunset. This finding is scoped to: the
defect exists, it is byte-identical to the already-fixed sunrise mechanism,
it is latent for the three shipped cities, and it is live for a large class
of longitudes a configurable-location product would plausibly need to
support.

## What is not addressed here

- No fix was written. `src/calendar/` has no sunset implementation at all
  today (confirmed: `src/calendar/` contains `masa.rs`, `mod.rs`,
  `sunrise.rs`, `tithi.rs` only); consolidating the two `examples/` copies
  into one location, whether in `src/` or not, is a separate decision.
- No golden test was touched. None exists for this sunset formula outside
  the shipped cities' latent range.
- No impact count (how many certified-range days, how many pāraṇa
  recommendations, at which longitudes) was produced, because none of that
  can be measured without deciding the day-boundary definition first, and
  because it was not requested here.
- The ±115 s magnitude-of-error question ORDEN-5/ORDEN-6 raised for the
  solar-position algorithm itself is not evaluated for sunset; this finding
  is about the day-carry mechanism only.

## Files examined (not modified)

```text
03_Source/rust/crates/pancanga-engine/src/calendar/sunrise.rs
03_Source/rust/crates/pancanga-engine/examples/parana_diagnostic.rs
03_Source/rust/crates/pancanga-engine/examples/rc1_experience.rs
04_Tests/Astronomy/Sunrise-Day-Shift-Fix.md
```

No source file was changed to produce this document. The longitude/latitude
scan was run outside the repository (throwaway script, not committed) as a
direct transliteration of the shipped Rust formula, cross-checked token for
token against the extracted source shown above.

## STOP

This is a finding record only. Phase 2 of ORDEN-3A remains unauthorized and
does not cover this defect in any case, since it was scoped to sunrise
before this defect was found. Remediation requires a named campaign of its
own.
