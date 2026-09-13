# Campaign 61.1 Required Changes

Campaign: 61.0 output  
Status: READY FOR PLANNING

## Objective for 61.1

Prepare reusable calendrical infrastructure for the future Festival Engine
without implementing Janmāṣṭamī or any additional festival rule.

## Required Changes

### 1. Pañcāṅga State

Create a read-only domain object that gathers existing calendar facts for a
date, location, and civil configuration.

Must reuse:

```text
solar longitude
lunar longitude
tithi
pakṣa
sunrise
Vaiṣṇava māsa
nakṣatra
```

Must not duplicate:

```text
Astronomy Engine
Calendar Engine
Ekādaśī Resolver
Mahādvādaśī logic
```

### 2. Tithi Interval Utility

Expose a reusable utility for finding tithi boundaries over arbitrary windows,
not only sunrise-to-sunrise.

Required by:

```text
Janmāṣṭamī
Nṛsiṁha Caturdaśī
Rāma Navamī
Gaura Pūrṇimā
```

### 3. Sunset Utility

Expose sunset calculation with the same care and validation standard as sunrise.

Required by:

```text
festivals with evening/dusk/sunset observance windows
```

### 4. Civil Ritual Windows

Represent:

```text
sunrise
noon
sunset
midnight
next sunrise
```

The model must be timezone-aware and must handle windows crossing midnight.

### 5. Nakṣatra Interval Utility

Do not implement until a documented festival rule requires it. If required,
derive it from the existing lunar longitude and nakṣatra index pipeline.

### 6. Festival Rule Status

Add status modeling so a future Festival Engine can return:

```text
IMPLEMENTED
RESEARCH_REQUIRED
RULE_CONFLICT
NOT_SUPPORTED
```

without pretending that unresolved festivals are implemented.

## Explicit Non-Goals for 61.1

```text
No Janmāṣṭamī calculation
No new festival resolver
No new festival content
No Ekādaśī migration
No Knowledge Base reinterpretation
No date fixtures as logic
```

## Campaign 61.1 Success Criteria

```text
Pañcāṅga State:
PASS

Tithi interval utility:
PASS

Sunset utility:
PASS

Civil ritual windows:
PASS

Ekādaśī Engine:
UNCHANGED

Astronomy Engine:
UNCHANGED

Festival calculations added:
0
```
