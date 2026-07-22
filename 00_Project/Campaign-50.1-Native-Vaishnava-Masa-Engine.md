# Campaign 50.1 - Native Vaiṣṇava Māsa Engine

Priority: HIGH.

Status:

```text
STOP

Implementation blocked by missing local specification.
```

## Objective

Implement native Vaiṣṇava Māsa calculation inside the Calendar Engine so the
Observance Engine can identify ordinary Ekādaśī observances without the RC1
temporary māsa adapter.

## Local Source Audit

Checked local project sources only.

Relevant findings:

```text
05_Documentation/Book/Capitulo-12-Masa-Gaurabda/README.md
```

Status:

```text
Contenido pendiente.
```

The local Hari-bhakti-vilāsa Vilāsa 13 appendix explains the general relation
between lunar months, solar months, and adhika-māsa, but it does not provide a
complete implementable project rule for:

```text
Vaiṣṇava Māsa naming
Adhika Māsa detection
Kṣaya Māsa detection
month-boundary ownership
Ekādaśī naming in intercalary months
```

## Existing Engine State

The engine already has:

```text
Solar apparent longitude
Lunar apparent longitude
Lunar-solar elongation
Tithi
Pakṣa
Sunrise
Observance identity resolver
```

The engine does not yet have:

```text
Native Vaiṣṇava Māsa
Adhika Māsa classification
Kṣaya Māsa classification
Formal māsa boundary rule
```

## Missing Rules

Implementation cannot proceed without a local project specification defining:

1. Which lunar-month system Pancanga Engine adopts for Vaiṣṇava Māsa.

2. How māsa is assigned from solar longitude, lunar phase, and month boundary.

3. Whether the month is named by the saṅkrānti occurring inside the lunar month
   or by another adopted rule.

4. How to detect and name Adhika Māsa.

5. How to detect and name Kṣaya Māsa, if supported.

6. How ordinary Ekādaśī names behave inside Adhika Māsa.

7. What the Calendar Engine API must expose:

```rust
VaishnavaMasa {
    masa,
    is_adhika,
    is_ksaya,
}
```

or an equivalent structure.

8. Minimum oracle cases for validation, including:

```text
normal māsa
adhika māsa
kṣaya māsa, if supported
ordinary Ekādaśī naming
Ekādaśī naming in adhika māsa
```

## Required Next Step

Before implementation, create a normative and technical specification:

```text
KB-CAL-001 / Calendar Engine - Vaiṣṇava Māsa
```

or an equivalent project document.

That document must include:

```text
source hierarchy
normative rule
logical rule
astronomical inputs
calendar outputs
edge cases
test cases
oracle references
implementation boundary
```

## Decision

No code was implemented for Native Vaiṣṇava Māsa in Campaign 50.1.

Reason:

```text
The repository does not yet contain an implementable local specification for
Vaiṣṇava Māsa, Adhika Māsa, or Kṣaya Māsa.
```

Methodological result:

```text
PASS

The campaign stopped before inventing undocumented calendar logic.
```

## Current Safe State

Campaign 50.0A remains valid:

```text
Observance identity resolver:
PASS

Ordinary Ekādaśī:
resolved from formal Vaiṣṇava Māsa + Pakṣa

Mahādvādaśī:
resolved from HBV-EK-004 rule output
```

Campaign 50.1 remains blocked until Native Vaiṣṇava Māsa is specified.

