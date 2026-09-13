# Validation Oracle Set for v1.0 — rationale for two sources

**Purpose:** close Release Gate items 1 and 3 by the documented route rather
than by acquiring a third oracle.

`CERTIFICATION.md § Release Gate` requires GCal fixtures "populated from a real
source" and GCal certification completing. This document argues that the
validation requirement is met by the two oracles already held, and that the
third slot should be recorded as deliberately unfilled for v1.0 rather than
left pending.

**This is an owner decision. Nothing here is authorised until the owner
records it in `CERTIFICATION.md`.**

---

## 1. What is actually held

| | PureBhakti | SCS Math |
|---|---|---|
| Publisher | Bhaktivedānta Trust International, Vṛndāvana | Sri Chaitanya Saraswat Math International |
| Edition | Śrī Caitanya Pañjikā, Gaurābda 540 | Śrī Gauḍīya Calendar, Gaurābda 540–541 |
| Tradition line (self-declared) | Viśuddha Sārasvata | Sārasvata Gauḍīya |
| Reference location | Buenos Aires (published) | Nabadwip (claimed, not published) |
| Rows | 16 | 26 |
| Coverage | May–Dec 2026 | Mar 2026 – Mar 2027 |

GCal has never been populated. The twelve rows in
`04_Tests/Vaishnava/GCal/gcal-fixtures.csv` are the pending template written by
`gcal_fixture_import.rs` when it finds no source, not an incomplete import.

## 2. The two are not redundant

Measured, not asserted. In the overlap window 2026-05-13 → 2026-12-20:

- **Six observances are published on different civil dates** by the two
  calendars: 05-26/05-27, 07-10/07-11, 07-24/07-25, 08-08/08-09, 08-23/08-24,
  11-04/11-05 (PureBhakti date first).
- Of the ten dates present in both files, **none agrees on every value**; each
  differs in at least five columns.
- The sources disagree on observance naming, on one Mahādvādaśī
  classification, and on every pāraṇa window.

Two oracles that agreed everywhere would confirm arithmetic. Two that publish
different dates for the same observances exercise different normative
decisions, and disagreement is where a validation set earns its value.

## 3. What this argument does not claim

Stated explicitly so the limits travel with the conclusion:

- **It does not claim the two are computationally independent.** Neither source
  states how its dates are computed. Whether they share a calculation lineage
  is UNVERIFIED from repository evidence. The argument rests on demonstrated
  divergence in published output, which holds either way.
- **It does not claim the six date differences are purely normative.** They are
  Nabadwip versus Buenos Aires, and ORDEN-6 established that the engine had a
  day-shift defect at east longitudes. Whether any part of the published
  divergence has the same shape is a question for 46F.2, not a premise here.
- **It does not claim three oracles would add nothing.** It claims that two
  demonstrably divergent Tier 5 sources satisfy the validation requirement for
  v1.0, and that acquiring a third is a v1.1 item rather than a release
  blocker.
- **It does not retire GCal.** The slot stays open.

## 4. Precondition

This rationale is only available if the two source records are filed. Until
then the position is not "two sufficient oracles" but "two oracles in use
without bibliographic entries", which is a weaker position than having none —
`KB-REF-001` requires the entry, and both oracles are in use today without one.

File first:
- `07_Sources/.../purebhakti-source-record.md`
- `07_Sources/.../scsmath-source-record.md`

Both carry unresolved items. The SCS Math record's reference-location question
is the one that bears on this argument: if the Nabadwip attribution turns out
to be wrong, the six divergent dates would need re-examination.

## 5. Proposed Release Gate disposition

For the owner to accept, amend or reject:

| Gate item | Proposed disposition |
|---|---|
| 1. GCal fixtures populated from a real source | Documented as deliberately unfilled for v1.0; validation set is PureBhakti + SCS Math |
| 3. GCal certification completes with no critical ENGINE BUG | Not applicable for v1.0 under the disposition above |
| 2. SCS Math follow-up configuration audit | 46F.2 — partially answered by ORDEN-6 |
| 4. Full external validation updated | Requires rerunning both harnesses post-ORDEN-6 |

Item 4 is now the live one: the recorded CSVs predate the day-shift fix and
have not been rerun.
