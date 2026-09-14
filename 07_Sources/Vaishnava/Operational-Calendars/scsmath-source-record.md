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
| Reference location (published, p. 5) | Śrī Chaitanya Sāraswat Maṭh, Nabadwīp, India — source text: "(Calculated for Śrī Chaitanya Sāraswat Maṭh, Nabadwīp, India)" |
| Coordinates published by the source | NONE — the PDF publishes no numeric latitude or longitude |
| Format | PDF |

Verification note: the extent (64 pages) and the page 3 attribution naming
"Sri Chaitanya Saraswat International Sangha" as preparer/publisher, and the
page 5 reference-location statement, were verified by the owner from the PDF
itself. Open Discrepancies 1 and 2 remain open; in particular, which of the
two publisher names is the publisher of record is not decided by this
verification. Discrepancy 3 is resolved by the page 5 statement.

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

Items 1 and 2 remain to be resolved before this record is final; item 3 is
resolved and retained for the record:

1. **Embedded title disagrees with the edition.** The PDF `/Title` metadata
   reads `Sri Gaudiya Calendar 2023-2024`, while the cover and the recorded
   edition read 2026–2027. Most likely an InDesign template carried forward
   on export, but that is a hypothesis. Confirm from the document body which
   Gaurābda year the contents actually cover. UNVERIFIED.

2. **Two publisher names.** `/Author` says "Math International"; page 3 says
   "International Sangha". Determine which is the corporate publisher of
   record and cite that one.

3. **Reference location — RESOLVED (owner verification).** Formerly recorded
   as a claim, not a publication. The source states its reference location
   explicitly on page 5, quoted verbatim by the owner from the PDF:

   ```text
   (Calculated for Śrī Chaitanya Sāraswat Maṭh, Nabadwīp, India)
   ```

   Evidence: `02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf`, page 5.
   The source names the place, not the coordinates: the PDF still publishes
   no numeric latitude or longitude, so the fixture values 23.4071 / 88.3679 /
   `Asia/Kolkata` remain extractor-supplied validation configuration and stay
   flagged as such.

Item 3 mattered because a published calendar is only meaningful for the place
it was computed for. With the reference location now published, comparisons
against this oracle inherit only the coordinate assumption, not the place.

## Unverified Items

- Whether this source shares a computation lineage with any other Tier 5
  oracle. Not stated by the source. UNVERIFIED.
- Discrepancies 1 and 2 above (discrepancy 3 is resolved).
