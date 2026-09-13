# GATE-REG-001 — Registry Schema Regression (61.0 → 62.0)

**Status:** OPEN
**Capacity severity:** HIGH — no registry row, of 97, has a field for the
affected dimensions
**Content severity:** MEDIUM — 3 rows lost recorded content; it is stranded,
not destroyed
**Production code impact:** none
**Raised by:** owner review + documentary audit, post-62.1. Not produced by a
campaign.

**Placement:** this gate must be registered in
`Festival-Research-Backlog.md § Gates That Remain Open` alongside
GATE-BIB-001, GATE-BIB-002 and GATE-FEST-001, and mirrored in
`Festival-Master-Registry.md § Open Gates Preserved`. Until it appears there,
it has no standing against the next research batch.

---

## 1. Statement

The registry supersession from `Festival-Registry.md` to
`Festival-Master-Registry.md` removed normative dimensions that are
date-changing, and never introduced one that a later campaign proved necessary.

Two distinct losses, deliberately separated:

**Loss of capacity — 97 rows.** The current registry has no field in which
fasting policy, parāṇa policy, purity/viddha handling, tithi-contact priority
or reference-location profile could be recorded, even if fully researched.

**Loss of recorded content — five cells across three rows.**
`Festival-Registry.md` was a 10-row seed registry. Of its ten rows, seven
carried `RESEARCH_REQUIRED` in both the *fasting requirement* and *parāṇa
requirement* columns and lost nothing on migration. Three rows carried
operational content, across five populated cells:

| Row | Content not carried forward |
|---|---|
| FEST-001 | fast until midnight |
| FEST-007 | fast until noon |
| FEST-008 | Ekādaśī / Dvādaśī nuance, HBV appendix with Śravaṇa |

The per-cell breakdown — which of the five populated cells are *fasting* and
which are *parāṇa*, row by row — is established as **step 0 of the migration**
(§ 4.5), by reading the superseded `Festival-Registry.md` directly. It is not
recorded here because it must come from the file, not from inference. The
FEST-007 and FEST-008 back-fill in § 4.3 inherits whatever scope that
breakdown establishes.

That content still exists in the superseded `Festival-Registry.md`. It is
stranded, not destroyed.

---

## 2. Evidence

All anchors below verified against the tree.

### 2.1 Fast and parāṇa — downgraded to a presence flag

`Festival-Registry.md § Initial Festival Registry` carried dedicated
*fasting requirement* and *parāṇa requirement* columns.

`Festival-Master-Registry.md § Identity and Calendar Inventory` has neither.
The FEST-001 row terminates at `Rohini: KNOWN RELEVANCE; rule gated`.

The only surviving carriers are presence flags:

- `Festival-Master-Registry.md § Delivery and Research Inventory`, row `001`:
  `YES / YES candidate`
- `Festival-Coverage-Matrix.md`, row `001`: `Fast/Parana = Y/Y`, per its
  `§ Legend`

### 2.2 The surviving flag is compatible with a rejected rule

`Rule-Analysis/FEST-001-Janmashtami-Canonical-Rule.md § 7 Vrata And Fast`
explicitly rejects `fast until midnight, then anukalpa` as a complete HBV rule
and demotes it to operational practice requiring separate approval.

`Decision-Spec § 9 Fasting Metadata` holds the resolved structure:
`vrata_required / fasting_scope / preliminary_day / observance_day /
completion_day`.

A reader with access only to the current registry sees `Y`. That value is
consistent with the rejected reading.

**This is the failure mode that "invented doctrine: 0" does not catch.** No
agent invents anything. A previously examined and rejected position simply
becomes re-derivable, because the registry preserved the existence of a rule
and discarded the record of a refusal.

### 2.3 Viddha / tithi contact — no field exists at all

Zero occurrences of viddha in `Festival-Master-Registry.md`.
Zero in `Festival-Coverage-Matrix.md`. Neither has a viddha or tithi-contact
column.

Meanwhile:

- `Canonical-Rule § 2` records `Saptamī-viddhā | Earlier contaminated candidate
  rejected | RESOLVED`
- `Decision-Spec § 6 Purity Classification` defines
  `SAPTAMI_VIDDHA: REJECT, even if Rohiṇī is present`
