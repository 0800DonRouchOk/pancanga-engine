# FEST-001: Śrī Kṛṣṇa Janmāṣṭamī Rule Analysis

Campaign: 61.0  
Status: PASS  
Mode: Analysis only

Campaign 61.0T update: this is the preliminary Campaign 61.0 analysis. The
current authority and unresolved-rule state are in
`FEST-001-Janmashtami-Canonical-Rule.md` and the Campaign 61.0T gate report.

## Candidate Rule Classification

```text
Primary candidate:
TITHI_PRESENT_DURING_WINDOW

Secondary candidates:
SPECIAL_VAISNAVA_RULE
TITHI_NAKSHATRA_COMBINATION, if Rohiṇī is adopted
FAST_UNTIL_RITUAL_EVENT
TITHI_RELATIVE_EVENT, for next-day break-fast
```

## Why Janmāṣṭamī Is a Good Golden Sample

Janmāṣṭamī exercises the exact capabilities the future Festival Engine must
handle without weakening Ekādaśī:

```text
non-Ekādaśī tithi
ritual window
possible nakṣatra condition
fasting boundary distinct from ordinary Ekādaśī
next-day parāṇa
operational calendar comparison
```

## Required Engine Capabilities

| Capability | Present now | Required change |
| --- | --- | --- |
| Tithi at arbitrary instant | Yes | Expose through Pañcāṅga State. |
| Tithi at sunrise | Yes | Reuse existing Calendar Engine API. |
| Tithi interval around civil day | Partial | Add reusable arbitrary-window tithi interval utility. |
| Midnight window | No dedicated API | Add timezone-aware civil ritual windows. |
| Nakṣatra at arbitrary instant | Yes | Expose through Pañcāṅga State if approved rule needs it. |
| Nakṣatra interval | No | Add interval finder only if documented rule requires it. |
| Fast-until-midnight | No | Model as festival fasting metadata after rule approval. |
| Festival parāṇa | No | Add separate festival parāṇa policy; do not reuse Ekādaśī Parāṇa blindly. |

## Provisional Resolver Shape

```text
FestivalCandidate(FEST-001)
        ↓
Build Pañcāṅga State for civil day
        ↓
Evaluate approved Janmāṣṭamī rule
        ↓
If unresolved source conflict exists:
    return RESEARCH_REQUIRED / RULE_CONFLICT
        ↓
If resolved:
    emit FestivalResult
```

## Rule Conflicts

```text
RULE_CONFLICT_JAN-001

The repository has operational evidence of midnight fasting, but the exact
selection rule is not yet incorporated as a Knowledge Base rule.
```

```text
RULE_CONFLICT_JAN-002

The possible Rohiṇī nakṣatra condition has not yet been certified from local
accepted sources.
```

## Implementation Guardrails

Campaign 61.1 must not:

```text
hardcode 2026-09-04
derive Janmāṣṭamī from PureBhakti alone
assume Rohiṇī without local evidence
reuse Ekādaśī Parāṇa as Janmāṣṭamī Parāṇa
modify Ekādaśī or Mahādvādaśī behavior
```

## Result

```text
Rule Analysis:
PASS

Implementation ready:
NO

Reason:
Normative Janmāṣṭamī rule still requires approval.
```
