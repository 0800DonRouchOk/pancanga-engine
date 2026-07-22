# KB-OBS-001: Observance Authoring Guidelines

Status: ACTIVE

Campaign: 54.0

Scope: Observance Library governance

## Purpose

This document defines the mandatory editorial standard for every content entry
added to the Pancanga Engine Observance Library.

It is not a calculation specification.

It does not introduce doctrine, interpret scripture, modify the engine, or
change any calendrical rule. It governs only the way observance knowledge is
documented, reviewed, and published.

## Layer Boundary

```text
Astronomy Engine
Calculates astronomical quantities.

Calendar Engine
Determines calendrical context such as tithi, pakṣa, and māsa.

Observance Resolver
Identifies the observance corresponding to the calculated context.

Observance Library
Stores structured knowledge about each observance.

KB-OBS-001
Defines the editorial rules for writing, reviewing, and publishing that
structured knowledge.
```

The Observance Library must never become a hidden calculation layer. It may
describe an observance, but it must not decide whether that observance occurs.

## Editorial Principles

```text
Traceability before completeness.

No doctrinal inference without source support.

Every doctrinal claim must be linked to a source.

Original text, editorial summary, and metadata must remain distinct.

The tone must be descriptive, not promotional, apologetic, or emotionally
coercive.

When evidence is incomplete, the entry must say pending rather than supply a
guess.

External calendars may be cited as references or certification oracles, but they
do not replace primary sources.
```

## Editorial States

Every observance entry must have a visible editorial state.

Allowed states:

```text
draft
sources_collected
reviewed
approved
published
deprecated
```

Meaning:

```text
draft
The file exists and validates structurally, but content may still be pending.

sources_collected
Primary or accepted secondary sources have been identified and recorded.

reviewed
The content has been checked against its recorded sources.

approved
The content is accepted for release use.

published
The content is approved and visible in a release channel.

deprecated
The entry remains for compatibility but should not be used as active content.
```

No entry may move directly from `draft` to `published`.

## Required Content Discipline

An entry may contain:

```text
direct quotation
editorial summary
metadata
calculation note
certification note
pending marker
```

An entry must not contain:

```text
unsourced doctrinal claims
unmarked interpretation
invented narrative detail
unverified benefits
ritual instructions presented as universal when source support is absent
calendar decisions that duplicate or override Pancanga Engine
```

## Section Rules

### Presentation

The `presentation` block is for client-facing copy.

It may contain:

```text
title
subtitle
short_description
hero_quote
```

Presentation text may be adapted for RC1, web, mobile, API, or CLI use, but it
must not alter the doctrinal content.

If no approved phrase exists for `hero_quote`, use an empty string.

### Summary

The summary must answer:

```text
What is this observance?
Why is it identified by this name?
What is known with source support?
```

It must be brief and source-aligned.

### History

The history section must describe:

```text
origin of the observance narrative
principal figures
narrative context
core event or teaching
```

It must not add psychological motives, dramatic detail, or devotional
interpretation unless the source explicitly supports them.

### Scriptural Basis

Each scriptural basis item should record:

```text
source work
chapter
verses
language
edition or local source
type of use
status
summary
```

Allowed status values:

```text
confirmed
derived
pending
not_applicable
```

`derived` must be used cautiously and must explain the derivation.

### Benefits

Benefits must be classified by evidence.

Allowed classifications:

```text
explicit
deduced
editorial
```

Release-quality entries should prefer explicit benefits. Deduced or editorial
benefits must be clearly marked and must not be presented as direct scriptural
claims.

### Observance

The observance section may describe general practice, but it must distinguish:

```text
preparation
fasting
night vigil
Parāṇa
exceptions
```

It must not decide the date or Parāṇa window. Those are calculated by Pancanga
Engine for each date and location.

### Procedure

Procedure content must be practical but bounded.

It may say:

```text
Follow the calculated observance and Parāṇa guidance for the selected date and
location.
```

It must not introduce new ritual requirements unless those requirements are
explicitly sourced and reviewed.

### Parāṇa

The Parāṇa section must explain only the relationship between the observance and
the calculated Parāṇa result.

It must not hardcode local times.

It must not override:

```text
Hari-vāsara
Dvādaśī end
Short Dvādaśī contingency
```

### Astronomy

The astronomy section must explain only the calendrical relationship, such as:

```text
This observance is identified from tithi, pakṣa, māsa, and applicable
Mahādvādaśī rules.
```

It must not repeat technical astronomy details that are generated dynamically by
the engine.

### Certification

The certification section must record how the content was prepared and reviewed.

It should include:

```text
source used
date
responsible author or reviewer
version
review state
```

Certification of content is separate from certification of calculation.

## References

Every observance file must contain at least one reference.

Reference fields should include:

```text
source
reference_type
chapter
verses
pages
local_path
url
note
```

Accepted `reference_type` values:

```text
scripture
commentary
calendar_oracle
local_pdf
project_document
certification_report
```

If a source exists only as a local PDF, the local path and page range should be
recorded.

## Translations

The base language of an observance entry is defined by:

```text
metadata.language
```

Other translations must derive from the approved base entry, not independently
from unsynchronized source summaries.

Translation changes must not modify:

```text
identity.id
identity.slug
classification
references
calculation behavior
```

If the base entry is not approved, translations must not be marked as approved.

## Editorial Metadata

Each observance should include editorial metadata either inside
`metadata.editorial` or in a future schema-compatible `editorial` block.

Recommended structure:

```json
{
  "editorial": {
    "status": "draft",
    "author": null,
    "reviewer": null,
    "approved_by": null,
    "last_review": null,
    "source_coverage": 0,
    "notes": []
  }
}
```

Field meanings:

```text
status
Current editorial maturity.

author
Person or process that prepared the entry.

reviewer
Person who checked source alignment.

approved_by
Person or authority approving publication.

last_review
Most recent review date.

source_coverage
Percentage estimate of required source material covered by the entry.

notes
Audit notes, unresolved questions, or known limitations.
```

## Acceptance Criteria

An observance may be considered complete only when:

```text
JSON validates against the approved schema.

All required sections are present.

All doctrinal claims have traceable sources.

Benefits are explicit or clearly classified.

Procedure text does not override the engine.

Parāṇa text does not hardcode dynamic results.

Translations derive from an approved base entry.

Editorial review is complete.

The entry is marked approved or published.
```

## Pending Content Policy

Pending content is acceptable when the structure exists but the source work has
not yet been completed.

Pending content must be explicit.

Preferred phrasing:

```text
Contenido pendiente de estructurar desde fuentes locales aprobadas.
```

Pending entries must not be presented as complete to users.

## Migration Policy

Existing RC1-compatible fields may remain during migration:

```text
name
calendar_context
story_summary
main_teaching
observance_notes
benefits_summary
rc1_scope_note
```

Canonical fields are:

```text
identity
classification
presentation
content
references
related_observances
metadata
```

Future clients should read canonical fields first and compatibility fields only
as fallback.

## Prohibited Shortcuts

Do not populate content from:

```text
memory
general internet summaries
unsourced devotional retellings
calendar names alone
automatic paraphrase without recorded source
```

Do not use an external calendar to create doctrinal content unless the calendar
itself is being documented as an external reference or oracle.

## Campaign 54.0 Result

```text
KB-OBS-001
Observance Authoring Guidelines

Status:
ACTIVE

Manual Editorial:
PASS

Schema governance:
PASS

Content completion:
NOT PERFORMED

Invented doctrine:
0

Motor:
SIN CAMBIOS

Knowledge Base normativa:
SIN CAMBIOS

Architecture:
SIN CAMBIOS
```
