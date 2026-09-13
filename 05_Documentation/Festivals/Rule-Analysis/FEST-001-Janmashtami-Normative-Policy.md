# FEST-001 Janmāṣṭamī Normative Policy

Campaign: 61.0U  
Mode: normative policy resolution only  
Production code changes: 0  
Primary authority: Hari-bhakti-vilāsa 15.247-542 with Dig-darśinī-ṭīkā

## 1. Purpose And Authority

This document converts the audited Janmāṣṭamī passages into an explicit
calendar policy without presenting a project choice as śāstra.

Authority is applied in this order:

```text
1. Explicit Hari-bhakti-vilāsa rule
2. Dig-darśinī explanation
3. Older authority quoted by Hari-bhakti-vilāsa
4. Recognized calendrical or jyotiṣa convention needed to operationalize a term
5. Documented lineage rule
6. Modern calendars, validation only
```

Every conclusion below is marked as one of:

```text
TEXTUAL RULE
TRADITIONAL CALENDAR DEFINITION
IMPLEMENTATION POLICY
```

## 2. Bhādra And Śrāvaṇa

### Resolution

```text
TEXTUAL MONTH NAME:
Bhādra / Bhādrapada in the HBV synthesis and several quoted passages;
Śrāvaṇa / Nabhas in other quoted passages.

COMPUTATIONAL MONTH ID:
Bhādrapada

CALENDAR SYSTEM:
pūrṇimānta for the engine-facing festival identity

EQUIVALENCE RULE:
amānta Śrāvaṇa Kṛṣṇa pakṣa
=
pūrṇimānta Bhādrapada Kṛṣṇa pakṣa

DATE-CHANGING:
NO
```

Classification:

```text
HBV names in the source:
TEXTUAL RULE

Amānta/pūrṇimānta correspondence:
TRADITIONAL CALENDAR DEFINITION

Use VaishnavaMasa::Bhadrapada as the engine-facing ID:
IMPLEMENTATION POLICY aligned with the current pūrṇimānta Calendar Engine
```

Evidence:

- HBV 15.328, 331, and 347-349 use Bhādra/Bhādrapada/Prauṣṭhapada.
- HBV 15.329, 334, and 350 use Śrāvaṇa/Nabhas.
- The Dig-darśinī explicitly glosses `nabhas` as Śrāvaṇa and
  `prauṣṭhapada` as Bhādra, showing that both names are knowingly preserved.
- *The Indian Calendar*, article 51 (p. 30), states that bright fortnights
  retain the same name in both systems while a pūrṇimānta dark fortnight
  receives the name of the following amānta month. Under that correspondence,
  amānta Śrāvaṇa Kṛṣṇa and pūrṇimānta Bhādrapada Kṛṣṇa are the same fortnight.
- The existing Calendar Engine is explicitly pūrṇimānta. No change to it is
  authorized by this policy.

External calendrical reference:

```text
Robert Sewell and Śaṅkara Bālakṛṣṇa Dīkṣita,
The Indian Calendar (London: Swan Sonnenschein & Co., Ltd., 1896),
Table II, Part I, and article 51, printed p. 30.
```

Local artifact, hash, and acquisition provenance are recorded in
`07_Sources/Calendrical/janmashtami-technical-source-record.md`.

Disposition:

```text
RULE_CONFLICT-JAN-001:
RESOLVED AS NOMENCLATURE EQUIVALENCE
```

The API or documentation may retain both source labels, but the resolver must
not search two different lunations or emit two dates because of them.

## 3. Two-Day Priority

### 3.1 Textually Supported Priority Edges

