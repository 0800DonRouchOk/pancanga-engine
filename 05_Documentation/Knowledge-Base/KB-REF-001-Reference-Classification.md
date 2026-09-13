# KB-REF-001: Bibliographic Authority Protocol v2

Status: ACTIVE

Campaigns: 54.1, 61.0W

Scope: source authority, acquisition, translation, and claim traceability

## Purpose

This document defines how Pancanga Engine identifies and cites authority. It
does not add doctrine, change a calendar rule, or modify the engine.

The governing principle is:

```text
WEBSITE != SOURCE
```

A website may locate or deliver an edition. It is never, by that fact alone,
the authority for a religious, doctrinal, or calendrical rule. The authority is
the work, passage, commentary, edition, or lineage document that the website
makes available.

Every claim must answer:

```text
Where does it come from?
What authority does that source have?
For what purpose may it be used?
Who verified that use?
```

If these questions cannot be answered, the claim is not publication-ready.

## Source Authority Order

### Tier 1: Primary Text

Śāstric and normative texts, including Purāṇas, smṛti, paddhati, original
works of the Gosvāmīs, and Hari-bhakti-vilāsa.

Required metadata, when the edition supplies it:

```text
author or traditional attribution
title
editor
publisher
place
year
volume
chapter or vilāsa
verse or page
language
local artifact and integrity hash
```

Permitted use: doctrine, narrated history, benefits, ritual procedure, and
calendar rules stated by the passage. The claim must not extend beyond the
passage.

### Tier 2: Traditional Commentary

Traditional commentary on a Tier 1 work, such as the Dig-darśinī-ṭīkā.

It must be identified independently and linked to the text it comments on.
Permitted use: traditional interpretation, technical clarification, conflict
analysis, and resolution of an ambiguity when the project records the
ambiguity and the commentary used.

Commentary does not silently replace or erase the primary text.

### Tier 3: Lineage Document

Verifiable documents of a specific lineage or institution, including lineage
paddhatis, pañjikās, instructions of accepted ācāryas, traditional manuals,
and Nityānanda Vaṁśa documents.

Permitted use: lineage-specific practice and declared project policy. A
lineage practice must not be presented as a universal śāstric rule unless a
Tier 1 or Tier 2 source establishes that scope.

### Tier 4: Scholarly Bibliography

Critical editions, academic monographs, historical studies, and specialized
technical works.

Permitted use: identifying calendar systems, resolving terminology, comparing
manuscripts or editions, understanding amānta and pūrṇimānta conventions, and
reconstructing astronomical concepts. Tier 4 may operationalize a technical
term but may not replace an available normative source.

### Tier 5: Operational Calendars

Historical or modern Gauḍīya calendars and calculation products, including
PureBhakti, SCS Math, and GCal outputs.

Permitted use:

```text
VALIDATION ONLY
```

They establish how a tradition or implementation applies a rule in a concrete
case. Dates observed in a calendar must never be reverse-engineered into a
śāstric rule without Tier 1-4 support. Differences are classified before any
rule or code change.

### Tier 6: Web Content

Blogs, websites, articles, forum posts, social media, and unsourced online
explanations.

```text
NORMATIVE AUTHORITY: PROHIBITED
```

Tier 6 may discover bibliography, locate an edition, provide acquisition
metadata, or point to a facsimile. Any substantive claim found there must be
traced to a Tier 1-4 source before use.

## Internal Project Artifacts

Project specifications, rule registries, evidence logs, tests, and editorial
summaries are not external authority tiers. They preserve a translation from
accepted evidence into project behavior.

They may be cited for implementation provenance, but their normative authority
is no greater than the source chain they record.

## Authority Matrix

Legend: `YES` means permitted; `LIMITED` requires explicit scope and labeling;
`NO` means prohibited as authority.

| Use | Tier 1 | Tier 2 | Tier 3 | Tier 4 | Tier 5 | Tier 6 |
|---|---:|---:|---:|---:|---:|---:|
| Doctrine | YES | LIMITED | LIMITED | NO | NO | NO |
| Traditional history | YES | YES | LIMITED | LIMITED | NO | NO |
| Benefits | YES | YES | LIMITED | NO | NO | NO |
| Ritual procedure | YES | YES | YES | NO | VALIDATION | NO |
| Calendar rule | YES | YES | LIMITED | TECHNICAL ONLY | VALIDATION | NO |
| Astronomy | LIMITED | LIMITED | LIMITED | YES | VALIDATION | NO |
| Bibliographic discovery | YES | YES | YES | YES | YES | YES |

