# EK-016: Rāmā Ekādaśī Evidence Log

Campaign: 55.2B

Status: PUBLISHED

Mode: KNOWLEDGE ACQUISITION & AUTHORING

Observance JSON modified: YES

## Evidence Status Values

```text
Verified
Pending
Conflict
Rejected
```

## Source Inventory

| Source ID | Source | Level | Use | Location |
| --- | --- | --- | --- | --- |
| SRC-EK-016-001 | Ekadashi, El Día Sagrado del Señor Hari | E | Local Spanish witness for līlā authoring | 07_Sources/Vaishnava/204176529-Ekadashi-Es-Ramanuja-Dasa.pdf; PDF pages 71-73; printed pages 70-72 |
| SRC-EK-016-002 | Brahma Vaivarta Purāṇa | A | Traditional source cited by local PDF | PDF pages 71-73; printed pages 70-72; chapter/verses pending |
| SRC-EK-016-003 | Festival Catalog | C | Identity, slug, māsa and pakṣa | 08_Examples/RC1-Experience/content/ekadasi/catalog.json |

## Evidence Log

| Claim ID | Claim | Section | Source(s) | Source Level | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| C-001 | The observance ID is EK-016. | identity | SRC-EK-016-003 | C | Verified | Operational catalog identity. |
| C-002 | The stable slug is `rama`. | identity | SRC-EK-016-003 | C | Verified | Must bind to `content/ekadasi/rama.json`. |
| C-003 | The display name is Rāmā Ekādaśī. | identity | SRC-EK-016-001; SRC-EK-016-003 | E; C | Verified | Source title and catalog agree in substance. |
| C-004 | The catalog classifies this observance as Kārtika Kṛṣṇa Ekādaśī. | classification | SRC-EK-016-003 | C | Verified | Resolver/catalog binding. |
| C-005 | The līlā text is located in the local Ekādaśī book at PDF pages 71-73; printed pages 70-72. | history | SRC-EK-016-001 | E | Verified | Campaign 55.2B source extraction. |
| C-006 | The local PDF cites Brahma Vaivarta Purāṇa as the traditional source. | scriptural_basis | SRC-EK-016-001; SRC-EK-016-002 | E; A | Verified | Exact chapter/verse metadata remains pending when absent. |
| C-007 | The authored history follows the chapter narrative documented in the local PDF. | history | SRC-EK-016-001 | E | Verified | Narrative order and principal characters preserved in structured prose. |
| C-008 | Observance notes are limited to practices described in the local chapter or to dynamic engine-calculated guidance. | observance | SRC-EK-016-001; SRC-EK-016-003 | E; C | Verified | No new ritual rule introduced. |
| C-009 | Benefits are presented only as source-described benefits. | benefits | SRC-EK-016-001 | E | Verified | No independent project assertion of benefit. |
| C-010 | Parāṇa and date-specific astronomy are dynamic Pancanga Engine results, not static content. | parana / astronomy | SRC-EK-016-003 | C | Verified | Content file must not hardcode local times. |
| C-011 | Exact chapter number for the cited Purāṇa passage. | scriptural_basis | SRC-EK-016-001 | E | Pending | Not found in the local PDF during 55.2B. |
| C-012 | Exact verse range for the cited Purāṇa passage. | scriptural_basis | SRC-EK-016-001 | E | Pending | Not found in the local PDF during 55.2B. |

## Evidence Summary

```text
Verified claims:
10

Pending claims:
2

Conflicts:
0

Invented doctrine:
0
```

## Section Readiness

| Section | Status | Notes |
| --- | --- | --- |
| identity | PASS | Catalog-backed. |
| classification | PASS | Catalog/resolver-backed. |
| history | PASS | Authored from the local PDF. |
| scriptural_basis | PARTIAL | Source named; chapter and verses pending if absent from PDF. |
| observance | PASS | Bounded to source and engine guidance. |
| benefits | PASS | Source-described only. |
| procedure | PASS | Does not alter calculation rules. |
| parana | PASS | Dynamic engine result. |
| astronomy | PASS | Dynamic engine result. |
| provenance | PASS | PDF, cited source, catalog and evidence log registered. |

## Campaign Result

```text
Research:
PASS

Evidence:
PASS

Authoring:
PASS

Review:
PASS

Approval:
PASS

Published:
PASS

Invented doctrine:
0
```
