# Pancanga Engine Master Reference

Campaign: 48 - Master Project Dossier  
Status: RC1 reference dossier  
Created: 2026-07-14  
Scope: documentation only  

## Purpose

This document is the master project dossier for Pancanga Engine.

It is not a README, not an implementation guide, and not a replacement for the
Knowledge Base. It is a consolidated reference meant to let a future
collaborator, reviewer, or external AI understand Pancanga Engine without
opening the entire repository first.

All information in this dossier is drawn from local repository documents and
source files. No external web information is used here.

If this dossier conflicts with a primary project document, the primary document
wins. For v1.0 release status, the highest-priority local sources are:

- `PROJECT_STATUS.md`
- `PROJECT_PRINCIPLES.md`
- `CERTIFICATION.md`
- `00_Project/Final-Certification-Stage.md`
- `05_Documentation/Knowledge-Base/Knowledge-Base-v1.0-Release.md`
- `05_Documentation/Knowledge-Base/KB-VAI-002-Hari-bhakti-vilasa-Ekadasi-Rules.md`

## Executive State

Pancanga Engine is in v1.0 Release Candidate status.

```text
Development:
CLOSED

Certification:
FINAL STAGE

Release:
PENDING

Knowledge Base:
v1.0 FROZEN

Astronomy Engine:
CERTIFIED

Calendar Engine:
STABLE

Vaiṣṇava Engine:
IMPLEMENTED

Open doctrinal issues:
0

Confirmed ENGINE BUG:
0
```

The project is in release freeze. Until v1.0, accepted changes are limited to:

- confirmed engine bugs;
- documentation;
- certification;
- release preparation.

No new rules, APIs, engines, optimizations, or features enter v1.0.

## Part 1 - Project Story

Pancanga Engine began as an Ekādaśī calculation project. It grew into an
offline, auditable Gaudiya calendar engine whose central value is traceability.

The project exists because many Vaiṣṇava practitioners depend on calendars
whose outputs are trusted and lived by, while the internal logic behind those
outputs is often difficult to inspect. Pancanga Engine does not try to replace a
living tradition. It tries to make one clearly declared methodology explicit,
testable, and auditable.

The guiding chain is:

```text
Sources
        ↓
Knowledge Base
        ↓
Specification
        ↓
Implementation
        ↓
Certification
```

The strongest design decision was to keep the project layered:

```text
Astronomy
        ↓
Calendar
        ↓
Vaiṣṇava
```

Astronomy computes physical magnitudes. Calendar determines civil-time
presence. The Vaiṣṇava layer interprets the calendar facts according to the
frozen Knowledge Base.

This separation means a disagreement can be classified before the engine is
changed:

- astronomy;
- calendar/civil configuration;
- normative interpretation;
- external-oracle difference;
- engine bug.

The v1.0 objective is narrow but strong:

```text
To provide a traceable, auditable implementation of the Ekādaśī calculation
methodology defined for this project.
```

## Part 2 - Core Principles

The project principles are short enough to function as a constitution:

1. The Knowledge Base governs the implementation.
2. Every rule must be traceable to its normative source.
3. External oracles certify the implementation; they do not replace primary
   sources.
4. The Knowledge Base is not modified to fit the code.
5. Every difference with an external oracle must be classified before the
   engine is changed.
6. Simplicity has priority over complexity when both implement the same rule
   correctly.
7. Certification campaigns do not add features. They produce evidence.
8. Every important decision must be reproducible and auditable.
9. A documented divergence is preferable to a forced resolution.
10. The v1.0 scope is protected once release freeze begins.

The v1.0 definition is not "nothing remains to do." It is:

```text
Astronomy certified
Calendar Engine stable
Knowledge Base complete
Vaiṣṇava Engine implemented
External-oracle certification performed
No confirmed ENGINE BUG
Release Freeze respected
```

Future work belongs to later release lines:

```text
v1.x:
bug fixes, compatibility, documentation

v2.0:
new functionality
```

## Part 3 - Research And Source Methodology

The project originally had to answer a basic question: which authority defines
Ekādaśī for this engine?

The answer frozen for v1.0 is:

```text
Primary normative authority:
Hari-bhakti-vilāsa

Primary commentary:
Digdarśinī-ṭīkā

Practical Gauḍīya application:
Navadvīpa Pañjikā

Validation oracles:
GCal, PureBhakti, SCS Math, and other calendars
```

The source precedence rule is:

1. Hari-bhakti-vilāsa, mūla.
2. Digdarśinī-ṭīkā.
3. Navadvīpa Pañjikā.
4. Tradition Notes.
5. Purāṇas cited by Hari-bhakti-vilāsa, only as context.

If a discrepancy cannot be resolved, the rule is not changed for convenience.
The divergence is documented and implementation remains protected until the
issue is resolved or explicitly classified.

Internet has a restricted role after authority selection:

