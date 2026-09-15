# Pancanga Engine v1.0 — Release Notes

Status at the time of writing: Release Candidate 1, external and
deterministic certification closed 2026-09-15. Campaign 46R (Release
Audit) and Campaign 47 (publication) are separate, later steps — this
document describes what the certified engine does and does not do, not
that it has been tagged and published yet. See `CHANGELOG.md` for the
itemized history and `CERTIFICATION.md` for full certification evidence.

This is a release note, not marketing copy: known boundaries are stated
plainly, with the same evidence standard the rest of the repository uses
(`CLAUDE.md`) — no claim here that isn't backed by a cited document.

## What Pancanga Engine v1.0 does

Pancanga Engine is an offline, auditable calendar engine for Gauḍīya
Vaiṣṇava Ekādaśī observance. Given a date and a location, it determines:

- whether Ekādaśī is observed that day, or displaced to the following
  Dvādaśī (Hari-bhakti-vilāsa rules HBV-EK-001 through HBV-EK-003);
- whether the day is one of the eight recognized Mahādvādaśī (HBV-EK-004);
- the Parāṇa (fast-breaking) window, including the short-Dvādaśī
  contingency (HBV-EK-005);
- the astronomical facts behind the decision (Sun and Moon longitude,
  elongation, tithi, nakṣatra, sunrise, Aruṇodaya), so the result can be
  checked, not just trusted.

Every rule traces to a specific Hari-bhakti-vilāsa reference; the "Why?"
view in the RC1 Experience shows that trace for any given day.

## Shipped locations

v1.0 ships pre-configured coordinates for three locations. These are the
only locations exercised by certification evidence — using the engine
with other coordinates is possible but uncertified:

| Location | Coordinates | Role |
|---|---|---|
| Valencia, España | 39.4699° N, 0.3763° W | Internal consistency and determinism certification range (1900–2100) |
| Buenos Aires, Argentina | 34.6167° S, 58.3833° W | PureBhakti external certification |
| Navadvīpa, India | 23.4071° N, 88.3679° E | SCS Math external certification |

## Certification status, per oracle

| Oracle | Result | Notes |
|---|---|---|
| Swiss Ephemeris | **1000 / 1000 PASS** | Astronomical layer only (solar/lunar longitude, elongation, tithi index); 0 tithi mismatches, 0 confirmed engine bugs. |
| PureBhakti (Buenos Aires) | **16 / 16 observance dates PASS** | 0 confirmed engine bugs. Parāṇa window differs from the published fixture in 15 of 16 rows — documented as 13 editorial-policy and 2 rounding differences, not engine bugs. Pancanga Engine reports the full HBV-EK-005 technical window (end of Hari-vāsara to end of Dvādaśī); PureBhakti publishes a shorter practical morning window. |
| Sri Chaitanya Saraswat Math (Navadvīpa) | **23 / 26 observance dates PASS** | 3 remaining differences (Vyañjulī Mahādvādaśī fact derivation not automated; two other dates under analysis), 0 confirmed engine bugs. Parāṇa windows differ from the published fixture in all 23 comparable rows, for the same reason as PureBhakti above. |
| GCal | **Not certified** | Deliberately left unfilled for v1.0 — see Known Boundaries below. |
| Internal Consistency (46STRESS) | **PASS** | 73,414 consecutive days, 1900-01-01 to 2100-12-31, Valencia: 0 crashes, panics, invalid dates, or logical contradictions. |
| Deterministic Certification (46DETERMINISM) | **PASS, aggregate scope** | 10 consecutive runs of the same release build produced an identical result hash every time. See Known Boundaries — this does not certify every individual day bit-for-bit. |

Confirmed ENGINE BUG across all certification work to date: **0**.

## Known boundaries — deferred to v1.0.1 / v1.1

Stated plainly, not as a footnote:

1. **GCal is not a certified oracle for v1.0.** No real GCal fixture
   source has been acquired; the local importer exists and is ready, but
   has nothing to import. The v1.0 validation set is PureBhakti + SCS
   Math instead — two oracles shown to diverge on 6 of 10 overlapping
   published dates in 2026, so they are not a redundant pair. GCal
   acquisition is a v1.1 item.

2. **Two civil-configuration defects exist outside the certified engine
   core** (`src/`), living instead in example/tooling code
   (`examples/`): a duplicated day-carry bug in a `sunset()` helper (the
   same class of defect fixed in the certified `sunrise()` under
   ORDEN-6, but not yet applied to this separate helper), and incomplete
   UTC-offset/DST handling in an `is_europe_dst` helper. Neither has been
   shown to change an Ekādaśī date at any of the three shipped locations.
   Remediation is planned for v1.0.1 / v1.1.

3. **Deterministic Certification proves reproducibility of aggregate
   statistics, not of every individual day.** The certified test binary
   records 14 summary counters, 6 extreme-value rows, and an issues list
   — all confirmed bit-for-bit identical across 10 runs — but it does not
   emit, and therefore cannot hash, a per-day trace of all 73,414
   individual computed results. It is possible in principle for two
   days' results to change in a way that leaves every aggregate
   unchanged; this method would not catch that. A full per-day hash is
   planned as a v1.1 certification-instrumentation enhancement.

4. **Parāṇa window differences against both external oracles are
   editorial, not doctrinal or computational.** Pancanga Engine always
   reports the full technical HBV-EK-005 window; published calendars
   commonly print a shorter practical morning window. This is documented
   per-row in the certification evidence, not silently accepted.

## What v1.0 does not do

- No festivals beyond Ekādaśī and Mahādvādaśī are calculated. A separate,
  much larger Festival Engine research track (Janmāṣṭamī and 96 other
  candidate observances) is in progress but explicitly out of v1.0 scope
  — zero festivals from that track are implemented or authorized.
- No location beyond the three listed above is certified.
- The Knowledge Base, Astronomy Engine, Calendar Engine, and public API
  are frozen for v1.0; no new rules, engines, or scope are added during
  the release-candidate freeze.
