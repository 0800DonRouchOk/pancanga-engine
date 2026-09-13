# PureBhakti Calendar 2026 Source Record

Campaign: 46D.1  
Acquisition date: UNRECOVERABLE  
Verification status: IMPORTED / REVIEW PENDING

## Bibliographic Identity

| Field | Value |
|---|---|
| Work | Calendario Vaishnavá 2026 — Śrī Caitanya Pañjikā |
| Tradition line (self-declared) | Viśuddha Sārasvata |
| Era | Śrī Gaurābda 540 |
| Version string | Kṛṣṇakal 0.98 |
| Authority (self-declared) | Tridaṇḍisvāmī Śrī Śrīmad Bhaktivedānta Nārāyaṇa Gosvāmī Mahārāja |
| Publisher | Bhaktivedānta Trust International, Śrī Gopīnāth Bhavan, Sevā Kuñj, Vṛndāvana (U.P.) 281121, India |
| Last updated (self-declared) | 19 May 2026 |
| Coverage | May – December 2026 |
| Reference location (self-declared) | Buenos Aires, Argentina — 34° 37′ S, 58° 23′ W |
| Reference timezone (self-declared) | UTC −3 |
| Language | Spanish |
| Format | HTML |

All fields above are transcribed from the source itself, not inferred.

## Acquisition Provenance

| Field | Value |
|---|---|
| Source site | purebhakti.com |
| Download route | Not recorded |
| Acquisition date | UNRECOVERABLE |
| Authority role of website | NONE |

The website located and delivered the calendar. It is an acquisition route
only; the source authority is the published calendar identified above, and
that authority is limited to Tier 5 validation.

## Local Artifacts

| Artifact | Local path | Bytes | SHA-256 |
|---|---|---:|---|
| Acquired source | `02_Research/PureBhakti/Calendario-Vaishnava-2026.html` | 95,878 | `889ce8717a4dc0dc6ff7116c057dda2fe5b7f55ba2fe0abae7ac46afd6f6c432` |

Derived validation artifacts (not sources):

| Field | Value |
|---|---|
| Extractor | `03_Source/rust/crates/pancanga-engine/examples/purebhakti_html_extract.rs` |
| Fixture file | `04_Tests/PureBhakti/purebhakti-fixtures.csv` |
| Rows extracted | 16 |
| Coordinates used in fixtures | −34.616667 / −58.383333 |
| Row status | `IMPORTED_REQUIRES_REVIEW` — manual review against the HTML pending |

The extractor takes every field — location, coordinates, timezone, māsa, pakṣa,
Mahādvādaśī, pāraṇa — from the HTML. It does not synthesise values.

## Authority Classification

```text
Calendario Vaishnavá 2026 (PureBhakti):
Tier 5 - Operational Calendar

Permitted use:
VALIDATION ONLY

Normative authority:
NEVER
```

Per `KB-REF-001` § Tier 5, this source may be used to validate computed dates
and may never establish a calendar rule. Disagreement between this source and
the engine is a finding to report, not a defect to fix.

## Unverified Items

- **`Kṛṣṇakal 0.98`.** The string appears in the source. Whether it names a
  computation engine, a data version, or something else is not stated anywhere
  in this repository. UNVERIFIED.
- **Computational origin.** The source does not state how its dates are
  computed. Whether it shares a computation lineage with any other Tier 5
  oracle is UNVERIFIED from repository evidence.
- **Access date.** Not recorded at import time. UNRECOVERABLE.
