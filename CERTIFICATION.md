# Pancanga Engine Certification

Status: Release Candidate / External Certification Closed for v1.0

## Summary

Pancanga Engine has completed local implementation and validation for the
astronomical, calendar, and Vaiṣṇava layers needed for Ekādaśī calculation.

External certification against Vaiṣṇava calendar fixtures closed for v1.0 on
2026-09-15. SCS Math is now a closed PASS oracle with analyzed differences;
GCal was declared deliberately unfilled for v1.0 (see Release Gate).
Deterministic Certification closed PASS the same day, at aggregate scope
(see below). Remaining pre-release work is Campaign 46BETA (RC1 Experience)
and Campaign 46R (Release Audit).

Current release state:

```text
Knowledge Base:
v1.0 FROZEN

Open doctrinal issues:
0

Confirmed ENGINE BUG:
0

External Certification:
CLOSED FOR v1.0 (2026-09-15)

Deterministic Certification:
PASS (2026-09-15, aggregate scope)

Status:
AWAITING RC1 EXPERIENCE, RELEASE AUDIT
```

## Cross-Oracle Pattern

Three independent oracles have been analyzed:

```text
Swiss Ephemeris
PureBhakti
Sri Chaitanya Saraswat Math
```

Across those oracles, the current certification evidence records:

```text
Confirmed ENGINE BUG:
0
```

Swiss Ephemeris certifies the astronomical layer for the precision required by
Pancanga Engine. PureBhakti certifies the imported observance decisions with
Parāṇa differences documented as editorial/rounding differences. SCS Math
closed PASS after Campaign 46C Final's rerun (2026-09-14): observance PASS
moved from 11/26 to 23/26, with the 3 remaining differences classified and no
confirmed engine bug.

## Internal Consistency Certification

Campaign:

```text
46STRESS
```

Objective:

```text
Validate the internal logical consistency of Pancanga Engine
across the complete supported historical range.
```

Configuration:

```text
Location:
Valencia, España

Range:
1900-01-01 -> 2100-12-31

Cases executed:
73,414 consecutive civil days
```

Result:

```text
Crash.........................0
Panic.........................0
NaN / Infinite values.........0
Invalid dates.................0
Negative intervals............0
Logical contradictions........0

Confirmed ENGINE BUG..........0
```

Certification status:

```text
PASS
```

Diagnostic note:

```text
The first stress pass exposed 107 apparent contradictions.
Those were traced to a sunrise-only assumption in the validation tool:
ObserveOnDvadasi was incorrectly treated as requiring Dvādaśī at the
following sunrise.

The first flagged case showed that Dvādaśī began after that sunrise and
continued into the next civil day. The validation tool was corrected to inspect
the actual Dvādaśī interval used for Parāṇa. The motor was not modified.
```

Evidence:

```text
04_Tests/Stress/Internal-Stress-Test.md
04_Tests/Stress/internal-stress-issues.csv
04_Tests/Stress/internal-stress-extremes.csv
```

Release gate:

```text
cargo run --release -p pancanga-engine --example internal_stress_test
```

Required release result:

```text
Campaign 46STRESS
PASS
Engine bugs: 0
```

## Deterministic Certification

Status:

```text
PASS (2026-09-15) — aggregate scope, see limitation below
```

Objective:

```text
Demonstrate that the same Pancanga Engine version produces identical output
for the same 73,414-day certification range across repeated executions.
```

Method used:

```text
Built cargo run --release -p pancanga-engine --example internal_stress_test
once; ran it 10 consecutive times. Hashed (SHA-256) the concatenation of
stdout, internal-stress-issues.csv, internal-stress-extremes.csv, and
Internal-Stress-Test.md with its live "Elapsed seconds" line stripped.

Result: all 10 hashes identical —
d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
```

Scope limitation:

```text
This certifies reproducibility of the 14 aggregate stats fields, the 6
extreme-value rows, and the issues list across 10 runs. It does NOT prove
bit-for-bit identity of every individual day's computed result — the
binary emits no per-day trace, only aggregates. A full per-day hash is
deferred to v1.1 as a certification-instrumentation enhancement.
```

This certification is not allowed to redefine any engine behavior. It may only
confirm reproducibility.

Evidence:

```text
04_Tests/Stress/Deterministic-Certification.md
04_Tests/Stress/Campaign-Report-2026-09-15-Determinism.md
```

## Astronomy

External oracle:

```text
Swiss Ephemeris
```

Result:

```text
Cases:
1000 / 1000 PASS

Tithi mismatches:
0

Maximum lunar longitude error:
0.000469445269°

Maximum elongation error:
0.000330443528°

ENGINE BUG:
0
```

Evidence:

```text
04_Tests/Astronomy/SwissEphemeris/Swiss-Validation.md
04_Tests/Astronomy/SwissEphemeris/swiss-validation.csv
```

## Normative Knowledge Base

Primary authority:

```text
Hari-bhakti-vilāsa
```

Status:

```text
Knowledge Base v1.0
FROZEN
```

Implemented rules:

```text
HBV-EK-001 Candidate Ekādaśī
HBV-EK-002 Viddhā
HBV-EK-003 Observance displacement
HBV-EK-004 Mahādvādaśī
HBV-EK-005 Parāṇa / Hari-vāsara
```

Evidence:

```text
05_Documentation/Knowledge-Base/Knowledge-Base-v1.0-Release.md
05_Documentation/Knowledge-Base/KB-VAI-002-Hari-bhakti-vilasa-Ekadasi-Rules.md
```

## Doctrinal Certification

Normative chain:

```text
Hari-bhakti-vilāsa:
✓

Digdarśinī-ṭīkā:
✓

Navadvīpa Pañjikā:
✓
```

Outstanding doctrinal issues:

```text
0
```

Final doctrinal issue closed:

```text
ISSUE-VAI-001
Vyañjulī
RESOLVED
```

Resolution:

```text
Vyañjulī Mahādvādaśī =
Ekādaśī pure
+
Dvādaśī present at sunrise
+
Dvādaśī continuing toward Trayodaśī
```

The two-muhūrta / 96-minute threshold belongs to Ekādaśī purity under
HBV-EK-002. It is not transferred to Dvādaśī for Vyañjulī.

## PureBhakti

External oracle:

```text
PureBhakti 2026 calendar
Buenos Aires, Argentina
```

Result:

```text
Observance dates:
16 / 16 PASS

ENGINE BUG:
0

Parāṇa:
13 editorial policy differences documented
2 rounding differences documented
```

Certification decision:

```text
PASS
```

PureBhakti certifies the observance decisions for the imported fixture. Parāṇa
differences are documented as external editorial/rounding differences and do
not indicate an engine bug.

Evidence:

```text
04_Tests/PureBhakti/PureBhakti-Validation.md
04_Tests/PureBhakti/PureBhakti-Parana-Difference-Analysis.md
04_Tests/PureBhakti/purebhakti-validation.csv
04_Tests/PureBhakti/purebhakti-parana-difference-analysis.csv
```

## SCS Math

External oracle:

```text
Sri Chaitanya Saraswat Math International
Sri Gaudiya Calendar 2026-2027
Nabadwip, India
```

Result:

```text
Fixtures:
26 real rows

Observance dates:
23 / 26 PASS

Calendar differences:
3

Parāṇa:
23 configuration differences among comparable rows

ENGINE BUG:
0 confirmed
```

Certification decision:

```text
PASS
```

SCS Math is a reproducible external oracle. Campaign 46C Final (2026-09-14)
reran the validator against the ORDEN-6 engine state (commit 2a6d5fd),
moving observance PASS from 11/26 to 23/26. The 12 rows that changed had a
prior DOC-001 classification ("configuration/tradition difference, not
confirmed engine bug") superseded by this measurement; that reconciliation
is recorded in normative-observance-certification.csv and
04_Tests/SCSMath/Campaign-Report-2026-09-15-Evidence-Reconciliation.md /
-Mirror-Sync.md. The 3 remaining differences (SCS-0012 Vyañjulī
Mahādvādaśī, SCS-0018, SCS-0026) are analyzed in Campaign 46F.1 /
SCSMath-Difference-Report.md, with no confirmed engine bug. ISSUE-VAI-001
was resolved from the HBV/DDT source chain, so Vyañjulī is not an open
doctrinal blocker.

Evidence:

```text
02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf
04_Tests/SCSMath/SCSMath-Fixtures.md
04_Tests/SCSMath/SCSMath-Validation.md
04_Tests/SCSMath/SCSMath-Difference-Report.md
04_Tests/SCSMath/scsmath-fixtures.csv
04_Tests/SCSMath/scsmath-validation.csv
04_Tests/SCSMath/Campaign-Report-2026-09-15-Evidence-Reconciliation.md
04_Tests/SCSMath/Campaign-Report-2026-09-15-Mirror-Sync.md
04_Tests/Vaishnava/normative-observance-certification.csv
```

## GCal

Status:

```text
EXTERNAL_ORACLE_PENDING (deliberately unfilled for v1.0)
```

The importer is ready, but a real local GCal source has not yet been added.
No synthetic fixtures are used. Per the Release Gate disposition below, this
is not a v1.0 release blocker; it remains open as a v1.1 acquisition item.

Evidence:

```text
04_Tests/Vaishnava/GCal/GCal-Fixtures.md
04_Tests/Vaishnava/GCal/gcal-fixtures.csv
```

## Release Gate

Owner disposition, 2026-09-15, per `04_Tests/Validation/Oracle-Set-Rationale.md`
§ 5 and `04_Tests/SCSMath/SCSMath-Civil-Configuration-Audit.md`:

```text
1. GCal fixtures populated from a real source.
   CLOSED for v1.0 — deliberately unfilled. Validation set is PureBhakti
   + SCS Math: both bibliographically filed under KB-REF-001, and shown
   to diverge on 6 of 10 overlapping observance dates, so the set is not
   redundant. GCal stays open as a v1.1 acquisition item.

2. SCS Math follow-up configuration audit resolved or documented as
   non-critical.
   CLOSED — non-critical for v1.0. Campaign 46F.2 found two civil-
   configuration defects (duplicated sunset() day-carry bug; incomplete
   UTC-offset/DST handling in is_europe_dst), both living in examples/,
   outside src/. Neither reaches an Ekādaśī date for the three shipped
   locations (Valencia, Buenos Aires, Nabadwip). Remediation deferred to
   v1.0.1 / v1.1 under the already-drafted Phase 2 of ORDEN 3A.

3. GCal certification completes with no critical ENGINE BUG.
   NOT APPLICABLE for v1.0, under disposition 1 above.

4. Full external validation is updated.
   CLOSED — Campaign 46C Final reran both validation harnesses against
   the ORDEN-6 engine state (commit 2a6d5fd) on 2026-09-14. SCS Math
   observance PASS moved 11/26 -> 23/26; PureBhakti confirmed unchanged
   at 16/16 (0 rows changed). See SCS Math and PureBhakti sections above.
```

No engine behavior may be changed to match an external oracle until the
difference is classified.

Release Gate status: ALL 4 ITEMS CLOSED FOR v1.0.
