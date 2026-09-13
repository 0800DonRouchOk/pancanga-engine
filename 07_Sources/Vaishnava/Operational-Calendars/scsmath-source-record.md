# SCS Math Gauḍīya Calendar 540–541 Source Record

Campaign: 46F  
Acquisition date: UNRECOVERABLE  
Verification status: IMPORTED / REVIEW REQUIRED

## Bibliographic Identity

| Field | Value |
|---|---|
| Work (cover) | Śrī Gauḍīya Calendar |
| Era (cover) | 540–541 Śrī Gaurābda (2026–2027) |
| Publisher (PDF `/Author`) | Sri Chaitanya Saraswat Math International |
| Publisher (page 3) | Sri Chaitanya Saraswat International Sangha — owner-verified from the PDF |
| Extent | 64 pages — owner-verified from the PDF |
| PDF `/Title` | `Sri Gaudiya Calendar 2023-2024` (see Open Discrepancies) |
| PDF `/CreationDate` | 2026-01-29 |
| PDF `/Creator` | Adobe InDesign 19.5 |
| Reference location (claimed) | Śrī Chaitanya Sāraswat Maṭh, Nabadwip, India |
| Coordinates published by the source | NONE — the PDF publishes no numeric latitude or longitude |
| Format | PDF |

Verification note: the extent (64 pages) and the page 3 attribution naming
"Sri Chaitanya Saraswat International Sangha" as preparer/publisher were
verified by the owner from the PDF itself. The three items under Open
Discrepancies remain open; in particular, which of the two publisher names is
the publisher of record is not decided by this verification.

## Acquisition Provenance

| Field | Value |
|---|---|
| Download route | Not recorded |
| Acquisition date | UNRECOVERABLE |
| Authority role of website | NONE |

## Local Artifacts

| Artifact | Local path | Bytes | SHA-256 |
|---|---|---:|---|
| Acquired source | `02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf` | 16,556,168 | `c35dc73ef692016b2715477e5673e24817a3ae608f1948907d653d364dacd987` |

Derived validation artifacts (not sources):

| Field | Value |
|---|---|
| Extractor | `04_Tests/SCSMath/scsmath_pdf_extract.py` (requires `pdfplumber`) |
| Fixture file | `04_Tests/SCSMath/scsmath-fixtures.csv` |
| Rows extracted | 26 |
| Coverage | 2026-03-15 → 2027-03-18 |
| Coordinates used in fixtures | 23.4071 / 88.3679 / `Asia/Kolkata` — injected by the extractor, NOT from the source |
| Row status | Manual review PENDING |

Published fast dates are used as-is, including "No fast / Fast on …"
displacement patterns.

## Authority Classification

```text
Śrī Gauḍīya Calendar 540-541 (SCS Math):
Tier 5 - Operational Calendar

Permitted use:
VALIDATION ONLY

Normative authority:
NEVER
```

Per `KB-REF-001` § Tier 5, this source may be used to validate computed dates
and may never establish a calendar rule.

Twelve rows of `scsmath-validation.csv` carry classifications that ORDEN-6
measured as engine day-shift cases rather than calendar differences. That
reclassification is campaign 46F.2's, not this record's.

## Open Discrepancies

To resolve before this record is final:

1. **Embedded title disagrees with the edition.** The PDF `/Title` metadata
   reads `Sri Gaudiya Calendar 2023-2024`, while the cover and the recorded
   edition read 2026–2027. Most likely an InDesign template carried forward
   on export, but that is a hypothesis. Confirm from the document body which
   Gaurābda year the contents actually cover. UNVERIFIED.

2. **Two publisher names.** `/Author` says "Math International"; page 3 says
   "International Sangha". Determine which is the corporate publisher of
   record and cite that one.

3. **Reference location is a claim, not a publication.** The record states the
   calendar is calculated for Nabadwip. The PDF does not publish coordinates.
   The values in the fixtures were supplied by the extractor as validation
   configuration. Either locate a statement of the reference location in the
   document, or record explicitly that the location is assumed and mark every
   fixture dependent on it accordingly.

Item 3 matters more than it looks: a published calendar is only meaningful for
the place it was computed for. If the reference location is assumed rather
than stated, every comparison against this oracle inherits that assumption.

## Unverified Items

- Whether this source shares a computation lineage with any other Tier 5
  oracle. Not stated by the source. UNVERIFIED.
- The three discrepancies above.
