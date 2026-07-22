# EK-010: Kāmikā Ekādaśī Research Dossier

Campaign: 55.1A

Status: APPROVED FOR 55.1B AUTHORING

Mode: RESEARCH ONLY

Observance JSON modified: NO

Invented doctrine: 0

## Observance Identity

```text
ObservanceId:
EK-010

Slug:
kamika

Display name:
Kāmikā Ekādaśī

Type:
Ekādaśī

Māsa:
Śrāvaṇa

Pakṣa:
Kṛṣṇa

Resolver source:
masa_paksha
```

## Candidate Rationale

Kāmikā Ekādaśī is selected as the first Golden Sample candidate because it has:

```text
clear local source coverage
compact narrative scope
explicit name and calendar context
explicit devotional procedure elements
explicit benefits
no Mahādvādaśī dependency
no known active normative difference in the project register
```

This makes it suitable for validating the authoring workflow without beginning
with an intentionally difficult or disputed case.

## Source Inventory

| Source ID | Source | KB-REF Level | Category | Allowed Use | Local Path / Location | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SRC-KAMIKA-001 | Brahma-vaivarta Purāṇa, as represented in the local Ekādaśī PDF | A | Canonical source through local translated compilation | History, benefits, observance narrative, named practice elements | `07_Sources/Vaishnava/204176529-Ekadashi-Es-Ramanuja-Dasa.pdf`, PDF pages 58-59, printed pages 57-58 | ACCEPTED FOR DOSSIER | Chapter and verse numbers are not present in the local PDF and remain pending. |
| SRC-KAMIKA-002 | RC1 Festival Catalog | C | Operational / project catalog | ObservanceId, slug, display name, māsa, pakṣa | `08_Examples/RC1-Experience/content/ekadasi/catalog.json` | ACCEPTED FOR IDENTITY | Identifies content binding only; does not define doctrine. |
| SRC-KAMIKA-003 | HBV-EK-005 / KB-VAI-002 | C / A-derived project rule | Operational rule specification | Parāṇa explanation and boundary discipline | `05_Documentation/Knowledge-Base/HBV-EK-005-Parana-Harivasara.md`; `05_Documentation/Knowledge-Base/KB-VAI-002-Hari-bhakti-vilasa-Ekadasi-Rules.md` | ACCEPTED FOR PARĀṆA CONTEXT | Used only to state that Parāṇa is calculated by the engine and not hardcoded in content. |
| SRC-KAMIKA-004 | KB-OBS-001 | E | Editorial governance | Section discipline, pending handling, publication rules | `05_Documentation/Knowledge-Base/KB-OBS-001-Observance-Authoring-Guidelines.md` | GOVERNANCE | Not a doctrinal source. |
| SRC-KAMIKA-005 | KB-REF-001 | E | Reference governance | Source classification and allowed-use policy | `05_Documentation/Knowledge-Base/KB-REF-001-Reference-Classification.md` | GOVERNANCE | Not a doctrinal source. |

## Primary Sources Found

### SRC-KAMIKA-001

The local PDF attributes the Kāmikā Ekādaśī section to the
Brahma-vaivarta Purāṇa.

The section includes:

```text
name of the observance
month and pakṣa context
Yudhiṣṭhira's question
Kṛṣṇa's narration
Nārada's question to Brahmā
Brahmā's answer
worship of Hari / Viṣṇu
Tulasī-centered observance elements
benefits connected with hearing, fasting, worship, and offerings
closing source attribution
```

Limitations:

```text
chapter number:
PENDING

verse numbers:
PENDING

Sanskrit source witness:
PENDING

publisher / edition details:
PENDING

license:
PENDING
```

## Secondary Sources Found

No Level B commentary source was located locally for this dossier.

Status:

```text
secondary_sources:
N-A for Campaign 55.1A
```

## Operational Sources Found

Operational sources are available for:

```text
ObservanceId
slug
display name
māsa
pakṣa
Parāṇa boundary discipline
content / resolver separation
```

These sources support identity and presentation boundaries, not the devotional
history or benefits.

## Technical Sources Found

No Kāmikā-specific technical source is required for this dossier.

Astronomical details remain calculated by Pancanga Engine at runtime and should
not be copied into the observance content file.

## Editorial Sources Found

The current `kamika.json` contains prior RC1 content, but that file is not used
as doctrinal evidence in this dossier.

It may be audited later against this dossier during Campaign 55.1B.

## Documented Facts

| Fact ID | Fact | Evidence | Status |
| --- | --- | --- | --- |
| F-001 | The observance is identified as Kāmikā Ekādaśī. | SRC-KAMIKA-001; SRC-KAMIKA-002 | VERIFIED |
| F-002 | The calendar context is Śrāvaṇa Kṛṣṇa Ekādaśī. | SRC-KAMIKA-001; SRC-KAMIKA-002 | VERIFIED |
| F-003 | The narrative opens with Yudhiṣṭhira asking Kṛṣṇa about the Ekādaśī after Deva-śayanī. | SRC-KAMIKA-001 | VERIFIED |
| F-004 | Kṛṣṇa relates Nārada's question to Brahmā. | SRC-KAMIKA-001 | VERIFIED |
| F-005 | Brahmā identifies the observance and describes its glories. | SRC-KAMIKA-001 | VERIFIED |
| F-006 | Worship of Hari / Viṣṇu under names such as Śrīdhara, Mādhava, and Madhusūdana is central to the section. | SRC-KAMIKA-001 | VERIFIED |
| F-007 | Tulasī offerings and service are repeatedly glorified in the section. | SRC-KAMIKA-001 | VERIFIED |
| F-008 | Hearing the glories of Kāmikā Ekādaśī is itself described as spiritually beneficial. | SRC-KAMIKA-001 | VERIFIED |