- allowed: locating or verifying sources;
- forbidden: defining HBV-EK rules, replacing HBV, replacing Digdarśinī, or
  filling normative gaps with summaries.

The source layer records this structure:

```text
07_Sources/
        Astronomy/
                Meeus
                ELP2000
                Swiss
        Vaishnava/
                Hari-bhakti-vilāsa
                Caitanya-caritāmṛta
                Interpretations
                Tradition-Notes
```

## Part 4 - Knowledge Base v1.0

Knowledge Base v1.0 was released and frozen on 2026-07-14.

It added:

- HBV-EK-001 - Candidate Ekādaśī.
- HBV-EK-002 - Ekādaśī Viddhā.
- HBV-EK-003 - Observance displacement.
- HBV-EK-004 - Eight Mahādvādaśī.
- HBV-EK-005 - Parāṇa / Hari-vāsara.
- source hierarchy;
- extraction protocol;
- citation normalization;
- issue tracking;
- Knowledge Base audit.

Freeze rule:

```text
No modification to the Knowledge Base may be made to facilitate implementation.
The Knowledge Base may change only if a primary source requires the change.
```

The Knowledge Base audit concluded:

```text
Knowledge Base:
v1.0 READY

Implicit required rules:
NO

Blocking contradictions:
NO

Open doctrinal issues:
0
```

A developer can implement the Vaiṣṇava Engine v1.0 from:

- `KB-VAI-002-Hari-bhakti-vilasa-Ekadasi-Rules.md`
- `HBV-EK-001-Candidate-Ekadasi.md`
- `HBV-EK-002-Viddha.md`
- `HBV-EK-003-Observance-Displacement.md`
- `HBV-EK-004-Mahadvadasi.md`
- `HBV-EK-005-Parana-Harivasara.md`
- `ISSUE-VAI-001-Vyanjuli-Divergence.md`

without re-reading Hari-bhakti-vilāsa to reconstruct rules.

## Part 5 - HBV-EK Rule Chain

The Vaiṣṇava Engine follows a composable rule chain:

```text
HBV-EK-001
Generate candidate
        ↓
HBV-EK-002
Invalidate Viddhā candidate
        ↓
HBV-EK-003
Move observance when needed
        ↓
HBV-EK-004
Classify Mahādvādaśī
        ↓
HBV-EK-005
Compute Parāṇa / Hari-vāsara window
```

### HBV-EK-001 - Candidate Ekādaśī

Question:

```text
What makes a civil day enter the initial Ekādaśī candidate set?
```

Normative rule:

```text
A civil day becomes an initial Ekādaśī observance candidate when the tithi
present at its local sunrise is Ekādaśī.
```

Evidence includes HBV 12.1, HBV 12.2, HBV 12.40, HBV 12.199, and HBV 12.315.
The Digdarśinī clarifies the role of Ekādaśī and Dvādaśī as Hari's days and
supports the sunrise-based calendrical frame.

Consumes:

- `CivilDayTithiPresence`;
- `AstronomicalTithi` at local sunrise.

Does not consume:

- Aruṇodaya;
- Nakṣatra;
- Mahādvādaśī;
- Hari-vāsara;
- Parāṇa;
- festivals.

Engineering form:

```text
tithi at sunrise
        ↓
is traditional number 11?
        ↓
CandidateEkadasi / NotCandidate
```

### HBV-EK-002 - Viddhā

Question:

```text
When does a candidate cease to be valid because it is Viddhā?
```

Normative rule:

```text
An Ekādaśī candidate is invalidated as Viddhā when Daśamī is present during
Aruṇodaya, the 96 minutes before local sunrise.
```

Evidence includes HBV 12.199, HBV 12.315-320, HBV 12.342-343, and the
Digdarśinī.

Important distinction:

```text
The 96-minute Aruṇodaya criterion applies to Ekādaśī purity.
It is not generalized to other tithis or festivals.
```

Engineering form:

```text
CandidateEkadasi
        +
tithi at Aruṇodaya
        ↓
Daśamī?
        ↓
InvalidViddha / ValidCandidate
```

### HBV-EK-003 - Observance Displacement

Question:

```text
What transformation applies when a candidate is Viddhā?
```

Normative rule:

```text
When an Ekādaśī candidate is invalidated as Viddhā by HBV-EK-002, the
observance is displaced to Dvādaśī.
```

Evidence includes HBV 12.317-320 and HBV 12.324.

Engineering form:

```text
ValidCandidate  -> ObserveOnEkadasi
InvalidViddha   -> ObserveOnDvadasi
NotCandidate    -> NoObservanceCandidate
```

This rule does not classify Mahādvādaśī and does not compute Parāṇa.

### HBV-EK-004 - Eight Mahādvādaśī

Question:

```text
How does Hari-bhakti-vilāsa classify a valid observance as one of the eight
Mahādvādaśī?
```

Evidence includes HBV 12.391-392 and HBV 13.265-266.

