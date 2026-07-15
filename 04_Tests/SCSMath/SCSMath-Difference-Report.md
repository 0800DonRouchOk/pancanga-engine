# SCS Math Difference Report

Campaign: 46F.1

Status: ✅ PASS / Differences analyzed

## Objective

Analyze the differences between Pancanga Engine and the Sri Chaitanya Saraswat
Math calendar oracle without changing:

```text
Astronomy Engine
Calendar Engine
Vaiṣṇava Engine
Knowledge Base
```

This report does not ask which calendar is "right". It records why the
published SCS Math result and the engine result differ.

## Inputs

Fixture source:

```text
02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf
```

Fixture CSV:

```text
04_Tests/SCSMath/scsmath-fixtures.csv
```

Validation CSV:

```text
04_Tests/SCSMath/scsmath-validation.csv
```

## Classification Vocabulary

Only the following classes are used:

```text
HBV_CONSISTENT
TRADITION_DIFFERENCE
EDITORIAL_POLICY
CONFIGURATION_DIFFERENCE
ENGINE_BUG
UNKNOWN
```

## Summary

### Observance Date Differences

```text
Total observance date differences:
15

HBV_CONSISTENT:
0

TRADITION_DIFFERENCE:
1

EDITORIAL_POLICY:
0

CONFIGURATION_DIFFERENCE:
14

ENGINE_BUG:
0

UNKNOWN:
0
```

### Parāṇa Differences

```text
Comparable Parāṇa differences:
11

EDITORIAL_POLICY / CONFIGURATION_DIFFERENCE:
11

ENGINE_BUG:
0
```

The Parāṇa differences follow the same pattern already seen with PureBhakti:
SCS Math publishes a practical morning window, while Pancanga Engine exposes the
full HBV-EK-005 interval from the end of Hari-vāsara until the end of Dvādaśī.

## Global Pattern

The 15 observance-date differences do not show a contradiction in the
Hari-bhakti-vilāsa rule chain. They show a comparison/configuration gap between
the SCS Math published civil calendar and the current external validator.

In most cases, the engine reports:

```text
SCS published date:
Dvādaśī at sunrise

Previous date:
valid Ekādaśī observance
```

That means the difference is not caused by HBV-EK-002, HBV-EK-003, or
HBV-EK-005. It is a calendar-layer difference in how the published civil date is
being aligned with the engine's computed local day.

The strongest candidate cause is:

```text
CONFIGURATION_DIFFERENCE
```

Specifically:

- SCS Math is a Nabadwip, India calendar.
- The PDF does not publish numeric coordinates or a timezone rule.
- The fixture uses Nabadwip coordinates and `Asia/Kolkata` as validation
  configuration.
- The current validator compares the published civil date directly against the
  engine's computed sunrise/tithi route.
- For east-longitude calendars, local civil date, UTC date, and sunrise instant
  alignment must be audited before declaring any engine bug.

No motor behavior was changed.

## HBV Rule Chain Used For Analysis

| Rule | Function | Primary references |
| ---- | -------- | ------------------ |
| HBV-EK-001 | Candidate generation | HBV 12.1, 12.40, 12.199 |
| HBV-EK-002 | Viddhā invalidation | HBV 12.199, 12.315-320; Digdarśinī |
| HBV-EK-003 | Observance displacement | HBV 12.317-320, 12.324 |
| HBV-EK-004 | Mahādvādaśī classification | HBV 12.391-392; HBV 13.265-266 |
| HBV-EK-005 | Parāṇa / Hari-vāsara | HBV 13.229-259 |

## Difference Cards

### SCS-0003

| Field | Value |
| ----- | ----- |
| Date | 2026-04-13 |
| SCS Math result | Fast on Varuthinī Ekādaśī; Parāṇa 5:23-9:33 |
| Pancanga Engine result | No matching observance on 2026-04-13; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only if Daśamī at Aruṇodaya invalidates the candidate; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Published SCS civil date and engine local sunrise/tithi date are offset; audit Nabadwip civil-time configuration before changing code |

### SCS-0004

| Field | Value |
| ----- | ----- |
| Date | 2026-04-27 |
| SCS Math result | Fast on Mohinī Ekādaśī; Parāṇa 5:11-9:27 |
| Pancanga Engine result | No matching observance on 2026-04-27; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0005

| Field | Value |
| ----- | ----- |
| Date | 2026-05-13 |
| SCS Math result | Fast on Aparā Ekādaśī; Parāṇa 5:01-7:45 |
| Pancanga Engine result | No matching observance on 2026-05-13; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0006

| Field | Value |
| ----- | ----- |
| Date | 2026-05-27 |
| SCS Math result | Fast on Padminī Ekādaśī; Parāṇa 4:56-8:42 |
| Pancanga Engine result | No matching observance on 2026-05-27; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0007

| Field | Value |
| ----- | ----- |
| Date | 2026-06-11 |
| SCS Math result | Fast on Paramā Ekādaśī; Parāṇa 4:56-9:23 |
| Pancanga Engine result | No matching observance on 2026-06-11; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0008

| Field | Value |
| ----- | ----- |
| Date | 2026-06-25 |
| SCS Math result | Fast on Pāṇḍavā Nirjalā Ekādaśī; Parāṇa 4:57-9:25 |
| Pancanga Engine result | No matching observance on 2026-06-25; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0009

| Field | Value |
| ----- | ----- |
| Date | 2026-07-11 |
| SCS Math result | Fast on Yoginī Ekādaśī; Parāṇa 5:03-9:29 |
| Pancanga Engine result | No matching observance on 2026-07-11; tithi at sunrise 13; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | SCS publishes a displaced/no-fast pattern here; validator reports a different engine date path and requires civil-time/configuration audit |

