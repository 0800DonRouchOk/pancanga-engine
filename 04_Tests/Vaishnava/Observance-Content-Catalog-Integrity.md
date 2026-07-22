# Observance Content Catalog Integrity Certification

Campaign: 52.5

Mode: CERTIFICATION ONLY

Status: OPEN

## Objective

Certify whether every `ObservanceId` and `slug` that the Native Observance
Resolver can produce has a corresponding RC1 historical content file.

This campaign does not correct missing content. It only identifies references
that are currently unresolved by the RC1 content layer.

## Methodological Boundary

No changes were made to:

```text
Calendar Engine
Observance Engine
Native Māsa
Knowledge Base
Festival Catalog
Rules
UI
```

## Test Case Trigger

The issue was observed from the RC1 Experience when a resolved observance tried
to load:

```text
/content/ekadasi/aja.json
```

The local server returned:

```text
404 Not Found
```

The browser then attempted to parse `Not found` as JSON, producing:

```text
Unexpected token 'N', "Not found" is not valid JSON
```

This is a content availability failure, not evidence of an astronomy,
calendar, or observance resolver bug.

## Resolver Slug Coverage

The resolver and catalog currently expose:

```text
Ordinary Ekādaśīs:
24

Mahādvādaśīs:
8

Total observance content references:
32
```

The RC1 content directory currently contains:

```text
sayana.json
kamika.json
pavitropana.json
```

## Integrity Summary

```text
Cataloged observance references:
32

Content files present:
3

Content files missing:
29

Orphan content files:
0

ENGINE BUG:
0
```

## Ordinary Ekādaśīs

| ID | Slug | Display name | Content file |
|---|---|---|---|
| EK-001 | kamada | Kāmadā Ekādaśī | MISSING |
| EK-002 | papamocani | Pāpamocanī Ekādaśī | MISSING |
| EK-003 | mohini | Mohinī Ekādaśī | MISSING |
| EK-004 | varuthini | Varūthinī Ekādaśī | MISSING |
| EK-005 | nirjala | Nirjalā Ekādaśī | MISSING |
| EK-006 | apara | Aparā Ekādaśī | MISSING |
| EK-007 | sayana | Śayanā Ekādaśī | OK |
| EK-008 | yogini | Yoginī Ekādaśī | MISSING |
| EK-009 | pavitropana | Putradā - Pavitraropani Ekādaśī | OK |
| EK-010 | kamika | Kāmikā Ekādaśī | OK |
| EK-011 | parsva | Pārśva Ekādaśī | MISSING |
| EK-012 | aja | Ajā Ekādaśī | MISSING |
| EK-013 | pasankusa | Pāśāṅkuśā Ekādaśī | MISSING |
| EK-014 | indira | Indirā Ekādaśī | MISSING |
| EK-015 | utthana | Utthāna Ekādaśī | MISSING |
| EK-016 | rama | Rāmā Ekādaśī | MISSING |
| EK-017 | mokshada | Mokṣadā Ekādaśī | MISSING |
| EK-018 | utpanna | Utpannā Ekādaśī | MISSING |
| EK-019 | putrada_pausa | Putradā Ekādaśī | MISSING |
| EK-020 | saphala | Saphalā Ekādaśī | MISSING |
| EK-021 | jaya | Jayā Ekādaśī | MISSING |
| EK-022 | sattila | Ṣaṭ-tilā Ekādaśī | MISSING |
| EK-023 | amalaki | Āmalakī Ekādaśī | MISSING |
| EK-024 | vijaya | Vijayā Ekādaśī | MISSING |

## Mahādvādaśīs

| ID | Slug | Display name | Content file |
|---|---|---|---|
| MD-001 | unmilani | Unmīlanī Mahādvādaśī | MISSING |
| MD-002 | vyanjuli | Vyañjulī Mahādvādaśī | MISSING |
| MD-003 | trisprsa | Triṣpṛṣā Mahādvādaśī | MISSING |
| MD-004 | paksavardhini | Pakṣavardhinī Mahādvādaśī | MISSING |
| MD-005 | jaya_mahadvadasi | Jayā Mahādvādaśī | MISSING |
| MD-006 | vijaya_mahadvadasi | Vijayā Mahādvādaśī | MISSING |
| MD-007 | jayanti | Jayantī Mahādvādaśī | MISSING |
| MD-008 | papanasini | Pāpanāśinī Mahādvādaśī | MISSING |

## Special Checks

```text
aja.json
MISSING

annada.json
NOT CATALOGED

kamika.json
OK

sayana.json
OK

Mahādvādaśī content files
0 / 8 present
```

Note:

```text
Annadā Ekādaśī is not currently represented as an ObservanceId slug in the
Native Observance Resolver catalog. Related naming remains part of the open
normative/catalog boundary already tracked under MAS-001 / MD-001.
```

## Orphan File Check

Every content file currently present corresponds to a cataloged slug:

```text
kamika
CATALOGED

pavitropana
CATALOGED

sayana
CATALOGED
```

Result:

```text
Orphan content files:
0
```

## Classification

```text
CONTENT-CAT-001
Resolved slug has no corresponding content JSON file.

Affected count:
29

Confirmed ENGINE BUG:
0
```

## Conclusion

The current failure for:

```text
/content/ekadasi/aja.json
```

is caused by an incomplete RC1 content library. The resolver may produce `aja`,
but the RC1 content directory does not yet contain `aja.json`.

The first point of inconsistency is:

```text
Festival Library content availability
```

Not:

```text
Astronomy Engine
Calendar Engine
Native Māsa
Observance Resolver
Knowledge Base
```

## Certification Result

```text
Campaign 52.5

Observance Content Catalog Integrity:
OPEN

Catalog references:
32

Files present:
3

Files missing:
29

Orphan files:
0

ENGINE BUG:
0

Correction required:
Content library completion or graceful missing-content handling in a future
campaign.
```