The eight Mahādvādaśī are:

| Type | Group | Condition Summary |
|---|---|---|
| Unmīlanī | Tithi | Ekādaśī is pure and continues beyond the following sunrise. |
| Vyañjulī / Vañjulī | Tithi | Ekādaśī is pure; Dvādaśī is present at sunrise and continues toward Trayodaśī. |
| Tri-spṛśā | Tithi | Ekādaśī, Dvādaśī, and Trayodaśī touch in a valid sequence without Daśamī involvement. |
| Pakṣa-vardhinī | Tithi | Amāvasyā or Pūrṇimā is complete and grows; the preceding Dvādaśī is classified. |
| Jayā | Nakṣatra | Śukla Dvādaśī combined with Punarvasu. |
| Vijayā | Nakṣatra | Śukla Dvādaśī combined with Śravaṇa. |
| Jayantī | Nakṣatra | Śukla Dvādaśī combined with Rohiṇī. |
| Pāpanāśinī | Nakṣatra | Śukla Dvādaśī combined with Puṣya. |

Engineering form:

```text
observance disposition
        +
tithi facts
        +
nakṣatra facts when needed
        ↓
Mahādvādaśī type / None
```

The four nakṣatra-based cases required a minimal Nakṣatra Engine:

```text
nakṣatra_index = floor(normalize_360(λ_luna) / 13°20′)
```

Only the nakṣatras required by HBV-EK-004 were needed for v1.0:

- Punarvasu;
- Śravaṇa;
- Rohiṇī;
- Puṣya.

### HBV-EK-005 - Parāṇa / Hari-vāsara

Question:

```text
How does Hari-bhakti-vilāsa determine the valid Parāṇa window?
```

Evidence includes HBV 13.229-259.

Normative rule:

```text
The valid Parāṇa window begins when Hari-vāsara has ended and must close before
Dvādaśī ends.
```

Operational rule:

```text
dvadasi_duration = dvadasi_end - dvadasi_start
hari_vasara_end = dvadasi_start + 0.25 * dvadasi_duration
parana_start    = max(sunrise, hari_vasara_end)
parana_end      = dvadasi_end
```

If Dvādaśī is too short for an ordinary window, the engine records
`ShortDvadasi` instead of inventing ritual detail.

The Knowledge Base also records a contingency: when Dvādaśī is short, duties
are advanced and Parāṇa may be minimally performed with water to avoid
transgressing Dvādaśī.

## Part 6 - Hard Cases

### Vyañjulī

Vyañjulī was the final doctrinal issue.

Two formulations circulated:

```text
Version A:
Ekādaśī not complete.

Version B:
Ekādaśī pure + Dvādaśī present at sunrise + Dvādaśī prolonged toward
Trayodaśī.
```

The final review of local priority sources resolved the issue in favor of
Version B.

Final Vyañjulī specification:

```text
1. Ekādaśī is pure.
2. Dvādaśī is present at sunrise.
3. Dvādaśī continues until the following sunrise, touching Trayodaśī.
```

Critical clarification:

```text
Ekādaśī purity depends on the two-muhūrta / 96-minute Aruṇodaya rule.
Dvādaśī presence for Vyañjulī does not inherit that two-muhūrta threshold.
```

The translation phrase "eat on Ekādaśī" in HBV 13.356-358 is marked as a
translator inconsistency and is not used as normative foundation.

Operational status:

```text
ISSUE-VAI-001:
RESOLVED

Open doctrinal issues:
0
```

### Tri-spṛśā

Tri-spṛśā is a Mahādvādaśī case where Ekādaśī, Dvādaśī, and Trayodaśī touch in
a valid sequence without Daśamī involvement.

During the RC1 Experience, a real Triṣpṛśā case showed an important product
limitation: the engine has HBV-EK-004 logic, but the first RC1 screen does not
yet derive all tithi-based Mahādvādaśī facts automatically for display. The
interface was corrected not to make a false claim such as "no Mahādvādaśī"
when that fact has not been fully derived by the beta screen.

This is a presentation limitation, not a doctrinal or engine issue.

### Parāṇa

The engine calculates the complete normative Parāṇa interval:

```text
from:
max(local sunrise, end of Hari-vāsara)

to:
end of Dvādaśī
```

External calendars often publish a shorter practical morning window.

PureBhakti case PB-0007 demonstrated the distinction:

```text
PureBhakti:
07:51-11:13

Pancanga Engine normative window:
07:51-23:32
```

Diagnosis:

```text
parana_end = dvadasi_end
```

The PureBhakti end time `11:13` matched the end of the first third of the civil
day in that case. The project classifies this as presentation/editorial policy
unless a normative source requires changing HBV-EK-005.

The RC1 Experience therefore separates:

```text
Recommended Parāṇa:
practical presentation window

Normative limit:
complete HBV-EK-005 window
```

