# Campaign Report — SCS Math Evidence Reconciliation (post-Campaign 46C Final)

Date: 2026-09-15

Scope: annotate stale DOC-001 classifications following Campaign 46C Final's
ORDEN-6 rerun. No engine code, golden test, or owner-only normative document
was touched.

## PHASE 0 CONCLUSION: editable

Checked each file's own header/preamble and cross-referenced CLAUDE.md § 9's
single-writer list plus `KB-REF-001` and `PROJECT_STATUS.md`:

- `normative-observance-certification.csv` — header is a plain CSV column
  row, no write-policy statement.
- `Normative-Observance-Certification.md` — preamble states "Campaign: 52.2 /
  Mode: certification only. No motor, Calendar Engine, Observance Engine,
  Knowledge Base, architecture, or rule logic was modified." — a scope note,
  not a write-restriction.
- `Normative-Differences-Register.md` — preamble states "Campaign: 52.2
  follow-up / Status: OPEN" — likewise no write-policy claim.
- `grep -n "canonical rule document" CLAUDE.md` → one hit (§ 9, the sentence
  naming `PROJECT_STATUS.md`, `CERTIFICATION.md`, the Festival Master
  Registry, the Research Backlog, `KB-REF-001`, "canonical rule documents").
  None of the three target files is named there, in `KB-REF-001`, or in
  `PROJECT_STATUS.md`.
- These three are Campaign 52.2 test/certification evidence artifacts under
  `04_Tests/Vaishnava/`, structurally identical in kind to
  `SCSMath-Validation.md` / `scsmath-validation.csv`, which were already
  updated in the prior (46C Final) campaign without objection. "Canonical
  rule documents" in § 9's context refers to normative rule specifications
  (HBV-EK-00X, KB-VAI-002, KB-CAL-001/002, etc.), not test-evidence reports.

Conclusion: **no owner-only restriction found**. Proceeded to Phase 1.

## PHASE 1 — 12 DOC-001 entries corrected

File: `04_Tests/Vaishnava/normative-observance-certification.csv`.

For each of the 12 confirmed flipped rows: `category` changed
`DOC-001` → `DOC-001 (SUPERSEDED)` (identifier preserved, not deleted);
`hypothesis` field now leads with a superseded note citing ORDEN-6 (commit
`2a6d5fd`) and `04_Tests/SCSMath/SCSMath-Validation.md § Rerun`, followed by
the verbatim original conclusion text under "Original DOC-001 conclusion,
recorded at time of classification: ..." — audit trail preserved, nothing
erased.

Build note (caught before this report, not a residual issue): the first
attempt wrote the file with Python's default CSV line terminator (`\r\n`),
which silently rewrote every line's ending and made the diff touch all 43
rows instead of 12. Reverted with `git checkout` and rebuilt with
`lineterminator='\n'` to match the file's actual convention. Final diff:
exactly 12 lines changed, byte-identical elsewhere — verified with
`git diff --stat` (12 insertions, 12 deletions) and manual inspection.

## PHASE 2 — `04_Tests/SCSMath/SCSMath-Difference-Report.md` reconciled

- Top summary count rewritten per instruction: "15 differences analyzed; 12
  superseded by the ORDEN-6 fix (Campaign 46C Final rerun, 2026-09-14); 3
  remain open: SCS-0012, SCS-0018, SCS-0026," with a pointer to
  `SCSMath-Validation.md § Rerun`.
- Added a one-line `**SUPERSEDED 2026-09-14**` annotation immediately after
  each of the 12 affected Difference Cards, citing the same evidence.
  Original card text (SCS Math result, engine result, HBV rule chain,
  classification, possible cause) left completely intact for every one of
  the 15 cards.
- `CONFIGURATION_DIFFERENCE` (underscore) spelling left exactly as-is — not
  touched, per instruction to leave the space-vs-underscore divergence
  alone.
