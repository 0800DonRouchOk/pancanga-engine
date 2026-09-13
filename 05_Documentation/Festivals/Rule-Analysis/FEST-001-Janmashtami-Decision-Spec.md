# FEST-001 Janmāṣṭamī Decision Specification

Campaign: 61.0U  
Status: NOT READY - ONE DATE-CHANGING POLICY DECISION REMAINS  
Scope: machine-oriented normative specification, no production code

## 1. Purpose

This specification defines every resolved input, derived fact, decision, and
failure state for FEST-001. It deliberately does not choose among the
sampradāya alternatives preserved by the Dig-darśinī for two clean candidate
dates.

A developer may implement the resolved common logic from this document. The
resolver must not emit a final date for an unresolved dual-candidate case until
the project adopts a named two-day policy profile.

## 2. Normative Identity

```text
canonical_id:
FEST-001

canonical_name:
Śrī Kṛṣṇa Janmāṣṭamī

engine_month_system:
pūrṇimānta

engine_month_id:
Bhādrapada

equivalent_source_label:
Śrāvaṇa Kṛṣṇa under amānta naming

pakṣa:
Kṛṣṇa

tithi:
Aṣṭamī
```

Month-label equivalence changes metadata only. It never creates an additional
candidate date.

## 3. Required Inputs

### 3.1 Location And Civil Time

```text
latitude
longitude
timezone identifier with historical offset rules
civil date under evaluation
```

### 3.2 Solar Events

```text
sunrise for each candidate civil date
sunset for each candidate civil date
first sunrise following each sunset
```

Sunrise is a textual purity boundary. Sunset and the next sunrise construct
niśītha; sunset is not otherwise a Janmāṣṭamī candidate-selection boundary in
HBV 15.328-406.

### 3.3 Lunar And Calendar Events

```text
tithi at each candidate sunrise
Aṣṭamī start and end as absolute instants
Navamī start and end as absolute instants
Rohiṇī start and end as absolute instants
pakṣa
native Vaiṣṇava māsa
weekday for each candidate
```

### 3.4 Required Policy Inputs

```text
two_day_policy_profile:
REQUIRED when more than one clean candidate survives

parana_policy_profile:
REQUIRED when pāraṇa metadata is requested

fasting_capability:
REQUIRED only by a profile that distinguishes capable and unable observers

festival_completion_instant:
REQUIRED only by FESTIVAL_END pāraṇa profile
```

There is no implicit default for either policy profile.

## 4. Derived Time Windows

For candidate date `D`:

```text
S = sunset(D)
R = first sunrise after S
N = R - S in absolute elapsed time

nisitha.start    = S + 7N / 15
nisitha.end      = S + 8N / 15
nisitha.midpoint = S + N / 2
nisitha.interval = [start, end)
```

Required validation:

```text
S < nisitha.start < nisitha.midpoint < nisitha.end < R
nisitha.end - nisitha.start = N / 15
```

All arithmetic uses absolute instants. Local offsets are applied separately to
each sunrise and sunset. A civil-date or DST transition inside the interval
must not turn it into a fixed-clock calculation.

## 5. Candidate Facts

For every adjacent candidate date, derive and retain:

```text
ashtami_at_sunrise
saptami_at_sunrise
navami_at_sunrise
ashtami_interval
rohini_interval
ashtami_overlaps_nisitha
rohini_overlaps_nisitha
ashtami_at_nisitha_midpoint
rohini_at_nisitha_midpoint
joint_ashtami_rohini_overlap
joint_ashtami_rohini_nisitha_overlap
rohini_present_at_any_time_with_ashtami
weekday
```

Predicates must not be substituted for one another. In particular:

```text
Rohiṇī overlaps Aṣṭamī
!=
Rohiṇī and Aṣṭamī overlap within niśītha
!=
Rohiṇī and Aṣṭamī are both present at the niśītha midpoint
```

## 6. Purity Classification

```text
if saptami_at_sunrise:
    purity = SAPTAMI_VIDDHA
else if ashtami_at_sunrise:
    purity = PURE_ASHTAMI
else if navami_at_sunrise and Ashtami ended after the prior sunrise:
    purity = LATER_NAVAMI_CONNECTED
else:
    purity = NOT_A_CANDIDATE
```

This is documentary pseudocode, not production code.

Normative consequences:

```text
SAPTAMI_VIDDHA:
REJECT, even if Rohiṇī is present.

PURE_ASHTAMI:
ELIGIBLE.

LATER_NAVAMI_CONNECTED:
ELIGIBLE when it is the prescribed uncontaminated alternative to a rejected
Saptamī-viddhā Aṣṭamī.

NOT_A_CANDIDATE:
REJECT.
```

The Ekādaśī Aruṇodaya threshold must not be used.

## 7. Jayantī Metadata

```text
if positive-duration intersection(Ashtami, Rohini) exists:
    jayanti = true
else:
    jayanti = false
```

Additional metadata may report:

```text
joint_overlap_at_nisitha
joint_presence_at_nisitha_midpoint
monday_conjunction
wednesday_conjunction
navami_connection
```

`jayanti = false` does not cancel the base Janmāṣṭamī vrata.

## 8. Candidate Selection

### 8.1 Resolved Common Logic

