# FEST-001 Śrī Kṛṣṇa Janmāṣṭamī Canonical Rule

Campaign: 61.0U  
Authority: Hari-bhakti-vilāsa 15.247-542 with Dig-darśinī-ṭīkā  
Source status: ACQUIRED AND VERIFIED  
Rule status: RESOLVED EXCEPT TWO-DAY SAMPRADĀYA PRIORITY  
Implementation gate: NORMATIVE DECISION REQUIRED

Detailed policy and machine-oriented decomposition:

```text
FEST-001-Janmashtami-Normative-Policy.md
FEST-001-Janmashtami-Decision-Spec.md
```

## 1. Canonical Rule Established By The Source

The acquired source establishes the following rule components directly:

```text
Festival:
Śrī Kṛṣṇa Janmāṣṭamī

Lunar context:
Bhādrapada Kṛṣṇa pakṣa in the engine's pūrṇimānta model; this is the same
lunation called Śrāvaṇa Kṛṣṇa under amānta nomenclature.

Base tithi:
Aṣṭamī

Base eligibility:
A pure Aṣṭamī is observed even without Rohiṇī, niśītha conjunction,
Monday, Wednesday, or Navamī conjunction.

Saptamī-viddhā:
Reject an Aṣṭamī contaminated by Saptamī at the sunrise purity boundary,
even if Rohiṇī is present. Do not apply the Ekādaśī Aruṇodaya threshold by
analogy.

Rohiṇī:
Not mandatory for the base Janmāṣṭamī vrata. Its overlap with Aṣṭamī is
merit-enhancing and classifies the conjunction as Jayantī.

Niśītha / ardha-rātra:
Aṣṭamī-Rohiṇī at the middle of the night is especially praised and is a
worship/birth window. It is not made a universal prerequisite for the base
vrata. The computable interval is the eighth of fifteen equal divisions of the
local sunset-to-following-sunrise night.

Vrata:
An annual observance including fasting. The vow may be taken after one meal
on Saptamī or on Aṣṭamī morning. The full HBV vow is not merely a fast ending
at civil midnight.

Pāraṇa:
Performed on the following day under an explicit source-backed policy profile.
The alternatives are retained as profiles; no default may be selected silently.
```

Primary references:

```text
Basic rule:
HBV 15.247, 262-268, 328

Rohiṇī and Jayantī:
HBV 15.329-357

Saptamī-viddhā:
HBV 15.358-374

Vrata procedure:
HBV 15.375-386, 408-414, 464-465, 500

Pāraṇa:
HBV 15.374, 385-407
```

## 2. Required Calendar Facts

| Component | Canonical result | Status |
|---|---|---|
| Māsa | Bhādrapada in pūrṇimānta; equivalent to Śrāvaṇa Kṛṣṇa in amānta naming | RESOLVED |
| Pakṣa | Kṛṣṇa | RESOLVED |
| Tithi | Aṣṭamī | RESOLVED |
| Sunrise | Decisive for Saptamī-viddhā purity | RESOLVED |
| Sunset | No candidate-selection role found in HBV 15.328-406 | NOT REQUIRED BY CURRENT RULE |
| Niśītha | Eighth of fifteen equal local night muhūrtas, centered on the sunset-to-next-sunrise midpoint | RESOLVED |
| Rohiṇī | Merit-enhancing and Jayantī-classifying, not mandatory | RESOLVED |
| Saptamī-viddhā | Earlier contaminated candidate rejected | RESOLVED |
| Navamī-yukta | Permitted and sometimes praised as the later candidate | RESOLVED IN PRINCIPLE |
| Two-day priority | Multiple subcases and practices remain | UNRESOLVED |
| Fast | Annual full vrata with fasting | RESOLVED |
| Pāraṇa | Next day; source alternatives are mandatory explicit profiles with no default | RESOLVED AS PROFILED POLICY |

## 3. Rohiṇī And Jayantī