## Online Acquisition Rule

When a work is found online:

1. Identify the work and edition.
2. Preserve a local copy when legally and technically possible.
3. Record full bibliographic metadata.
4. Calculate an integrity hash.
5. Record the URL and access date only as acquisition provenance.
6. Cite the work, edition, and passage in claims, not the URL.

Required separation:

```text
SOURCE
Work, author or attribution, commentary, edition, editor, publisher, place,
year, volume, chapter/vilāsa, verse/page.

ACQUISITION
Repository or host, URL, access date, local path, format, bytes, hash.
```

A repository's reputation does not convert its webpage into doctrinal
authority. If the repository disappears, the source identity and local
integrity record must remain sufficient to identify what was used.

## Date-Changing Claim Gate

Every rule capable of changing a calculated civil date, fast, observance, or
pāraṇa must preserve this chain:

```text
WHAT IS THE RULE?
        ↓
WHICH TEXT?
        ↓
WHICH EDITION?
        ↓
WHICH VERSE OR PAGE?
        ↓
WHICH COMMENTARY?
        ↓
HOW WAS IT TRANSLATED INTO A COMPUTABLE RULE?
```

If any required link is absent:

```text
NORMATIVE CLAIM: NOT READY
```

Operational agreement, source popularity, and repeated web assertions do not
close this gate.

## Translation Policy

When a rule depends on a technical term:

- retain the original term and relevant source wording;
- identify the translation or translator used;
- review the mūla when available;
- review the traditional commentary when available;
- record ambiguity instead of hiding it in fluent prose.

No critical date-changing rule may rest solely on a modern secondary
translation. A translation witness can assist reading and discovery, but the
accepted rule must retain a reviewable chain to the underlying text and
commentary.

## Web Search Policy

Internet use is permitted to:

```text
DISCOVER SOURCE
LOCATE EDITION
DOWNLOAD SOURCE
VERIFY BIBLIOGRAPHIC METADATA
```

Internet use is prohibited to:

```text
ESTABLISH DOCTRINE
CREATE CALENDAR RULE
RESOLVE CONFLICT BY POPULARITY
USE A SEARCH RESULT AS AUTHORITY
```

## Provenance Requirement

`references` and `provenance` remain distinct:

```text
references
Bibliography or evidence intended for citation.

provenance
Internal record of acquisition, derivation, review, and approval.
```

A provenance item should record:

```text
source identity
authority tier
permitted use
edition
chapter/vilāsa
verse/page
language
local path
hash or source-record link
acquisition provenance
review note
```

Existing Observance Library fields named `level` (`A`-`E`) are legacy storage
labels. Until a separately approved schema migration, they must be interpreted
through this compatibility map:

| Legacy value | v2 interpretation |
|---|---|
| A | Tier 1 primary text |
| B | Tier 2 traditional commentary |
| C | Tier 3 lineage document or Tier 5 operational calendar; determine from the source itself |
| D | Tier 4 scholarly or technical bibliography |
| E | Project editorial derivative; not an authority tier |

Web material is always Tier 6 regardless of a legacy label. A legacy label
cannot upgrade its authority.

## Publication Gate

An observance entry may not reach or retain `published` authority if it:

- contains a substantive claim without traceable provenance;
- uses a source outside its permitted tier;
- treats web acquisition as source authority;
- relies only on a secondary translation for a critical rule;
- lacks editorial review or approval;
- hides unresolved source ambiguity;
- marks pending evidence as complete.

Discovery of a gate failure does not authorize silent content correction. The
entry is registered for bibliographic remediation, then reviewed through the
approved research and authoring workflow.

## Relationship To Other Governance

```text
KB-OBS-001
How observance content is researched, written, reviewed, and published.

KB-REF-001
Which evidence may authorize each claim and how that evidence is cited.

SRC-001
The living registry of accepted source artifacts and editions.
```

## Retroactive Audit Rule

Changes to this protocol require a source audit of affected existing material.
The audit must classify, without silently rewriting content:

- Ekādaśī rules and content;
- Mahādvādaśī rules and content;
- Janmāṣṭamī research;
- Festival Registry claims.

It must report every normative rule whose only authority is Tier 5 or Tier 6.

## Campaign 61.0W Result

The campaign result is recorded in:

```text
04_Tests/Festivals/Campaign-61.0W-Bibliographic-Authority-Audit.md
```

The policy itself is approved. Existing content retains its exact text during
the audit; authority defects are recorded rather than concealed.
