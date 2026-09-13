# Festival Engine v1.0 Scope Proposal

Campaign: 62.0  
Status: PROPOSAL ONLY  
Decision authority: future release planning

## Scope Boundary

This document proposes the first scope of the future **Festival Engine v1.0**.
It does not expand the already frozen Pancanga Engine v1.0 RC scope and does not
authorize implementation.

```text
Current Pancanga Engine v1.0 freeze: UNCHANGED
Festival Engine v1.0 proposal: FUTURE SCOPE
```

## Proposed Core

Tier 1 contains the ten candidates already named in Campaign 61.0:

| ID | Festival | Why it belongs in the product core | Gate before implementation |
| --- | --- | --- | --- |
| FEST-001 | Sri Krsna Janmastami | central appearance festival and Golden Sample | normative Policy A/B/C must be resolved |
| FEST-002 | Sri Radhastami | central Vraja appearance festival | canonical date-selection rule |
| FEST-003 | Gaura Purnima | central Gaudiya appearance festival | boundary, fasting and parana rules |
| FEST-004 | Nityananda Trayodasi | central Panca-tattva appearance festival | date and fasting/parana rules |
| FEST-005 | Nrsimha Caturdasi | major avatara fast | boundary, fasting and parana rules |
| FEST-006 | Rama Navami | major avatara fast | boundary, fasting and parana rules |
| FEST-007 | Balarama Purnima | major appearance festival | noon boundary and parana authority |
| FEST-008 | Vamana Dvadasi | major tithi/naksatra candidate | Sravana overlap and fasting policy |
| FEST-009 | Govardhana Puja | central Vraja/Karttika observance | civil-day selection rule |
| FEST-010 | Jagannatha Ratha-yatra | central Jagannatha cycle event | primary and relative-cycle rules |

## Proposed Release Gates

Each Tier 1 festival must independently satisfy:

```text
canonical identity: PASS
primary/accepted source record: PASS
rule dossier: PASS
evidence log: PASS
date-changing conflicts: RESOLVED
required astronomy: READY
resolver tests: PASS
external fixtures: CLASSIFIED
content provenance: PASS or explicitly out of release scope
```

The release does not require Tier 2-4 implementation. It does require that no
unimplemented record be accidentally exposed as calculated or certified.

## Deferred Scope

### Tier 2 - Major Gaudiya

Fifty-nine records cover major festival cycles and major acarya observances.
They are high-value follow-on work but should be implemented by shared rule
family, not by isolated date tables.

### Tier 3 - Extended Calendar

Twenty-two records cover extended, regional, seasonal or secondary calendar
items. They remain visible in the roadmap with no promise for the first Festival
Engine release.

### Tier 4 - Lineage Profile

Six records reserve explicit capacity for Nityananda Vamsa and solar/profile
policies. They must never alter the universal calendar unless the user selects an
approved profile.

## Non-Goals

```text
- no hardcoded annual dates
- no rule inferred from a published calendar
- no automatic inclusion of every institutional commemoration
- no universalization of lineage-specific policy
- no changes to the certified Ekadasi or Mahadvadasi resolver
- no closure of the current bibliographic gates
```

## Recommendation

Proceed with the Festival Research Program in family order. Do not schedule
Festival Engine v1.0 implementation until the Tier 1 rule gates have passed.

```text
PROPOSED CORE: 10
RESEARCH-FIRST: REQUIRED
IMPLEMENTATION AUTHORIZED BY THIS DOCUMENT: NO
CURRENT PANCANGA v1.0 FREEZE: PRESERVED
```
