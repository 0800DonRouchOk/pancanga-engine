# Observance Knowledge Base Schema

Campaign: 53.0

Status: PASS

## Objective

Define the canonical data contract for every observance entry in the Festival
Library before generating the missing content files identified in Campaign 52.5.

This is a domain-design campaign. It does not generate the 29 missing JSON
files and does not alter the calculation engine.

## Canonical Schema

Created:

```text
08_Examples/RC1-Experience/content/schema/observance.schema.json
```

The schema is a content contract only. It does not define or modify:

```text
tithi
Viddhā
Mahādvādaśī
Vaiṣṇava Māsa
Hari-vāsara
Parāṇa
any normative rule
```

## Required Top-Level Sections

Every observance entry must contain:

```text
identity
classification
content
references
related_observances
metadata
```

## Required Identity Fields

```text
identity.id
identity.slug
identity.display_name
identity.observance_type
```

Purpose:

```text
id:
stable ObservanceId such as EK-010 or MD-003

slug:
stable content key and filename base

display_name:
default client-facing name

observance_type:
ekadasi, mahadvadasi, or future festival
```

Optional identity fields:

```text
identity.aliases
identity.display
```

These allow alternate names and future localization without changing stable IDs.

## Required Classification Fields

```text
classification.masa
classification.paksha
classification.source
```

Optional classification fields:

```text
classification.masa_type
classification.fasting_for
```

`fasting_for` is a typed ObservanceId reference, not free text.

Example:

```json
{
  "fasting_for": "EK-010"
}
```

This keeps Mahādvādaśī relationships stable across languages and UI formats.

## Required Content Sections

Every entry must include:

```text
content.summary
content.history
content.scriptural_basis
content.observance
content.benefits
content.procedure
content.parana
content.astronomy
content.certification
```

Each text section has:

```text
status
text
points
```

Allowed statuses:

```text
present
not_applicable
pending
```

This means a section can remain structurally present even when it is not
applicable to a given observance. For example, a future non-Ekādaśī festival may
mark `content.parana.status` as `not_applicable`.

## References

`references` is mandatory and must contain at least one item.

Supported reference types:

```text
scripture
commentary
calendar_oracle
local_pdf
project_document
certification_report
```

This preserves the project principle that content remains traceable to local or
explicitly recorded sources.

## Metadata

Required metadata:

```text
metadata.version
metadata.language
metadata.last_updated
```

Optional metadata:

```text
metadata.status
metadata.content_source
metadata.schema_version
```

Recommended default for the v1 content library:

```json
{
  "schema_version": "observance.schema.v1",
  "status": "draft",
  "content_source": "editorial_summary"
}
```

## Mahādvādaśī Reuse

Mahādvādaśī entries use the same schema.

Differences:

```text
identity.observance_type:
mahadvadasi

classification.source:
mahadvadasi_rule

classification.masa:
null when not part of the identity source

classification.paksha:
null when not part of the identity source

classification.fasting_for:
ObservanceId of the Ekādaśī for which the fast is observed, when known
```

This keeps the Festival Library descriptive while leaving calculation and
Mahādvādaśī detection in the Pancanga Engine.

## Future Festival Reuse

The schema reserves:

```text
identity.observance_type = festival
classification.source = festival_rule
```

Future festival entries can use the same contract without forcing RC1 or v1.0 to
implement a Festival Engine now.

## RC1 Compatibility

The current RC1 loads story content by:

```text
observance_content.slug
```

The canonical schema preserves this field:

```text
identity.slug
```

For backward compatibility during migration, a client may map:

```text
old entry id
→ identity.slug

old name
→ identity.display_name

old aliases
→ identity.aliases

old calendar_context.vedic_month
→ classification.masa

old calendar_context.paksha
→ classification.paksha

old story_summary / main_teaching
→ content.history / content.summary

old observance_notes
→ content.observance / content.procedure

old benefits_summary
→ content.benefits

old source
→ references
```

No RC1 calculation behavior needs to change for the schema to exist.

## Example Shape

```json
{
  "identity": {
    "id": "EK-010",
    "slug": "kamika",
    "display_name": "Kāmikā Ekādaśī",
    "observance_type": "ekadasi",
    "aliases": [
      "Kamika Ekadashi",
      "Śrāvaṇa Kṛṣṇa Ekādaśī"
    ]
  },
  "classification": {
    "masa": "Śrāvaṇa",
    "paksha": "Kṛṣṇa",
    "source": "masa_paksha",
    "masa_type": "normal_masa",
    "fasting_for": null
  },
  "content": {
    "summary": {
      "status": "present",
      "text": "...",
      "points": []
    },
    "history": {
      "status": "present",
      "text": "...",
      "points": []
    },
    "scriptural_basis": [
      {
        "source": "Brahma-vaivarta Purāṇa",
        "status": "confirmed",
        "summary": "..."
      }
    ],
    "observance": {
      "status": "present",
      "text": "...",
      "points": []
    },
    "benefits": {
      "status": "present",
      "text": "...",
      "points": []
    },
    "procedure": {
      "status": "present",
      "text": "...",
      "points": []
    },
    "parana": {
      "status": "present",
      "text": "Parāṇa timing is calculated by the engine for each date.",
      "points": []
    },
    "astronomy": {
      "status": "present",
      "text": "Astronomical details are calculated by Pancanga Engine.",
      "points": []
    },
    "certification": {
      "status": "present",
      "text": "Certification data is maintained in project reports.",
      "points": []
    }
  },
  "references": [
    {
      "source": "Brahma-vaivarta Purāṇa",
      "reference_type": "scripture"
    }
  ],
  "related_observances": [],
  "metadata": {
    "version": "1.0.0",
    "language": "es",
    "last_updated": "2026-07-22",
    "status": "draft",
    "content_source": "editorial_summary",
    "schema_version": "observance.schema.v1"
  }
}
```

## Deferred Work

Campaign 53.0 intentionally does not create:

```text
EK-001.json ... EK-024.json
MD-001.json ... MD-008.json
```

The next campaign may populate the library only after this schema is accepted.

## Certification Result

```text
Campaign 53.0

Observance Knowledge Base Schema:
PASS

Canonical schema:
PASS

Mandatory fields:
PASS

Mahādvādaśī reuse:
PASS

ObservanceId binding:
PASS

RC1 compatibility:
PASS

Generated content files:
0

Motor:
SIN CAMBIOS

Knowledge Base normativa:
SIN CAMBIOS

Architecture:
SIN CAMBIOS
```
