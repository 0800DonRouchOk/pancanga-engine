# Festival Calculation Family Map

Campaign: 62.0  
Status: PRELIMINARY CAPABILITY MAP

## Purpose

This map groups research by reusable calendrical capability. Assignments are
planning hypotheses based on names, existing dossiers and operational calendar
shape. They are not approved festival rules.

## Families

| Family | Question it will eventually answer | Current engine position | Research action |
| --- | --- | --- | --- |
| `TITHI` | Which tithi is relevant? | AVAILABLE as instantaneous and sunrise state | Specify each festival's selection rule |
| `MASA` | Which lunar/Vaisnava month contains the event? | AVAILABLE through native Vaisnava masa | Certify festival-specific month mapping |
| `PAKSA` | Which fortnight applies? | AVAILABLE | Specify only where required |
| `SUNRISE` | Is the tithi present at sunrise? | AVAILABLE | Reuse; do not duplicate |
| `SUNSET` | Is the tithi present at sunset or a sunset-bound window? | DERIVABLE; no festival-domain adapter | Define a reusable boundary adapter if authorized |
| `NISITHA` | Is the tithi present at local ritual midnight? | NEW DOMAIN CAPABILITY REQUIRED | Janmastami gate remains open |
| `NAKSATRA` | Which naksatra is present at an instant? | AVAILABLE instantaneously | Festival rule still required |
| `TITHI_NAKSATRA_OVERLAP` | Do tithi and naksatra overlap in a required window? | INTERVAL CAPABILITY REQUIRED | Reuse astronomy; add no parallel ephemeris |
| `SOLAR_MONTH` | Which solar month contains the event? | DERIVABLE from solar longitude | Research adopted boundary convention |
| `SANKRANTI` | When does the Sun enter the next sidereal sign? | DERIVABLE from existing solar longitude/root solving | Specify event boundary and ayanamsa policy |
| `RELATIVE_TO_OTHER_FESTIVAL` | Is this event N days before/after another? | NEW OBSERVANCE-DOMAIN CAPABILITY | Define dependency graph and cycle protection |
| `MULTI_DAY_INTERVAL` | Does the observance span a start/end interval? | NEW OBSERVANCE-DOMAIN CAPABILITY | Define inclusive boundaries and interruptions |
| `FASTING` | Is fasting required and until when? | AVAILABLE only for certified Ekadasi domain | Research per festival; do not inherit |
| `PARANA` | Is a break-fast window required? | AVAILABLE only for certified Ekadasi domain | Research per festival; do not reuse semantically without authority |
| `LINEAGE_POLICY` | Which profile owns or enables the observance? | MISSING | Define profile governance before implementation |
| `UNKNOWN` | Insufficient information to select a family | UNKNOWN | Research before architecture or code |

## Reuse Boundary

```text
REUSE WITHOUT CHANGE
- solar and lunar longitude
- instantaneous tithi and naksatra
- sunrise
- native Vaisnava masa
- paksa
- timezone and location input

ADAPTER OR DOMAIN SERVICE MAY BE REQUIRED
- sunset exposure to festival rules
- local civil midnight / nisitha window
- arbitrary tithi and naksatra intervals
- overlap duration and priority comparison

NEW FESTIVAL-DOMAIN CAPABILITY
- relative observances
- multi-day intervals
- lineage profiles
- festival-specific fasting and parana metadata
```

## Research Batches

| Batch | Families | Representative candidates |
| --- | --- | --- |
| FAM-01 | `TITHI`, `MASA`, `PAKSA`, `SUNRISE` | simple appearance/disappearance observances |
| FAM-02 | `SUNSET`, `NISITHA`, `FASTING`, `PARANA` | Janmastami, Gaura Purnima, Nrsimha Caturdasi |
| FAM-03 | `NAKSATRA`, `TITHI_NAKSATRA_OVERLAP` | Vamana Dvadasi and any sourced jayanti rule |
| FAM-04 | `RELATIVE_TO_OTHER_FESTIVAL` | Nandotsava, Gundica Marjana, Hera Pancami, Bahuda |
| FAM-05 | `MULTI_DAY_INTERVAL` | Jhulan, Karttika, Caturmasya, Purusottama vrata |
| FAM-06 | `SOLAR_MONTH`, `SANKRANTI` | Akasa Dipika and solar observance profiles |
| FAM-07 | `LINEAGE_POLICY`, `UNKNOWN` | Nityananda Vamsa and institution-specific days |

## Result

```text
Capability families mapped: 16
Production code changes: 0
Invented rules: 0
```
