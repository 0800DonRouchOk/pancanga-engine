# Festival Catalog Schema

Status: RC1 schema candidate

This schema is a content model, not a calculation model.

It does not define tithi, Viddhā, Mahādvādaśī, Hari-vāsara, Parāṇa, or any
normative rule. The Pancanga Engine calculates the observance first. The
catalog only identifies the content attached to that observance.

## Identity

Each festival entry has two identifiers:

- `id`: stable catalog identifier. This should not change after publication.
- `slug`: technical content key. This maps to local story files such as
  `kamika.json`.

Examples:

```json
{
  "id": "EK-010",
  "slug": "kamika",
  "type": "ekadasi",
  "display_name": "Kāmikā Ekādaśī",
  "source": "masa_paksha"
}
```

```json
{
  "id": "MD-003",
  "slug": "trisprsa",
  "type": "mahadvadasi",
  "display_name": "Triṣpṛṣā Mahādvādaśī",
  "source": "mahadvadasi_rule"
}
```

## Source

`source` explains why the catalog selected the entry.

- `masa_paksha`: ordinary Ekādaśī identified by lunar month and pakṣa.
- `mahadvadasi_rule`: Mahādvādaśī identified by the HBV-EK-004 rule already
  calculated by the engine.

The source field is diagnostic. It does not authorize or modify the rule.

## Display

`display_name` remains for RC1 compatibility.

`display` prepares future localization:

```json
{
  "sa": "Kāmikā Ekādaśī",
  "es": "Kāmikā Ekādaśī",
  "en": "Kāmikā Ekadashi"
}
```

The stable `id` must not change when translations change.

## RC1 Contract

The RC1 calculation response exposes:

```json
{
  "observance_content": {
    "type": "ekadasi",
    "id": "EK-010",
    "slug": "kamika",
    "display_name": "Kāmikā Ekādaśī",
    "source": "masa_paksha",
    "masa": "Śrāvaṇa",
    "paksha": "Kṛṣṇa"
  }
}
```

The frontend loads story content by `slug`, not by stable `id`.

