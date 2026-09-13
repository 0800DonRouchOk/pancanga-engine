# Campaign 61.0: Vaiṣṇava Festival Engine Foundation

Status: FOUNDATION PASS

## Objective

Prepare Pancanga Engine for a future Gauḍīya Vaiṣṇava Festival Engine without
changing the certified Ekādaśī, Mahādvādaśī, Parāṇa, astronomy, resolver,
knowledge-base, or desktop packaging layers.

Campaign 61.0 is a foundation and audit campaign. It does not implement
Janmāṣṭamī or any other new festival calculation.

## Scope Boundary

No changes are made to:

```text
Astronomy Engine
Ekādaśī Engine
Mahādvādaśī logic
Existing Parāṇa logic
Stable Observance Resolver
Published Knowledge Base
Desktop packaging
```

## Deliverables

Created:

```text
04_Tests/Festivals/Existing-Engine-Audit.md
05_Documentation/Festivals/Festival-Registry.md
05_Documentation/Festivals/Festival-Rule-Taxonomy.md
05_Documentation/Festivals/Dossiers/FEST-001-Janmashtami-Research-Dossier.md
05_Documentation/Festivals/Evidence/FEST-001-Janmashtami-Evidence-Log.md
05_Documentation/Festivals/Rule-Analysis/FEST-001-Janmashtami-Rule-Analysis.md
05_Documentation/Festivals/Festival-Engine-Architecture-Proposal.md
05_Documentation/Festivals/Campaign-61.1-Required-Changes.md
```

## Findings

The current engine already exposes or can reuse several foundational inputs:

```text
Tithi at sunrise
Sunrise
Instantaneous tithi
Pakṣa
Vaiṣṇava māsa
Solar longitude
Lunar longitude
Location
```

The following capabilities are not yet exposed as reusable festival-engine APIs:

```text
Sunset
Tithi start/end for arbitrary civil or ritual windows
Nakṣatra start/end
Civil timezone model
Midnight-crossing events
```

## Registry

An initial registry was created with ten festival candidates:

```text
FEST-001 Śrī Kṛṣṇa Janmāṣṭamī
FEST-002 Śrī Rādhāṣṭamī
FEST-003 Gaura Pūrṇimā
FEST-004 Nityānanda Trayodaśī
FEST-005 Nṛsiṁha Caturdaśī
FEST-006 Rāma Navamī
FEST-007 Balarāma Pūrṇimā
FEST-008 Vāmana Dvādaśī
FEST-009 Govardhana Pūjā
FEST-010 Jagannātha Ratha-yātrā
```

Rules not yet documented by local accepted sources are explicitly marked:

```text
RESEARCH_REQUIRED
```

## Golden Sample

Śrī Kṛṣṇa Janmāṣṭamī was selected as the Golden Sample for the future Festival
Engine.

Created:

```text
Research Dossier
Evidence Log
Rule Analysis
Required astronomical inputs
Known conflicts / ambiguities
Proposed resolver logic
```

No definitive mathematical implementation was added.

## Result

```text
Campaign 61.0

Vaiṣṇava Festival Engine Foundation:
PASS

Existing engine audit:
PASS

Festival Registry:
PASS

Rule taxonomy:
PASS

Janmāṣṭamī dossier:
PASS

Evidence:
PASS

Invented doctrine:
0

Ekādaśī Engine:
UNCHANGED

Astronomy Engine:
UNCHANGED

Janmāṣṭamī implementation:
NOT STARTED
```

## Campaign 61.0W Bibliographic Gate

The Festival Engine foundation is governed by
`KB-REF-001: Bibliographic Authority Protocol v2`.

```text
WEBSITE != SOURCE

Primary and commentary authority:
The identified work, edition, passage, and commentary

Operational calendars:
Tier 5 - validation only

Websites and repositories:
Tier 6 - discovery and acquisition only
```

The retroactive audit is recorded in
`04_Tests/Festivals/Campaign-61.0W-Bibliographic-Authority-Audit.md`. It changed
no engine behavior and did not authorize any additional festival rule.
