# Campaign 50.4 - Provisional Vaiṣṇava Māsa Hypothesis Incorporation

Priority: HIGH.

Status:

```text
PASS

Provisional specification incorporated.
```

## Objective

Transform the current Vaiṣṇava Māsa hypothesis into local project
documentation so future implementation does not depend on external
conversation context.

This campaign does not implement code.

## Scope

Updated:

```text
KB-CAL-001
KB-CAL-002
Roadmap
Project status
Knowledge Base index
```

Not modified:

```text
Rust
Calendar Engine
Observance Engine
Astronomy Engine
HBV-EK Knowledge Base
tests
```

## Incorporated Hypothesis

The project now records the following provisional implementation hypothesis:

```text
Lunar month system:
pūrṇimānta

Māsa name:
determined by the saṅkrānti occurring inside the lunar month

Adhika Māsa:
lunar month with 0 saṅkrāntis

Kṣaya Māsa:
lunar month with 2 saṅkrāntis
```

## Evidence Classification

```text
Confirmed:
The repository already requires a native māsa source of truth in the Calendar
Engine and prohibits date tables, chronological inference, and RC1 adapters.

Derived from technical hypothesis:
pūrṇimānta lunar month, māsa by saṅkrānti, Adhika = 0 saṅkrāntis,
Kṣaya = 2 saṅkrāntis.

Pending validation:
exact ayanāṁśa / zodiac frame used by the adopted reference;
definitive Ekādaśī identity handling inside Kṣaya Māsa;
exact bibliographic source for the technical hypothesis.
```

## Bibliographic References

Exact bibliographic references for the provisional technical hypothesis remain
pending.

This campaign incorporates the hypothesis into the local corpus so it can be
reviewed, implemented provisionally, and validated without relying on external
conversation context. It does not claim final bibliographic closure.

## Implementation Consequence

Campaign 51.0 may be opened only as:

```text
Native Vaiṣṇava Māsa Implementation - Provisional
```

The implementation must:

```text
make ayanāṁśa configurable or explicitly marked pending validation;
return explicit unsupported/error behavior for unresolved Kṣaya Ekādaśī cases;
avoid hardcoded dates, year tables, and chronological Ekādaśī sequences;
preserve MissingMasa behavior where formal māsa cannot be determined;
record validation gaps in tests and documentation.
```

## Certification Consequence

This campaign does not certify native Vaiṣṇava Māsa.

The provisional implementation can be developed and tested, but it cannot be
declared final until the pending validation items are closed.

## Required Evidence To Close Provisional Status

At least one of the following must be incorporated locally:

```text
official Gauḍīya / GCal / Navadvīpa Pañjikā algorithm documentation;
primary or commentarial source establishing the adopted māsa rule;
project-authored normative decision explicitly adopting the technical
hypothesis as Pancanga Engine's interpretation, with limitations;
oracle cases proving normal, adhika, and kṣaya behavior under the selected
configuration.
```

## Decision

Campaign 50.4 changes the state from:

```text
No implementable rule available
```

to:

```text
Provisional implementable hypothesis available
```

It does not change the motor.