- `Decision-Spec § 12 Required Invariants` fixes
  `No Saptamī-viddhā date is selected`
- `Simple-Tithi-Festival-Family.md § Finding` establishes
  `TITHI_WITH_CONTACT_PRIORITY` as a required subclass

A date-changing dimension applies across a whole calculation family and has no
column in the 97-row registry.

### 2.4 Reference-location policy has no field either

`Simple-Tithi-Rule-Comparison.md § ST-CONFLICT-001 - Location Policy` is
`UNRESOLVED` and applies to all non-Ekādaśī observances.
`Festival-Research-Backlog.md § Gates That Remain Open` contains only
GATE-BIB-001, GATE-BIB-002 and GATE-FEST-001 — ST-CONFLICT-001 is absent, and
has no registry column.

---

## 3. Consequence for sequencing

Any research batch run against the current schema loses the same dimensions in
the same way. The next batch — whatever its label; no successor campaign to
62.1 is declared anywhere in the tree — would add further rows with the loss
already baked in, converting a column migration into a re-audit.

```
Fix the schema BEFORE the next research batch is opened.
```

---

## 4. Proposed remediation — single migration

Extend the registry with columns for:

| Column | Carries |
|---|---|
| `fast_policy` | resolved fasting rule, or the gate blocking it |
| `parana_policy` | resolved parāṇa rule, or the gate blocking it |
| `purity_policy` | viddha / contamination handling, incl. explicit rejections |
| `contact_priority` | tithi-contact and boundary priority, per `TITHI_WITH_CONTACT_PRIORITY` |
| `reference_profile` | location / lineage reference policy, per ST-CONFLICT-001 |

### 4.0 Step 0 — establish the cell breakdown (do this first)

Before any column is added, read the superseded `Festival-Registry.md
§ Initial Festival Registry` and report, per row, which of its *fasting
requirement* and *parāṇa requirement* cells carry operational content and
which carry `RESEARCH_REQUIRED`.

Expected shape: ten rows, seven with `RESEARCH_REQUIRED` in both columns,
three rows carrying content across five populated cells. Confirm or correct
that from the file — the numbers above come from an audit, not from the file
in front of you.

Report the breakdown before proceeding. It determines the scope of the
back-fill in § 4.3.

### 4.1 Vocabulary

The registry is markdown tables with no JSON schema.
`observance.schema.json` governs observance **content** and does not apply here.

The governing vocabulary for these columns is
`Festival-Master-Registry.md § Status Vocabulary` (`RR / RC / LRR / P / U / NI
/ PD`). Do not invent values outside it.

### 4.2 Authorized vocabulary extension

That vocabulary cannot express *researched, documented and rejected*. It has no
token for it. Distinguishing an unresearched dimension from a rejected reading
is the entire point of this gate, so:

**This gate authorizes exactly one extension to
`Festival-Master-Registry.md § Status Vocabulary`:** a token meaning
*researched, documented, rejected by cited authority*.

**Collision surface — read before assigning a code.** Single letters are
largely taken, across two documents that cite each other:

- `Festival-Master-Registry.md § Status Vocabulary`:
  `RR / RC / LRR / P / U / NI / PD`, plus `T` as the tier prefix in T1–T4.
- `Festival-Coverage-Matrix.md § Legend`, positional — the same letter means
  different things by column: Rule `P N C L`, Source `P N`, Engine `A D X U`,
  Content `PD`, Fast/Parāṇa `Y N U`, Implementation `NI`.

Occupied at least once: `P N C L A D X U Y T`.

**Recommendation: use a two-character token, not a letter.** The registry
vocabulary is already multi-character (RR, RC, LRR, NI, PD), so a two-letter
code matches the existing pattern and sidesteps the collision surface
entirely. The choice remains the owner's.

**ASSIGNED CODE:**

```
RJ
```

Meaning: *researched, documented, rejected by cited authority.*

Two characters, matching the existing multi-character tokens (RR, RC, LRR, NI,
PD), and colliding with nothing in either vocabulary.

This code is also stated in the order manifest given to the executing agent.
If it is ever changed here, it must be changed there too.