| Priority ID | Higher condition | Lower condition | HBV | Dig-darśinī support | Older source represented | Classification | Confidence |
|---|---|---|---|---|---|---|---|
| JAN-PRI-001 | Aṣṭamī pure at sunrise | Aṣṭamī joined with Saptamī at sunrise | 15.358-373 | The commentary to 15.361 rejects the Ekādaśī Aruṇodaya analogy and uses sunrise for non-Ekādaśī tithis. | Brahma-vaivarta, Padma, Skanda, Yājñavalkya-smṛti | TEXTUAL RULE | HIGH |
| JAN-PRI-002 | Later Navamī-connected observance day | Earlier Saptamī-viddhā Aṣṭamī | 15.362, 365, 368, 373 | The adopted Vaiṣṇava treatment rejects the earlier contaminated date even when it has Rohiṇī. | Padma, Skanda | TEXTUAL RULE | HIGH |
| JAN-PRI-003 | Earlier day when pure Aṣṭamī and Rohiṇī extend across both days in the exact 15.374 case | Later day in that same base case | 15.374 | The first explanation gives the earlier date when its stated conjunction is present. | HBV synthesis | TEXTUAL RULE, subject to the alternative readings below | HIGH for the mūla statement |
| JAN-PRI-004 | An eligible pure Aṣṭamī without Rohiṇī | Omitting the annual vrata | 15.356-357 | Rohiṇī, niśītha, Navamī, Monday, and Wednesday enhance merit; they are not universal eligibility requirements. | Sūta statement and DDT synthesis | TEXTUAL RULE | HIGH |
| JAN-PRI-005 | Later clean/Navamī-linked date with Rohiṇī | Earlier clean date lacking Rohiṇī at niśītha, in the first DDT reading | DDT to 15.374 | Stated explicitly as one interpretation of the competing dates. | Dig-darśinī adjudication | TEXTUAL INTERPRETATION | HIGH for existence of the position |

### 3.2 Alternatives Preserved By The Commentary

The Dig-darśinī to 15.374 does not provide one universal edge for every pair of
clean candidates. It records all of the following:

```text
Position A:
The earlier pure date is retained when the earlier Aṣṭamī has the praised
Rohiṇī/niśītha conjunction described by the first explanation.

Position B:
The later date is selected when the earlier date lacks Rohiṇī at niśītha and
Rohiṇī is joined with the later Navamī-linked date.

Position C:
Under an alternative Monday/Wednesday reading, the later date can outrank an
earlier complete, pure Aṣṭamī even when the earlier date has Rohiṇī at
ardha-rātra.

Position D:
Some Vaiṣṇavas prefer the later date in the dual-day case even without those
additional conjunctions.
```

The commentary closes the discussion with:

```text
atra sampradāyācāra eva gatiḥ
```

The existence of these positions is a `TEXTUAL RULE/COMMENTARIAL FACT`. Choosing
one of them for Pancanga Engine is an `IMPLEMENTATION POLICY` representing a
declared sampradāya practice.

### 3.3 Exhaustive Decision Classes

The following classes exhaust the engine-facing two-day decision space without
pretending that every row already has a universal outcome.

| Case | Day 1 | Day 2 | Rohiṇī / niśītha facts | Result | Status |
|---|---|---|---|---|---|
| JAN-2D-001 | Saptamī at sunrise; Aṣṭamī later | Aṣṭamī or Navamī-connected clean date | Any | Day 2 | RESOLVED |
| JAN-2D-002 | Pure Aṣṭamī at sunrise | No eligible Aṣṭamī relationship on Day 2 | Any | Day 1 | RESOLVED |
| JAN-2D-003 | No Rohiṇī conjunction | Only one pure Aṣṭamī candidate | Absent | The pure candidate; do not omit vrata | RESOLVED |
| JAN-2D-004 | Pure Aṣṭamī | Pure/extended Aṣṭamī on the next sunrise | Aṣṭamī and Rohiṇī extend across both days as in 15.374 | Day 1 in the mūla; later-day alternatives remain in DDT | NORMATIVE_DECISION_REQUIRED |
| JAN-2D-005 | Pure Aṣṭamī, no Rohiṇī at niśītha | Clean or Navamī-linked later candidate | Rohiṇī joins the later date | Day 2 in the first DDT reading | RESOLVED WITHIN THAT READING; PROFILE NOT SELECTED |
| JAN-2D-006 | Complete pure Aṣṭamī with Rohiṇī at niśītha | Later candidate | Monday/Wednesday or other praised conjunction on later date | Earlier under one reading; later under another | NORMATIVE_DECISION_REQUIRED |
| JAN-2D-007 | Two otherwise clean candidates | Two otherwise clean candidates | No decisive conjunction | Earlier under the mūla-oriented reading; later in the recorded Vaiṣṇava practice | NORMATIVE_DECISION_REQUIRED |
| JAN-2D-008 | Aṣṭamī begins after sunrise and is Saptamī-viddhā | Aṣṭamī has ended before the next sunrise, leaving Navamī | Rohiṇī cannot rehabilitate Day 1 | Day 2 as the uncontaminated later observance day | RESOLVED |

Weekday, Rohiṇī, Aṣṭamī-at-niśītha, and joint Aṣṭamī-Rohiṇī-at-niśītha facts
must be computed and retained. They must not be assigned an unstated priority.

Disposition:

