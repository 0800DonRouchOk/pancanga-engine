# Simple Tithi Festival Family

Campaign: 62.1
Status: RESEARCH FOUNDATION

## Definition Under Test

A candidate belongs to this family only if its date may be determined primarily
from:

```text
Vaisnava masa + paksa + nominal tithi
```

Membership is not yet a rule. It must be removed or subclassified if accepted
evidence introduces naksatra, nisitha, another ritual window, sankranti,
relative-event logic, a multi-day interval, or lineage policy.

## Initial Candidates

The Campaign 62.0 inventory yields 22 candidates whose current metadata does
not yet require a prohibited dependency:

| Research tier | IDs | Candidates |
| --- | --- | --- |
| ST-TIER-A | FEST-002, FEST-009, FEST-010 | Radhastami; Govardhana Puja; Ratha-yatra |
| ST-TIER-B | FEST-011, FEST-015, FEST-018, FEST-019, FEST-020, FEST-023, FEST-068, FEST-078, FEST-091, FEST-093, FEST-094 | Varaha Dvadasi; Lalita Saptami; Bahulastami; Gopastami; Rasa Purnima; Vasanta Pancami; Snana-yatra; Dipavali; Aksaya Trtiya; Guru Purnima; Gita Jayanti |
| ST-TIER-C | FEST-012, FEST-013, FEST-073, FEST-076, FEST-077, FEST-079, FEST-088, FEST-089, FEST-090, FEST-092 | Buddha Purnima; Kurma Dvadasi; Odana Sasthi; Yama Dipa-dana; Yama Caturdasi; Yama Dvitiya; Ganga Dasahara; Damanaka-ropana; Krsna Pavitropana; Jahnu Saptami |

Thirty-five Caitanya-lila and acarya records remain outside this count even
though they may later use the same engine primitive. Their nominal markers are
still `UNKNOWN`, so assigning them now would be circular.

## First Batch

| ID | Reason selected | Result of first pass |
| --- | --- | --- |
| FEST-002 | Tier 1; known nominal Astami; fasting disagreement is visible | possible simple-tithi candidate; unresolved source and day-selection rule |
| FEST-009 | Tier 1; exact HBV passage discovered | removed from generic baseline; explicit viddha/day-part priority exists |
| FEST-010 | Tier 1; primary-text leads and institutional dispute available | removed from generic baseline; cycle anchor and policy conflict exist |
| FEST-015 | Tier 2; Sanskrit bibliographic lead exists | possible candidate; identity and interpretation unresolved |
| FEST-093 | Tier 2; broad tradition and local-calendar evidence available | possible candidate; adopted Gaudiya authority and location policy unresolved |

## Family Questions

Every candidate must answer:

1. Which lunar or Vaisnava masa is authoritative?
2. Which paksa and nominal tithi apply?
3. Must the tithi be present at sunrise, another boundary, or merely during the civil day?
4. Does contact with the previous tithi invalidate or prefer a day?
5. Does contact with the following tithi invalidate or prefer a day?
6. What happens when the tithi spans two sunrises?
7. What happens when it spans no sunrise?
8. Is purva or para preferred, and by what source?
9. Is the observance calculated locally or inherited from a reference location?
10. Is fasting required, until what boundary, and is parana distinct from an ordinary meal?

## Finding

The batch does not support one undifferentiated `GENERIC_SIMPLE_TITHI_RULE`.
Two evidence-driven subclasses are already necessary:

```text
TITHI_WITH_CONTACT_PRIORITY
  Evidence: Govardhana Puja distinguishes Amavasya contact from Dvitiya contact
  and adds a morning/before-noon boundary.

TITHI_AS_CYCLE_ANCHOR
  Evidence: Ratha-yatra is presented as the opening of a multi-day Jagannatha
  cycle, with a separate return event.
```

The remaining baseline class cannot be specified until the sunrise/two-day and
reference-location questions are answered.

## Status

```text
Initial candidate festivals: 22
Batch researched: 5
Still plausible baseline members from batch: 3
Reclassified from baseline: 2
Generic rule discovered: NO
Evidence-driven subclasses: 2
```
