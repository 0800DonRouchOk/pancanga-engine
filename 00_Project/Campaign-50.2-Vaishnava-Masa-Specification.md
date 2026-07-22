# Campaign 50.2 - Vaiṣṇava Māsa Specification

Priority: HIGH.

Status:

```text
STOP

Specification boundary documented.
Implementation still blocked by missing normative rule.
```

## Objective

Create a technical and verifiable specification for native Vaiṣṇava Māsa
calculation so the Calendar Engine can later provide formal māsa data to the
Observance Engine.

This campaign does not implement code.

## Deliverable

Created:

```text
05_Documentation/Knowledge-Base/KB-CAL-001-Vaishnava-Masa-Specification.md
```

## Authority Answer

The project has already adopted the HBV/Gauḍīya source hierarchy for Ekādaśī
observance rules:

```text
Hari-bhakti-vilāsa mūla
↓
Digdarśinī-ṭīkā
↓
Navadvīpa Pañjikā
↓
Tradition Notes
```

However, the repository does not yet contain an implementable local rule that
uses that hierarchy to define native Vaiṣṇava Māsa.

Therefore the authority question remains open for this specific calendar
component:

```text
What exact normative source and rule will Pancanga Engine follow for
Vaiṣṇava Māsa, Adhika Māsa, and Kṣaya Māsa?
```

## Local Source Findings

Checked local project files only.

Relevant local source:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.txt
```

The appendix explains the general relation between lunar months, solar months,
tithi, and adhika-māsa. It does not provide the full operational rule needed by
the engine.

Relevant project placeholder:

```text
05_Documentation/Book/Capitulo-12-Masa-Gaurabda/README.md
```

Status:

```text
Contenido pendiente.
```

## Rules Still Missing

Implementation remains blocked until the project documents:

```text
lunar month boundary rule
month naming rule
saṅkrānti ownership rule, if applicable
ayanāṁśa / zodiac frame, if applicable
Adhika Māsa detection
Adhika Māsa naming
Ekādaśī identity inside Adhika Māsa
Kṣaya Māsa support decision
Kṣaya Māsa detection, if supported
Ekādaśī māsa ownership when observance shifts to Dvādaśī
```

## Decision

No code was implemented.

No Calendar Engine behavior changed.

No Vaiṣṇava Engine behavior changed.

No Knowledge Base HBV-EK rule was modified.

Methodological result:

```text
PASS

The project did not invent undocumented māsa logic.
```

Implementation result:

```text
BLOCKED

Campaign 50.1 remains stopped until KB-CAL-001 becomes implementable.
```

