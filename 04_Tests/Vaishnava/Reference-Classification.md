# Reference Classification

Campaign: 54.1

Status: PASS

## Objective

Define the official taxonomy of sources used by Pancanga Engine and enforce
content provenance as part of the Observance Library contract.

## Deliverables

Created:

```text
05_Documentation/Knowledge-Base/KB-REF-001-Reference-Classification.md
```

Updated:

```text
08_Examples/RC1-Experience/content/schema/observance.schema.json
08_Examples/RC1-Experience/content/ekadasi/*.json
```

## Reference Taxonomy

KB-REF-001 defines:

```text
Level A
Canonical Sources

Level B
Ācārya Commentaries

Level C
Operational Calendar Documents

Level D
Technical Sources

Level E
Editorial Sources
```

It also defines a permission matrix for:

```text
History
Benefits
Procedure
Parāṇa
Astronomy
Editorial presentation
```

## Provenance Contract

The Observance Library schema now requires:

```text
provenance.primary_sources
provenance.secondary_sources
provenance.operational_sources
provenance.technical_sources
provenance.editorial_sources
provenance.reviewed_by
provenance.approved_by
provenance.verification_date
```

This separates:

```text
references
visible bibliography or evidence

provenance
internal trace of how the file was built, reviewed, and verified
```

## Library Migration

All 32 Observance Library files now include `provenance`.

For entries with approved RC1 content:

```text
sayana
pavitropana
kamika
```

the provenance records the local primary source summary and the Campaign 53.1
canonical migration.

For placeholder entries, the provenance records:

```text
Campaign 52.5
missing-content identification

Campaign 53.1
schema-valid placeholder generation

Festival Catalog
identity and classification metadata
```

No placeholder provenance claims doctrinal or historical authority.

## Verification

```text
All JSON parse:
PASS

Observance files:
32

Missing files:
0

Provenance present:
32 / 32

Required provenance fields:
PASS

git diff --check:
PASS
```

## Future Work

KB-REF-001 is the policy. The living source registry remains future work:

```text
KB-REF-002
Canonical Source Registry
```

## Certification Result

```text
Campaign 54.1

KB-REF-001:
PASS

Reference taxonomy:
PASS

Permission matrix:
PASS

Provenance schema:
PASS

Observance provenance:
32 / 32

Content added:
0

Invented doctrine:
0

Motor:
SIN CAMBIOS

Knowledge Base normativa:
SIN CAMBIOS

Architecture:
SIN CAMBIOS
```