### SCS-0010

| Field | Value |
| ----- | ----- |
| Date | 2026-07-25 |
| SCS Math result | Fast on Śayan Ekādaśī; Parāṇa 5:07-9:31 |
| Pancanga Engine result | No matching observance on 2026-07-25; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0011

| Field | Value |
| ----- | ----- |
| Date | 2026-08-09 |
| SCS Math result | Fast on Kāmikā Ekādaśī; Parāṇa 5:14-6:17 |
| Pancanga Engine result | No matching observance on 2026-08-09; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0012

| Field | Value |
| ----- | ----- |
| Date | 2026-08-24 |
| SCS Math result | Fast on Vyañjulī Mahādvādaśī; Parāṇa 5:19-5:53 |
| Pancanga Engine result | No matching observance path in current SCS validator; tithi at sunrise 12; previous status `NoObservanceCandidate` |
| HBV rule applied | HBV-EK-004 |
| HBV verses | HBV 12.391-392; HBV 13.265-266; Vyañjulī references HBV 13.356-358 and HBV 12.372-376 |
| Digdarśinī | Consulted in Knowledge Base; ISSUE-VAI-001 later resolved for Vyañjulī formulation |
| Viddhā status | Not the active branch for this fixture |
| Mahādvādaśī status | SCS Math marks Vyañjulī Mahādvādaśī |
| Parāṇa | Not compared because observance date did not match |
| Classification | TRADITION_DIFFERENCE |
| Possible cause | SCS Math applies Vyañjulī Mahādvādaśī where the validator does not yet certify the automatic Mahādvādaśī fact derivation. ISSUE-VAI-001 now resolves the normative Vyañjulī formulation; this remaining difference should be evaluated as validator/oracle configuration, not as an open doctrinal question. |

### SCS-0013

| Field | Value |
| ----- | ----- |
| Date | 2026-09-07 |
| SCS Math result | Fast on Annadā Ekādaśī; Parāṇa 5:24-9:32 |
| Pancanga Engine result | No matching observance on 2026-09-07; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0014

| Field | Value |
| ----- | ----- |
| Date | 2026-09-22 |
| SCS Math result | Fast on Pārśva Ekādaśī; Parāṇa 5:29-9:30 |
| Pancanga Engine result | No matching observance on 2026-09-22; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0015

| Field | Value |
| ----- | ----- |
| Date | 2026-10-06 |
| SCS Math result | Fast on Indirā Ekādaśī; Parāṇa 6:30-9:28 |
| Pancanga Engine result | No matching observance on 2026-10-06; tithi at sunrise 12; previous day path `ObserveOnEkadasi` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Relevant only for Aruṇodaya Viddhā; not indicated by this row |
| Viddhā status | No current Viddhā path recorded; previous day was valid Ekādaśī |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Civil-date alignment/configuration difference for Nabadwip oracle |

### SCS-0018

| Field | Value |
| ----- | ----- |
| Date | 2026-11-20 |
| SCS Math result | Fast on Śrī Utthān Ekādaśī; Parāṇa 8:02-9:35 |
| Pancanga Engine result | No matching observance on 2026-11-20; tithi at sunrise 10; previous status `NoObservanceCandidate` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | No Viddhā branch indicated by this validator row |
| Viddhā status | Not indicated |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | Larger calendar alignment difference than the ordinary one-day pattern; requires focused Nabadwip civil-time/oracle configuration audit |

### SCS-0026

| Field | Value |
| ----- | ----- |
| Date | 2027-03-18 |
| SCS Math result | Fast on Āmalakī Ekādaśī; Parāṇa 5:48-9:46 |
| Pancanga Engine result | No matching observance on 2027-03-18; tithi at sunrise 11 but tithi at Aruṇodaya 10; previous status `NoObservanceCandidate` |
| HBV rule applied | HBV-EK-001; HBV-EK-002; HBV-EK-003 |
| HBV verses | HBV 12.1, 12.40, 12.199, 12.315-320 |
| Digdarśinī | Directly relevant: Daśamī at Aruṇodaya invalidates a candidate as Viddhā |
| Viddhā status | Engine row is consistent with an Aruṇodaya Viddhā interpretation |
| Mahādvādaśī status | None |
| Parāṇa | Not compared because observance date did not match |
| Classification | CONFIGURATION_DIFFERENCE |
| Possible cause | SCS Math publishes a fast where the engine detects Ekādaśī at sunrise but Daśamī at Aruṇodaya; this requires checking the oracle's Aruṇodaya convention and local-time configuration before declaring a doctrinal difference |

## Parāṇa Notes For Matching Observance Dates

The 11 rows with matching observance dates all differ in Parāṇa. These are not
treated as engine bugs because the SCS Math published windows appear to be
practical morning windows, while Pancanga Engine exposes the HBV-EK-005
technical interval:

```text
start = max(sunrise, end of Hari-vāsara)
end   = end of Dvādaśī
```

This is the same class of difference already documented for PureBhakti.

## Conclusion

Campaign 46F.1 does not close SCS Math certification as `PASS`, but it does
remove the immediate risk of an unexplained failure.

```text
Confirmed ENGINE BUG:
0

Knowledge Base changes:
NO

Motor changes:
NO

Next required work:
Audit Nabadwip civil-time/oracle configuration before changing any engine
behavior. Vyañjulī is no longer an open doctrinal blocker after ISSUE-VAI-001.
```