```text
locate the single Bhādrapada Kṛṣṇa Aṣṭamī lunation
accept Śrāvaṇa Kṛṣṇa only as its amānta source label
build adjacent civil candidates intersecting the Aṣṭamī event
compute sunrise purity for each candidate
reject every Saptamī-viddhā candidate
retain all relevant Rohiṇī, niśītha, Navamī, and weekday facts

if exactly one eligible candidate remains:
    select it

if no pure candidate remains because Aṣṭamī is Saptamī-viddhā on the earlier
date and has ended before the next sunrise:
    select the later Navamī-connected observance date

if more than one clean candidate remains:
    require two_day_policy_profile
```

### 8.2 Unresolved Policy Branch

The policy profile must select one of the documented Dig-darśinī positions.
Campaign 61.0U does not authorize a default.

Required profile contract:

```text
profile_id
documented sampradāya or interpretive position
priority edges used
required niśītha predicate
weekday role
Rohiṇī role
Navamī role
source references
approval record
```

Until that record exists, the required outcome is:

```text
NORMATIVE_POLICY_REQUIRED

conflict_id:
RULE_CONFLICT-JAN-002

candidate_dates:
all surviving clean candidates

facts:
all facts needed to compare the documented positions
```

The engine must not choose the earlier or later date as an engineering
fallback.

## 9. Fasting Metadata

The selected result must report:

```text
vrata_required:
true under the adopted HBV rule

fasting_scope:
full Janmāṣṭamī vrata

preliminary_day:
Saptamī discipline documented in HBV

observance_day:
nirāhāra / fasting and festival observance

completion_day:
following-day pāraṇa under an explicit profile
```

The resolver must not translate this into “fast only until civil midnight”
without a separately approved operational profile.

## 10. Pāraṇa Decision

### 10.1 Shared Preconditions

```text
selected observance date exists
following sunrise exists
Aṣṭamī end is known
Rohiṇī end is known when the selected observance uses Rohiṇī metadata
parana_policy_profile is supplied
```

### 10.2 Profiles

```text
FIRST_RELEVANT_FACTOR_END
    only Aṣṭamī extended -> max(following sunrise, Aṣṭamī end)
    only Rohiṇī extended -> max(following sunrise, Rohiṇī end)
    both extended        -> max(following sunrise, min(Aṣṭamī end, Rohiṇī end))
    neither extended     -> following sunrise

BOTH_FACTORS_END_CAPABLE
    earliest = max(following sunrise, Aṣṭamī end, Rohiṇī end)

FIRST_FACTOR_END_UNABLE
    earliest = max(following sunrise, min(Aṣṭamī end, Rohiṇī end))

TITHI_END_ROHINI_EXCEPTION
    earliest = max(following sunrise, Aṣṭamī end)

FESTIVAL_END
    earliest = following-day festival completion instant
    if that instant is not supplied, return NON_COMPUTABLE_RITUAL_BOUNDARY
```

For every profile:

```text
latest = NOT_SPECIFIED_BY_SOURCE
hari_vasara_rule = NOT_APPLICABLE
```

The `max` and `min` expressions above are implementation translations of the
documented profile constraints. They are not quotations from HBV.

## 11. Output Requirements

Every successful result must expose enough evidence to audit the decision:

```text
festival_id
selected_civil_date
engine_month_id
source_month_labels
paksha
tithi
purity_class
jayanti
nisitha_start
nisitha_end
nisitha_midpoint
ashtami_nisitha_fact
rohini_nisitha_fact
joint_nisitha_fact
two_day_policy_profile
priority_edges_applied
parana_policy_profile
parana_earliest
parana_latest_status
source_rule_ids
```

If the date profile is missing in a contested case, return the conflict data
instead of a successful observation date.

## 12. Required Invariants

```text
No Saptamī-viddhā date is selected.
Rohiṇī absence never cancels the base vrata.
Amānta/pūrṇimānta labels never create duplicate festivals.
Civil 00:00 is never substituted for niśītha.
Niśītha is location- and date-dependent.
Janmāṣṭamī pāraṇa never calls the Ekādaśī Hari-vāsara algorithm.
No pāraṇa profile is selected implicitly.
No two-day profile is selected implicitly.
Every selected profile is returned in the audit output.
```

## 13. Test Classes Required After The Gate Opens

```text
single pure Aṣṭamī candidate without Rohiṇī
single pure Aṣṭamī candidate with Rohiṇī
Saptamī-viddhā earlier date and clean later date
Aṣṭamī kṣaya across sunrise with later Navamī-connected observance
two clean candidates with Rohiṇī on both
two clean candidates with Rohiṇī only on the later date
two clean candidates with Rohiṇī at earlier niśītha
Monday/Wednesday competing-day case
niśītha interval crossing civil midnight
timezone offset transition during the local night
all computable pāraṇa profiles
FESTIVAL_END without a ritual timestamp
```

Expected dates for disputed dual-clean-candidate tests remain pending until the
project approves the two-day policy profile.

## 14. Readiness Gate

```text
Resolved common logic:
READY

Month mapping:
READY

Niśītha interval:
READY

Pāraṇa profile family:
READY

Two-day project profile:
MISSING

Date-changing ambiguities:
1

Developer discretion permitted:
0

Decision Spec:
NOT READY

Blocking decision:
Select and approve the project two-day sampradāya policy represented by
RULE_CONFLICT-JAN-002.
```
