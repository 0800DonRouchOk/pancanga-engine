# Pancanga Engine Certification

Status: Release Candidate / External Certification In Progress

## Summary

Pancanga Engine has completed local implementation and validation for the
astronomical, calendar, and Vaiṣṇava layers needed for Ekādaśī calculation.

The remaining v1.0 work is external certification against additional Vaiṣṇava
calendar fixtures. SCS Math is now included as a real oracle with analyzed
differences; GCal remains pending a real local source.

Current release state:

```text
Knowledge Base:
v1.0 FROZEN

Open doctrinal issues:
0

Confirmed ENGINE BUG:
0

Status:
READY FOR FINAL CERTIFICATION
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
remains open, but Campaign 46F.1 classified every observed difference and found
no confirmed engine bug.

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
PENDING
```

Objective:

```text
Demonstrate that the same Pancanga Engine version produces identical output
for the same 73,414-day certification range across repeated executions.
```

Expected method:

```text
Run the complete 1900-01-01 -> 2100-12-31 internal certification range
10 consecutive times.

Compute a stable output hash for each pass.

PASS only if all 10 hashes are identical.
```

This certification is not allowed to redefine any engine behavior. It may only
confirm reproducibility.

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
11 / 26 PASS

Calendar differences:
15

Parāṇa:
11 configuration differences among comparable rows

ENGINE BUG:
0 confirmed
```

Certification decision:

```text
OPEN
```

SCS Math is a reproducible external oracle, but certification is not closed.
The observed differences are analyzed in Campaign 46F.1. They currently point
to calendar/configuration differences and one Vyañjulī tradition-difference
case, with no confirmed engine bug. ISSUE-VAI-001 was later resolved from the
HBV/DDT source chain, so Vyañjulī is no longer an open doctrinal blocker.

Evidence:

```text
02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf
04_Tests/SCSMath/SCSMath-Fixtures.md
04_Tests/SCSMath/SCSMath-Validation.md
04_Tests/SCSMath/SCSMath-Difference-Report.md
04_Tests/SCSMath/scsmath-fixtures.csv
04_Tests/SCSMath/scsmath-validation.csv
```

## GCal

Status:

```text
EXTERNAL_ORACLE_PENDING
```

The importer is ready, but a real local GCal source has not yet been added.
No synthetic fixtures are used.

Evidence:

```text
04_Tests/Vaishnava/GCal/GCal-Fixtures.md
04_Tests/Vaishnava/GCal/gcal-fixtures.csv
```

## Release Gate

Pancanga Engine v1.0 remains blocked until:

```text
GCal fixtures are populated from a real source.
SCS Math follow-up configuration audit is resolved or documented as
non-critical.
GCal certification completes with no critical ENGINE BUG.
Full external validation is updated.
```

No engine behavior may be changed to match an external oracle until the
difference is classified.