HBV 15.329-340 repeatedly says that even a small conjunction of Aṣṭamī and
Rohiṇī is highly meritorious and is called Jayantī. The Dig-darśinī then makes
the controlling distinction at 15.357:

```text
Rohiṇī, midnight, Navamī, and weekday conjunctions provide special merit.
They are not indispensable conditions for the Janmāṣṭamī vrata.
Without them, pure Aṣṭamī is still observed.
```

Therefore the resolver must distinguish two different facts:

```text
Janmāṣṭamī eligibility:
Pure Bhādra Kṛṣṇa Aṣṭamī

Jayantī classification:
Aṣṭamī with an actual Rohiṇī overlap
```

Rohiṇī at niśītha is a stronger conjunction, but the source does not make that
stronger conjunction a requirement for the base observance.

## 4. Niśītha

The source uses both `niśītha` and `ardha-rātra`; it does not equate either
term with `00:00` civil time. Campaign 61.0U applies the recognized traditional
calendar definition needed to operationalize that term:

```text
ritual night = local sunset to the following local sunrise
night muhūrta = one fifteenth of that elapsed night
niśītha = the eighth night muhūrta
midpoint = midpoint of local sunset and following sunrise
```

For sunset `S`, following sunrise `R`, and `N = R - S`:

```text
start = S + 7N / 15
end = S + 8N / 15
midpoint = S + N / 2
```

This is supported operationally by the *Report of the Calendar Reform
Committee* (Government of India, CSIR, 1955), which defines
niśītha/madhyarātri as two ghaṭikās covering midnight and records night
muhūrtas as fifteenths of the local night. The interval formula is a declared
implementation representation of that traditional definition, not a new HBV
rule.

Status:

```text
Traditional ritual role:
RESOLVED

Traditional definition:
RESOLVED

Computational interval:
READY

niśītha = civil 00:00:
PROHIBITED
```

Whether a disputed two-day profile tests midpoint presence or positive overlap
with the niśītha interval must be declared by that profile.

## 5. Saptamī-Viddhā

HBV 15.358-370 establishes the strict Vaiṣṇava rule:

```text
If Aṣṭamī is joined with Saptamī at the applicable purity boundary,
reject that candidate even when Rohiṇī is present.

Prefer the later, Navamī-connected date rather than observe the
Saptamī-contaminated date.
```

The Dig-darśinī to HBV 15.361 rejects an Aruṇodaya analogy with Ekādaśī and
explains that non-Ekādaśī tithis are complete from sunrise. The implementation
must therefore not reuse the Ekādaśī Daśamī-at-Aruṇodaya rule.

Contrary statements permitting Saptamī-viddhā are quoted in the commentary to
HBV 15.371 and assigned to a different subject; they are not adopted as the
Vaiṣṇava Janmāṣṭamī rule.

## 6. Two-Day Candidate Rule

The source resolves some cases but not the complete deterministic hierarchy.

### Resolved Cases

```text
Case A:
The earlier date is Saptamī-viddhā at sunrise.

Result:
Reject the earlier date and observe the later date.

Case B:
Rohiṇī is absent from every candidate.

Result:
Do not omit the vrata. Observe the eligible pure Aṣṭamī.

Case C:
Pure Aṣṭamī and Rohiṇī extend across both days in the situation stated by
HBV 15.374.

Mūla result:
Observe the earlier date and perform pāraṇa after the stated termination.
```

### Unresolved Cases

The Dig-darśinī to HBV 15.374 records more than one interpretation for clean
competing dates:

```text
Position A:
Prefer the earlier pure Aṣṭamī when its conjunctions justify doing so.

Position B:
Prefer the later date when Rohiṇī occurs there with Navamī, or under the
Monday/Wednesday readings discussed in the commentary.

Position C:
Some Vaiṣṇavas prefer the later date even without those additional
conjunctions.
```

The commentary concludes this discussion with:

```text
atra sampradāyācāra eva gatiḥ
```

Accordingly, Campaign 61.0U does not silently choose among these practices.
This unresolved point can change the civil observance date.