### Dvādaśī Corta

The engine represents Dvādaśī-short cases as `ShortDvadasi`.

The RC1 Experience was adjusted so it does not construct an artificial
recommended window when no ordinary Parāṇa window exists. It presents the case
as a contingency instead.

## Part 7 - Astronomy Engine

The astronomical layer is independent from Vaiṣṇava rules.

It exposes the public quantities required for tithi:

```text
astronomy::solar::apparent_longitude(jd)
astronomy::moon::apparent_longitude(jd)
astronomy::lunar_solar_elongation(moon_longitude, sun_longitude)
astronomy::tithi_index(elongation)
astronomy::astronomical_tithi(elongation)
```

### Solar Pipeline

The Solar Pipeline v1.0 is frozen.

It computes:

```text
Earth heliocentric longitude
        ↓
geocentric solar longitude
        ↓
Earth radius vector
        ↓
solar aberration
        ↓
nutation in longitude
        ↓
apparent solar longitude
```

Documented components include:

- AST-0002 - Earth heliocentric longitude, VSOP87D.
- AST-0003 - Geocentric solar longitude.
- AST-0004 - Earth radius vector.
- AST-0005 - Solar aberration.
- AST-0006 - Nutation in longitude.
- AST-0007 - Apparent solar longitude.

The solar layer does not know tithi, Ekādaśī, Parāṇa, festivals, or Vaiṣṇava
rules.

### Lunar Pipeline

The Lunar Pipeline computes:

```text
ELP2000 Import Pipeline
        ↓
AST-L001  Fundamental Lunar Arguments
        ↓
AST-L002  Periodic Longitude Correction
        ↓
AST-L003  Lunar Mean Longitude W1
        ↓
AST-L004  Geocentric Lunar Longitude
        ↓
AST-L006  ELP2000 Frame Transformation
        ↓
AST-0006  Nutation in Longitude
        ↓
AST-L005  Apparent Lunar Longitude
```

ELP2000 data flow:

```text
official IMCCE files
        ↓
elp2000-importer
        ↓
generated Rust data
        ↓
lunar evaluator
```

The import pipeline processed 36 official IMCCE files and generated 37,872
terms. The ELP2000 interpreter principles are:

```text
Importer = data fidelity
Evaluator = theory fidelity
```

### AST-L006

AST-L006 was the decisive astronomy validation campaign.

Initial Swiss comparison produced a large lunar difference:

```text
Moon error:
~1.41°

Tithi mismatches:
56 / 1000
```

The difference was not random and grew almost linearly from J2000, indicating a
reference-frame issue rather than a numerical implementation bug.

Diagnostic comparison:

```text
Swiss apparent/date:
Moon error ~1.4105°

Swiss -j2000:
Moon error ~0.00626°
```

Conclusion:

```text
Pancanga Moon was close to a J2000-like ELP2000 frame.
Pancanga Sun was already in apparent/date frame.
Tithi validation requires Sun and Moon in the same frame.
```

AST-L006 applies the ELP2000 scalar frame precession:

```text
λ_mean_of_date =
normalize_360(
    λ_elp + (5029.0966 / 3600) × t
)
```

Then nutation is applied:

```text
λ_apparent_of_date =
normalize_360(λ_mean_of_date + Δψ)
```

AST-L006 lives in the Astronomy Engine. It does not belong in the ELP2000
interpreter, the Swiss adapter, the Calendar Engine, or the Vaiṣṇava Engine.

## Part 8 - Calendar Engine

The Calendar Pipeline v1.0 freezes the boundary between astronomy and calendar.

It explicitly states:

```text
The calendar engine does not implement astronomical algorithms.
It consumes only the public Astronomy Engine API.
```

Pipeline:

```text
Geographic coordinates
        ↓
local sunrise
        ↓
JulianDate of sunrise
        ↓
solar apparent longitude
        ↓
moon apparent longitude
        ↓
lunar-solar elongation
        ↓
AstronomicalTithi
        ↓
tithi transition within the civil day
        ↓
calendar presence of tithi
```

Calendar Engine capabilities:

- `calendar::sunrise(date, location)` returns local sunrise as `JulianDate`.
- `calendar::tithi_at_sunrise(sunrise_jd)` returns the tithi present at
  sunrise.
- `calendar::tithi_transition_between_sunrises(...)` detects the first tithi
  change inside `[sunrise_today, sunrise_tomorrow)`.
- `CivilDayTithiPresence` records the tithi at sunrise, optional transition,
  and post-transition tithi.

The Calendar Engine does not apply Ekādaśī, Mahādvādaśī, Parāṇa, festivals, or
Vaiṣṇava rules.

## Part 9 - Vaiṣṇava Engine Implementation

The Vaiṣṇava Engine implements HBV-EK-001 through HBV-EK-005.

The public module re-exports:

- `ekadasi_candidate_at_sunrise(...)`
- `ekadasi_candidate_for_civil_day(...)`
- `arunodaya_start(...)`
- `invalidate_viddha_candidate(...)`
- `observance_displacement(...)`
- `classify_mahadvadasi(...)`
- `hari_vasara_end(...)`
- `parana_window(...)`
- `classify_vaishnava_day(...)`

Core types include:

- `EkadasiCandidate`
- `ViddhaCandidateStatus`
- `EkadasiObservanceDisposition`
- `MahadvadasiType`
- `TithiMahadvadasiCondition`
- `MahadvadasiClassification`
- `MahadvadasiFacts`
- `ParanaWindow`
- `ParanaMode`
- `ParanaInput`
- `VaishnavaDayInput`
- `VaishnavaDayClassification`
- `VaishnavaEngineError`

The integrated facade:

```text
classify_vaishnava_day(input)
```

consumes facts already produced by astronomy and calendar layers. It does not
calculate solar longitude, lunar longitude, sunrise, tithi transitions, or
nakṣatra internally.

Integration behavior:

```text
NotCandidate
        ↓
NoObservanceCandidate
        ↓
No Parāṇa

CandidateEkadasi
        ↓
Viddhā rule
        ↓
Observance disposition
        ↓
Optional Mahādvādaśī facts
        ↓
Parāṇa window
```

## Part 10 - Validation And Certification

Certification is evidence, not source authority.

External oracles certify the implementation but do not replace primary sources.

### Swiss Ephemeris

Swiss Ephemeris certifies the astronomical layer.

After AST-L006:

```text
Cases:
1000 / 1000 PASS

Tithi mismatches:
0

Maximum lunar longitude error:
0.000469445269°

Maximum elongation error:
0.000330443528°

Confirmed ENGINE BUG:
0
```

The Swiss validation runner uses local `swetest` 2.10.03 and local ephemeris
files. If `swetest` is unavailable, the runner does not invent results; it
marks the oracle as pending.

### PureBhakti

PureBhakti 2026 Buenos Aires was imported from a real local HTML source.

Result:

```text
Fixtures:
16

Observance dates:
16 / 16 PASS

ENGINE BUG:
0

Parāṇa:
13 Editorial Policy differences
2 Rounding Difference cases
```

Certification decision:

```text
PureBhakti:
PASS
```

The observed differences concentrate in published Parāṇa end times, not in
observance decisions. The project records them as editorial/rounding
differences and does not alter HBV-EK-005 to match them.

### Sri Chaitanya Saraswat Math

SCS Math was imported from the real annual PDF:

```text
02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf
```

Result:

```text
Fixtures:
26

Observance date PASS:
11 / 26

Calendar differences:
15

Parāṇa comparable differences:
11

Confirmed ENGINE BUG:
0
```

Difference analysis:

```text
CONFIGURATION_DIFFERENCE:
14

TRADITION_DIFFERENCE:
1

ENGINE_BUG:
0

UNKNOWN:
0
```

SCS Math certification is not yet closed as PASS. The remaining work is a civil
configuration audit for Nabadwip / IST and related calendar alignment. Vyañjulī
is no longer an open doctrinal blocker after ISSUE-VAI-001.

### GCal

GCal is configured as a validation oracle, not as normative authority.

Current state:

```text
Importer:
ready

Real local fixtures:
pending

Synthetic fixtures:
forbidden

Status:
EXTERNAL_ORACLE_PENDING
```

### Cross-Oracle Pattern

The current cross-oracle pattern is:

```text
Swiss Ephemeris:
PASS

PureBhakti:
PASS, editorial differences documented

SCS Math:
differences analyzed, no confirmed engine bug

GCal:
pending real local source

Confirmed ENGINE BUG:
0
```

The correct statement is not "the engine can never be wrong." The correct
evidence-based statement is:

```text
Across the oracles analyzed so far, no confirmed ENGINE BUG has been found.
```

## Part 11 - RC1 Experience

The RC1 Experience exists because passing tests is not the same as being usable
for daily practice.

The private beta asks:

```text
Would the maintainer trust tomorrow's Ekādaśī observance to this application?
```

It is a minimal application, not a developer-only command. It runs with:

```text
cargo run -p pancanga-engine --example rc1_experience
```

and opens at:

```text
http://127.0.0.1:7878
```

The first screen must answer in less than ten seconds:

1. Do I fast today?
2. When is Parāṇa?
3. Why did the engine decide this?

The allowed beta changes are presentation, reports, UX notes, comparison
display, and release-blocking usability fixes. The beta must not change:

- astronomy;
- calendar semantics;
- Vaiṣṇava rules;
- Knowledge Base;
- doctrine;
- v1.0 scope.

### RC1 Parāṇa Presentation

The beta separates:

```text
Recommended Parāṇa:
practical presentation window

Normative limit:
complete HBV-EK-005 window
```

This came from the PureBhakti PB-0007 diagnosis and does not modify the motor.

