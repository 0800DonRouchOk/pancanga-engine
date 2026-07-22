# KB-REF-001: Reference Classification

Status: ACTIVE

Campaign: 54.1

Scope: Reference governance

## Purpose

This document defines the official taxonomy of sources used by Pancanga Engine
and the permitted use of each category.

It does not add observance content, evaluate new doctrinal claims, modify the
engine, or change any rule. It defines how the project decides what kind of
authority a source has and where that authority may be used.

## Core Questions

Every statement included in the Observance Library must be able to answer:

```text
Where does this information come from?

What authority does that source have?

For what purpose may that source be used?

Who verified that this use is valid?
```

If any of these questions cannot be answered, the entry must remain in a
non-published editorial state.

## Source Levels

### Level A: Canonical Sources

Definition:

```text
Primary doctrinal authority adopted by the project.
```

Examples:

```text
Śrīmad Bhāgavatam
Bhagavad-gītā
Caitanya-caritāmṛta
Hari-bhakti-vilāsa
```

Permitted use:

```text
History
Procedure
Benefits
Doctrinal basis
```

Restrictions:

```text
Do not infer beyond the passage.
Do not use a translation note as if it were the source text.
Record edition, language, chapter, verse, or local path whenever available.
```

### Level B: Ācārya Commentaries

Definition:

```text
Authorized traditional explanation, interpretation, or contextualization.
```

Examples:

```text
Śrīla Prabhupāda
Bhaktisiddhānta Sarasvatī
Jīva Gosvāmī
Sanātana Gosvāmī
Digdarśinī-ṭīkā
```

Permitted use:

```text
Explanation
Context
Commentary
Traditional interpretation
Clarification of ambiguous source passages
```

Restrictions:

```text
Do not use commentary to erase an explicit primary-source rule.
When commentary resolves ambiguity, record the ambiguity and the commentary used.
```

### Level C: Operational Calendar Documents

Definition:

```text
Documents that define or evidence calendar practice and operational calendar
outcomes.
```

Examples:

```text
Official pañjikās
Adopted project rules
KB-CAL documents
PureBhakti fixtures
SCS Math fixtures
GCal fixtures when available
```

Permitted use:

```text
Calendrical resolution
Ekādaśī comparison
Parāṇa comparison
Mahādvādaśī comparison
Operational certification
```

Restrictions:

```text
Calendar oracles certify or compare outcomes.
They do not replace primary normative sources.
Differences must be classified before any code or rule change.
```

### Level D: Technical Sources

Definition:

```text
Sources used to explain astronomy, computation, models, or historical technical
context.
```

Examples:

```text
Sūrya-siddhānta
Siddhānta-śiromaṇi
Swiss Ephemeris
Richard L. Thompson
modern astronomical literature
project validation reports
```

Permitted use:

```text
Astronomy
Models
Algorithms
Technical context
Validation methodology
```

Restrictions:

```text
Do not use technical sources to establish devotional benefits.
Do not use modern astronomical literature to reinterpret HBV rules.
```

### Level E: Editorial Sources

Definition:

```text
Project-generated summaries, translations, explanations, and presentation copy.
```

Examples:

```text
summary
synthesis
translation
RC1 presentation text
content notes
release explanations
```

Permitted use:

```text
Editorial presentation
User-facing explanation
Summaries derived from higher-level sources
```

Restrictions:

```text
Editorial content is never an authority by itself.
It must derive from Level A, B, C, or D sources.
It must not introduce claims that are absent from its recorded provenance.
```

## Permission Matrix

Legend:

```text
YES
Permitted source category for the section.

WARN
Permitted only with explicit labeling and limited scope.

NO
Not permitted as authority for that section.
```

| Section | Level A Canonical | Level B Commentary | Level C Operational | Level D Technical | Level E Editorial |
|---|---:|---:|---:|---:|---:|
| History | YES | YES | NO | WARN | NO |
| Benefits | YES | YES | NO | NO | NO |
| Procedure | YES | YES | YES | NO | NO |
| Parāṇa | NO | YES | YES | WARN | NO |
| Astronomy | NO | WARN | YES | YES | WARN |
| Editorial presentation | NO | NO | NO | NO | YES |

## Provenance Requirement

Every observance entry must include:

```json
{
  "provenance": {
    "primary_sources": [],
    "secondary_sources": [],
    "operational_sources": [],
    "technical_sources": [],
    "editorial_sources": [],
    "reviewed_by": null,
    "approved_by": null,
    "verification_date": null
  }
}
```

`references` and `provenance` are distinct:

```text
references
Bibliography or evidence that may be displayed or cited.

provenance
Internal trace of how the content was built, reviewed, and verified.
```

## Required Provenance Item Fields

Each provenance source item should record:

```text
source
level
use
local_path
chapter
verses
pages
language
edition
note
```

The `level` must correspond to one of:

```text
A
B
C
D
E
```

The `use` field must stay within the permitted use for that level.

## Published State Gate

An observance entry may not reach:

```text
published
```

if it:

```text
contains claims without recorded provenance
uses a source outside its permitted category
lacks editorial review
lacks approval
has unresolved source ambiguity
marks pending content as complete
```

## Relationship To KB-OBS-001

KB-OBS-001 defines how observance content is written.

KB-REF-001 defines which sources may support that content.

Together:

```text
KB-OBS-001
How to write and review observance entries.

KB-REF-001
Which sources can authorize each kind of statement.
```

## Future Document

KB-REF-001 is a policy document. It does not list every accepted source.

The source registry belongs in:

```text
KB-REF-002
Canonical Source Registry
```

KB-REF-002 should record the living list of accepted sources, including:

```text
source title
source level
edition
language
local path
scope of allowed use
status
review notes
```

## Campaign 54.1 Result

```text
KB-REF-001
Reference Classification

Status:
ACTIVE

Reference taxonomy:
PASS

Permission matrix:
PASS

Provenance policy:
PASS

Content added:
0

Invented doctrine:
0

Motor:
SIN CAMBIOS

Knowledge Base normativa:
SIN CAMBIOS

Architecture:
SIN CAMBIOS
```
