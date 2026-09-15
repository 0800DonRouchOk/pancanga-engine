# SCS Math Validation

Campaign: 46F

Status: 🟡 OPEN / DIFFERENCES ANALYZED

## Objective

Validate Pancanga Engine against Sri Chaitanya Saraswat Math as an independent
Vaiṣṇava calendar oracle.

This validation does not modify:

```text
Astronomy Engine
Calendar Engine
Vaiṣṇava Engine
Knowledge Base
```

## Inputs

Fixture CSV:

```text
04_Tests/SCSMath/scsmath-fixtures.csv
```

Validator:

```text
03_Source/rust/crates/pancanga-engine/examples/scsmath_validation.rs
```

Validation CSV:

```text
04_Tests/SCSMath/scsmath-validation.csv
```

## Result

```text
Fixtures:
26

Observance date PASS:
23 / 26 (rerun 2026-09-14, was 11 / 26 pre-ORDEN-6)

Calendar differences:
3 (was 15 pre-ORDEN-6)

Parana PASS:
0 / 23 compared (was 0 / 11 compared pre-ORDEN-6)

Parana configuration differences:
23 (was 11 pre-ORDEN-6)

Confirmed ENGINE BUG:
0
```

## Rerun — Campaign 46C Final (2026-09-14)

Re-executed against engine state at commit `2a6d5fd` (`Fix sunrise() day
carry: build the instant from the date's 0h UT plus unnormalised UT`, ORDEN-6),
golden corrections applied in `ed9450a`. No engine code was modified by this
rerun; only `scsmath_validation.rs` was re-invoked against the unchanged
`scsmath-fixtures.csv`.

Rows that changed (12 flip `no_matching_observance_path`/`DIFFERENCE` →
`current_valid_ekadasi`/`PASS`; 1 unchanged in status but with a changed
intermediate value): SCS-0003, 0004, 0005, 0006, 0007, 0008, 0009, 0010, 0011,
0013, 0014, 0015 (status DIFFERENCE → PASS); SCS-0012 (status unchanged at
DIFFERENCE, but `engine_tithi_at_arunodaya` changed 12 → 11). Rows unchanged:
SCS-0001, 0002, 0016, 0017, 0019–0025 (13 rows, byte-identical).

Still `DIFFERENCE` after rerun: SCS-0012 (Vyañjulī Mahādvādaśī — this
validator does not derive Mahādvādaśī facts), SCS-0018 (2026-11-20, tithi 10
at sunrise), SCS-0026 (2027-03-18). None of these three is a day-shift case.

The 12 rows that flipped to PASS carry a static `DOC-001` classification in
`04_Tests/Vaishnava/normative-observance-certification.csv` ("prior SCS
analysis classified this pattern as configuration/tradition difference, not
confirmed engine bug"). That classification is now contradicted by
measurement for those 12 rows. This document does not resolve that
contradiction; see the campaign report for the owner.

## Interpretation

SCS Math is now a real external oracle in the certification suite.

The result is not a `PASS` certification yet. The current (post-rerun)
comparison shows:

- 23 observance dates match the engine path.
- 3 observance dates differ and are classified as `CALENDAR DIFFERENCE`.
- 23 comparable Parāṇa windows differ and are classified as
  `CONFIGURATION DIFFERENCE`.
- No difference is classified as a confirmed `ENGINE BUG`.

Campaign 46F.1 analyzed the pre-ORDEN-6 differences in:

```text
04_Tests/SCSMath/SCSMath-Difference-Report.md
```

Pre-ORDEN-6 result (not rerun; this document's own analysis, superseded in
part by the rerun above for 12 of its 15 rows — see the rerun section):

```text
Observance date differences:
15

CONFIGURATION_DIFFERENCE:
14

TRADITION_DIFFERENCE:
1

ENGINE_BUG:
0
```

No code should be changed until the differences are analyzed and assigned to a
responsible layer.

## Difference Classes

The validator uses:

```text
PASS
CALENDAR DIFFERENCE
CONFIGURATION DIFFERENCE
```

Campaign 46F does not attempt to resolve those differences. It only makes the
oracle reproducible and records the first comparison.

## Status

```text
SCS Math fixtures:
PASS

SCS Math validator:
PASS

SCS Math certification:
OPEN / ANALYZED

Knowledge Base:
SIN CAMBIOS

Motor:
SIN CAMBIOS

Architecture:
NO
```