```text
RULE_CONFLICT-JAN-002:
UNRESOLVED

DATE-CHANGING:
YES

REQUIRED PROJECT ACTION:
Adopt and name a Gauḍīya two-day policy profile, or obtain a lineage rule that
selects one of the positions preserved by the Dig-darśinī.
```

## 4. Niśītha

### 4.1 Traditional Definition

```text
TRADITIONAL DEFINITION:
Niśītha, also called madhyarātri, is the central muhūrta of the local night.
The night is reckoned from local sunset to the following local sunrise and is
divided into fifteen equal night muhūrtas. Niśītha is the eighth division,
covering the midpoint of that night.

SOURCE:
Report of the Calendar Reform Committee, Government of India, CSIR, 1955,
pp. 108 and 160. The report defines niśītha/madhyarātri as two ghaṭikās
covering midnight and separately records each night muhūrta as one fifteenth
of the sunset-to-sunrise night.

CLASSIFICATION:
TRADITIONAL CALENDAR DEFINITION, operationalized from a Tier 4 technical
calendar authority. It does not replace the HBV rule.
```

The complete bibliographic citation is Government of India, Calendar Reform
Committee, *Report of the Calendar Reform Committee* (New Delhi: Council of
Scientific and Industrial Research, 1955), printed pp. 108 and 160. Its local
artifact, hash, and acquisition provenance are recorded in
`07_Sources/Calendrical/janmashtami-technical-source-record.md`.

Modern pañcāṅga methodology independently uses the same sunset-to-next-sunrise
middle-muhūrta construction. The PanchangTime page used during discovery is
Tier 6 corroboration only; it is not part of the normative evidence chain.

### 4.2 Computational Representation

For a location and civil date `D`, define:

```text
S = local sunset on D
R = local sunrise immediately following S
N = elapsed absolute time from S to R
M = S + N / 2
U = N / 15

NIŚĪTHA_START    = S + 7N / 15
NIŚĪTHA_END      = S + 8N / 15
NIŚĪTHA_MIDPOINT = M
```

Classification:

```text
Choosing the eighth of fifteen variable night muhūrtas:
TRADITIONAL CALENDAR DEFINITION

Representing the interval as [start, end):
IMPLEMENTATION POLICY
```

Inputs required:

```text
location latitude and longitude
timezone rules for D and D + 1
local sunset on D
following local sunrise
absolute instants for tithi and nakṣatra transitions
```

Timezone behavior:

```text
Compute sunset and following sunrise for the location using the applicable
timezone offset at each event. Perform interval arithmetic on absolute
instants. Convert to local civil time only for display.
```

Cross-midnight behavior:

```text
The interval normally crosses the civil-date boundary. The following sunrise
must belong to the first sunrise after S, not to a fixed 24-hour addition.
DST or offset changes do not change elapsed-time arithmetic.
```

Required predicates must remain distinct:

```text
tithi_at_nisitha_midpoint
nakshatra_at_nisitha_midpoint
tithi_overlaps_nisitha_interval
nakshatra_overlaps_nisitha_interval
joint_ashtami_rohini_overlap_with_nisitha
```

The two-day policy must state which predicate it uses. A developer may not
substitute midpoint presence for interval overlap, or the reverse.

Disposition:

```text
RULE_CONFLICT-JAN-004:
RESOLVED FOR INTERVAL CONSTRUCTION

Use within disputed two-day priority:
DEFERRED TO THE SELECTED TWO-DAY POLICY PROFILE
```

## 5. Janmāṣṭamī Pāraṇa

### 5.1 Common Textual Constraints

HBV 15.385-406 establishes:

```text
Pāraṇa is on the following day.
The ordinary Ekādaśī Hari-vāsara algorithm does not apply.
The relevant Aṣṭamī and Rohiṇī endings depend on the adopted reading.
HBV/DDT preserves more than one legitimate method.
No universal latest clock-time boundary is stated.
```

These are `TEXTUAL RULES`.

### 5.2 Required Policy Profiles

Pancanga Engine must model the alternatives as explicit
`PARANA_POLICY_PROFILE` values. It must not select a default silently.