### Campaign 46BETA.6

A visual discrepancy appeared during private testing:

```text
Selected date:
2027-02-17

UI appeared to show:
2027-02-02
```

Diagnosis:

```text
The motor did not process the wrong date.
The frontend retained a previous result when Parāṇa calculation failed.
```

Verified date flow:

```text
Selected:  2027-02-17
Received:  2027-02-17
Processed: 2027-02-17
Rendered:  17 February 2027
```

Fix:

```text
Clear the screen before each calculation and do not keep stale results visible
after an error.
```

Also improved:

- Dvādaśī-short presentation;
- Parāṇa failure handling;
- Mahādvādaśī messaging so the beta does not overclaim facts it has not yet
  derived.

No engine, Knowledge Base, or architecture change was required.

## Part 12 - Current Contradictions And Tensions

This section records local documentation tensions rather than hiding them.

### Component-Maturity.md Is Behind Current Status

`00_Project/Component-Maturity.md` still lists:

```text
Vaiṣṇava Engine:
v0.1 Planned
```

But `PROJECT_STATUS.md`, `README.md`, and the implemented Rust source show the
Vaiṣṇava Engine is implemented and integrated through Campaign 45.

Interpretation:

```text
Component-Maturity.md is stale relative to the current RC1 status.
```

### Lunar-Pipeline.md Status Is Older Than Certification

`05_Documentation/Algorithms/Lunar-Pipeline.md` still says "En desarrollo" in
its header, but its table includes AST-L006 and AST-L005 as implemented, and
`PROJECT_STATUS.md` records the Lunar Engine API and Astronomy Engine v1.0 as
closed/certified.

Interpretation:

```text
The Lunar Pipeline document contains older status language. Current release
state is governed by PROJECT_STATUS.md and CERTIFICATION.md.
```

### SCS Math Is Analyzed But Not Fully Closed

SCS Math has no confirmed engine bug and no unknown differences, but the local
documents do not close it as PASS. They require a civil configuration audit.

Interpretation:

```text
SCS Math is a strong oracle analysis, not a final closed certification.
```

### GCal Is Pending

The GCal importer exists, but real local GCal fixtures are not yet present.
The repository correctly does not invent synthetic oracle data.

Interpretation:

```text
GCal remains EXTERNAL_ORACLE_PENDING.
```

### RC1 Experience Does Not Yet Derive Every Display Fact

The engine implements HBV-EK-004. The RC1 screen does not yet automatically
derive every tithi-based Mahādvādaśī fact for presentation.

Interpretation:

```text
This is a beta experience limitation, not a motor limitation.
```

## Part 13 - Final Certification Stage

The objective release path is:

```text
Campaign 46F.2
Civil Configuration Audit
        ↓
Campaign 46C Final
Full External Certification
        ↓
Campaign 46BETA
Pancanga Engine RC1 Experience
        ↓
Campaign 46R
Release Audit
        ↓
Campaign 47
v1.0 Release
```

Campaign 46F.2 must show that remaining SCS Math differences come from
civil/calendar configuration rather than engine behavior.

Campaign 46C Final must update `CERTIFICATION.md` with final external
certification evidence.

Campaign 46BETA must show that the maintainer can use Pancanga Engine for
ordinary Ekādaśī decisions without needing another calendar.

Campaign 46R checks:

- Knowledge Base frozen;
- principles updated;
- certification updated;
- changelog updated;
- release notes prepared;
- README updated;
- tests pass;
- formatting passes;
- clippy passes;
- docs build;
- no critical TODO/FIXME;
- no open technical campaigns;
- no open doctrinal issues.

Campaign 47 publishes:

```text
Version:
1.0.0

Tag:
v1.0.0

Status:
STABLE
```

## Part 14 - Chronology

This chronology is reconstructed from local repository documents. Early
ELP2000 campaign numbering is partially recorded in `PROJECT_STATUS.md`, but
the repository does not preserve a complete individual title for every campaign
from 1 to 21. Where exact local campaign names are absent, this dossier groups
the recorded work by milestone.

### Foundation And Solar Engine

- Foundation phase: project structure, Rust workspace, specifications, core
  math, time, and Julian Day were created.
- AST-0001: Julian centuries implemented.
- AST-0002: Earth heliocentric longitude implemented with VSOP87D and
  validated against `vsop87.chk`.
- AST-0003: geocentric solar longitude implemented by geometric identity.
- AST-0004: Earth radius vector implemented with VSOP87D.
- AST-0005: solar aberration implemented from Meeus formula.
- AST-0006: nutation in longitude implemented and later reused by both Sun and
  Moon.
- AST-0007: apparent solar longitude integrated.
- Milestone 01: Solar Engine v1.0 closed.
- Solar Engine API review: stable public API established.

### Lunar Engine And ELP2000 Foundation

