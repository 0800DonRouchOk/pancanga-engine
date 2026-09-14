# SCS Math Civil Configuration Audit

Order: ORDEN-3A (Campaign 46F.2 - Civil Configuration Audit)

Status: PHASE 1 ONLY. Phase 2 (remediation) not started; this order stops
before it by explicit instruction.

## Governing document and its framing

`00_Project/Final-Certification-Stage.md § Campaign 46F.2` states the
objective as "Demonstrate that the remaining SCS Math differences come from
civil/calendar configuration and not from the engine" and records an
"Expected result: ENGINE BUG: 0". Per ORDEN-3A's own integrity note, that
expected result is not treated here as a target. This report states what was
measured, including where a difference originates in configuration and where
it does not.

## Context carried in from ORDEN-6 (do not re-derive)

`04_Tests/Astronomy/Sunrise-Day-Shift-Fix.md` fixed a day-carry defect in
`src/calendar/sunrise.rs`: the almanac universal-time-of-day was normalised
into `[0,24)` and attached to the requested civil date, silently discarding
the day carry. At Nabadwip this shifted the reported sunrise, for
day-of-year 91-279 (~7 April - 3 October) every year, one calendar day later
than the true local-mean-time sunrise. The fix was applied to
`src/calendar/sunrise.rs` only and is merged to `main`
(commit `2a6d5fd`). Four Sydney golden expectations were corrected under
named owner authorization (commit `ed9450a`). `04_Tests/SCSMath/
scsmath-validation.csv` was deliberately left un-overwritten as the pre-fix
baseline.

## Re-derivation, not reuse, of the ORDEN-6 SCS Math numbers

`scsmath-validation.csv` on disk (26 rows, pre-fix baseline) records
`observance_status` PASS for 11 rows (SCS-0001, 0002, 0016, 0017, 0019-0025)
and DIFFERENCE (classified `CALENDAR DIFFERENCE`) for the remaining 15
(SCS-0003-0015, 0018, 0026).

I built the current workspace (`cargo build`, toolchain 1.85.0, unmodified)
and ran the unmodified `examples/scsmath_validation.rs` binary against
`04_Tests/SCSMath/scsmath-fixtures.csv`, writing output to a scratch path
(not committed, not overwriting the recorded CSV):

```text
cargo run -p pancanga-engine --example scsmath_validation \
  -- --input 04_Tests/SCSMath/scsmath-fixtures.csv --output <scratch>.csv
```

Result, current `main` (post-ORDEN-6-fix engine):

```text
observance_status PASS: 23 / 26
observance_status DIFFERENCE: 3 / 26  (SCS-0012, SCS-0018, SCS-0026)
```

This independently reproduces the "11 -> 23 of 26" figure carried over from
ORDEN-6's own re-run rather than assuming it. Rows SCS-0003 through
SCS-0011, SCS-0013, SCS-0014, SCS-0015 (12 rows) move from
`no_matching_observance_path` (tithi 12/13 at sunrise) to
`current_valid_ekadasi` (tithi 11), matching the ORDEN-6 report exactly.
SCS-0012 (Vyañjulī Mahādvādaśī - not a day-shift case; the validator does
not derive Mahādvādaśī facts), SCS-0018, and SCS-0026 (Aruṇodaya Viddhā
case) remain DIFFERENCE, consistent with `SCSMath-Difference-Report.md`'s
own per-case notes for those three rows ("larger calendar alignment
difference than the ordinary one-day pattern" for SCS-0018; an Aruṇodaya
Viddhā read for SCS-0026).

The binary's own stdout summary ("PASS: 0") counts the combined `status`
column (observance AND pāraṇa both PASS), which stays at 0 because no
pāraṇa window is PASS under either engine version - a separate question,
addressed below. This is not a contradiction of the 23/26 observance figure;
it is a different, coarser counter in the same output.

## Vocabulary divergence - reported, not normalised

`scsmath-validation.csv` writes `CONFIGURATION DIFFERENCE` (space).
`SCSMath-Difference-Report.md` writes `CONFIGURATION_DIFFERENCE`
(underscore). Both spellings are used verbatim below depending on which
document is being quoted; neither file was edited.

## Checklist items

### 1. Sunset

