# Campaign 61.0R: Janmāṣṭamī Implementation Gate Review

Status: GATE REVIEW COMPLETE  
Mode: Review only  
Decision: DO NOT IMPLEMENT YET

Historical note: this gate records the pre-acquisition state. Campaign 61.0T
subsequently acquired and verified HBV Vilāsa 15. The current gate is
`Campaign-61.0T-Janmashtami-Source-Gate-Review.md`; this report is retained for
traceability and must not be read as the current source inventory.

## 1. CANONICAL RULE

The repository does not yet contain a complete approved canonical rule for
determining the civil observance date of FEST-001 Śrī Kṛṣṇa Janmāṣṭamī.

The current proposed rule is only a provisional shape:

```text
Candidate festival:
Śrī Kṛṣṇa Janmāṣṭamī

Required māsa:
Bhādrapada / Hṛṣīkeśa mapping requires certification

Required pakṣa:
Kṛṣṇa

Required tithi:
Aṣṭamī

Likely rule class:
TITHI_PRESENT_DURING_WINDOW
SPECIAL_VAISNAVA_RULE
TITHI_NAKSHATRA_COMBINATION if Rohiṇī is confirmed
```

Required details not yet canonically resolved:

```text
Does sunrise matter?
UNRESOLVED

Does sunset matter?
UNRESOLVED

Does midnight / niśītha matter?
UNRESOLVED

Does Rohiṇī nakṣatra matter?
UNRESOLVED

What occurs if Aṣṭamī occupies two civil dates?
UNRESOLVED

What occurs if Rohiṇī appears on only one of those dates?
UNRESOLVED

What occurs if Aṣṭamī and Rohiṇī do not coincide?
UNRESOLVED

What are the priority rules between candidate days?
UNRESOLVED

What viddhā rules apply?
PARTIAL. HBV evidence indicates biddha-day handling matters for Janmāṣṭamī-like
Vaiṣṇava vows, but the full FEST-001 rule is not specified.

What is the vrata / fasting rule?
PARTIAL. PureBhakti operational evidence records fast until midnight for 2026,
but this is not yet an approved śāstric implementation rule.

What is the parāṇa rule?
PARTIAL. PureBhakti operational evidence records a next-day break-fast window
for 2026, but the governing rule is not yet documented in the local KB.
```

Conclusion:

```text
Canonical Janmāṣṭamī rule:
FAIL
```

## 2. SOURCE BASIS

| Rule component | Source | Source classification | Evidence reference | Confidence |
| --- | --- | --- | --- | --- |
| Janmāṣṭamī is a Vaiṣṇava vrata whose biddha handling matters | Hari-bhakti-vilāsa, Vilāsa 12, local raw text | Level A/B boundary pending normalization | FEST-JAN-C001 | MEDIUM |
| Rāma-navamī and Nṛsiṁha-caturdaśī belong to the same broader rule family | Hari-bhakti-vilāsa, Vilāsa 12, local raw text | Level A/B boundary pending normalization | FEST-JAN-C002 | MEDIUM |
| Janmāṣṭamī uses Kṛṣṇa Aṣṭamī in the operational reference | PureBhakti 2026 calendar extract | Level C, operational calendar | FEST-JAN-C004 | MEDIUM |
| Janmāṣṭamī fast until midnight | PureBhakti 2026 calendar extract | Level C, operational calendar | FEST-JAN-C005 | LOW for normative rule; MEDIUM for observed practice |
| Next-day break-fast window after Janmāṣṭamī | PureBhakti 2026 calendar extract | Level C, operational calendar | FEST-JAN-C006 | LOW for normative rule; MEDIUM for observed practice |
| General lunar festival tithi method | HBV Vilāsa 13 Appendix I local raw text | Level E/D pending normalization | FEST-JAN-REF-003 | LOW for Janmāṣṭamī-specific rule |
| Decisive midnight / niśītha window | Not yet present in approved local KB | Missing | PENDING-JAN-001 | FAIL |
| Rohiṇī nakṣatra priority | Not yet present in approved local KB | Missing | FEST-JAN-C008 | FAIL |
| Priority when Aṣṭamī spans two civil dates | Not yet present in approved local KB | Missing | PENDING-JAN-001 | FAIL |
| Priority when Aṣṭamī and Rohiṇī do not coincide | Not yet present in approved local KB | Missing | PENDING-JAN-002 | FAIL |
| Janmāṣṭamī parāṇa rule | Not yet present in approved local KB | Missing | PENDING-JAN-003 | FAIL |

