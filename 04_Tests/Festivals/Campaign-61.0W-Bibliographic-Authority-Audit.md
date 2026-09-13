# Campaign 61.0W: Bibliographic Authority Audit

Mode: governance and retroactive source audit  
Date: 2026-09-13  
Engine modification: NONE

## Audit Question

Does every existing religious, doctrinal, and date-changing claim distinguish
the authority of a work and edition from the website or repository through
which it was acquired?

## Scope

The audit reviewed:

- HBV-EK-001 through HBV-EK-005;
- the eight supported Mahādvādaśī classifications;
- Janmāṣṭamī research, rule matrices, and policy documents;
- the initial Festival Registry;
- the 24 published ordinary Ekādaśī content records and their evidence logs;
- operational calendars and web material referenced by those documents.

No observance content, calendar rule, engine code, or schema was changed.

## Source Classification Inventory

| Source or artifact | v2 classification | Permitted role | Normative authority |
|---|---|---|---|
| *Śrī Hari-bhakti-vilāsa*, Vilāsa 12, Haridāsa Śāstrī ed., mūla | Tier 1 | Ekādaśī candidate, viddhā, displacement, Mahādvādaśī source text | YES |
| Dig-darśinī-ṭīkā to Vilāsa 12 | Tier 2 | Traditional interpretation and technical clarification | YES within commentary scope |
| *Śrī Hari-bhakti-vilāsa*, Vilāsa 13 as represented in the local English translation | Tier 4 reading witness; underlying HBV passages remain Tier 1/2 | Reading, passage discovery, translation comparison | NOT BY ITSELF for a critical new rule |
| *Śrī Hari-bhakti-vilāsa*, Vilāsa 15, Haridāsa Śāstrī ed., mūla | Tier 1 | Janmāṣṭamī rule and procedure | YES |
| Dig-darśinī-ṭīkā to Vilāsa 15 | Tier 2 | Janmāṣṭamī interpretation and conflict analysis | YES within commentary scope |
| *The Indian Calendar* (Sewell and Dīkṣita, 1896) | Tier 4 | Amānta/pūrṇimānta nomenclature | TECHNICAL ONLY |
| *Report of the Calendar Reform Committee* (1955) | Tier 4 | Niśītha and night-muhūrta operational definition | TECHNICAL ONLY |
| PureBhakti calendars | Tier 5 | External validation fixtures | NO |
| SCS Math calendars | Tier 5 | External validation fixtures | NO |
| GCal output | Tier 5 | External validation fixtures | NO |
| PanchangTime methodology page | Tier 6 | Discovery and corroboration | PROHIBITED |
| Gaudiya Grantha Mandira / Jiva Institute pages | Tier 6 acquisition metadata | Catalog and artifact acquisition | PROHIBITED |
| Ramanuja Das Spanish Ekādaśī compilation | Tier 6 for authority; translation/discovery witness | Locate narratives and named source leads | PROHIBITED as independent authority |
| Project KB, registries, evidence logs, and tests | Internal derivation artifacts | Preserve specification and implementation trace | Inherit authority from recorded source chain |

## Date-Changing Rule Traceability

| Rule family | Text and edition | Exact passage | Commentary | Computational translation | Result |
|---|---|---|---|---|---|
| HBV-EK-001 Candidate Ekādaśī | HBV Vilāsa 12, Haridāsa Śāstrī ed. | 12.1, 12.2, 12.40, 12.199, 12.315 | Dig-darśinī present | Explicit in rule document | PASS |
| HBV-EK-002 Viddhā | Same | 12.199, 12.315-320, 12.342-343 | Dig-darśinī present | Explicit Aruṇodaya predicate | PASS |
| HBV-EK-003 Displacement | Same | 12.317-320, 12.324 | Dig-darśinī present | Explicit displacement rule | PASS |
| HBV-EK-004 Mahādvādaśī | Same, with Vilāsa 13 reading witness | 12.372-392; 13.265-537 | Dig-darśinī represented | Eight classifications documented | PASS for rules grounded in Vilāsa 12; Vilāsa 13-only elaborations require mūla verification before future expansion |
| HBV-EK-005 Hari-vāsara | HBV Vilāsa 13, Haridāsa Śāstrī-based translation witness | 13.257-259 | DDT represented; Sanskrit of 13.257 retained | First-quarter predicate documented | PASS for 13.257; source-text verification remains desirable for quoted 13.258-259 |
| HBV-EK-005 Dvādaśī end and short-Dvādaśī contingency | Same | 13.236, 13.238-256 | Translation witness only in the current local artifact | End and contingency behavior documented | `NORMATIVE CLAIM: NOT READY` under the v2 translation gate until mūla/commentary are locally verified |
| FEST-001 base eligibility and Saptamī-viddhā | HBV Vilāsa 15, Haridāsa Śāstrī ed. | 15.247-396 | Dig-darśinī present | Rule matrix and policy explicit | PASS |
| FEST-001 niśītha representation | HBV 15 plus two Tier 4 works | HBV 15.328-379; technical printed pp. 108, 160 | Dig-darśinī present | Central local night muhūrta documented | PASS |
| FEST-001 clean two-day priority | HBV 15 | 15.352-374 | Multiple Dig-darśinī positions | No silent default | `NORMATIVE CLAIM: NOT READY`; conflict is explicitly unresolved and unimplemented |

