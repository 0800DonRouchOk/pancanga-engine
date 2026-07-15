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
11 / 26

Calendar differences:
15

Parana PASS:
0 / 11 compared

Parana configuration differences:
11

Confirmed ENGINE BUG:
0
```

## Interpretation

SCS Math is now a real external oracle in the certification suite.

The result is not a `PASS` certification yet. The current comparison shows:

- 11 observance dates match the engine path.
- 15 observance dates differ and are classified as `CALENDAR DIFFERENCE`.
- 11 comparable Parāṇa windows differ and are classified as
  `CONFIGURATION DIFFERENCE`.
- No difference is classified as a confirmed `ENGINE BUG`.

Campaign 46F.1 analyzed the differences in:

```text
04_Tests/SCSMath/SCSMath-Difference-Report.md
```

Result:

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