Classification notes:

```text
Explicit source rule:
Janmāṣṭamī belongs to the Vaiṣṇava vrata class and biddha handling matters.

Inference:
Janmāṣṭamī likely requires a festival-rule engine rather than ordinary
sunrise-only resolution.

Observed calendrical practice:
PureBhakti 2026: Kṛṣṇa Aṣṭamī, fast until midnight, next-day break-fast.

Implementation decision:
None approved yet.
```

No modern operational practice is converted into a śāstric rule in this review.

## 3. RULE CONFLICTS

### RULE_CONFLICT-JAN-001

Position A:

```text
General lunar-festival handling may use sunrise presence when a tithi spans
more than one day.
```

Source A:

```text
HBV Vilāsa 13 Appendix I local raw text
Classification: Level E/D pending normalization
```

Position B:

```text
Janmāṣṭamī operational practice records fasting until midnight, implying a
night-window rule may be decisive.
```

Source B:

```text
PureBhakti 2026 operational calendar
Classification: Level C
```

Impact on computed date:

```text
Potentially high. A sunrise rule and a midnight / niśītha rule can select
different civil dates when Aṣṭamī spans adjacent days.
```

Recommended disposition:

```text
UNRESOLVED
```

### RULE_CONFLICT-JAN-002

Position A:

```text
The currently documented candidate rule can be expressed by Kṛṣṇa Aṣṭamī and a
ritual window.
```

Source A:

```text
FEST-001 Janmāṣṭamī Research Dossier
Classification: Internal research synthesis
```

Position B:

```text
Rohiṇī nakṣatra may affect priority, but its role is not certified in local
accepted sources.
```

Source B:

```text
FEST-JAN-C008 / PENDING-JAN-002
Classification: Pending source gap
```

Impact on computed date:

```text
Potentially high. If Rohiṇī priority is part of the adopted rule, a date with
Aṣṭamī plus Rohiṇī may outrank a date with Aṣṭamī alone.
```

Recommended disposition:

```text
UNRESOLVED
```

### RULE_CONFLICT-JAN-003

Position A:

```text
PureBhakti operational calendar provides a concrete 2026 Janmāṣṭamī date and
parāṇa window.
```

Source A:

```text
PureBhakti 2026 operational calendar
Classification: Level C
```

Position B:

```text
The project method requires a local approved normative rule before implementing
calendar behavior.
```

Source B:

```text
KB-REF-001 and Campaign 61.0 guardrails
Classification: Project governance
```

Impact on computed date:

```text
High if implementation copies operational fixtures without deriving the
underlying rule.
```

Recommended disposition:

```text
UNRESOLVED
```

## 4. REQUIRED ASTRONOMICAL INPUTS

| Input | Status | Notes |
| --- | --- | --- |
| sunrise | AVAILABLE NOW | Existing `calendar::sunrise`. |
| sunset | MISSING | No public sunset API found. |
| local civil midnight | DERIVABLE FROM EXISTING ENGINE | Needs timezone-aware civil window model. |
| niśītha window | MISSING | Requires approved definition and civil window support. |
| tithi start/end | DERIVABLE FROM EXISTING ENGINE | Current sunrise-to-sunrise transition API is insufficient for arbitrary windows. |
| tithi at specific instant | AVAILABLE NOW | Derivable from solar/lunar longitude and elongation. |
| tithi at sunrise | AVAILABLE NOW | Existing `calendar::tithi_at_sunrise`. |
| nakṣatra at specific instant | AVAILABLE NOW | Existing `nakshatra::index` from lunar longitude. |
| nakṣatra start/end | DERIVABLE FROM EXISTING ENGINE | Needs reusable interval finder. |
| Rohiṇī overlap | MISSING | Requires nakṣatra interval and approved rule. |
| lunar māsa | AVAILABLE NOW | Existing native Vaiṣṇava māsa API, with provisional ayanāṁśa status. |
| pakṣa | AVAILABLE NOW | Existing `AstronomicalTithi::paksha`. |
| timezone | PARTIAL | Present in RC1/city layer, not a core civil configuration model. |
| location | AVAILABLE NOW | Existing `GeoLocation`. |

