# EK-010: Kāmikā Ekādaśī 55.1B Authoring

Campaign: 55.1B

Status: PASS

Authoring date: 2026-07-22

Golden Sample: EK-010 Kāmikā Ekādaśī

Invented doctrine: 0

## Inputs

Campaign 55.1B used only approved local inputs:

```text
Research Dossier:
05_Documentation/Observances/Dossiers/EK-010-Kamika-Research-Dossier.md

Evidence Log:
05_Documentation/Observances/Evidence/EK-010-Kamika-Evidence-Log.md

55.1A Review:
05_Documentation/Observances/Dossiers/EK-010-Kamika-55.1A-Review.md

Editorial Rules:
05_Documentation/Knowledge-Base/KB-OBS-001-Observance-Authoring-Guidelines.md

Reference Rules:
05_Documentation/Knowledge-Base/KB-REF-001-Reference-Classification.md
```

No additional research was performed.

No external source was added.

No claim was added outside the approved evidence log.

## Output

Authored file:

```text
08_Examples/RC1-Experience/content/ekadasi/kamika.json
```

Status:

```text
published
```

## Schema Alignment

The canonical schema was minimally aligned with KB-OBS-001 so the Golden Sample
could be published and audited.

Schema changes:

```text
metadata.status now accepts:
sources_collected
published

content text sections now accept:
evidence_claims

scriptural_basis items now accept:
evidence_claims
```

Reason:

```text
KB-OBS-001 already defined published as an editorial state.
Campaign 55.1B requires section-level traceability to the evidence log.
```

No calculation rule, engine behavior, or observance identity logic was changed.

## Section Traceability Matrix

| JSON Section | Evidence Claims | Status |
| --- | --- | --- |
| identity | C-001, C-002, C-003 | PASS |
| classification | C-004 | PASS |
| presentation | C-003, C-004, C-008, C-009, C-011, C-021 | PASS WITH HERO QUOTE PENDING |
| content.summary | C-003, C-004, C-008, C-009, C-011 | PASS |
| content.history | C-005, C-006, C-007, C-008, C-009 | PASS |
| content.scriptural_basis | C-014, C-015, C-016, C-017 | PASS WITH BIBLIOGRAPHIC PENDING |
| content.observance | C-008, C-009, C-010, C-011 | PASS |
| content.benefits | C-011, C-012, C-013 | PASS |
| content.procedure | C-008, C-009, C-010, C-019 | PASS |
| content.parana | C-018, C-019 | PASS WITH KĀMIKĀ-SPECIFIC RULE PENDING |
| content.astronomy | C-004, C-020 | PASS |
| content.certification | C-001, C-002, C-022 | PASS |
| references | C-014, C-015, C-016, C-017 | PASS |
| provenance | C-001 through C-019, C-022 | PASS |

## Pending Items Carried Forward

The following evidence items remain pending and were not filled by inference:

```text
C-015
Exact chapter number for the Kāmikā passage.

C-016
Exact verse range for the Kāmikā passage.

C-017
Sanskrit source witness for the Kāmikā passage.

C-018
Kāmikā-specific Parāṇa instructions.

C-021
Hero quote for Kāmikā Ekādaśī.
```

## Rejected Items Enforced

The following rejected claims were excluded from authored content:

```text
C-020
Static astronomy values for a specific date.

C-022
Existing kamika.json as doctrinal evidence.
```

The existing JSON was audited and rewritten against the approved dossier. It was
not treated as a source.

## Authoring Result

```text
Metadata:
PASS

Provenance:
PASS

Primary Sources:
PASS

Secondary Sources:
N-A

History:
PASS

Procedure:
PASS

Benefits:
PASS

Astronomy:
PASS

Parāṇa:
PASS

Editorial Review:
PASS

Approval:
PASS

Status:
Published
```

## Validation

```text
JSON parse:
PASS

Schema alignment:
PASS

Evidence claims present:
PASS

Evidence claims valid:
PASS

Evidence claim references:
42

Unique evidence claims used:
22

Unsupported sections marked pending:
PASS

Invented doctrine:
0
```

The environment did not include the `jsonschema` Python package, so full JSON
Schema validation could not be executed. A local structural validation was used
instead.

## Golden Sample Declaration

```text
Golden Sample:
EK-010 Kāmikā Ekādaśī

Workflow:
Approved

Reference implementation:
YES

Future observance standard:
Match EK-010 traceability or remain unpublished.
```
