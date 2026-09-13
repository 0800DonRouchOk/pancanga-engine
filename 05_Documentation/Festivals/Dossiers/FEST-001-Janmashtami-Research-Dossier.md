# FEST-001: Śrī Kṛṣṇa Janmāṣṭamī Research Dossier

Campaign: 61.0  
Status: RESEARCH DOSSIER PASS  
Golden Sample: YES  
Mode: Research only

Campaign 61.0T update: this is the original foundation dossier. The primary
HBV Vilāsa 15 source has since been acquired and audited. Current rule evidence
is recorded in `FEST-001-Janmashtami-Primary-Source-Matrix.md` and
`FEST-001-Janmashtami-Canonical-Rule.md`.

## Objective

Create the first research dossier for the future Festival Engine using Śrī
Kṛṣṇa Janmāṣṭamī as the Golden Sample.

This dossier does not implement Janmāṣṭamī calculation.

## Identity

```text
canonical_id: FEST-001
name: Śrī Kṛṣṇa Janmāṣṭamī
spanish_display_name: Śrī Kṛṣṇa Janmāṣṭamī
category: appearance_day
nominal_tithi: Kṛṣṇa Aṣṭamī
implementation_status: RESEARCH_REQUIRED
provenance_status: PARTIAL
```

## Sources Reviewed

### FEST-JAN-REF-001

```text
Source:
Hari-bhakti-vilāsa, Vilāsa 12, local raw text

Classification:
Tier 1 HBV mūla / Tier 2 Dig-darśinī. Source normalization completed by
Campaign 61.0W.

Evidence:
The local text explicitly groups Janmāṣṭamī with other Vaiṣṇava vows and warns
that such vows are not to be performed on biddha / contaminated days.

Use:
Preliminary doctrinal constraint only. It does not by itself provide the full
Janmāṣṭamī resolution algorithm.
```

### FEST-JAN-REF-002

```text
Source:
PureBhakti operational calendar extract for 2026

Classification:
Tier 5, operational calendar reference.

Evidence:
Records Śrī Kṛṣṇa Janmāṣṭamī on 2026-09-04, Aṣṭamī K, with fast until midnight
and next-day break-fast window.

Use:
Validation fixture and operational behavior. Not sufficient as the sole
normative implementation rule.
```

### FEST-JAN-REF-003

```text
Source:
Hari-bhakti-vilāsa, Vilāsa 13, Appendix I local raw text

Classification:
Tier 4 translation/technical reading witness. It cannot authorize a critical
rule without review of the underlying mūla and commentary.

Evidence:
Gives a general translated/editorial explanation that lunar festivals are
computed by identifying the relevant tithi and applying sunrise presence rules
when the tithi spans more than one civil day.

Use:
Technical orientation only. Requires normalization before becoming a normative
rule.
```

## Evidence Summary

### Confirmed from local sources

```text
Janmāṣṭamī is treated as a Vaiṣṇava vrata.
Janmāṣṭamī cannot be resolved casually on biddha days.
PureBhakti operational calendar uses Kṛṣṇa Aṣṭamī and fast until midnight.
PureBhakti operational calendar publishes a next-day break-fast window.
```

### Not yet confirmed from approved local specification

```text
Exact Janmāṣṭamī selection rule.
Whether the decisive window is midnight, sunrise, or another boundary.
How Rohiṇī nakṣatra affects the rule, if adopted.
How to rank conflicting tithi / nakṣatra combinations.
Exact parāṇa rule after Janmāṣṭamī.
Whether local Gauḍīya authority follows the same rule as PureBhakti in all cases.
```

## Known Rule Conflicts / Ambiguities

```text
RULE_CONFLICT_JAN-001

Potential conflict:
General lunar-festival sunrise rule versus operational Janmāṣṭamī fast-until-
midnight behavior.

Resolution:
Do not choose until a local accepted Janmāṣṭamī rule is incorporated into the
Knowledge Base.
```

```text
RULE_CONFLICT_JAN-002

Potential conflict:
Possible Rohiṇī nakṣatra condition is not yet confirmed from local accepted
sources.

Resolution:
Do not implement nakṣatra priority until documented.
```

## Required Astronomical Inputs

```text
local sunrise
local sunset, if the adopted rule uses sunset or civil evening
local midnight window
tithi at sunrise
tithi at midnight
tithi start/end around the civil day
pakṣa
Vaiṣṇava māsa
nakṣatra at relevant window, if adopted
nakṣatra start/end, if adopted
timezone and DST
location
```

## Proposed Resolver Logic

The following is not an implementation. It is the minimum resolver shape needed
after the normative rule is approved.

```text
1. Build a timezone-aware Pañcāṅga State for the candidate civil day.
2. Determine whether Kṛṣṇa Aṣṭamī occurs in the relevant ritual window.
3. If nakṣatra is part of the approved rule, calculate Rohiṇī presence and
   interval overlap.
4. Apply documented biddha / contamination constraints.
5. Select the civil observance day.
6. Derive fasting boundary.
7. Derive parāṇa window from the approved festival rule.
8. Emit FestivalResult(FEST-001) with source and unresolved-rule markers when
   certification is incomplete.
```

## Pending Items

```text
PENDING-JAN-001
Find and incorporate the accepted Gauḍīya Janmāṣṭamī rule.

PENDING-JAN-002
Confirm Rohiṇī nakṣatra role.

PENDING-JAN-003
Confirm fast-until-midnight and post-fast parāṇa rule from normative source.

PENDING-JAN-004
Create implementation fixtures only after the rule is approved.
```

## Result

```text
Janmāṣṭamī dossier:
PASS

Rule implemented:
NO

Invented doctrine:
0
```
