# Campaign 51.0 - Native Vaiṣṇava Māsa Implementation

Priority:

```text
HIGH
```

Status:

```text
PASS
```

## Objective

Implement native Vaiṣṇava Māsa calculation in the Calendar Engine using the
provisional specification incorporated in `KB-CAL-001` and `KB-CAL-002`.

This campaign removes the RC1 temporary māsa adapter. It does not alter the
astronomical algorithms, HBV-EK rules, or normative Knowledge Base.

## Implemented Rule

The implementation follows the currently approved provisional rule:

```text
pūrṇimānta lunar month
↓
māsa by saṅkrānti inside the lunar month
↓
Adhika Māsa = 0 saṅkrāntis
↓
Kṣaya Māsa = 2 saṅkrāntis
```

## Public Calendar Engine API

The Calendar Engine now exposes:

```rust
vaishnava_masa_at(jd)
vaishnava_masa_at_with_config(jd, config)

VaishnavaMasaInfo {
    masa,
    masa_type,
    sankranti_count,
    month_start,
    month_end,
    ayanamsa_status,
}
```

The `VaishnavaMasa` type now belongs to the Calendar Engine and is re-exported
through the Vaiṣṇava layer for compatibility with existing observance APIs.

## Provisional Boundaries

The following items remain explicitly pending certification:

```text
exact ayanāṁśa / zodiac frame
Ekādaśī identity behavior in Kṣaya Māsa
exact bibliographic source for the technical rule
```

The implementation isolates the provisional ayanāṁśa in
`VaishnavaMasaConfig`. If Kṣaya Māsa is detected, the Calendar Engine returns
an explicit `KsayaMasaIdentityPending` error instead of inventing an
observance identity.

## RC1 Integration

The RC1 Experience no longer contains a civil-month māsa adapter.

Previous temporary flow:

```text
RC1 date
↓
civil-month adapter
↓
Observance Engine
```

Current flow:

```text
Calendar Engine
↓
native Vaiṣṇava Māsa
↓
Observance Engine
↓
RC1 presentation
```

The RC1 now resolves ordinary Ekādaśī names from:

```text
native Vaiṣṇava Māsa
+
Pakṣa
```

## Verification Cases

Campaign 51.0 verified:

```text
2026-07-24
Āṣāḍha + Śukla
→ Śayanā Ekādaśī

2026-08-08
Śrāvaṇa + Kṛṣṇa
→ Kāmikā Ekādaśī
```

Both cases are resolved by `source = masa_paksha`.

## Result

```text
Native Vaiṣṇava Māsa:
PASS

Temporary RC1 māsa adapter:
REMOVED

Observance Engine:
uses Calendar Engine māsa

Astronomy Engine:
SIN CAMBIOS

HBV-EK rules:
SIN CAMBIOS

Knowledge Base normative layer:
SIN CAMBIOS
```