- SCS-0012, SCS-0018, SCS-0026 cards: no marker inserted, no text changed —
  verified by line-number inspection (markers appear only immediately
  before/after the 12 target headings, with clean gaps at SCS-0012 and
  SCS-0018).

## ROWS CORRECTED (both files, identical set)

SCS-0003, SCS-0004, SCS-0005, SCS-0006, SCS-0007, SCS-0008, SCS-0009,
SCS-0010, SCS-0011, SCS-0013, SCS-0014, SCS-0015 (12 rows).

## ROWS LEFT UNTOUCHED

- **SCS-0012** — observance status did not change (still
  `no_matching_observance_path` / DIFFERENCE); only an internal
  `engine_tithi_at_arunodaya` value shifted 12 → 11. Its
  DOC-001 / TRADITION_DIFFERENCE classification is not contradicted.
- **SCS-0018** — status unchanged (DIFFERENCE, tithi 10 at sunrise); not a
  day-shift case per the rerun.
- **SCS-0026** — status unchanged (DIFFERENCE, Aruṇodaya Viddhā pattern);
  not a day-shift case.

## Files modified

- `04_Tests/Vaishnava/normative-observance-certification.csv`
- `04_Tests/SCSMath/SCSMath-Difference-Report.md`

Retained from the prior campaign's uncommitted-until-now state, not touched
further this campaign: `04_Tests/PureBhakti/PureBhakti-Validation.md`,
`04_Tests/SCSMath/SCSMath-Validation.md`,
`04_Tests/SCSMath/scsmath-validation.csv`,
`04_Tests/Validation/Validation-Report.md`,
`04_Tests/Validation/validation-results.csv` — these remain as working-tree
changes from the 46C Final rerun campaign; none were re-edited here.

## Tests executed and results

`cargo fmt --all --check`: clean. `cargo clippy --all-targets --all-features
-- -D warnings`: clean. No engine code exists in this campaign's diff, so
`cargo test` / `cargo doc` were not re-run in full — only fmt/clippy
re-verified as a sanity check since no `.rs` file was touched.

## Golden changes: NO

## Dependency or lockfile changes: NO

## Astronomy Engine impact: NONE

## Calendar-rule impact: NONE

## Knowledge Base impact: NONE

## Date-changing impact: NONE

This campaign only annotates prior classification records; it computes
nothing and changes no observance date.

## Location of computation: NO

No solar, calendrical, UTC-offset, or DST computation was added or modified
anywhere, in `src/` or outside it. This campaign edited only documentation
and CSV classification text.

## Silent defaults: NONE introduced, encountered, or preserved

## Unresolved issues, contradictions and surprises

1. **Markdown mirrors not updated.** `Normative-Observance-Certification.md`'s
   Difference Matrix table (rows for these same 12 case IDs) and its
   `DOC-001: 15` summary count, and `Normative-Differences-Register.md`'s
   `DOC-001` section (`Observed in:` list, still names all 15 including the
   12 superseded), still show the pre-reconciliation state. The campaign's
   "Established" section named only the CSV as carrying the classification,
   and Phase 1 said "in this file" (singular) — read narrowly, correcting
   only `normative-observance-certification.csv`, not its two narrative
   mirrors. This is a real inconsistency now on disk between the raw data
   and its markdown presentations; flagged rather than silently extended
   into scope. If the owner wants the mirrors brought in line, that is an
   explicit follow-up.
2. `SCSMath-Difference-Report.md`'s "Parāṇa Notes" section still says "The
   11 rows with matching observance dates" — now stale (23 rows compare
   after the rerun). Not in Phase 2's scope (which named only the 15
   observance-date difference cards and the top count), so left untouched —
   flagged, not fixed.
3. The CSV line-terminator hazard above (caught and corrected before this
   report) — worth knowing if any other tooling in this repo writes CSVs
   with Python's `csv` module, since the default silently corrupts diffs
   against files using bare `\n`.

## `git diff --check`: clean

## CERTIFICATION.md / PROJECT_STATUS.md TOUCHED: NO

Confirmed via `git status --short` — neither file appears in the diff.