### Traceability Interpretation

No accepted date-changing rule is derived only from a Tier 5 calendar or Tier
6 webpage. Every implemented rule identifies its work, edition, passage, and
computational representation.

One implemented Parāṇa rule family nevertheless fails the stronger v2
translation gate because the current local Vilāsa 13 witness does not expose
the mūla and commentary for all supporting passages. This is a bibliographic
verification gap, not evidence that the calculation is incorrect.

## Operational Calendar Audit

PureBhakti, SCS Math, and GCal are now uniformly Tier 5. Existing comparison
reports use them as fixtures and classify differences before changing code.
No audited rule document treats their dates as independent śāstric authority.

The Festival Registry mentions operational fasting and break-fast behavior for
FEST-001 and FEST-007, but explicitly marks the normative rules
`RESEARCH_REQUIRED`. These entries are observations, not implementation
authority.

## Janmāṣṭamī Audit

The Janmāṣṭamī source chain now separates:

```text
Normative authority:
HBV Vilāsa 15 mūla and Dig-darśinī, Haridāsa Śāstrī edition

Technical definitions:
The Indian Calendar; Report of the Calendar Reform Committee

Validation:
PureBhakti and SCS Math

Acquisition/discovery only:
Gaudiya Grantha Mandira / Jiva Institute; PanchangTime
```

Raw URLs were removed from the normative matrices and retained in source
records only. The unresolved clean two-day choice remains an explicit policy
gate and has not been implemented.

## Observance Library Finding

The 24 ordinary Ekādaśī JSON records are structurally complete and marked
`published`. Their līlā authoring pipeline uses the local Spanish compilation
*Ekadashi: El Día Sagrado del Señor Hari*. Twenty-three ordinary evidence logs
name that PDF directly; the Kāmikā golden sample reaches it through its dossier
and source register.

Under KB-REF-001 v2:

- the compilation is a translation/discovery witness, not Tier 1 authority;
- printed Purāṇa names are unverified source leads until an identified edition
  and passage are inspected;
- claims of history, benefits, and procedure cannot retain published authority
  solely from that chain;
- 24 content records require bibliographic remediation and re-review;
- the eight Mahādvādaśī content placeholders remain `draft` and do not create
  the same publication-state conflict.

Per campaign instructions, no JSON, Evidence Log, dossier, or published text
was altered. This audit records the problem instead of rewriting evidence
retroactively.

```text
CONTENT AUTHORITY RE-AUDIT:
OPEN

Affected published records:
24

Engine impact:
NONE
```

## Tier 5/6-Only Normative Findings

| Finding | Count | Disposition |
|---|---:|---|
| Implemented date-changing rules authorized only by Tier 5 or Tier 6 | 0 | PASS |
| Festival Registry rules authorized only by Tier 5 or Tier 6 | 0 | All such observations remain `RESEARCH_REQUIRED` |
| Published content records whose doctrinal/history authority currently terminates in an unverified compilation | 24 | REMEDIATION REQUIRED; content unchanged |
| Date-changing rule families failing the translation gate | 1 partial family | Acquire/verify HBV Vilāsa 13 mūla and Dig-darśinī for 13.236, 13.238-256 |

## Result

```text
Bibliographic policy:
PASS

All normative sources classified:
PASS

Web-only implemented date-changing claims:
0

Untraceable date-changing rules:
0

Date-changing rule families failing translation verification:
1 PARTIAL (HBV-EK-005)

Published content records requiring authority remediation:
24

Invented doctrine in Campaign 61.0W:
0

Existing Engine:
UNCHANGED

Campaign 61.0W:
POLICY PASS
RETROACTIVE CONTENT AUTHORITY GATE OPEN
```

A clean global statement of `Web-only normative claims: 0` would conceal the
24 content records identified above. The narrower and verified statement is
that no implemented date-changing engine rule is web-only.