`03_Source/rust/crates/pancanga-engine/examples/parana_diagnostic.rs` and
`examples/rc1_experience.rs` each define an `enum SolarEvent` and
`fn solar_event(...)`. Byte-for-byte diff of both blocks (enum through the
closing brace of `solar_event`): **identical, 0 divergence** between the two
files. Same `OFFICIAL_ZENITH_DEGREES = 90.833`, same single-pass almanac
formula, same helper `julian_date_at_universal_hours`, duplicated separately
in each file with identical bodies.

**That helper carries the exact pre-ORDEN-6 pattern for sunset, unfixed.**
Structurally:

```text
src/calendar/sunrise.rs (fixed, current):
  local_mean_time = normalize_hours(A)              // normalise FIRST
  universal_time  = local_mean_time - longitude_hour // no renormalisation - carries the day
  attach: day_start(date) + universal_time / 24       // carry preserved

examples/{parana_diagnostic,rc1_experience}.rs solar_event(Sunset) (unfixed):
  universal_time = normalize_hours(A - longitude_hour) // normalise the FINAL UT
  attach: CivilTime(universal_time) on `date` directly  // carry silently dropped
```

This is the identical defect class ORDEN-6 fixed, present a second time, for
a different solar event, outside `src/`. It was never in ORDEN-6's scope
(`src/calendar/sunrise.rs` only) and it is not protected by the freeze
(CLAUDE.md § 3.3: computation in `examples/` sits "outside the freeze by
accident, not by design").

Measured (exact re-implementation of the shipped formula, run outside the
repository, cross-checked against the byte-identical extracted Rust source):

- **The three shipped cities (Valencia, Buenos Aires, Nabadwip) show zero
  day-shift for sunset, at every sampled date (four seasonal points -
  15 Jan, both equinoxes, both solstices - per city, 2026).** The unfixed
  ordering happens to produce the same calendar date as the carry-preserving
  ordering would, for all three shipped longitudes, year-round. Confirmed
  latent for the shipped app, same conclusion ORDEN-1 reached for the
  `timezone_offset_hours` `_ => 0.0` arm, but for an unrelated reason (here
  it is a numeric coincidence of the three shipped longitudes, not an
  exhaustive match arm).
- **It is not latent for arbitrary longitude.** A longitude/date/latitude
  scan (721 longitude samples per row, five seasonal dates, four latitudes:
  0°, ±30°, 60°N) found the defect triggers for **west longitudes**,
  starting around -74° to -140° depending on latitude and season and
  extending to -180° - i.e., most of the Americas and the central/east
  Pacific. Example: at 30°N on the June solstice, 213 of 721 sampled
  longitudes (about -74° westward) show the shift. East longitudes did not
  trigger it in any sampled case; the mechanism is asymmetric because
  evening local-mean-time (~15-23h) minus a longitude offset bounded to
  ±12h stays inside `[0,24)` for the calculation's east side but can exceed
  24h for large negative (west) longitude offsets.
- Where it triggers, the shift is always exactly one calendar day
  (equivalent to ORDEN-6's measured ±86,400.0 s for sunrise), always in the
  same direction as the sunrise defect: the buggy code reports the sunset of
  the **following** local day under the label of the requested date.
- This feeds `parana_sunset -> daylight_duration -> first_third_end`
  in both `parana_diagnostic.rs` and `rc1_experience.rs`; in
  `rc1_experience.rs` this produces the pāraṇa `recommended` field shown to
  the user. Given the product context (arbitrary user-configured location,
  not just the three shipped cities), this is a live, user-reachable defect
  for any west-longitude location a future configurable build would allow,
  even though it does not reach the three cities shipped today.
- The validation harnesses (`scsmath_validation.rs`, `purebhakti_validation.rs`,
  `swiss_validation.rs`, `sunrise_validation.rs`) do not import or call
  `solar_event`/`SolarEvent` - confirmed by `grep`, zero matches. They do
  not compute sunset at all; there is nothing to rewire in them for this
  finding.

### 2. UTC offset and DST

Confirmed by direct reading, not re-derived from inference:
`timezone_offset_hours(city, date)` and `is_europe_dst(date)` are defined
once in `rc1_experience.rs`; `timezone_offset_hours` has exactly one call
site (`rc1_experience.rs:302`, inside `calculate_response`), also in that
file. `grep -rn "timezone_offset_hours|is_europe_dst(" examples/*.rs`
returns only these definitions and that one call. The three validation
harnesses use their own constants (`SCSMATH_UTC_OFFSET_HOURS = 5.5`,
`BUENOS_AIRES_UTC_OFFSET_HOURS = -3.0` duplicated in
`purebhakti_validation.rs` and `parana_diagnostic.rs`), confirmed below
under item 5a.

`cities()` returns a fixed `[City; 3]` (Valencia/`Europe/Madrid`,
Buenos Aires/`America/Argentina/Buenos_Aires`, Nabadwip/`Asia/Kolkata`), and
all three are covered by `timezone_offset_hours`'s match arms, so the
`_ => 0.0` fallthrough is confirmed unreachable from the shipped app - no
other call site or code path reaches `timezone_offset_hours` with an
uncovered `city.timezone` value. If the shipped city list is ever extended
without a matching arm, the new city would silently render at UTC+0 with no
error - a silent civil-time default per CLAUDE.md § 5.3, currently dormant.

### 3. EU DST transition boundary

The EU rule transitions at 01:00 UTC on the last Sunday of March (CET->CEST)
and the last Sunday of October (CEST->CET). `is_europe_dst` compares whole
`CivilDate` values (`date >= start && date < end`, `start`/`end` from
`last_sunday(year, 3)` / `last_sunday(year, 10)`), with no time-of-day
component. This cannot represent an intra-day transition:

- **Start date** (e.g. 2026-03-29, computed): the function returns `true`
  (DST, offset 2.0) for the entire calendar date, including the interval
  `[00:00, 01:00)` UTC, which is still CET (offset 1.0) until the real
  01:00 UTC transition. Any displayed instant in that one-hour UTC window on
  the start date is rendered **one hour ahead** of the correct wall clock.
- **End date** (e.g. 2026-10-25, computed): the function returns `false`
  (offset 1.0) for the entire calendar date, but `[00:00, 01:00)` UTC on
  that date is still CEST (offset 2.0) until the transition. Any displayed
  instant in that window is rendered **one hour behind**.
- Magnitude: exactly 3,600 s, confined to a one-hour UTC window on exactly
  two calendar dates per year, symmetric in each direction.

This is a direct instance of what
`FEST-001-Janmashtami-Decision-Spec.md § 4` requires generally: "local
offsets are applied separately to each sunrise and sunset... a civil-date or
DST transition inside the interval must not turn it into a fixed-clock
calculation." `timezone_offset_hours` is exactly a fixed-clock calculation,
called once per request and applied uniformly to every instant in the
displayed pāraṇa window (`calculate_response` at `rc1_experience.rs:302-304`
computes `offset_hours` once from the requested civil date and passes the
single value into `parana_presentation`).

I did not exhaustively scan the 46STRESS certified range (1900-2100) for a
pāraṇa-window instant that actually lands inside the affected one-hour UTC
window at Valencia's longitude (sunrise/sunset there fall well outside
`[00:00,01:00)` UTC on ordinary dates, so the practical exposure is confined
to whichever pāraṇa-window endpoint can drift into that hour on the two
transition dates specifically) - the mechanism and its exact magnitude are
confirmed from the code and calendar arithmetic; whether it has actually
displaced a rendered pāraṇa time in the certified range is UNVERIFIED.

### 4. `last_sunday` / `days_in_month` divergence

`last_sunday` (only in `rc1_experience.rs`) decrements `day` from
`days_in_month(year, month)` and compares `weekday_name(date) == "Domingo"`.
`rc1_experience.rs`'s own `days_in_month` ends `_ => 30` (not `_ => 0`), so
the previously-suspected u8 underflow via an invalid days-in-month value
does not exist in this file - confirmed.

`weekday_name` matches exhaustively on the `Weekday` enum computed from the
real Julian-date weekday, always returning one of the seven hardcoded
Spanish strings. The loop's termination is **not independent of that
string**: it terminates because the code path always reaches
`Weekday::Sunday -> "Domingo"` for some `day` within 7 decrements, and the
comparison is an exact-string match against the literal `"Domingo"`. If
`weekday_name` were ever localised (a different language, or the match arms
reordered/renamed), the equality would never hold, `day` would decrement
past 1 to 0, and `civil_date(year, month, 0)` would panic (`u8` underflow on
the next decrement, or an invalid-date panic first) - there is no
independent bound. This is latent under the current hardcoded Spanish
strings and reachable only if `weekday_name` changes.

`days_in_month` exists five times across `examples/` (`internal_stress_test.rs`,
`parana_diagnostic.rs`, `purebhakti_validation.rs`, `rc1_experience.rs`,
`scsmath_validation.rs`). Four end `_ => 0`; `rc1_experience.rs` alone ends
`_ => 30`. Checked every decrementing month/day loop in the four `_ => 0`
files: `purebhakti_validation.rs` and `scsmath_validation.rs` each have a
`previous_day` function that decrements `month` but guards it explicitly
(`if month == 1 { year -= 1; month = 12 } else { month -= 1 }`) before
calling `days_in_month`, so `days_in_month` is never called with an
out-of-range month there - the `_ => 0` arm is unreachable via that path.
`parana_diagnostic.rs`'s only month loop (`day_of_year`) iterates forward
(`for month in 1..date.month()`), never invalid. `internal_stress_test.rs`
does not contain a decrementing month loop. No divergence between `_ => 0`
and `_ => 30` was found to produce a different result for the same input in
any of the code paths that actually call these functions.

Separately, noted in passing: `sunrise_validation.rs`'s `is_leap_year`
delegates to `pancanga_engine::core::time::is_leap_year` (the `src/`
canonical implementation); the other five copies (including
`rc1_experience.rs`) reimplement the identical formula locally instead of
calling that function. All six bodies are computationally identical
(standard Gregorian rule); no divergence, but five of the six duplicate a
function that already exists in `src/`.

### 4b. Year guard in `is_europe_dst`

Confirmed: `is_europe_dst` calls `last_sunday(date.year(), 3)` and
`last_sunday(date.year(), 10)` unconditionally for any year, applying one
DST rule across all of them. Whether this rule is historically correct for
Spain before 1974 (Spain did not observe DST in some periods before EU
harmonisation) is knowledge external to this repository -
marked **EXTERNAL**, not evidenced from the repo. Within the repo: 46STRESS
certifies 1900-2100 (`04_Tests/Stress/Internal-Stress-Test.md`), and its
only civil input is `const CITY_NAME: &str = "Valencia, España"` with no
call path through `is_europe_dst`/`timezone_offset_hours` (confirmed by
`grep`, no matches outside `rc1_experience.rs`) - so the unconditional rule
is unexercised by that certification regardless of its historical accuracy.

### 5. Hardcoded offset constants and the Navadvīpa discrepancy

**5a. The three constants.** Confirmed exactly three, not two:
`SCSMATH_UTC_OFFSET_HOURS = 5.5` in `scsmath_validation.rs`;
`BUENOS_AIRES_UTC_OFFSET_HOURS = -3.0` in `purebhakti_validation.rs` and,
separately, in `parana_diagnostic.rs`. All three are fixed `f64` values with
no DST logic. They agree with each other and with `rc1_experience.rs`'s
`timezone_offset_hours` match arms for the same locations (Kolkata 5.5,
Buenos Aires -3.0) - no numeric disagreement was found. `parana_diagnostic.rs`
is a single fixed-location, fixed-date diagnostic (Buenos Aires,
2026-08-08/09 hardcoded in `main`); its constant and its hardcoded location
always agree by construction, so it cannot itself surface a
location/offset mismatch, only the general absence of DST handling.

What the harnesses compare and over what range:
`scsmath_validation.rs` compares against 26 fixture rows dated
2026-03-15 to 2027-03-18, all Asia/Kolkata (India does not observe DST;
5.5 is the correct fixed offset for this range and, per public record, has
been since 1947 - that historical claim is **EXTERNAL**, not repository
evidence). `purebhakti_validation.rs` compares fixtures dated 2026 only, all
Buenos Aires; Argentina has not observed DST since 2009 (again
**EXTERNAL**), so -3.0 is correct for this range, but the constant carries
no indication that this was ever otherwise and would silently mis-render a
fixture dated inside one of Argentina's pre-2009 DST intervals if one were
ever added. Determining the true historical offset for any date requires
timezone history not present in this repository; this report does not
import one and states only what the repository does and does not settle.

**5b. Navadvīpa sunrise-tithi discrepancy
(`FEST-001-Janmashtami-Policy-Decision.md § 6`).** That table records, for
the pre-ORDEN-6 engine: "Sep 3: Kṛṣṇa Aṣṭamī; Sep 4: Kṛṣṇa Navamī;
Sep 5: Kṛṣṇa Daśamī", against the SCS Math PDF's "Sep 4, printed as
Kṛṣṇa Aṣṭamī" - a one-day disagreement.

I re-ran this directly against the current (post-ORDEN-6-fix) engine
(`sunrise` + `tithi_at_sunrise`, Nabadwip 23.4071/88.3679, via a throwaway
diagnostic binary, built and executed, not committed):

```text
2026-09-03 Navadvipa: tithi=7  (Saptamī)
2026-09-04 Navadvipa: tithi=8  (Aṣṭamī)
2026-09-05 Navadvipa: tithi=9  (Navamī)
```

**This now matches the SCS Math source exactly** (Aṣṭamī on 4 September).
4 September 2026 is day-of-year 247, inside the 91-279 window ORDEN-6
identified as affected. This documented discrepancy **originated in the
engine** - specifically in the sunrise day-carry defect ORDEN-6 already
fixed - not in civil configuration. It is not a remaining open question:
the fix already resolves it. `FEST-001-Janmashtami-Policy-Decision.md § 6`
itself is now stale on this point; it records pre-fix engine output. No
change was made to that document (single-writer, CLAUDE.md § 9); this is
reported as a finding for the owner to merge.

**5c. Would consolidation change any recorded validation result?**
Measured, starting with SCS-0001 as instructed: its pāraṇa discrepancy
(SCS 5:51-7:59 vs engine 05:45-09:42) is **not** an offset problem - Kolkata
carries no DST and 5.5 is exact for these dates - and it is **not** a
day-shift case: 15 March is outside the 91-279 window and SCS-0001's
observance date was already PASS before ORDEN-6. The difference is
structural: `SCSMath-Difference-Report.md`'s existing "Parāṇa Notes" section
already identifies it - the engine returns the full HBV-EK-005 technical
interval (`max(sunrise, end of Hari-vāsara)` through end of Dvādaśī), while
SCS Math publishes a narrower practical morning window. Re-checked against
the current binary's re-run: this row's classification is unaffected by the
ORDEN-6 fix and would be unaffected by consolidating the three offset
constants, because the offset was never the cause here.

More generally, across the re-run: no observance-date PASS/DIFFERENCE
result changed as a function of the offset constants (Kolkata carries no
DST across the fixture range, so 5.5 is exact regardless of consolidation);
the 12 rows that changed (SCS-0003 etc.) changed because of the ORDEN-6
sunrise fix, already merged, not because of anything this order's
Phase 1 measured. Consolidating the three duplicate constants into one
DST-aware `src/` implementation would not by itself change any currently
recorded Kolkata or Buenos Aires validation result for the 2026-2027
fixture dates, because none of those dates falls inside a real DST interval
for either location under current-era rules. It would remove the latent
risk described in 5a for any future fixture date that does.

## Freeze classification (informational only - no remediation performed)

Per `Final-Certification-Stage.md`, allowed change classes are
ENGINE BUG, DOCUMENTATION, CERTIFICATION, UX without semantic change, and
RELEASE; forbidden: new rules, new APIs, new architecture, new algorithms,
new functionality, scope expansion. The sunset day-carry defect (item 1) is
the same defect class ORDEN-6 already classified and fixed as ENGINE BUG for
sunrise; extending that fix to sunset falls under the same classification
by the same reasoning ORDEN-6 recorded, and by `FEST-001-Janmashtami-
Decision-Spec.md § 4`'s already-normative requirement that interval
arithmetic use absolute instants. The `is_europe_dst`/`timezone_offset_hours`
consolidation (items 2, 3, 5) is likewise a defect-repair candidate, not new
functionality - it replaces a fixed-clock match with an existing pattern
(the file already imports the civil-time infrastructure) rather than adding
a new API surface, mirroring how ORDEN-6 justified its own fix. This is a
recommendation for scope classification only; it is not a decision, and no
code was changed to act on it.

## Summary against the checklist

```text
Correct locality:       Nabadwip coordinates confirmed identical across
                         scsmath-fixtures.csv, rc1_experience.rs cities(),
                         and this report. No disagreement found.
Latitude / longitude:   Confirmed identical (23.4071 / 88.3679) across all
                         26 fixture rows and the shipped city entry.
Time zone:              Asia/Kolkata, no DST (external fact). Constant
                         5.5 is exact for the fixture range.
DST:                    Not applicable to SCS Math (Kolkata). Applicable
                         and defective for Valencia (item 3), latent for
                         the shipped city set otherwise.
Sunrise:                Already audited and fixed under ORDEN-6; the fix
                         resolves the one Navadvipa observance-date
                         discrepancy this order was pointed at (5b).
Ayanamsa:               Not examined in this Phase 1 pass - out of the
                         items ORDEN-3A enumerates; no finding either way.
Calendar configuration: Pāraṇa-window definitional difference (structural,
                         not civil-config) confirmed for SCS-0001 and, by
                         the existing Difference-Report pattern, the other
                         10 comparable rows.
```

```text
ENGINE BUG (this pass, newly found, unfixed): 1
  - sunset() day-carry defect duplicated in parana_diagnostic.rs and
    rc1_experience.rs, latent for the three shipped cities, live for
    west-longitude configurations.

ENGINE BUG (already fixed, re-confirmed by independent re-run): 1
  - sunrise() day-carry defect (ORDEN-6), ⁠resolves the Navadvipa
    Aṣṭamī/Navamī discrepancy in FEST-001-Janmashtami-Policy-Decision.md § 6.

CIVIL-CONFIGURATION DEFECT (not an ENGINE BUG in the astronomy/calendar
sense, but silent and civil-time-adjacent per CLAUDE.md § 5.3):
  - is_europe_dst whole-date granularity vs the 01:00 UTC transition
    (item 3), reachable for Valencia.
  - _ => 0.0 fallthrough in timezone_offset_hours, unreachable today,
    dormant if the city list is ever extended (item 2).

NOT AN ENGINE BUG:
  - SCS-0001 and the ten comparable Parāṇa-window differences: structural/
    editorial (technical HBV-EK-005 interval vs a practical published
    window), unaffected by any civil-configuration fix (item 5c).
  - SCS-0012 (Vyañjulī Mahādvādaśī derivation not implemented in the
    validator), SCS-0018, SCS-0026 (Aruṇodaya Viddhā case): calendar/rule
    questions, not civil configuration, unresolved by this pass.
```

## Recommendation (not a disposition)

The Release Gate clause "SCS Math follow-up configuration audit is resolved
or documented as non-critical" offers a non-critical exit. I am not
recording one. Item 1 (sunset day-carry) is a real, unfixed engine defect
sitting outside `src/`, feeding a user-facing field, currently latent only
because of which three coordinates are shipped today - not because it does
not exist. Item 3 (DST transition granularity) is real, reachable today for
Valencia, and quantified. Whether either is "critical" for the v1.0 release
gate is the owner's call, not mine; this report supplies the measurement.

## Files examined (not modified)

```text
00_Project/Final-Certification-Stage.md
CERTIFICATION.md
PROJECT_STATUS.md
04_Tests/SCSMath/scsmath-fixtures.csv
04_Tests/SCSMath/scsmath-validation.csv
04_Tests/SCSMath/SCSMath-Validation.md
04_Tests/SCSMath/SCSMath-Difference-Report.md
04_Tests/Astronomy/Sunrise-Day-Shift-Fix.md
04_Tests/PureBhakti/purebhakti-fixtures.csv
05_Documentation/Festivals/Rule-Analysis/FEST-001-Janmashtami-Decision-Spec.md
05_Documentation/Festivals/Rule-Analysis/FEST-001-Janmashtami-Policy-Decision.md
03_Source/rust/crates/pancanga-engine/src/calendar/sunrise.rs
03_Source/rust/crates/pancanga-engine/examples/parana_diagnostic.rs
03_Source/rust/crates/pancanga-engine/examples/rc1_experience.rs
03_Source/rust/crates/pancanga-engine/examples/scsmath_validation.rs
03_Source/rust/crates/pancanga-engine/examples/purebhakti_validation.rs
03_Source/rust/crates/pancanga-engine/examples/internal_stress_test.rs
03_Source/rust/crates/pancanga-engine/examples/sunrise_validation.rs
```

## Commands run

```text
cargo run -p pancanga-engine --example scsmath_validation -- \
  --input 04_Tests/SCSMath/scsmath-fixtures.csv --output <scratch path>
```

plus one throwaway diagnostic binary (`_orden3a_scratch_probe.rs`, calling
only `pancanga_engine::calendar::{sunrise, tithi_at_sunrise}` with no logic
of its own) built, run, and deleted before this report was written -
`git status --short` is clean, confirmed after deletion.

No golden test was run, modified, or needed changing in this pass.
`cargo fmt` / `cargo clippy` / the canonical pipeline were not re-run since
no source file changed; `git diff --check` is not applicable (no diff).

## STOP

Phase 1 complete. Phase 2 (remediation) requires separate explicit owner
authorization per the order and is not started. No code was modified;
`git status --short` at the end of this session shows only this new report
file.