Constraints on that extension:

1. The code above is assigned by the owner. Use exactly `RJ`. Do not
   substitute, abbreviate or extend it.
2. The token alone is insufficient. Every use must carry an anchor to the
   document and section recording the rejection — e.g. the FEST-001 fasting
   cell must point at `Canonical-Rule § 7`. A rejection token without a
   citation reproduces this very defect at lower resolution.
3. No other vocabulary change is authorized by this gate.

### 4.3 Back-fill — partial, and must remain visibly partial

FEST-001 back-fills from `Canonical-Rule` and `Decision-Spec`, which already
hold resolved values — but not uniformly:

Two kinds of source, and they are not interchangeable.
`Canonical-Rule § 2` is a **status table** — use it for the state of each
dimension. The **operative rule text** that goes into the cell lives in the
prose sections. Cite both; fill the cell from the prose.

| Column | State | State from | Rule text from |
|---|---|---|---|
| `purity_policy` | RESOLVED | `Canonical-Rule § 2` | `Canonical-Rule § 5 Saptamī-Viddhā`; `Decision-Spec § 6`, `§ 12` |
| `fast_policy` | RESOLVED, with rejection recorded | `Canonical-Rule § 2` | `Canonical-Rule § 7 Vrata And Fast`; `Decision-Spec § 9` |
| `contact_priority` | GATED — two-day priority UNRESOLVED | `Canonical-Rule § 2` | `Canonical-Rule § 6 Two-Day Candidate Rule`; GATE-FEST-001 |
| `parana_policy` | GATED — no default parāṇa profile | `Decision-Spec § 10.2` | `Decision-Spec § 10.2` |

`Navamī-yukta` is `RESOLVED IN PRINCIPLE` in `Canonical-Rule § 2`, not
`RESOLVED`; it must not be recorded as closed.

The FEST-001 row must not read as complete after back-fill. If it does,
GATE-FEST-001 is lost.

FEST-007 and FEST-008 back-fill from the superseded `Festival-Registry.md`.
That registry predates the 61.0T primary-source acquisition and its authority
is not established; lifted content must be marked as requiring source
verification, not imported as resolved.

### 4.4 Constraints

- No production code changes.
- No Astronomy Engine, Ekādaśī Engine or Knowledge Base content changes.
- No resolution of GATE-FEST-001 or ST-CONFLICT-001 as a side effect.

---

## 5. Related open items

- `GATE-FEST-001` — Janmāṣṭamī two-day policy A/B/C; owner decision required
- `ST-CONFLICT-001` — reference location policy; recommend promoting to the
  formal gate table, since by scope it exceeds the three currently listed
- **Authoring-guideline / schema mismatch** —
  `KB-OBS-001-Observance-Authoring-Guidelines.md § Editorial States` defines
  `reviewed`; `observance.schema.json`, property `metadata.status`, enumerates
  `review`. An entry written per the guideline fails schema validation.
- **Coverage baseline not annotated as superseded** —
  `Festival-Coverage-Matrix.md § Totals` still reads
  *Rule research at least partial: 1*, and rows 009 and 010 retain `Rule=N`,
  while `Campaign-62.1-Research-Batch.md § Results` records rule analysis PASS
  for 002, 009, 010, 015 and 093.

---

## 6. Note on provenance

A prior review flagged a supposed loss of `viddha` and `fast` between campaigns
"61.0S" and 61.0U. No campaign labelled `61.0S` exists in the repository. The
label originated in an external synthesis document, not in the project tree.
The real campaign sequence for FEST-001 is:

```
61.0 → 61.0R → 61.0T → 61.0U → 61.0V → 61.0W
```

In 61.0U, `viddha` and `fast` are at their point of maximum resolution. That
part of the original flag is withdrawn. The loss documented above occurs at the
61.0 → 62.0 registry supersession instead.

**Corollary.** The same external synthesis carries a phantom campaign entry, a
planned-campaign block with no counterpart in the tree, a simple-tithi
candidate count of 22 against 24 tabulated, and a `content_access: 32/32` claim
against 8 Mahādvādaśī still in `draft`. It must not be used as a state
reference. `PROJECT_STATUS.md` is the source of truth.
