# KB-CAL-001 - Vaiṣṇava Māsa Specification

Status:

```text
SPECIFICATION DRAFT

Implementation readiness:
BLOCKED
```

Campaign:

```text
50.2 - Vaiṣṇava Māsa Specification
```

## Purpose

This document defines the implementation boundary for native Vaiṣṇava Māsa
calculation in Pancanga Engine.

The goal is to make the Calendar Engine the single source of truth for:

```text
Vaiṣṇava Māsa
Adhika Māsa
Kṣaya Māsa, if supported
Ekādaśī māsa ownership
```

This document does not implement code.

## Authority Decision

The project source hierarchy remains:

```text
Hari-bhakti-vilāsa mūla
↓
Digdarśinī-ṭīkā
↓
Navadvīpa Pañjikā
↓
Tradition Notes
↓
Purāṇas cited by Hari-bhakti-vilāsa as context
```

For Ekādaśī observance rules, this hierarchy is sufficient and already frozen
in KB-VAI-002.

For native Vaiṣṇava Māsa calculation, the repository does not yet contain an
implementable local specification. The local Hari-bhakti-vilāsa material
explains the general relation between lunar months, solar months, tithi, and
adhika-māsa, but it does not by itself define a complete project rule for:

```text
month-boundary ownership
saṅkrānti ownership
sidereal solar sign calculation
Adhika Māsa detection
Kṣaya Māsa detection
Ekādaśī naming in intercalary months
```

Therefore Pancanga Engine cannot implement native Vaiṣṇava Māsa until the
missing rules are supplied by local project sources or an explicit documented
authority decision.

## What Is Already Established

The engine already has the astronomical and calendrical primitives needed by a
future implementation:

```text
solar longitude
lunar longitude
lunar-solar elongation
tithi
pakṣa
sunrise
Julian day
observance disposition
Mahādvādaśī classification
```

The Observance Engine already accepts a formal `VaiṣṇavaMasa + Paksha` pair and
resolves the ordinary Ekādaśī identity from that pair.

If a valid ordinary Ekādaśī observance does not have a formal māsa, the engine
now fails explicitly instead of inventing an identity.

## Required Data Model

A future Calendar Engine API should expose a structure equivalent to:

```rust
VaishnavaMasa {
    masa,
    is_adhika,
    is_ksaya,
    source,
}
```

Minimum fields:

```text
masa
The named lunar month used by the Vaiṣṇava calendar.

is_adhika
Whether the lunar month is intercalary.

is_ksaya
Whether the month is affected by kṣaya, if the project supports kṣaya.

source
Why the engine assigned the month:
normal_masa
adhika_masa
ksaya_masa
```

The final Rust names may differ, but the semantic fields must remain explicit.

## Determination Of Māsa

Implementation requires a documented rule for:

```text
1. the lunar month boundary;
2. the solar sign or solar month relation used for naming;
3. the event that assigns a lunar month its name;
4. the moment at which that assignment becomes operative for civil-day
   calculations;
5. how the observance day inherits the māsa.
```

Local source status:

```text
General lunar-month explanation:
AVAILABLE

Operational month-boundary rule:
MISSING

Operational month-naming rule:
MISSING
```

The local source material states the general principle that lunar months are
based on the Moon's relation to the Sun and that an additional lunar month is
periodically inserted to reconcile the lunar and solar years. This is
insufficient for implementation because it does not specify the exact rule
Pancanga Engine must use to assign a Vaiṣṇava māsa name to a calculated
observance.

## Adhika Māsa

Implementation requires a documented rule for:

```text
1. when Adhika Māsa appears;
2. which astronomical event or missing event identifies it;
3. how the intercalary month is named;
4. how ordinary Ekādaśī identities behave inside Adhika Māsa;
5. whether Adhika Māsa Ekādaśīs use separate observance identities.
```

Local source status:

```text
General existence of adhika-māsa:
AVAILABLE

Detection rule:
MISSING

Naming rule:
MISSING

Ekādaśī identity rule inside adhika-māsa:
MISSING
```

No implementation may infer Adhika Māsa from date lists, fixtures, or a
chronological Ekādaśī sequence.

## Kṣaya Māsa

Implementation requires a documented project decision for:

```text
1. whether v1/v2 Calendar Engine supports Kṣaya Māsa;
2. when Kṣaya Māsa can occur;
3. how it is detected;
4. how it affects the sequence and naming of months;
5. how it affects Ekādaśī identity.
```

Local source status:

```text
Kṣaya Māsa support decision:
MISSING

Detection rule:
MISSING

Observance impact rule:
MISSING
```

If the project chooses not to support Kṣaya Māsa in the next implementation
cycle, that limitation must be explicit in the API and documentation.

## Relation With Ekādaśī

The future rule must define how an Ekādaśī belongs to a Vaiṣṇava Māsa.

Required decision:

```text
Does the Ekādaśī inherit the māsa of:

the civil day of observance;
the tithi at sunrise;
the lunar month containing that tithi;
the shifted Dvādaśī observance day;
another formally adopted rule.
```

This question is especially important when:

```text
Ekādaśī is viddhā and the observance shifts to Dvādaśī;
Dvādaśī is Mahādvādaśī;
the lunar month boundary is near the observance;
Adhika Māsa is present;
Kṣaya Māsa is present.
```

No implementation may choose one of these ownership rules by convenience.

## Ayanāṁśa And Saṅkrānti Dependency

ARCH-0001 includes ayanāṁśa in the intended calendar scope. Native Vaiṣṇava
Māsa calculation may require sidereal solar signs or saṅkrānti events, depending
on the adopted rule.

Before implementation, the specification must state:

```text
which zodiac frame is used;
which ayanāṁśa is used;
how solar sign ingress is calculated;
whether saṅkrānti is evaluated in local civil time, Julian time, or another
project-defined scale.
```

Without this decision, month naming through solar sign relation is not
implementation-ready.

## Prohibited Shortcuts

The implementation must not use:

```text
hardcoded dates;
year tables;
fixture-derived rules;
published calendar rows as the rule itself;
chronological assumptions such as "the next Ekādaśī is Kāmikā";
RC1 content adapters;
manual exceptions.
```

External calendars may validate the implementation after the rule is specified.
They may not define the rule.

## Implementation Readiness Gate

Campaign 50.1 may resume only when this document or a successor document
contains:

```text
source authority for Vaiṣṇava Māsa;
normal māsa rule;
Adhika Māsa rule;
Kṣaya Māsa decision;
Ekādaśī māsa ownership rule;
ayanāṁśa / saṅkrānti configuration, if required;
data model;
minimum tests;
oracle cases.
```

Until then, the correct engine behavior is:

```text
Missing formal māsa
↓
explicit error
↓
no invented observance identity
```

## Minimum Test Suite For A Future Implementation

The implementation campaign must include at least:

```text
normal māsa identification;
all 12 Vaiṣṇava Māsa names;
both pakṣas in multiple months;
ordinary Ekādaśī naming from māsa + pakṣa;
Adhika Māsa identification;
Ekādaśī identity in Adhika Māsa;
Kṣaya Māsa behavior or explicit unsupported-case error;
boundary cases near lunar month transition;
boundary cases near saṅkrānti, if saṅkrānti is part of the adopted rule;
shifted observance due to viddhā;
Mahādvādaśī observance identity.
```

## Current Decision

Campaign 50.2 does not unlock implementation.

Result:

```text
STOP

Vaiṣṇava Māsa cannot be implemented from the current local specification
without adding undocumented calendar logic.
```

Required next action:

```text
Select and document the normative authority for Vaiṣṇava Māsa calculation.
Then extend KB-CAL-001 with the exact normal, adhika, and kṣaya rules.
```