Overall:

```text
Astronomical inputs:
NOT READY
```

## 5. EXISTING ENGINE CAPABILITY

### REUSE

```text
Solar apparent longitude
Lunar apparent longitude
Tithi from elongation
Pakṣa from tithi
Tithi at sunrise
Sunrise
Native Vaiṣṇava māsa
Instantaneous nakṣatra
GeoLocation
JulianDate interval representation
```

### ADAPTER REQUIRED

```text
Pañcāṅga State object gathering the above facts into one read-only daily state.
Civil date + timezone + location configuration.
Ritual windows such as midnight, noon, sunset, and next sunrise.
Festival result metadata distinct from Ekādaśī observance output.
```

### NEW CAPABILITY REQUIRED

```text
Public sunset calculation.
Arbitrary tithi interval finder.
Nakṣatra interval finder, if Rohiṇī is adopted.
Rohiṇī / tithi overlap evaluation, if adopted.
Festival-specific parāṇa policy.
Rule status output: IMPLEMENTED, RESEARCH_REQUIRED, RULE_CONFLICT, NOT_SUPPORTED.
```

No astronomical calculation should be duplicated.

## 6. PROPOSED RESOLVER ALGORITHM

The following pseudocode is not production code and must not be implemented
until the canonical rule is approved.

```text
function resolve_janmashtami(year, location, civil_config):
    candidates = candidate civil dates around expected Bhādrapada Kṛṣṇa Aṣṭamī

    evaluated = []

    for date in candidates:
        state = build_pancanga_state(date, location, civil_config)

        facts = {
            masa: state.vaishnava_masa,
            paksha: state.paksha,
            tithi_at_sunrise: state.tithi_at_sunrise,
            tithi_at_midnight: state.tithi_at(local_midnight_or_approved_window),
            tithi_intervals: state.tithi_intervals(approved_window),
            nakshatra_at_window: state.nakshatra_at(approved_window),
            nakshatra_intervals: state.nakshatra_intervals(approved_window),
            rohini_overlap: overlap(Aṣṭamī, Rohiṇī, approved_window)
        }

        if canonical_rule_missing:
            return RULE_CONFLICT / RESEARCH_REQUIRED

        rule_result = evaluate_approved_janmashtami_rule(facts)

        evaluated.append({
            date,
            facts,
            rule_result
        })

    selected = compare_candidates_using_approved_priority(evaluated)

    fasting = derive_fast_metadata(selected, approved_fasting_rule)
    parana = derive_parana_metadata(selected, approved_parana_rule)

    return FestivalResult(
        festival_id = FEST-001,
        selected_observance_date = selected.date,
        rule_source = approved_source_id,
        fasting = fasting,
        parana = parana,
        conflicts = []
    )
```

Required flow:

```text
candidate dates
↓
astronomical state
↓
rule evaluation
↓
priority comparison
↓
selected observance date
↓
fast/parāṇa metadata
```

## 7. TEST VECTORS

The following vectors are proposed for future certification. Expected results
must remain pending unless backed by accepted authority.

