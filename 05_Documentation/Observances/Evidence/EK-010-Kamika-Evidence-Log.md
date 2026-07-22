# EK-010: Kāmikā Ekādaśī Evidence Log

Campaign: 55.1A

Status: READY FOR REVIEW

Mode: RESEARCH ONLY

Observance JSON modified: NO

## Evidence Status Values

```text
Verified
Pending
Conflict
Rejected
```

## Evidence Log

| Claim ID | Claim | Section | Source(s) | Source Level | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| C-001 | The observance ID is EK-010. | identity | SRC-KAMIKA-002 | C | Verified | Operational catalog identity. |
| C-002 | The stable slug is `kamika`. | identity | SRC-KAMIKA-002 | C | Verified | Must bind to `content/ekadasi/kamika.json`. |
| C-003 | The display name is Kāmikā Ekādaśī. | identity | SRC-KAMIKA-001; SRC-KAMIKA-002 | A; C | Verified | Source text and catalog agree. |
| C-004 | Kāmikā is associated with Śrāvaṇa Kṛṣṇa Ekādaśī. | classification | SRC-KAMIKA-001; SRC-KAMIKA-002 | A; C | Verified | PDF gives Śrāvaṇa Kṛṣṇa context; catalog binds resolver identity. |
| C-005 | Yudhiṣṭhira asks Kṛṣṇa about the Ekādaśī after Deva-śayanī. | history | SRC-KAMIKA-001 | A | Verified | Supports opening narrative. |
| C-006 | Kṛṣṇa says Nārada previously asked Brahmā about this observance. | history | SRC-KAMIKA-001 | A | Verified | Supports narration chain Kṛṣṇa -> Nārada -> Brahmā. |
| C-007 | Brahmā identifies the observance and answers about name, deity, process, and merits. | history / scriptural_basis | SRC-KAMIKA-001 | A | Verified | Supports source structure and observance scope. |
| C-008 | The section identifies worship of Hari / Viṣṇu as central. | observance | SRC-KAMIKA-001 | A | Verified | Names include Śrīdhara, Hari, Viṣṇu, Mādhava, Madhusūdana. |
| C-009 | The section explicitly emphasizes Tulasī offerings and service. | observance / benefits | SRC-KAMIKA-001 | A | Verified | Multiple Tulasī-related acts are described. |
| C-010 | Fasting should be described according to the practitioner's capacity, not as a new rule invented by the content file. | observance | SRC-KAMIKA-001; SRC-KAMIKA-003 | A; C | Verified | Content must not override Pancanga Engine calculation. |
| C-011 | Hearing the glories of Kāmikā Ekādaśī is described as spiritually beneficial. | benefits | SRC-KAMIKA-001 | A | Verified | May be summarized as source-described benefit. |
| C-012 | The observance is described as removing sins. | benefits | SRC-KAMIKA-001 | A | Verified | Must be worded as source-described, not as project assertion. |
| C-013 | Offering a lamp is described as meritorious. | observance / benefits | SRC-KAMIKA-001 | A | Verified | Use cautiously; do not expand beyond source. |
| C-014 | The closing attribution connects the Kāmikā section to Brahma-vaivarta Purāṇa. | references | SRC-KAMIKA-001 | A | Verified | Local source attribution; exact chapter/verses pending. |
| C-015 | Exact chapter number for the Kāmikā passage. | scriptural_basis | SRC-KAMIKA-001 | A | Pending | Not present in local PDF. |
| C-016 | Exact verse range for the Kāmikā passage. | scriptural_basis | SRC-KAMIKA-001 | A | Pending | Not present in local PDF. |
| C-017 | Sanskrit source witness for the Kāmikā passage. | references | none | none | Pending | Not located locally in Campaign 55.1A. |
| C-018 | Kāmikā-specific Parāṇa instructions. | parana | none | none | Pending | No special Kāmikā Parāṇa rule found in local source. Use engine-calculated Parāṇa only. |
| C-019 | General Parāṇa should be calculated dynamically by Pancanga Engine. | parana | SRC-KAMIKA-003 | C | Verified | Content must not hardcode local times. |
| C-020 | Specific sunrise, tithi boundaries, nakṣatra, yoga, or karaṇa for a date. | astronomy | none | none | Rejected | These are runtime engine results, not static observance content. |
| C-021 | Hero quote for Kāmikā Ekādaśī. | presentation | none | none | Pending | No short approved quote selected from accepted source. |
| C-022 | Existing `kamika.json` content is doctrinal evidence. | editorial | none | none | Rejected | Existing JSON may be audited during 55.1B but is not a source. |

## Evidence Summary

```text
Verified claims:
16

Pending claims:
5

Rejected claims:
2

Conflicts:
0

Invented doctrine:
0
```

## Section Readiness

| Section | Status | Notes |
| --- | --- | --- |
| identity | READY | Supported by source and catalog. |
| classification | READY | Supported by source and catalog. |
| summary | READY WITH REVIEW | Must be a short editorial synthesis from verified claims only. |
| history | READY | Narrative chain and core content are supported. |
| scriptural_basis | PARTIAL | Source work verified; chapter and verse pending. |
| observance | READY WITH REVIEW | Supported elements must remain bounded. |
| benefits | READY WITH REVIEW | Use explicit source-described benefits only. |
| procedure | READY WITH REVIEW | Must not create new ritual obligations. |
| parana | OPERATIONAL ONLY | Dynamic engine-calculated Parāṇa; no Kāmikā-specific rule found. |
| astronomy | OPERATIONAL ONLY | Runtime engine data only. |
| certification | READY | Should point to project certification reports, not duplicate them. |
| presentation | PARTIAL | Title/subtitle supported; hero quote pending. |

## 55.1B Authoring Gate

Campaign 55.1B may begin only after this evidence log and the corresponding
research dossier are reviewed.

Authoring must use only:

```text
Verified claims
Pending markers
Rejected-claim exclusions
```

No claim may be added directly to `kamika.json` unless it first appears in this
evidence log.
