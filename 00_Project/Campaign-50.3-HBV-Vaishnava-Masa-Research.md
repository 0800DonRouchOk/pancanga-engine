# Campaign 50.3 - HBV Vaiṣṇava Māsa Research

Priority: HIGH.

Status:

```text
STOP

Normative dependency not available in local repository.
```

## Objective

Find the missing HBV/Gauḍīya rule needed to implement native Vaiṣṇava Māsa.

The campaign asked:

```text
How does the HBV/Gauḍīya tradition define Vaiṣṇava Māsa, Adhika Māsa, and
Kṣaya Māsa for deterministic implementation?
```

This campaign does not implement code.

## Local Research Scope

Checked local project sources only:

```text
Hari-bhakti-vilāsa raw files
Digdarśinī material embedded in local HBV files
Vaiṣṇava source hierarchy
Knowledge Base
Algorithms
GCal / PureBhakti / SCS Math validation documents
RC1 Festival Catalog documents
```

Internet was not used to define a rule.

## Deliverable

Created:

```text
05_Documentation/Knowledge-Base/KB-CAL-002-HBV-Vaishnava-Masa-Rules.md
```

## Findings

### Vaiṣṇava Māsa Name

Result:

```text
NOT FOUND AS IMPLEMENTABLE RULE
```

The local HBV appendix gives general lunar-calendar context but does not define
the operational month-naming rule.

### Adhika Māsa

Result:

```text
GENERAL CONCEPT FOUND
IMPLEMENTABLE RULE NOT FOUND
```

The repository does not define detection, naming, or Ekādaśī identity inside
Adhika Māsa.

### Kṣaya Māsa

Result:

```text
NOT FOUND
```

The repository does not define whether or how Pancanga Engine supports Kṣaya
Māsa.

### GCal Algorithm

Result:

```text
NOT AVAILABLE LOCALLY
NOT NORMATIVE UNDER CURRENT AUTHORITY DECISION
```

GCal remains a validation oracle, not a rule source.

## Decision

Campaign 50.3 confirms the Campaign 50.2 boundary:

```text
The blocker is not Rust.
The blocker is the missing normative/algorithmic authority for māsa.
```

No code was modified.

No Knowledge Base HBV-EK rule was modified.

No Calendar Engine rule was added.

## Required Next Step

Before Campaign 50.1 can resume, the project must choose one path:

```text
1. Incorporate an official HBV/Gauḍīya/Navadvīpa/GCal algorithm source locally.

2. Adopt an explicit project interpretation, with source basis and limits.

3. Keep native Vaiṣṇava Māsa unsupported and preserve explicit MissingMasa
   behavior.
```

Until then:

```text
Native Vaiṣṇava Māsa:
BLOCKED

Observance Engine MissingMasa behavior:
CORRECT
```