| Test ID | Case type | Location | Year/date | Intended validation | Expected result |
| --- | --- | --- | --- | --- | --- |
| JAN-TV-001 | Normal year | Buenos Aires | 2026-09-04 operational fixture | Validate Kṛṣṇa Aṣṭamī handling against PureBhakti 2026 once rule is approved. | EXPECTED_RESULT_PENDING |
| JAN-TV-002 | Normal year | Valencia | Year TBD | Validate same rule in northern hemisphere civil configuration. | EXPECTED_RESULT_PENDING |
| JAN-TV-003 | Normal year | Māyāpura or reference location TBD | Year TBD | Validate against adopted Gauḍīya reference calendar. | EXPECTED_RESULT_PENDING |
| JAN-TV-004 | Boundary case | Location TBD | Date where Aṣṭamī begins after sunrise but before midnight | Determine whether midnight / niśītha outranks sunrise. | EXPECTED_RESULT_PENDING |
| JAN-TV-005 | Boundary case | Location TBD | Date where Aṣṭamī ends before midnight | Determine whether partial presence during approved window is enough. | EXPECTED_RESULT_PENDING |
| JAN-TV-006 | Aṣṭamī over two sunrises | Location TBD | Date TBD | Validate priority when Aṣṭamī touches two civil dates. | EXPECTED_RESULT_PENDING |
| JAN-TV-007 | Rohiṇī relevant | Location TBD | Date with Rohiṇī on only one candidate day | Validate nakṣatra priority if adopted. | EXPECTED_RESULT_PENDING |
| JAN-TV-008 | Timezone/location contrast | Buenos Aires and Valencia | Same festival year TBD | Validate date movement and civil-window handling by longitude/timezone. | EXPECTED_RESULT_PENDING |

Test strategy:

```text
PASS as a plan.
FAIL as executable certification until expected dates are sourced.
```

## 8. 61.1 CHANGE SET

Minimum exact change set before FEST-001 can be implemented:

| Path / module | Change type | Reason |
| --- | --- | --- |
| `03_Source/rust/crates/pancanga-engine/src/calendar/` | MODIFY | Add reusable non-festival-specific tithi interval utility for arbitrary windows. |
| `03_Source/rust/crates/pancanga-engine/src/calendar/sunrise.rs` or sibling module | MODIFY | Add public sunset support with validation parity to sunrise. |
| `03_Source/rust/crates/pancanga-engine/src/calendar/` | NEW | Add civil ritual window model: sunrise, noon, sunset, midnight, next sunrise. |
| `03_Source/rust/crates/pancanga-engine/src/calendar/` | NEW | Add read-only Pañcāṅga State object that aggregates existing engine facts. |
| `03_Source/rust/crates/pancanga-engine/src/astronomy/nakshatra.rs` or calendar interval module | MODIFY / NEW | Add nakṣatra interval finder only if approved Janmāṣṭamī or Vāmana rules require it. |
| `03_Source/rust/crates/pancanga-engine/src/festival/` | MODIFY | Add rule status types: IMPLEMENTED, RESEARCH_REQUIRED, RULE_CONFLICT, NOT_SUPPORTED. |
| `05_Documentation/Knowledge-Base/` | NEW | Add approved FEST-001 Janmāṣṭamī rule before resolver implementation. |
| `04_Tests/Festivals/` | TEST ONLY | Add fixture and boundary tests only after expected results are backed by authority. |

Files explicitly not to modify for 61.1:

```text
Ekādaśī Engine
Mahādvādaśī logic
Existing Parāṇa logic
Observance Resolver behavior
Published Ekādaśī Knowledge Base
Desktop packaging
```

## 9. IMPLEMENTATION GATE

```text
Canonical rule:
FAIL

Source authority:
FAIL

Rule conflicts:
UNRESOLVED

Astronomical inputs:
NOT READY

Existing engine reuse:
PASS

Test strategy:
PASS

Invented doctrine:
0

Recommendation:
DO NOT IMPLEMENT YET
```

Gate conclusion:

```text
Campaign 61.0 prepared the correct foundation, but FEST-001 Śrī Kṛṣṇa
Janmāṣṭamī must not be implemented until the canonical rule, Rohiṇī status,
candidate-day priority, viddhā handling, and festival parāṇa rule are approved
from accepted sources.
```
