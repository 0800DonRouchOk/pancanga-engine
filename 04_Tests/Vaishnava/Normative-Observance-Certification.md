# Normative Observance Certification

Campaign: 52.2

Mode: certification only. No motor, Calendar Engine, Observance Engine, Knowledge Base, architecture, or rule logic was modified.

## Reference Sources

- PureBhakti: `04_Tests/PureBhakti/purebhakti-fixtures.csv`
- SCS Math: `04_Tests/SCSMath/scsmath-fixtures.csv`
- GCal: `EXTERNAL_ORACLE_PENDING`

GCal remains outside the compared set because the local fixture file is still a pending template rather than a real oracle source.

## Scope Boundary

This campaign compares observance identity fields: date, id, slug, display name, type, māsa, pakṣa, and fasting_for. Parāṇa windows remain covered by the prior Parāṇa-specific oracle reports.

## Summary

- Observances compared: 42
- Coincidences: 20
- Differences: 22
- Confirmed ENGINE BUG: 0

## Classification

- AST-001: 0
- MAS-001: 6
- AYA-001: 0
- OBS-001: 0
- MD-001: 1
- FAST-001: 0
- DOC-001: 15
- PASS: 20

## Difference Matrix

| Case | Oracle | Date | Reference | Engine | Category | Hypothesis |
| --- | --- | --- | --- | --- | --- | --- |
| PB-0001 | PureBhakti | 2026-05-13 | EK-006 / apara | EK-004 / varuthini | MAS-001 | Reference māsa is Jyeṣṭha, while engine native māsa is Vaiśākha. |
| PB-0002 | PureBhakti | 2026-05-26 | REFERENCE-PENDING / reference_pending | EK-003 / mohini | MAS-001 | Reference uses an Ekādaśī identity not present in the current v1 ordinary catalog, commonly indicating Adhika Māsa naming coverage pending certification. Reference name is not present in the current v1 ordinary Ekādaśī catalog. |
| PB-0003 | PureBhakti | 2026-06-11 | REFERENCE-PENDING / reference_pending | EK-006 / apara | MAS-001 | Reference uses an Ekādaśī identity not present in the current v1 ordinary catalog, commonly indicating Adhika Māsa naming coverage pending certification. Reference name is not present in the current v1 ordinary Ekādaśī catalog. |
| PB-0009 | PureBhakti | 2026-09-07 | MD-003 / trisprsa | EK-012 / aja | MD-001 | Reference marks Mahādvādaśī, while this date-level comparison did not derive Mahādvādaśī facts automatically. |
| SCS-0001 | SCS Math | 2026-03-15 | EK-002 / papamocani | EK-024 / vijaya | MAS-001 | Reference māsa is Chaitra, while engine native māsa is Phālguna. |
| SCS-0002 | SCS Math | 2026-03-29 | EK-001 / kamada | EK-023 / amalaki | MAS-001 | Reference māsa is Chaitra, while engine native māsa is Phālguna. |
| SCS-0003 | SCS Math | 2026-04-13 | EK-004 / varuthini |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0004 | SCS Math | 2026-04-27 | EK-003 / mohini |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0005 | SCS Math | 2026-05-13 | EK-006 / apara |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0006 | SCS Math | 2026-05-27 | REFERENCE-PENDING / reference_pending |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. Reference name is not present in the current v1 ordinary Ekādaśī catalog. |
| SCS-0007 | SCS Math | 2026-06-11 | REFERENCE-PENDING / reference_pending |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. Reference name is not present in the current v1 ordinary Ekādaśī catalog. |
| SCS-0008 | SCS Math | 2026-06-25 | EK-005 / nirjala |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0009 | SCS Math | 2026-07-11 | EK-008 / yogini |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0010 | SCS Math | 2026-07-25 | EK-007 / sayana |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0011 | SCS Math | 2026-08-09 | EK-010 / kamika |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0012 | SCS Math | 2026-08-24 | MD-002 / vyanjuli |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0013 | SCS Math | 2026-09-07 | EK-012 / aja |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0014 | SCS Math | 2026-09-22 | EK-011 / parsva |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0015 | SCS Math | 2026-10-06 | EK-014 / indira |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0018 | SCS Math | 2026-11-20 | EK-015 / utthana |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |
| SCS-0023 | SCS Math | 2027-02-02 | REFERENCE-PENDING / reference_pending | EK-022 / sattila | MAS-001 | Reference uses an Ekādaśī identity not present in the current v1 ordinary catalog, commonly indicating Adhika Māsa naming coverage pending certification. Reference name is not present in the current v1 ordinary Ekādaśī catalog. |
| SCS-0026 | SCS Math | 2027-03-18 | EK-023 / amalaki |  /  | DOC-001 | The reference date has no matching current/previous Viddhā observance path under the current local configuration; prior SCS analysis classified this pattern as configuration/tradition difference, not confirmed engine bug. |

## Interpretation

The comparison did not identify any confirmed ENGINE BUG. The differences are concentrated in known certification boundaries: unresolved oracle/civil configuration in SCS Math rows, Adhika Māsa naming not present in the current v1 ordinary catalog, and Mahādvādaśī fact derivation not performed by this date-level validator.

## Certification Status

OPEN / DIFFERENCES CLASSIFIED