| Profile | Earliest permitted instant | Dependencies | Textual basis | Notes |
|---|---|---|---|---|
| `FIRST_RELEVANT_FACTOR_END` | Following-day sunrise, or the first applicable Aṣṭamī/Rohiṇī ending if later | Aṣṭamī end, Rohiṇī end, following sunrise | 15.397-400; DDT | If only one factor is extended, wait for that factor; if both are extended, the first ending opens pāraṇa. |
| `BOTH_FACTORS_END_CAPABLE` | Following-day sunrise and the end of both Aṣṭamī and Rohiṇī | Both endings, following sunrise, capability class | 15.401; strict reading of 15.402-403 | For a person capable of continuing. |
| `FIRST_FACTOR_END_UNABLE` | Following-day sunrise, or the first of the two endings if later | Both endings, following sunrise, capability class | 15.401 | Expressly distinguished for a person unable to sustain the longer fast. |
| `TITHI_END_ROHINI_EXCEPTION` | Following-day sunrise, or Aṣṭamī end if later | Aṣṭamī end, following sunrise | Alternate DDT reading of 15.402; 15.404 | Does not wait independently for Rohiṇī end. |
| `FESTIVAL_END` | Following-day daytime after the festival completion supplied by the observance layer | Festival completion event | 15.405-407 | Not derivable from astronomy alone; requires an explicit ritual-event timestamp or a non-computable result. |

The formula phrase “following-day sunrise, or X if later” is an
`IMPLEMENTATION POLICY` that preserves both `pare'hni`/`divase` and the stated
factor ending. It must not be described as a verbatim HBV formula.

### 5.3 Output Contract

```text
EARLIEST PERMITTED:
Computed by the selected profile.

LATEST RELEVANT LIMIT:
No universal limit established by HBV 15.385-406. Return NONE / NOT SPECIFIED
rather than inventing a window end.

DEPENDENCIES:
Following sunrise; Aṣṭamī end; Rohiṇī end; capability class when applicable;
festival completion only for FESTIVAL_END.

EXCEPTIONS:
Yājñavalkya/Rohiṇī interpretations and the festival-end Vaiṣṇava practice are
preserved as separate profiles.
```

If no profile is supplied, the resolver must return:

```text
PARANA_POLICY_REQUIRED
```

It must not fall back to Ekādaśī pāraṇa and must not choose the earliest or
strictest profile by convenience.

Disposition:

```text
RULE_CONFLICT-JAN-003:
RESOLVED AS AN EXPLICIT PROFILE FAMILY

DEFAULT PROFILE:
NONE
```

## 6. Policy Decisions Register

| Decision ID | Conclusion | Classification | Authority | Status |
|---|---|---|---|---|
| JAN-POL-001 | Śrāvaṇa Kṛṣṇa amānta and Bhādrapada Kṛṣṇa pūrṇimānta identify the same lunation. | TRADITIONAL CALENDAR DEFINITION | Indian calendrical correspondence plus HBV labels | RESOLVED |
| JAN-POL-002 | Use `Bhadrapada` as the engine-facing month ID because the existing model is pūrṇimānta. | IMPLEMENTATION POLICY | Existing Calendar Engine contract | RESOLVED |
| JAN-POL-003 | Reject Saptamī-viddhā at sunrise even with Rohiṇī. | TEXTUAL RULE | HBV 15.358-373 and DDT | RESOLVED |
| JAN-POL-004 | Do not omit the vrata solely because Rohiṇī or niśītha conjunction is absent. | TEXTUAL RULE | HBV/DDT 15.357 | RESOLVED |
| JAN-POL-005 | Represent niśītha as the eighth of fifteen local night muhūrtas. | TRADITIONAL CALENDAR DEFINITION | Calendar Reform Committee technical definition | RESOLVED |
| JAN-POL-006 | Use half-open absolute-time intervals and localize only for display. | IMPLEMENTATION POLICY | Deterministic interval semantics | RESOLVED |
| JAN-POL-007 | Preserve Janmāṣṭamī pāraṇa alternatives as explicit profiles with no silent default. | IMPLEMENTATION POLICY preserving TEXTUAL ALTERNATIVES | HBV/DDT 15.397-406 | RESOLVED |
| JAN-POL-008 | Choose one two-day sampradāya profile as the project default. | IMPLEMENTATION POLICY | DDT to 15.374 requires sampradāyācāra | NORMATIVE_DECISION_REQUIRED |

## 7. Campaign Gate

```text
Bhādra/Śrāvaṇa mapping:
RESOLVED

Two-day priority:
UNRESOLVED

Niśītha traditional definition:
RESOLVED

Niśītha computational representation:
READY

Pāraṇa:
RESOLVED

Date-changing ambiguities:
1

Unrecorded implementation choices:
0

Source authority:
PASS

Invented doctrine:
0

Decision Spec:
NOT READY

Recommendation:
NORMATIVE DECISION REQUIRED
```
