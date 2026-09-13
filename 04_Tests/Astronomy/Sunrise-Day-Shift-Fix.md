# Sunrise Day-Shift Fix

Order: ORDEN-6 (day-shift defect in `sunrise()`)

Status: PHASE 2 IMPLEMENTED, PHASE 3 MEASURED, PHASE 4 AUTHORIZED AND
APPLIED — four golden expectations corrected under owner authorization

## Defect

Established by `Sunrise-Ephemeris-Validation.md § Finding 1`:
`calendar::sunrise` normalised the almanac universal time into `[0, 24)` and
attached it to the requested civil date. Wherever the local sunrise falls on
the previous UTC date (east longitudes, roughly λ > 15° × LMT sunrise hour —
at Nabadwip every day from ≈ 7 April to ≈ 3 October), the returned instant
was the sunrise of the **following** local day.

## Day definition (owner decision, Phase 1)

```text
Local mean time.
The local day at (date, location) starts at 0h UT − longitude / 15h.
No API change.
```

Rationale recorded: the two candidate definitions (local-mean-time midnight
vs civil-timezone midnight) select the same sunrise in 4,771 of 4,771 sampled
rows, so the choice is definitional rather than observational; local mean
time keeps the calendar engine free of civil configuration, consistent with
the frozen boundary of `Calendar-Pipeline.md`; the civil option would require
importing the `rc1_experience.rs` DST logic into `src/`, which is itself
defective (ORDEN-1 § 9).

Reported contradiction, not resolved: `Calendar-Pipeline.md § Entrada Mínima`
and `FEST-001-Janmashtami-Decision-Spec.md § 3 Required Inputs` both list a
timezone as an engine input. This decision contradicts both. No document was
changed; the owner holds the contradiction.

## Change (Phase 2)

File: `03_Source/rust/crates/pancanga-engine/src/calendar/sunrise.rs` only.

- The almanac local mean time is normalised into `[0, 24)` — that is the
  local-mean-time-of-day on the requested date, i.e. the day definition
  itself.
- It is converted to UT by subtracting `longitude / 15` **without**
  normalisation; the result may be negative or exceed 24 h.
- `julian_date_at_universal_hours` now builds the instant as
  `JD(date at 0h UT) + universal_time / 24` instead of packing the hour into a
  `CivilTime` on `date`. A negative offset falls into the previous UTC date
  and an offset past 24 h into the next, so the carry is never lost. The
  normalisation step that caused the defect no longer exists in the UT path.
- Doc comment updated to state the local-mean-time day boundary.
- The solar position algorithm (mean anomaly, true longitude, right
  ascension, declination, hour angle, zenith 90.833°) is untouched. The
  year-independence and the ±115 s magnitude reported in
  `Sunrise-Ephemeris-Validation.md § Finding 2` remain and are out of scope.

## Phase 3 — impact, measured

### 1. Sunrise instants that change (ORDEN-5 CSV, 5,020 rows, rerun)

- **459 rows change, all Nabadwip, every one by exactly −86,400.0 s** (one
  day earlier). No other site changes; the reference instants are identical
  in all rows; the in-day error distribution is identical row for row
  (shipped-city max |error| stays +115.0 s).
- `day_shift ≠ 0` rows after the fix: **0** (was 459).
- The affected Nabadwip rows are the ORDEN-5 day-shift set (day-of-year
  97–276 in the sample; the full-range scan below widens this to 91–279).

### 2. Tithi-at-sunrise labels that change

- ORDEN-5 sample: **453** rows (all Nabadwip). After the fix the engine label
  disagrees with the reference label in exactly one row, the pre-existing
  polar case `SUNRISE-04082` (66.5N, 1912-04-10, error −261 s).
- Full-range scan (every civil day 1900-01-01 → 2100-12-31 at Nabadwip,
  73,414 days, through the SCS Math harness with the old and the new
  `sunrise()`): the tithi-at-sunrise label changes on **36,853 days
  (50.2 %)**.
- Control sites, every day 2020–2030: Valencia **0** label changes, Buenos
  Aires **0**; Sydney (151.2°E) 3,892 of 4,018 days.

### 3. Ekādaśī observance-date impact

Method: the SCS Math harness (`examples/scsmath_validation.rs`) evaluates a
civil day exactly as the RC1 pipeline does (`sunrise → tithi at sunrise and at
aruṇodaya → HBV-EK-001/002/003 → disposition`) and reports whether the day is
an observance day (`current_valid_ekadasi` / `previous_viddha_displacement`).
It was fed a synthetic fixture file with every civil day and run with the
binary built before and after the fix. The before-binary reproduces the
recorded `scsmath-validation.csv` in 26 of 26 rows (path, observance date,
pāraṇa window, status), so the baseline is faithful.

Nabadwip, 1900–2100 (73,414 days):