## 7. Vrata And Fast

HBV 15.247 and 266-282 present Janmāṣṭamī as an annual observance. HBV
15.377 and 411-414 explicitly include fasting. HBV 15.408 maps the ordinary
three-day discipline to Saptamī, Aṣṭamī, and the pāraṇa day.

The practical sequence established by the source is:

```text
Saptamī:
One meal and/or preliminary discipline; the vow may be taken afterward.

Aṣṭamī morning:
Alternative time for taking the vow.

Observance day:
Remain nirāhāra and refrain from ordinary enjoyment while performing the
festival observance.

Following day:
Complete the vrata through the applicable pāraṇa rule.
```

The operational instruction `fast until midnight, then anukalpa` appears in
modern calendar evidence, not as the complete HBV vrata rule in the acquired
passage. It may be retained as operational practice only after separate policy
approval.

## 8. Pāraṇa

The primary source closes the former source gap but reveals multiple accepted
positions.

Established common ground:

```text
Pāraṇa occurs on the following day.
It must respect the surviving Aṣṭamī/Rohiṇī factor under the applicable rule.
It is not the Ekādaśī Hari-vāsara algorithm.
```

Policies preserved by HBV 15.397-406 and the Dig-darśinī:

```text
Policy A:
If only Aṣṭamī is extended, break at Aṣṭamī end.
If only Rohiṇī is extended, break at Rohiṇī end.
If both are extended, break when either one ends.

Policy B:
Those capable of continuing wait until both Aṣṭamī and Rohiṇī have ended;
those unable may break when either one ends.

Policy C:
The Yājñavalkya exception is read as requiring both the tithi and Rohiṇī to
end.

Policy D:
An alternate reading requires the tithi end for Rohiṇī-linked Aṣṭamī without
waiting separately for Rohiṇī end.

Policy E:
Some Vaiṣṇavas break after the festival; the Garuḍa statement permits tithi
end or festival end.
```

Campaign 61.0U resolves this multiplicity by requiring an explicit
`PARANA_POLICY_PROFILE` rather than manufacturing one universal rule:

```text
FIRST_RELEVANT_FACTOR_END
BOTH_FACTORS_END_CAPABLE
FIRST_FACTOR_END_UNABLE
TITHI_END_ROHINI_EXCEPTION
FESTIVAL_END
```

No profile is the silent default. If a caller requests pāraṇa without choosing
a profile, the required result is `PARANA_POLICY_REQUIRED`. No universal
latest clock-time limit is established by HBV 15.385-406, so the output must
state `NOT_SPECIFIED_BY_SOURCE` rather than invent a closing time.

Conclusion:

```text
NEXT-DAY PARANA:
RESOLVED

POLICY MULTIPLICITY:
RESOLVED AS EXPLICIT PROFILES

DEFAULT PROFILE:
NONE
```

## 9. Source Authority

| Evidence | Classification | Use |
|---|---|---|
| Hari-bhakti-vilāsa mūla, Vilāsa 15 | Tier 1 | Canonical rule and procedure |
| Dig-darśinī-ṭīkā | Tier 2 | Resolve or expose interpretation and scope |
| Older texts quoted inside HBV 15 | Source chain retained; not independently verified in this campaign | Evidence as represented and adjudicated by HBV/DDT |
| PureBhakti and SCS Math calendars | Tier 5 | Validation only |
| *The Indian Calendar* (Sewell and Dīkṣita, 1896) | Tier 4 | Amānta/pūrṇimānta month correspondence |
| *Report of the Calendar Reform Committee* (Government of India, 1955) | Tier 4 | Traditional calendrical time divisions and niśītha representation |
| Modern translated summaries | Tier 4 reading witness or Tier 6 web content, according to artifact identity | Reading aid only |

Source authority for Campaign 61.0U:

```text
PASS
```

## 10. Remaining Rule Conflicts

### RULE_CONFLICT-JAN-001 Month Nomenclature