- Milestone 02: first official ELP2000 term evaluated and validated.
- Campaign 02: first official `ELP4` term validated with
  `ShortPeriodicEvaluator`.
- Campaign 03: first 10 official `ELP4` terms validated.
- Campaign 04: first 100 official `ELP4` terms validated.
- Campaign 05: complete `ELP4` family validated.
- Campaign 06: first official `ELP7` term validated with temporal order 1.
- Campaign 07: first 10 official `ELP7` terms validated.
- Campaign 08: complete `ELP7` family validated.
- Campaign 09: first official `ELP22` term validated.
- Campaign 10: complete `ELP22` family validated.
- Campaigns 11-21: remaining ELP2000 longitude-family semantics were
  consolidated. Local status records validated `ShortPeriodic` orders 0, 1,
  and 2, plus `Planetary` orders 0 and 1, across the remaining longitude
  families.
- Campaign 22: AST-L002 integrated the 12 ELP2000 longitude families:
  `ELP1`, `ELP4`, `ELP7`, `ELP10`, `ELP13`, `ELP16`, `ELP19`, `ELP22`,
  `ELP25`, `ELP28`, `ELP31`, and `ELP34`.
- AST-L001: fundamental lunar arguments implemented.
- AST-L003: lunar mean longitude `W1` implemented.
- AST-L004: geocentric lunar longitude implemented as `W1 + Δλ/3600`.
- AST-L005: lunar apparent longitude implemented using shared nutation.
- AST-L006: ELP2000 frame transformation specified and implemented after Swiss
  diagnosis.
- Lunar Engine API: `astronomy::moon::apparent_longitude(jd)` stabilized.

### Astronomical Tithi

- Campaign 27: lunar-solar elongation implemented.
- Campaign 28: astronomical tithi index implemented as `floor(elongation/12°)`.
- Campaign 29: `AstronomicalTithi` and `Paksha` implemented.
- Campaign 30: Astronomical Engine v1.0 milestone documented.

### Calendar Engine

- Campaign 31: Calendar Engine boundary frozen.
- Campaign 32: Sunrise Engine implemented.
- Campaign 33: tithi at sunrise implemented.
- Campaign 34: tithi transition inside the civil day implemented.
- Campaign 35: `CivilDayTithiPresence` implemented.
- Calendar Engine Base v1.0 closed.

### Vaiṣṇava Knowledge Base

- Campaign 36: Vaiṣṇava rules boundary/specification created.
- Campaign 37: normative authority frozen as Hari-bhakti-vilāsa, with GCal and
  other calendars as validation oracles only.
- Campaign 38: provisional Ekādaśī rules specification superseded by the later
  Knowledge Base.
- Campaign 39: Hari-bhakti-vilāsa source extraction opened.
- Campaign 39.5: HBV-EK-001 candidate rule extracted.
- Campaign 39.6: HBV-EK-002 Viddhā rule extracted.
- Campaign 39.7: HBV-EK-003 observance displacement extracted.
- Campaign 39.8: HBV-EK-004 eight Mahādvādaśī extracted.
- Campaign 39.9: HBV-EK-005 Parāṇa / Hari-vāsara extracted.
- Campaign 39.10: Knowledge Base audit closed; Knowledge Base v1.0 READY.
- Knowledge Base v1.0: frozen on 2026-07-14.

### Vaiṣṇava Engine Implementation

- Campaign 40: HBV-EK-001 implemented.
- Campaign 41: HBV-EK-002 implemented.
- Campaign 42: HBV-EK-003 implemented.
- Campaign 43A: minimal Nakṣatra Engine implemented.
- Campaign 43B: HBV-EK-004 Mahādvādaśī classification implemented.
- Campaign 44: HBV-EK-005 Parāṇa / Hari-vāsara implemented.
- Campaign 45: Vaiṣṇava Engine facade integrated.

### Certification And RC1

- Campaign 46: full Vaiṣṇava Engine validation opened.
- Campaign 46A: Swiss Ephemeris infrastructure created.
- Campaign 46A.2: Swiss difference analysis identified reference-frame cause.
- Campaign 46A.3: AST-L006 frame transformation specified.
- Campaign 46A.4: AST-L006 implemented; Swiss validation closed 1000/1000 PASS.
- Campaign 46B: GCal fixture structure created without synthetic data.
- Campaign 46B.1: GCal real-oracle importer prepared.
- Campaign 46D: PureBhakti HTML extractor created.
- Campaign 46D.1: PureBhakti real fixture validation closed observance 16/16
  PASS.
- Campaign 46D.2: PureBhakti Parāṇa differences classified.
- Campaign 46F: SCS Math PDF oracle imported and first validation executed.
- Campaign 46F.1: SCS Math differences analyzed; no engine bug confirmed.
- ISSUE-VAI-001: Vyañjulī divergence resolved.
- Campaign 46BETA: RC1 Experience opened as private product experience.
- Campaign 46BETA.6: frontend stale-result issue diagnosed and fixed without
  engine/Knowledge Base changes.
