# Campaign 55.2: Ekādaśī Production

Status: DEFINED

Phase: Knowledge Acquisition & Authoring

Target: 24 Ekādaśī observances

## Objective

Produce the complete Ekādaśī content collection using EK-010 Kāmikā Ekādaśī as
the reference implementation for editorial quality, evidence traceability, and
publication discipline.

Campaign 55.2 is a production campaign, not a process-design campaign.

## Reference Implementation

```text
Golden Sample:
EK-010 Kāmikā Ekādaśī

Commit:
7c1c0e1

Status:
Published

Evidence claims:
42 refs / 22 unique

Invented doctrine:
0
```

Every new Ekādaśī entry must be measured against this standard.

## Golden Sample Freeze

EK-010 is frozen as the reference implementation for the Observance Library
authoring workflow.

It must not be casually edited during production work.

Allowed changes to EK-010:

```text
source correction
evidence correction
schema compatibility fix
documented editorial improvement
bug in the published content
```

Required for any EK-010 change:

```text
update the dossier or evidence log
record the reason
preserve claim-level traceability
document the change in a campaign report
```

The reference implementation must never drift silently.

## Production Rule

Each Ekādaśī must follow the same pipeline:

```text
Research
↓
Evidence Collection
↓
Research Dossier
↓
Evidence Log
↓
Authoring
↓
Review
↓
Approval
↓
Publish
```

No JSON may be marked `published` unless it has:

```text
approved dossier
approved evidence log
section-level evidence_claims
definitive provenance
pending markers for unsupported content
0 invented doctrine
```

## Scope

Campaign 55.2 covers the 24 ordinary Ekādaśīs currently exposed by the Native
Observance Resolver and Festival Catalog.

Current status:

```text
Published:
1 / 24

Reference implementation:
EK-010 Kāmikā Ekādaśī

Remaining:
23 / 24
```

## Production Queue

| ID | Slug | Display Name | Status |
| --- | --- | --- | --- |
| EK-001 | kamada | Kāmadā Ekādaśī | TODO |
| EK-002 | papamocani | Pāpamocanī Ekādaśī | TODO |
| EK-003 | mohini | Mohinī Ekādaśī | TODO |
| EK-004 | varuthini | Varūthinī Ekādaśī | TODO |
| EK-005 | nirjala | Nirjalā Ekādaśī | TODO |
| EK-006 | apara | Aparā Ekādaśī | TODO |
| EK-007 | sayana | Śayanā Ekādaśī | TODO |
| EK-008 | yogini | Yoginī Ekādaśī | TODO |
| EK-009 | pavitropana | Putradā - Pavitraropani Ekādaśī | TODO |
| EK-010 | kamika | Kāmikā Ekādaśī | PUBLISHED / GOLDEN SAMPLE |
| EK-011 | parsva | Pārśva Ekādaśī | TODO |
| EK-012 | aja | Ajā Ekādaśī | TODO |
| EK-013 | pasankusa | Pāśāṅkuśā Ekādaśī | TODO |
| EK-014 | indira | Indirā Ekādaśī | TODO |
| EK-015 | utthana | Utthāna Ekādaśī | TODO |
| EK-016 | rama | Rāmā Ekādaśī | TODO |
| EK-017 | mokshada | Mokṣadā Ekādaśī | TODO |
| EK-018 | utpanna | Utpannā Ekādaśī | TODO |
| EK-019 | putrada_pausa | Putradā Ekādaśī | TODO |
| EK-020 | saphala | Saphalā Ekādaśī | TODO |
| EK-021 | jaya | Jayā Ekādaśī | TODO |
| EK-022 | sattila | Ṣaṭ-tilā Ekādaśī | TODO |
| EK-023 | amalaki | Āmalakī Ekādaśī | TODO |
| EK-024 | vijaya | Vijayā Ekādaśī | TODO |

## Progress Metrics

```text
Golden Sample:
1 / 1

Ekādaśī published:
1 / 24

Ekādaśī remaining:
23 / 24

Mahādvādaśī published:
0 / 8

Major Līlā published:
0 / TBD

Appearance Days published:
0 / TBD

Disappearance Days published:
0 / TBD

Invented doctrine:
0
```

## Definition of Done

Campaign 55.2 is complete when:

```text
24 / 24 Ekādaśī JSON files are published
24 / 24 have approved dossiers
24 / 24 have approved evidence logs
24 / 24 have definitive provenance
24 / 24 pass local JSON validation
0 unsupported sections are completed by inference
0 invented doctrine
```

## Restrictions

Do not modify:

```text
Calendar Engine
Observance Resolver
Astronomy Engine
Native Māsa
Knowledge Base normativa
Calendar rules
Architecture
Public API
```

Campaign 55.2 may update:

```text
Observance research dossiers
Evidence logs
Observance JSON content
Production reports
Project status / roadmap
```

Only content and documentation may change.