```text
Position A:
HBV synthesis 15.328 says Bhādra Kṛṣṇa Aṣṭamī.

Position B:
Several quoted passages say Śrāvaṇa/Nabhas.

Resolution:
These are two names for the same dark fortnight under amānta and pūrṇimānta
month conventions. The engine-facing ID is Bhādrapada because the Calendar
Engine is pūrṇimānta; Śrāvaṇa remains source-label metadata.

Impact:
No date change.

Disposition:
RESOLVED
```

### RULE_CONFLICT-JAN-002 Two-Day Priority

```text
Position A:
Earlier pure Aṣṭamī under HBV 15.374.

Position B:
Later Navamī-linked or weekday/Rohiṇī candidate in cases described by the
Dig-darśinī.

Position C:
Later-day Vaiṣṇava practice recorded by the commentary.

Impact:
Can change the civil observance date.

Disposition:
UNRESOLVED
```

### RULE_CONFLICT-JAN-003 Pāraṇa Termination

```text
Positions:
First factor ends / both factors end / tithi end / festival end, with
capability distinctions.

Resolution:
Preserve every source-backed method as a named `PARANA_POLICY_PROFILE`. Require
the profile explicitly and provide no silent default.

Impact:
Changes pāraṇa output, not the observance date.

Disposition:
RESOLVED AS PROFILED POLICY
```

### RULE_CONFLICT-JAN-004 Niśītha Boundary

```text
Source fact:
Niśītha/ardha-rātra is relevant to merit and some priority readings.

Traditional calendar definition:
Central, eighth muhūrta of the local sunset-to-following-sunrise night.

Computational representation:
[S + 7(R-S)/15, S + 8(R-S)/15), centered on S + (R-S)/2.

Impact:
Can change a tiebreak result if the project adopts a niśītha-based priority.

Disposition:
RESOLVED FOR INTERVAL CONSTRUCTION; its use in a disputed priority remains part
of RULE_CONFLICT-JAN-002.
```

## 11. Minimum Resolver Shape After Policy Approval

This is a documentary decomposition, not production code:

```text
locate the pūrṇimānta Bhādrapada Kṛṣṇa Aṣṭamī interval
↓
construct adjacent civil candidates
↓
evaluate Aṣṭamī at sunrise for Saptamī-viddhā purity
↓
reject every Saptamī-viddhā earlier candidate
↓
evaluate Aṣṭamī-Rohiṇī overlap for Jayantī metadata
↓
if one eligible candidate remains, select it
↓
if multiple clean candidates remain and no approved two-day profile exists,
return NORMATIVE_POLICY_REQUIRED with both candidates and their facts
↓
attach full-vrata metadata
↓
apply the caller-selected Janmāṣṭamī pāraṇa profile; never select a default
```

## 12. Implementation Gate

```text
Primary source acquired:
PASS

Primary source verified:
PASS

Māsa:
RESOLVED - amānta Śrāvaṇa Kṛṣṇa equals pūrṇimānta Bhādrapada Kṛṣṇa

Pakṣa:
RESOLVED

Tithi:
RESOLVED

Sunrise / Saptamī-viddhā:
RESOLVED

Niśītha role:
RESOLVED

Niśītha computable boundary:
READY

Rohiṇī role:
RESOLVED

Two-day priority:
UNRESOLVED

Vrata:
RESOLVED

Pāraṇa date:
RESOLVED

Pāraṇa termination policy:
RESOLVED AS EXPLICIT PROFILES; DEFAULT NONE

Source authority:
PASS

Rule conflicts capable of changing the observance date:
1

Unrecorded implementation choices:
0

Invented doctrine:
0

Decision Spec:
NOT READY

Recommendation:
NORMATIVE DECISION REQUIRED
```

Reason:

```text
The source gap, month-name mapping, niśītha interval, and pāraṇa policy family
are resolved. The Dig-darśinī itself still records alternative two-day
practices capable of selecting different civil dates. Campaign 61.1 remains
closed until the project adopts a named Gauḍīya two-day policy profile.
```
