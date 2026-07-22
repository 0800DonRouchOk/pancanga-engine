# Campaign 52.0 - Native Observance Resolver

Priority:

```text
HIGH
```

Status:

```text
PASS
```

## Objective

Consolidate a single domain output for observances already supported by
Pancanga Engine.

This campaign does not add festivals, rules, date tables, or exceptions. It
only makes the existing observance identity model explicit and reusable by
RC1, APIs, and future interfaces.

## Scope

Implemented:

```text
typed ObservanceId
unified ObservanceContent output
masa and paksha on ordinary Ekādaśī output
fasting_for as an ObservanceId relationship
RC1 JSON output from the unified structure
```

Not implemented:

```text
new festivals
new doctrinal rules
year-specific date mappings
chronological Ekādaśī sequence logic
free-text fasting_for labels
```

## Domain Output

The Vaiṣṇava Engine now exposes observance identity through one structure:

```rust
ObservanceContent {
    id,
    slug,
    display_name,
    observance_type,
    masa,
    paksha,
    fasting_for,
    source,
}
```

`Observance` remains available as a backwards-compatible public alias for
`ObservanceContent`.

## Stable Identifiers

`id` is no longer raw text. It is now a typed `ObservanceId`.

Examples:

```text
ObservanceId::Ek007
→ EK-007
→ Śayanā Ekādaśī

ObservanceId::Md003
→ MD-003
→ Triṣpṛṣā Mahādvādaśī
```

The string label is produced only at the API/presentation boundary.

## `fasting_for`

`fasting_for` is modeled as:

```rust
Option<ObservanceId>
```

It is not free text.

For ordinary Ekādaśī:

```text
fasting_for = None
```

For Mahādvādaśī:

```text
fasting_for = Some(EK-xxx)
```

only when the related ordinary Ekādaśī can be derived from already supplied
calendar facts. If those facts are not available, the field remains `None`
instead of inventing a relationship.

## Source Of Truth

RC1 no longer owns observance identity logic.

The flow is:

```text
Astronomy Engine
↓
Calendar Engine
↓
Vaiṣṇava Engine
↓
ObservanceContent
↓
RC1 presentation
```

## Verification

The existing tests now verify:

```text
24 ordinary Ekādaśī IDs
8 Mahādvādaśī IDs
typed stable identifiers
masa/pakṣa on ordinary Ekādaśī
Mahādvādaśī precedence
fasting_for relationship when derivable
explicit missing-māsa error when ordinary identity cannot be resolved
```

## Result

```text
Native Observance Resolver:
PASS

Unified ObservanceContent:
PASS

Typed ObservanceId:
PASS

fasting_for as relation:
PASS

New rules:
NO

Date tables:
NO

Motor astronomy:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS
```

