# KB-CAL-002 - HBV Vaiṣṇava Māsa Rules

Campaign:

```text
50.3 - HBV Vaiṣṇava Māsa Research
```

Status:

```text
RESEARCH STOP

Implementable rule:
NOT FOUND IN LOCAL REPOSITORY
```

## Purpose

This document records the research result for the missing native Vaiṣṇava Māsa
rule required by the Calendar Engine.

It answers one question:

```text
Does the local repository contain an HBV/Gauḍīya rule precise enough to
implement native Vaiṣṇava Māsa, Adhika Māsa, and Kṣaya Māsa?
```

Current answer:

```text
NO
```

## Method

Local repository sources only.

No internet search was used to define a rule.

Checked source layers:

```text
Hari-bhakti-vilāsa local raw files
Digdarśinī material embedded in those files
Vaiṣṇava source hierarchy documents
Knowledge Base
Algorithm documents
GCal / PureBhakti / SCS Math validation documents
RC1 Festival Catalog documents
```

## Authority Boundary

For Ekādaśī observance rules, the v1.0 hierarchy remains:

```text
Hari-bhakti-vilāsa
↓
Digdarśinī-ṭīkā
↓
Navadvīpa Pañjikā
↓
Tradition Notes
```

For Vaiṣṇava Māsa, the project has not yet incorporated a local source that
turns this hierarchy into an implementable calendar algorithm.

GCal, PureBhakti, and SCS Math are external validation oracles in the current
project state. They do not define normative rules unless a later campaign
explicitly changes the authority decision.

## Research Questions

### 1. How Does HBV Determine The Name Of The Vaiṣṇava Māsa?

Local finding:

```text
NOT IMPLEMENTABLE FROM CURRENT LOCAL SOURCES
```

The local Hari-bhakti-vilāsa material explains that the project calendar uses
lunar months and that lunar months are based on the Moon's relation to the Sun.
It does not provide a complete operational rule for assigning a Vaiṣṇava māsa
name to a calculated observance.

Missing:

```text
lunar month boundary rule
month naming rule
solar-sign relation, if used
saṅkrānti ownership rule, if used
ayanāṁśa / zodiac frame, if used
```

### 2. What Is The Exact Rule For Adhika Māsa?

Local finding:

```text
GENERAL CONCEPT AVAILABLE
IMPLEMENTABLE RULE MISSING
```

The local appendix explains that an extra lunar month is periodically added to
reconcile lunar and solar years. It does not define the project rule for:

```text
detecting Adhika Māsa
naming Adhika Māsa
assigning Ekādaśī identities inside Adhika Māsa
deciding whether intercalary Ekādaśīs receive separate catalog identities
```

No implementation may infer this from external calendar rows.

### 3. What Is The Exact Rule For Kṣaya Māsa?

Local finding:

```text
NOT FOUND
```

The repository does not currently contain a normative or technical rule for:

```text
when Kṣaya Māsa occurs
how it is detected
whether Pancanga Engine supports it
how it affects month naming
how it affects Ekādaśī observance identity
```

If Kṣaya Māsa is not supported in the next implementation cycle, the Calendar
Engine must say so explicitly instead of returning a guessed result.

### 4. How Do Māsa Rules Affect Ekādaśī Identification?

Current engine state:

```text
Ordinary Ekādaśī identity
=
formal VaiṣṇavaMasa + Pakṣa
```

This is correct as an interface contract, but native māsa ownership is still
missing.

The unresolved question is:

```text
Which māsa owns an Ekādaśī when the observance is shifted, near a month
boundary, inside Adhika Māsa, or affected by Kṣaya Māsa?
```

Until this is specified, the engine must continue to fail explicitly when an
ordinary Ekādaśī identity requires māsa and no formal māsa exists.

### 5. Are There Official HBV Documents In The Repository That Describe This Algorithm?

Local finding:

```text
NO
```

Relevant local placeholder:

```text
05_Documentation/Book/Capitulo-12-Masa-Gaurabda/README.md
```

Status:

```text
Contenido pendiente.
```

Relevant local source:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.txt
```

Status:

```text
General explanatory appendix available.
Operational algorithm not available.
```

### 6. Does HBV Adopt The GCal Algorithm?

Local finding:

```text
NOT ESTABLISHED
```

The repository currently defines GCal as:

```text
validation oracle
not normative authority
```

Therefore Pancanga Engine cannot claim that HBV adopts the GCal algorithm unless
the exact GCal/HBV algorithm is incorporated locally and the authority decision
is explicitly updated.

## Required Source To Continue

Campaign 50.3 cannot produce implementable rules until one of the following
exists locally:

```text
1. A primary or commentarial HBV/Gauḍīya source that defines the māsa algorithm.

2. A Navadvīpa Pañjikā or tradition document that explicitly states the
   adopted Gauḍīya māsa rule and its relation to HBV.

3. Official GCal / Gaurābda Calendar algorithm documentation, if the project
   decides to adopt it formally as the technical implementation of the
   HBV/Gauḍīya calendar tradition.

4. A project-authored normative specification that openly states it is an
   adopted interpretation, with source basis and limits.
```

## Methodological Options

### Option A - Incorporate An Existing Official Algorithm

Use an official Gauḍīya / GCal / Navadvīpa Pañjikā algorithm as the technical
rule.

Requirement:

```text
The source must be local.
The authority decision must be updated.
The rule must be written as deterministic implementation logic.
```

### Option B - Write A Project Interpretation

Create a Pancanga Engine māsa specification from the adopted source hierarchy.

Requirement:

```text
Every decision must be documented.
Every inferred step must be marked as project interpretation.
External oracles may validate but not define the rule.
```

### Option C - Keep Native Māsa Unsupported

Leave native Vaiṣṇava Māsa outside the next implementation cycle.

Requirement:

```text
The engine must return explicit MissingMasa / UnsupportedMasa errors.
The RC1 temporary adapter remains presentation-only and cannot become normative.
```

## Current Decision

No rule was added.

No implementation was unlocked.

No motor behavior changed.

Result:

```text
STOP

The missing dependency is normative, not technical.
```

Next required campaign:

```text
Select and incorporate the normative/algorithmic source for Vaiṣṇava Māsa.
Then revise KB-CAL-001 and KB-CAL-002 into an implementable specification.
```

