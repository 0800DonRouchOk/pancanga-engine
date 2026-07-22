# Normative Differences Register

Campaign: 52.2 follow-up

Status: OPEN

This register records known non-bug differences discovered during normative
observance certification. It exists to prevent classified certification
differences from being reopened later as undiagnosed engine bugs.

## Project State

```text
ENGINE BUGS
Confirmed: 0

Normative Differences
Open: 3
```

## Source Reports

```text
04_Tests/Vaishnava/Normative-Observance-Certification.md
04_Tests/Vaishnava/normative-observance-certification.csv
04_Tests/Vaishnava/Native-Observance-Resolver-Certification.md
04_Tests/SCSMath/SCSMath-Difference-Report.md
04_Tests/PureBhakti/PureBhakti-Validation.md
```

## Classification Summary

```text
AST-001
Astronomy differences:
0

OBS-001
Observance Resolver differences:
0

FAST-001
fasting_for differences:
0

MAS-001
Vaiṣṇava Māsa / Adhika Māsa differences:
6

MD-001
Mahādvādaśī date-pipeline differences:
1

DOC-001
External reference / configuration differences:
15
```

## MAS-001

Title:

```text
Adhika Māsa / Native Māsa / Catalog v1
```

Status:

```text
OPEN
```

Type:

```text
Normative Difference
```

Description:

The provisional native Vaiṣṇava Māsa implementation differs from the reference
fixtures in cases connected with māsa boundaries, Adhika Māsa naming, or
observance names not yet present in the v1 ordinary Ekādaśī catalog.

Observed in:

```text
PB-0001
PB-0002
PB-0003
SCS-0001
SCS-0002
SCS-0023
```

Impact:

```text
Ekādaśī identity
Māsa name
Intercalary month cases
Catalog coverage for Adhika Māsa observances
```

Motor Assessment:

```text
No confirmed Calendar Engine bug.
No confirmed Observance Resolver bug.
```

Current Interpretation:

The difference belongs to the provisional Vaiṣṇava Māsa / Adhika Māsa boundary
of the project. Campaign 52.1 certified that the resolver covers the complete
implemented v1 domain. Campaign 52.2 shows that the normative catalog still
requires explicit treatment of Adhika Māsa and related naming conventions before
these rows can be considered closed.

Next Campaign:

```text
Normative Resolution: Adhika Māsa naming and catalog policy
```

Exit Criteria:

```text
Formal rule for Adhika Māsa Ekādaśī naming is added to the local specification.
Catalog entries for supported Adhika Māsa observances are explicit.
Reference cases are reclassified as PASS or as documented tradition differences.
```

## MD-001

Title:

```text
Automatic Mahādvādaśī Derivation From Civil Date
```

Status:

```text
OPEN
```

Type:

```text
Pipeline / Domain Completion
```

Description:

The engine resolves a Mahādvādaśī identity correctly when supplied with the
necessary HBV-EK-004 facts. The current date-level normative validator does not
yet derive every Mahādvādaśī fact automatically from a civil date.

Observed in:

```text
PB-0009
```

Impact:

```text
Automatic Mahādvādaśī identification
Mahādvādaśī display in civil-date validators
fasting_for propagation in date-derived Mahādvādaśī cases
```

Motor Assessment:

```text
No confirmed Observance Resolver bug.
No confirmed fasting_for bug.
```

Current Interpretation:

Campaign 52.1 verified direct resolver coverage for all eight supported
Mahādvādaśī identifiers. Campaign 52.2 shows that the remaining gap is the
date-to-Mahādvādaśī fact pipeline, not the identity resolver itself.

Next Campaign:

```text
Mahādvādaśī Fact Derivation Certification
```

Exit Criteria:

```text
Civil-date validation derives HBV-EK-004 Mahādvādaśī facts automatically.
PB-0009 is reclassified as PASS or as a documented external reference difference.
fasting_for remains a typed ObservanceId relationship.
```

## DOC-001

Title:

```text
External Reference / Civil Configuration
```

Status:

```text
OPEN
```

Type:

```text
External Authority / Configuration Difference
```

Description:

Several SCS Math rows do not match the current local current-day or previous-day
Viddhā observance path under the configuration used by the validator. Prior SCS
Math analysis classified this pattern as configuration or tradition difference,
with no confirmed engine bug.

Observed in:

```text
SCS-0003
SCS-0004
SCS-0005
SCS-0006
SCS-0007
SCS-0008
SCS-0009
SCS-0010
SCS-0011
SCS-0012
SCS-0013
SCS-0014
SCS-0015
SCS-0018
SCS-0026
```

Impact:

```text
External certification against SCS Math
Civil-date comparison
Reference authority selection
Configuration audit for Nabadwip / IST
```

Motor Assessment:

```text
No confirmed Astronomy Engine bug.
No confirmed Calendar Engine bug.
No confirmed Vaiṣṇava Engine bug.
```

Current Interpretation:

These differences remain classified as external reference / configuration
differences until the SCS Math civil configuration and normative assumptions are
audited. They must not be used as justification for changing the engine without
a dedicated resolution campaign.

Next Campaign:

```text
Civil Configuration Audit: SCS Math / Nabadwip / IST
```

Exit Criteria:

```text
SCS Math location, timezone, DST policy, sunrise convention, ayanāṁśa, and
calendar configuration are audited.
Each DOC-001 row is reclassified as PASS, configuration difference, tradition
difference, or a confirmed engine bug with evidence.
```

## Closed / Not Present Difference Classes

The following classes were present in the 52.2 classification vocabulary but
did not appear as differences:

```text
AST-001
Count: 0
Meaning: no astronomy-origin difference found in the 52.2 scope.

AYA-001
Count: 0
Meaning: no difference was isolated specifically to ayanāṁśa in the 52.2 scope.

OBS-001
Count: 0
Meaning: no direct Native Observance Resolver bug found in the 52.2 scope.

FAST-001
Count: 0
Meaning: no fasting_for relationship difference found in the 52.2 scope.
```

## Methodological Rule

```text
A classified normative difference is not an ENGINE BUG.

No code, rule, Knowledge Base, or architecture change may be made from this
register alone. A future campaign must first resolve the relevant difference
with evidence and then define the minimal implementation or documentation
change, if any.
```

## Release Impact

```text
Confirmed ENGINE BUG:
0

Release blocker:
No confirmed engine bug from Campaign 52.2.

Open certification work:
MAS-001
MD-001
DOC-001
```