| Metric | Before | After |
| --- | ---: | ---: |
| Observance days | 4,790 | 4,794 |
| Observance days removed by the fix | 2,466 | |
| Observance days added by the fix | | 2,470 |
| Removed dates whose next day is an added date | 2,439 of 2,466 | |
| Years affected | **201 of 201** (10–13 dates per year) | |
| Day-of-year range of removed dates | 91 – 279 (≈ 1 April – 6 October) | |

In other words: for every year of the certified range, roughly half of the
Ekādaśī observance dates at Nabadwip (those between April and early October)
were one day **early**; after the fix they move to the following day. The
27 / 31 unpaired removals/additions are fortnights whose displacement pattern
(viddhā / non-viddhā, or a two-sunrise pair) changes shape rather than
shifting as a unit. Full lists: scratch files `nabadwip_removed.txt` /
`nabadwip_added.txt` (not committed; reproducible with the method above).

Sample year 2026 (Nabadwip): removed 04-12, 04-26, 05-12, 05-26, 06-10,
06-24, 07-10, 07-24, 08-08, 08-22, 09-06, 09-21, 10-05; added 04-13, 04-27,
05-13, 05-27, 06-11, 06-25, 07-11, 07-25, 08-09, 08-23, 09-07, 09-22, 10-06.

Other locations:

- Valencia and Buenos Aires, every day 2020–2030: **0 observance dates
  change** (262 and 258 observance days, identical before/after).
- Sydney (151.2°E), every day 2020–2030: 265 of 267 observance dates change
  (all months — at that longitude the UT sunrise is on the previous UTC date
  year-round).
- General rule: any location whose local sunrise precedes 0h UT is affected;
  west of about 75°E no shipped or sampled location is.

Real SCS Math fixtures (26 rows, 2026-03-15 → 2027-03-18), same harness,
outputs written to the scratchpad — the recorded
`04_Tests/SCSMath/scsmath-validation.csv` was **not** overwritten:

- Observance status PASS: **11 → 23 of 26**.
- Rows that change: SCS-0003, 0004, 0005, 0006, 0007, 0008, 0009, 0010, 0011,
  0013, 0014, 0015 — each from `no_matching_observance_path` (tithi at
  sunrise 12 or 13) to `current_valid_ekadasi` (tithi 11), observance status
  DIFFERENCE → PASS. Their recorded classification `CALENDAR DIFFERENCE` /
  `DOC-001` is contradicted by measurement; reclassification is an owner /
  46F.2 decision and no recorded file was edited.
- Rows still DIFFERENCE: SCS-0012 (Vyañjulī Mahādvādaśī — the harness does not
  derive Mahādvādaśī facts), SCS-0018 (2026-11-20, tithi 10 at sunrise) and
  SCS-0026 (2027-03-18). These are not day-shift cases.
- Pāraṇa window status PASS: 0 → 0. The pāraṇa discrepancies are a separate
  question (46F.2).

### 4. Tests that fail after the fix

Canonical `cargo test --all-targets --all-features`: **288 passed, 3 failed**;
doc-tests 33 passed; fmt and clippy clean.

| Test | Current expectation | Value with the fix | Ephemeris support |
| --- | --- | --- | --- |
| `calendar::sunrise::tests::calculates_mid_southern_latitude_summer_sunrise` (Sydney −33.8688 / 151.2093, 2026-12-21) | JD `2_461_396.278_178_977` = 2026-12-21 18:40:34 UT = **22 Dec** 05:40 local | JD `2_461_395.278_178_977` = 2026-12-20 18:40:34 UT = **21 Dec** 05:40 local (exactly −1 day) | Same mechanism as the 459 Nabadwip rows measured against the −50′ reference; a Sydney row is not in the ORDEN-5 sample (UNVERIFIED against swetest for this specific date; the mechanism is verified) |
| `calendar::tithi::tests::finds_transition_after_sydney_summer_sunrise` (interval 2026-12-21 → 22) | transition JD `2_461_396.871_763_413`, 12 → 13 | transition JD `2_461_396.005_551_903`, **11 → 12** | Follows from the corrected interval |
| `calendar::tithi::tests::describes_presence_for_representative_locations`, Sydney entry | tithi at sunrise 12, transition 12 → 13 | tithi at sunrise **11**, transition **11 → 12** | Follows from the corrected interval |

Affected in meaning but not failing:
`calendar::tithi::tests::calculates_tithi_at_sydney_summer_sunrise` hardcodes
JD `2_461_396.278_178_977` (the 22 December sunrise) and asserts index 12; it
does not call `sunrise()`, so it still passes while its name no longer
describes its input.

Unchanged goldens (verified to 9 decimals with the fix): equator 2026-03-20
`2461119.753038483`, New York 2026-06-21 `2461212.892268492`, Buenos Aires
2026-06-21 `2461212.958473451`; polar 80N 2026-12-21 → `None`.

## Phase 4 — golden correction (owner-authorized, by name)

