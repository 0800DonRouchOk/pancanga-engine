# Campaign 61.0: Vaiṣṇava Festival Engine Foundation

Status: FOUNDATION PASS

## Objective

Create the first foundation layer for a future Vaiṣṇava Festival Engine without
expanding the certified calendrical scope.

Campaign 61.0 does not add new festivals, new rules, fixed dates, annual tables,
or external calendar assumptions. It only creates a unified festival-domain
output for observances already resolved by Pancanga Engine.

## Scope Boundary

No changes are made to:

```text
Astronomy Engine
Calendar Engine
Native Vaiṣṇava Māsa
Ekādaśī rules
Mahādvādaśī rules
Parāṇa logic
Knowledge Base
Observance JSON content
Schemas
```

## Architecture

Before:

```text
Calendar Engine
        ↓
Observance Resolver
        ↓
ObservanceContent
        ↓
RC1 / API / Library
```

After:

```text
Calendar Engine
        ↓
Observance Resolver
        ↓
ObservanceContent
        ↓
Festival Engine foundation
        ↓
Festival
        ↓
RC1 / API / Library / future interfaces
```

## Implemented

Created:

```text
03_Source/rust/crates/pancanga-engine/src/festival/mod.rs
```

The module introduces:

```text
Festival
FestivalId
FestivalDomain
FestivalKind
FestivalSource
festival_from_observance()
```

In Campaign 61.0, the only supported domain is:

```text
Ekādaśī
```

covering the already implemented:

```text
24 ordinary Ekādaśīs
8 Mahādvādaśīs
```

## Design Decision

The Festival Engine foundation wraps certified observance output instead of
recalculating it.

This preserves the Single Source of Truth:

```text
Observance Resolver
source of truth for Ekādaśī / Mahādvādaśī identity

Festival Engine
unified domain surface for festival-like outputs
```

## Explicit Non-Goals

Campaign 61.0 does not implement:

```text
Janmāṣṭamī
Gaura Pūrṇimā
Nṛsiṁha Caturdaśī
Rāma Navamī
Nityānanda Trayodaśī
Appearance / disappearance days
Cāturmāsya
Fixed-date festival tables
```

Those require future documented rules and source-backed specifications.

## Verification

```text
cargo fmt
cargo test -p pancanga-engine festival
cargo check -p pancanga-engine --examples
git diff --check
```

## Result

```text
Campaign 61.0

Vaiṣṇava Festival Engine Foundation:
PASS

New festival rules:
0

Hardcoded dates:
0

Motor semantics changed:
NO

Knowledge Base changed:
NO
```
