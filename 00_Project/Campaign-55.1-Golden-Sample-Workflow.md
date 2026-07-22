# Campaign 55.1: Golden Sample Workflow

Status: GOLDEN SAMPLE PUBLISHED

Phase: Knowledge Acquisition & Authoring

## Purpose

Campaign 55.1 defines the mandatory workflow for the first authored
observance in the Observance Library.

The purpose is not to complete many observances quickly. The purpose is to
prove that one observance can move from evidence collection to publication
without inventing doctrine and without losing provenance.

This first published observance becomes the editorial golden sample for all
future observance authoring.

## Governing Rule

No observance JSON may be authored, expanded, reviewed, approved, or published
until its research dossier and evidence log have passed Campaign 55.1A.

The JSON file is modified only in Campaign 55.1B, and only from the approved
dossier.

## Campaign 55.1A: Golden Sample Research Dossier

Status: PASS

Mode: RESEARCH ONLY

### Objective

Create the complete documentary dossier for one candidate observance selected
as the golden sample.

Campaign 55.1A does not modify the observance JSON.

### Deliverables

```text
Research dossier
Source inventory
KB-REF-001 source classification
Evidence log
Conflict register
Pending-section register
Preliminary provenance
```

### Required Dossier Sections

```text
Observance identity
Candidate rationale
Source inventory
Primary sources found
Secondary sources found
Operational sources found
Technical sources found
Editorial sources found
Documented facts
Documented narrative
Documented benefits
Documented procedure
Documented Parāṇa information
Documented astronomy / calendar context
Missing information
Conflicts between sources
Rejected claims
Preliminary provenance
Recommendation for authoring
```

### Source Inventory

Every source considered for the golden sample must be recorded before it is
used.

Minimum fields:

```text
source_id
title
category
authority_level
allowed_use
local_path / citation location
edition
language
status
notes
```

Sources must be classified according to KB-REF-001 before any claim is accepted
into the dossier.

### Evidence Log

Each significant claim must be recorded as a separate evidence item.

The evidence log is distinct from both the dossier and the observance JSON. It
is the audit trail between source material and authored content.

Required format:

| Claim ID | Claim | Section | Source(s) | Source Level | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| C-001 | Name of the observance | identity | SRC-TBD | Primary | Verified | Pending source registry ID if not yet assigned |
| C-002 | Traditional narrative | history | SRC-TBD | Primary | Verified | Exact citation required |
| C-003 | Spiritual benefit | benefits | SRC-TBD | Primary | Verified | Only explicit claims allowed |
| C-004 | Procedure of observance | procedure | SRC-TBD | Operational | Verified | Must match allowed-use policy |
| C-005 | Uncorroborated detail | pending | none | none | Pending | Must not be authored as fact |

Allowed evidence statuses:

```text
Verified
Pending
Conflict
Rejected
```

### Conflict Handling

If two accepted sources disagree, Campaign 55.1A must record the difference
without resolving it through interpretation.

Conflicts must remain visible in the dossier until a later review approves a
resolution.

### Pending Handling

If a section lacks sufficient evidence, it must be marked as pending in the
dossier.

Pending sections must not be filled during authoring unless new evidence is
added to the dossier and evidence log first.

### Restrictions

Do not modify:

```text
Observance JSON
Calendar Engine
Observance Resolver
Astronomy Engine
Native Māsa
Knowledge Base normativa
Calendar rules
Architecture
Public API
```

Do not:

```text
invent doctrine
invent narrative details
invent benefits
write unsourced summaries
use memory as a source
convert pending evidence into published content
```

### Campaign 55.1A Exit Criteria

```text
Research:
PASS

Evidence Collection:
PASS

Source Classification:
PASS

Dossier:
PASS

Evidence Log:
PASS

JSON modified:
NO

Invented Doctrine:
0
```

## Campaign 55.1B: Golden Sample Authoring

Status: PASS

Mode: AUTHORING FROM APPROVED DOSSIER ONLY

### Prerequisite

Campaign 55.1A must be marked PASS before Campaign 55.1B begins.

### Objective

Complete the selected observance JSON using only the approved research dossier
and evidence log.

No information may be added directly to the JSON if it does not already exist
in the dossier.

### Authoring Rules

```text
Use only claims recorded in the evidence log.
Preserve all provenance.
Respect KB-OBS-001.
Respect KB-REF-001.
Keep unsupported sections pending.
Do not resolve conflicts during writing.
Do not upgrade editorial status without review.
```

### Required JSON Outcomes

The authored JSON must:

```text
validate against observance.schema.json
preserve identity fields
preserve resolver-compatible slug
include definitive provenance
include only sourced content
mark unsupported sections as pending
record editorial status accurately
```

### Completion Checklist

```text
Metadata:
PASS

Provenance:
PASS

Primary Sources:
PASS

Secondary Sources:
PASS / N-A

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

An observance may not be marked `published` until every completed claim is
traceable and every unsupported claim remains pending.

## Full Golden Sample Flow

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

## Campaign 55.1 Definition

Campaign 55.1 is complete because both subcampaigns are complete:

```text
Campaign 55.1A:
PASS

Campaign 55.1B:
PASS

Golden Sample:
EK-010 Kāmikā Ekādaśī

Invented Doctrine:
0
```

EK-010 Kāmikā Ekādaśī is the reference implementation for the Observance
Library authoring workflow.

## Reference Freeze

EK-010 is frozen as the Golden Sample.

Future production campaigns must treat it as the standard to match, not as a
draft to adjust casually.

Any future EK-010 change must:

```text
update the dossier or evidence log
preserve section-level evidence_claims
record why the reference implementation changed
avoid silent drift
```