Authorization basis, recorded beside each corrected value in the source:
the Sydney 2026-12-21 measurement below and
`Sunrise-Ephemeris-Validation.md § Finding 1`.

Sydney (−33.8688 / 151.2093), civil date 2026-12-21, Option A reference
(Sun-centre true altitude −50′, local-mean-time day window starting
2026-12-20 13:55:09 UT; the `sunrise_validation.rs` procedure applied directly
with swetest 2.10.03 because Sydney is not among the binary's fixed sites):

```text
engine before fix (previous golden)  2026-12-21 18:40:34.6 UT  JD 2461396.278178977  = 22 Dec local
engine after fix                     2026-12-20 18:40:34.6 UT  JD 2461395.278178977  = 21 Dec local
Option A reference                   2026-12-20 18:40:38.9 UT  JD 2461395.278228180
after  − reference                   −4.3 s
before − reference                   +86,395.7 s
(before matches the 22 December reference, 2026-12-21 18:41:07.6 UT, to −33.0 s:
a correct-magnitude sunrise for the wrong local day)
swetest -rise default (data only)    2026-12-20 18:40:39.7 UT
swetest -rise -hindu (data only)     2026-12-20 18:45:13.4 UT
```

Corrections applied:

| Test | Previous expectation | Corrected expectation |
| --- | --- | --- |
| `calendar::sunrise::tests::calculates_mid_southern_latitude_summer_sunrise` | JD `2_461_396.278_178_977` | JD `2_461_395.278_178_977` |
| `calendar::tithi::tests::finds_transition_after_sydney_summer_sunrise` | JD `2_461_396.871_763_413`, 12 → 13 | JD `2_461_396.005_551_903`, 11 → 12 |
| `calendar::tithi::tests::describes_presence_for_representative_locations`, Sydney entry | 12, 12 → 13 | 11, 11 → 12 |
| `calendar::tithi::tests::calculates_tithi_at_sydney_summer_sunrise` | JD `2_461_396.278_178_977`, index 12 | JD `2_461_395.278_178_977`, index 11 |

The previous expectations encoded the day-shift defect as specification:
each was the sunrise of 22 December 2026 at Sydney, or a quantity derived
from it, recorded under the label of 21 December. The fourth test hardcoded
an instant the engine no longer produces, so it was passing without verifying
anything; it is re-anchored to the 21 December instant (owner rationale).

No other expected value was changed.

## Separate defect surfaced (not fixed, not in scope)

### Pāraṇa windows whose end lies on the following day

Among the Nabadwip days that are observance days both before and after the
fix, 18 show a changed pāraṇa window. Resolved to absolute instants (example
1903-08-03 → 04, IST): sunrise of the pāraṇa day 05:07 still in Ekādaśī;
Dvādaśī 1903-08-04 05:08 → 1903-08-05 07:18 (26.2 h); Hari-vāsara end 11:41;
window `1903-08-04 11:41 → 1903-08-05 07:18`, mode Standard. The window is
valid in absolute time; the harness and `rc1_experience.rs`
`parana_presentation` render it as `HH:MM – HH:MM` without the end date, so
it reads as "11:41 – 07:18", start after end.

Characterisation:

1. **Rendering.** Any pāraṇa window whose end (Dvādaśī end) falls after local
   midnight of the pāraṇa date is printed without its date. In the Nabadwip
   1900–2100 scan **1,137 of 4,609** `current_valid_ekadasi` days (24.7 %)
   have such a window after the fix (1,143 before). The pattern already
   exists in the recorded files: `scsmath-validation.csv` SCS-0019
   (`06:04–00:54`), SCS-0022 (`06:20–04:43`); `purebhakti-validation.csv`
   PB-0001 (`07:39–02:52`), PB-0006, PB-0009, PB-0013. The 18 rows are simply
   the subset whose two-sunrise pair moved by a day. Pre-existing; not
   introduced by this fix; user-facing in RC1 (UNVERIFIED how the HTML
   presents it).
2. **Two consecutive observance days (Ekādaśī vṛddhi).** The 18 rows are all
   first days of a pair of consecutive days with Ekādaśī present at both
   sunrises (Dvādaśī begins within ~1 minute after the second sunrise in the
   example). HBV-EK-001 marks both days `ObserveOnEkadasi`; no KB rule in
   `HBV-EK-001…005` or `KB-VAI-002` addresses which day is observed when
   Ekādaśī spans two sunrises (grep for vṛddhi / two sunrises: no hits).
   Count at Nabadwip 1900–2100: **29 pairs after the fix** (39 before; the
   set changes because the pairs relocate). Because Dvādaśī starts within
   minutes of the second sunrise in these cases, the pair membership is also
   sensitive to the ±115 s sunrise error of Finding 2. Normative status:
   RESEARCH_REQUIRED — owner decision; nothing changed.

## Methodological Rule

```text
The fix removes the normalisation step that dropped the day carry.
It does not change the solar position algorithm.
No golden expectation changes until the owner authorises it by name.
```
