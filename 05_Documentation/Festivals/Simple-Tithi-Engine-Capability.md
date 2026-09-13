# Simple Tithi Engine Capability

Campaign: 62.1
Status: READ-ONLY ENGINE AUDIT

## Capability Matrix

| Required datum | Status | Existing component | Note |
| --- | --- | --- | --- |
| local civil date | AVAILABLE | calendar input types | already used by sunrise and Ekadasi flows |
| location | AVAILABLE | `GeoLocation` | latitude and longitude available |
| timezone/civil offset | AVAILABLE at application input boundary | RC1/API configuration | festival-domain contract still needs specification |
| local sunrise | AVAILABLE | `calendar::sunrise` | reusable without astronomy changes |
| tithi at sunrise | AVAILABLE | `calendar::tithi_at_sunrise` | reusable without Ekadasi semantics |
| tithi presence between consecutive sunrises | AVAILABLE | `calendar::tithi_presence_between_sunrises` | useful for zero/one/two-sunrise analysis |
| tithi transition between sunrises | AVAILABLE | `calendar::tithi_transition_between_sunrises` | one transition helper; arbitrary interval API remains limited |
| instantaneous tithi | AVAILABLE | `astronomy::astronomical_tithi` | reusable at any supplied Julian instant |
| tithi start/end for arbitrary target tithi | DERIVABLE | existing longitude and transition solving | no generic public festival interval object yet |
| native Vaisnava masa | AVAILABLE | `calendar::vaishnava_masa_at` | provisional ayanamsa status remains documented |
| paksa | AVAILABLE | `AstronomicalTithi::paksha` | no new calculation required |
| consecutive civil-day evaluation | DERIVABLE | existing date loop patterns and calendar functions | festival selector not implemented |
| previous/next tithi contact classification | DERIVABLE | tithi presence and transition data | normative meaning is festival-specific |
| morning/before-noon ritual boundary | MISSING as a domain primitive | no generic festival day-part API | civil/solar definition must be specified first |
| reference-location policy | MISSING | no festival policy profile | cannot be inferred from `GeoLocation` |
| festival fasting/parana | MISSING outside Ekadasi | Ekadasi-only certified logic | must not inherit Ekadasi rules |

## Reuse Decision

```text
Astronomy recomputation required: NO
Ekadasi resolver reuse as festival policy: NO
Calendar primitives reusable: YES
Festival selection adapter eventually required: YES
Engine changes in Campaign 62.1: 0
```

The principal blocker is normative selection, not astronomical capability.