## Documented Narrative

The documented narrative may be summarized as follows:

```text
Yudhiṣṭhira asks Kṛṣṇa about the Ekādaśī of the dark fortnight of Śrāvaṇa.
Kṛṣṇa says that Nārada once asked Brahmā the same question.
Brahmā identifies the observance as Kāmikā Ekādaśī and describes its worship,
merits, and purifying power.
The account strongly emphasizes worship of Hari / Viṣṇu and the devotional
importance of Tulasī.
```

This is sufficient to support an editorial history section.

It is not sufficient to support a fully literal translation or verse-by-verse
commentary.

## Documented Benefits

The local source explicitly supports these benefit categories:

```text
removal of sins
spiritual merit from hearing the glories
benefits from worshiping Hari / Viṣṇu
benefits from offering or serving Tulasī
benefits connected with lamps offered to Kṛṣṇa / Tulasī
return to Viṣṇu-loka in the closing statement
```

All benefits must be presented as source-described benefits, not as claims
independently asserted by Pancanga Engine.

## Documented Procedure

The local source supports these observance elements:

```text
fasting according to capacity
worshiping Hari / Viṣṇu
meditating on or honoring divine names such as Śrīdhara, Hari, Viṣṇu, Mādhava,
and Madhusūdana
offering or honoring Tulasī
offering a lamp
hearing or reading the glories of the observance
```

Procedure content must remain bounded by KB-OBS-001. It must not add universal
requirements that the source does not explicitly establish.

## Documented Parāṇa Information

No Kāmikā-specific Parāṇa rule is found in SRC-KAMIKA-001.

For authoring, the Parāṇa section should remain operational:

```text
Parāṇa is calculated dynamically by Pancanga Engine for the selected date and
location.
```

Source:

```text
SRC-KAMIKA-003
```

The observance content must not hardcode local Parāṇa times.

## Documented Astronomy / Calendar Context

The source and catalog support:

```text
Śrāvaṇa
Kṛṣṇa pakṣa
Ekādaśī
```

All runtime astronomical values remain outside the content file.

Unsupported astronomy claims:

```text
specific sunrise
specific tithi boundaries
specific lunar elongation
nakṣatra
yoga
karaṇa
```

These must remain calculated by Pancanga Engine, not authored into the library.

## Missing Information

```text
Exact chapter number in Brahma-vaivarta Purāṇa:
PENDING

Exact verse range:
PENDING

Sanskrit edition / witness:
PENDING

Publisher and license of local PDF:
PENDING

Independent Level B commentary:
PENDING / N-A

Canonical source registry ID:
PENDING
```

## Conflicts Between Sources

No direct source conflict was found during Campaign 55.1A.

Known limitations are bibliographic rather than doctrinal:

```text
local PDF gives a source attribution but not chapter / verse numbers
local PDF is a translated compilation, not a critical edition
no independent Sanskrit witness was checked in this campaign
```

## Rejected Claims

No claim was rejected as false.

The following claim types are rejected for authoring until supported by
evidence:

```text
exact verse references
literal Sanskrit quotations
universal ritual requirements not stated in the source
specific Parāṇa times
specific astronomical values
hero quotes not present in an accepted source
```

## Preliminary Provenance

```text
primary_sources:
- SRC-KAMIKA-001

secondary_sources:
- none

operational_sources:
- SRC-KAMIKA-002
- SRC-KAMIKA-003

technical_sources:
- none

editorial_sources:
- SRC-KAMIKA-004
- SRC-KAMIKA-005

reviewed_by:
PENDING

approved_by:
PENDING

verification_date:
PENDING
```

## Authoring Recommendation

Campaign 55.1A is sufficient to begin Campaign 55.1B only after review approval.

Recommended 55.1B action:

```text
Audit and revise kamika.json strictly against this dossier and the evidence log.
Do not add new content not present in the dossier.
Keep missing bibliographic data pending.
Keep Parāṇa and astronomy operational, not hardcoded.
Do not mark published until review and approval are complete.
```

## Campaign 55.1A Result

```text
Research:
PASS

Evidence Collection:
PASS

Source Classification:
PASS

Dossier:
PASS

Evidence Log:
PASS

Conflicts:
0 doctrinal conflicts found

Pending bibliographic items:
OPEN

JSON modified:
NO

Invented Doctrine:
0

Status:
APPROVED FOR 55.1B AUTHORING
```

## Review

Campaign 55.1A was reviewed and approved for authoring.

Review file:

```text
05_Documentation/Observances/Dossiers/EK-010-Kamika-55.1A-Review.md
```
