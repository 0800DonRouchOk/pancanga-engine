# KB-CAL-001 - Vaiṣṇava Māsa Specification

Status:

```text
PROVISIONAL SPECIFICATION

Implementation readiness:
IMPLEMENTED PROVISIONALLY

Certification readiness:
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

Campaign 51.0 implemented this provisional specification in the Calendar
Engine. Final certification remains blocked by the pending items listed below.

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

For native Vaiṣṇava Māsa calculation, Campaign 50.4 incorporated a provisional
implementation hypothesis into the local Knowledge Base. The local
Hari-bhakti-vilāsa material explains the general relation between lunar months,
solar months, tithi, and adhika-māsa, but it does not by itself define every
technical detail needed for final certification.

The following items remain pending validation:

```text
exact bibliographic source for the algorithmic hypothesis
exact ayanāṁśa / zodiac frame
Ekādaśī identity behavior in Kṣaya Māsa
```

Therefore Pancanga Engine may implement native Vaiṣṇava Māsa only as a
provisional, validation-pending feature until those items are closed.

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

Campaign 51.0 update:

```text
Calendar Engine now exposes native VaishnavaMasaInfo.
The RC1 temporary civil-month adapter has been removed.
```

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

Campaign 50.4 records the following provisional rule:

```text
1. Use a pūrṇimānta lunar month.
2. Determine the māsa name from the saṅkrānti occurring inside that lunar month.
3. A normal lunar month contains exactly 1 saṅkrānti.
4. Adhika Māsa is a lunar month containing 0 saṅkrāntis.
5. Kṣaya Māsa is a lunar month containing 2 saṅkrāntis.
```

Evidence status:

```text
General lunar-month explanation:
AVAILABLE

Pūrṇimānta month system:
PROVISIONAL - pending exact source citation

Māsa by saṅkrānti:
PROVISIONAL - pending exact source citation

Saṅkrānti count classification:
PROVISIONAL - pending validation
```

The local source material states the general principle that lunar months are
based on the Moon's relation to the Sun and that an additional lunar month is
periodically inserted to reconcile the lunar and solar years. Campaign 50.4
adds the provisional technical rule above so implementation work can proceed
without relying on conversation context.

## Adhika Māsa

Campaign 50.4 records the following provisional rule:

```text
Adhika Māsa
=
pūrṇimānta lunar month containing 0 saṅkrāntis
```

Still pending:

```text
exact naming convention for the intercalary month;
complete ordinary/ad hoc Ekādaśī identity behavior inside Adhika Māsa;
validation against adopted Gauḍīya/GCal/Navadvīpa examples.
```

No implementation may infer Adhika Māsa from date lists, fixtures, or a
chronological Ekādaśī sequence.

## Kṣaya Māsa

Campaign 50.4 records the following provisional detection rule:

```text
Kṣaya Māsa
=
pūrṇimānta lunar month containing 2 saṅkrāntis
```

Still pending:

```text
whether the provisional implementation supports Kṣaya Māsa fully;
how Kṣaya Māsa affects month naming;
how Kṣaya Māsa affects Ekādaśī observance identity.
```

If the project chooses not to support Kṣaya Māsa observance identity in the
next implementation cycle, that limitation must be explicit in the API and
documentation.

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
certification-ready. A provisional implementation may proceed only if the
chosen ayanāṁśa / zodiac frame is explicitly marked pending validation and kept
configurable or isolated.

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

Campaign 50.1 may resume as a provisional implementation only with:

```text
pūrṇimānta lunar month boundary;
māsa by saṅkrānti;
Adhika Māsa = 0 saṅkrāntis;
Kṣaya Māsa = 2 saṅkrāntis;
Ekādaśī māsa ownership rule;
ayanāṁśa / saṅkrānti configuration marked pending validation;
data model;
minimum tests;
oracle cases for normal and adhika behavior.
```

If any required fact cannot be determined, the correct engine behavior remains:

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

## Campaign 50.4 Provisional Decision

Campaign 50.4 updates the Campaign 50.2 / 50.3 STOP state.

Result:

```text
PROVISIONAL IMPLEMENTATION MAY PROCEED

Native Vaiṣṇava Māsa may be implemented as validation-pending logic based on
the provisional pūrṇimānta / saṅkrānti-count hypothesis.
```

Still not final:

```text
Exact ayanāṁśa:
PENDING VALIDATION

Kṣaya Māsa Ekādaśī identity:
PENDING VALIDATION

Bibliographic source:
PENDING LOCAL CITATION
```