- Campaign 46F.2: pending civil configuration audit.
- Campaign 46C Final: pending full external certification closure.
- Campaign 46R: pending release audit.
- Campaign 47: pending v1.0 release.

## Part 15 - What Another AI Should Not Do

Do not:

- reinterpret Hari-bhakti-vilāsa from web summaries;
- treat GCal, PureBhakti, or SCS Math as normative authorities;
- modify the Knowledge Base to make implementation easier;
- modify the engine to match an external oracle before classifying the
  difference;
- add v1.0 features during release freeze;
- call SCS Math PASS until the civil configuration audit is closed or
  documented as non-critical;
- invent GCal fixtures;
- treat the RC1 Experience display limitations as engine bugs without tracing
  data flow;
- generalize the 96-minute Aruṇodaya rule beyond Ekādaśī purity;
- collapse normative Parāṇa and practical recommended Parāṇa into one concept.

## Part 16 - Most Useful Local Source Documents

Project and release:

- `README.md`
- `WHY_PANCANGA_ENGINE_EXISTS.md`
- `PROJECT_STATUS.md`
- `PROJECT_PRINCIPLES.md`
- `CERTIFICATION.md`
- `00_Project/Final-Certification-Stage.md`
- `00_Project/Roadmap-v1.0.md`
- `00_Project/RC1-Experience.md`

Knowledge Base:

- `05_Documentation/Knowledge-Base/Knowledge-Base-v1.0-Release.md`
- `05_Documentation/Knowledge-Base/KB-VAI-002-Hari-bhakti-vilasa-Ekadasi-Rules.md`
- `05_Documentation/Knowledge-Base/HBV-Extraction-Protocol.md`
- `05_Documentation/Knowledge-Base/HBV-Citation-Normalization.md`
- `05_Documentation/Knowledge-Base/HBV-EK-001-Candidate-Ekadasi.md`
- `05_Documentation/Knowledge-Base/HBV-EK-002-Viddha.md`
- `05_Documentation/Knowledge-Base/HBV-EK-003-Observance-Displacement.md`
- `05_Documentation/Knowledge-Base/HBV-EK-004-Mahadvadasi.md`
- `05_Documentation/Knowledge-Base/HBV-EK-005-Parana-Harivasara.md`
- `05_Documentation/Knowledge-Base/ISSUE-VAI-001-Vyanjuli-Divergence.md`
- `05_Documentation/Knowledge-Base/Campaign-39.10-Knowledge-Base-Audit.md`

Algorithms:

- `05_Documentation/Algorithms/Solar-Pipeline.md`
- `05_Documentation/Algorithms/Lunar-Pipeline.md`
- `05_Documentation/Algorithms/Calendar-Pipeline.md`
- `05_Documentation/Algorithms/AST-L006-ELP2000-Frame-Transformation.md`
- `05_Documentation/Algorithms/Vaishnava-Authority.md`
- `05_Documentation/Algorithms/Vaishnava-Rules.md`

Certification:

- `04_Tests/Astronomy/SwissEphemeris/Swiss-Validation.md`
- `04_Tests/Astronomy/SwissEphemeris/Swiss-Difference-Analysis.md`
- `04_Tests/PureBhakti/PureBhakti-Validation.md`
- `04_Tests/PureBhakti/PureBhakti-Parana-Difference-Analysis.md`
- `04_Tests/PureBhakti/PureBhakti-Practical-Parana-Origin.md`
- `04_Tests/SCSMath/SCSMath-Validation.md`
- `04_Tests/SCSMath/SCSMath-Difference-Report.md`
- `04_Tests/Validation/Validation-Report.md`

RC1 Experience:

- `08_Examples/RC1-Experience/README.md`
- `08_Examples/RC1-Experience/index.html`
- `03_Source/rust/crates/pancanga-engine/examples/rc1_experience.rs`

## Final Summary

Pancanga Engine v1.0 is not a generic pañcāṅga, not a festival engine, and not
a universal doctrinal resolver.

It is a traceable implementation of the Ekādaśī methodology defined by this
project:

```text
Hari-bhakti-vilāsa
        ↓
Digdarśinī-ṭīkā
        ↓
Knowledge Base v1.0
        ↓
Rust implementation
        ↓
Local tests
        ↓
External certification
        ↓
RC1 Experience
```

The technical center is complete. The release is protected by final
certification, RC1 product experience, and release audit.

The most important current facts are:

```text
Knowledge Base:
FROZEN

Open doctrinal issues:
0

Astronomy:
Swiss certified, 1000 / 1000 PASS

PureBhakti:
16 / 16 observance PASS

SCS Math:
differences analyzed, 0 confirmed ENGINE BUG

GCal:
real fixtures pending

Confirmed ENGINE BUG:
0

Release state:
RC1, final certification stage
```

