# Festival Rule Taxonomy

Campaign: 61.0  
Status: INITIAL TAXONOMY - SUPERSEDED FOR COVERAGE BY CAMPAIGN 62.0  
Scope: Required rule classes observed from current festival candidates

> Historical note: Campaign 62.0 expands this seed taxonomy without approving
> new rules. See
> [Festival-Calculation-Family-Map.md](Festival-Calculation-Family-Map.md).

## Purpose

This document names the initial rule classes a future Festival Engine is likely
to need. It does not implement those rules and does not decide disputed cases.

Names are provisional until validated by implementation and certification.

## Rule Classes

### TITHI_AT_SUNRISE

The festival is selected according to whether the nominal tithi is present at
local sunrise.

Existing reusable inputs:

```text
sunrise
tithi_at_sunrise
pakṣa
māsa
```

Known candidates:

```text
Rādhāṣṭamī
Govardhana Pūjā
Ratha-yātrā
```

Status:

```text
RESEARCH_REQUIRED
```

### TITHI_PRESENT_DURING_WINDOW

The festival depends on whether the nominal tithi is present during a ritual
window such as midnight, noon, sunset, or another interval.

Existing reusable inputs:

```text
instantaneous tithi
solar longitude
lunar longitude
```

Missing reusable inputs:

```text
ritual window boundaries
arbitrary tithi start/end
timezone-aware civil windows
```

Known candidates:

```text
Janmāṣṭamī
Gaura Pūrṇimā
Nṛsiṁha Caturdaśī
Rāma Navamī
Balarāma Pūrṇimā
```

Status:

```text
RESEARCH_REQUIRED
```

### TITHI_NAKSHATRA_COMBINATION

The festival depends on a combination of tithi and nakṣatra.

Existing reusable inputs:

```text
instantaneous nakṣatra
instantaneous tithi
```

Missing reusable inputs:

```text
nakṣatra start/end
overlap detection between tithi and nakṣatra windows
```

Known candidates:

```text
Vāmana Dvādaśī / Śravaṇa Dvādaśī
```

Status:

```text
RESEARCH_REQUIRED
```

### TITHI_RELATIVE_EVENT

The festival is linked to another calendrical event, or a secondary observance
is generated relative to a primary one.

Known candidates:

```text
Nandotsava after Janmāṣṭamī
Ratha-yātrā return festival
Break-fast windows after festival fasts
```

Status:

```text
RESEARCH_REQUIRED
```

### FAST_UNTIL_RITUAL_EVENT

The fasting requirement extends until a ritual boundary such as noon, sunset, or
midnight.

Known candidates:

```text
Janmāṣṭamī: operational source records fast until midnight
Balarāma Pūrṇimā: operational source records fast until noon
```

Status:

```text
RESEARCH_REQUIRED
```

### SPECIAL_VAISNAVA_RULE

The festival follows a Vaiṣṇava-specific rule that cannot be represented by
ordinary tithi selection alone.

Known candidates:

```text
Janmāṣṭamī
Rāma Navamī
Nṛsiṁha Caturdaśī
Vāmana Dvādaśī
```

Status:

```text
RESEARCH_REQUIRED
```

## Non-Goals

This taxonomy does not:

```text
Implement festival resolution
Choose between conflicting traditions
Introduce new Knowledge Base rules
Replace the Ekādaśī resolver
```

## Result

```text
Rule taxonomy:
PASS

Rules implemented:
0

Invented doctrine:
0
```
