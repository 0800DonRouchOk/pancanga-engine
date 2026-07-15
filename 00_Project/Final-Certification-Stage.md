# Final Certification Stage

Status: RC1 final stage.

Purpose:

```text
Protect the v1.0 release.
Do not expand certified engine scope.
Do not add engine functionality.
Produce final evidence.
```

Release freeze:

```text
Knowledge Base:
FROZEN

Astronomy:
CERTIFIED

Architecture:
FROZEN

Vaiṣṇava Engine:
IMPLEMENTED

API:
FROZEN

Scope:
FROZEN
```

Allowed changes:

```text
ENGINE BUG
DOCUMENTATION
CERTIFICATION
UX without semantic changes
RELEASE
```

Forbidden changes:

```text
new rules
new APIs
new architecture
new algorithms
new functionality
scope expansion
```

Freeze rule:

```text
If an improvement does not increase trust in the engine,
it does not belong to v1.0.
```

## Exit Criterion

When the seven steps in this document are complete, there are no remaining
technical campaigns before v1.0.

## Campaign 46F.2 - Civil Configuration Audit

Priority: HIGH.

Objective:

```text
Demonstrate that the remaining SCS Math differences come from civil/calendar
configuration and not from the engine.
```

Checklist:

```text
□ Correct locality
□ Latitude / longitude
□ Time zone
□ DST
□ Sunrise
□ Ayanamsa
□ Calendar configuration
```

Expected result:

```text
ENGINE BUG:
0
```

## Campaign 46C Final - Full External Certification

Priority: HIGH.

Objective:

```text
Close external certification in CERTIFICATION.md.
```

Required summary:

```text
Swiss Ephemeris
PASS
1000 / 1000

PureBhakti
PASS
16 / 16 observances

Sri Chaitanya Saraswat Math
PASS
Differences classified

Outstanding doctrinal issues:
0

Confirmed ENGINE BUG:
0
```

## Campaign 46STRESS - Internal Consistency Certification

Priority: CRITICAL.

Objective:

```text
Attempt to break Pancanga Engine internally across the complete supported
historical range without changing the motor, Knowledge Base, architecture, or
supported-location scope.
```

Required scope:

```text
Location:
Valencia, España

Range:
1900-01-01 -> 2100-12-31

Cases:
73,414 consecutive civil days
```

Required result:

```text
Crash:
0

Panic:
0

NaN / Infinite values:
0

Invalid dates:
0

Negative intervals:
0

Logical contradictions:
0

Confirmed ENGINE BUG:
0
```

Release command:

```text
cargo run --release -p pancanga-engine --example internal_stress_test
```

Status:

```text
PASS
```

Evidence:

```text
04_Tests/Stress/Internal-Stress-Test.md
```

## Campaign 46DETERMINISM - Deterministic Certification

Priority: HIGH.

Objective:

```text
Demonstrate that the same Pancanga Engine version produces identical output
for identical inputs across repeated executions.
```

Required scope:

```text
Run the full 73,414-day internal certification range 10 consecutive times.
Generate a stable output hash for each pass.
```

Required result:

```text
Hash 1 == Hash 2 == ... == Hash 10

Confirmed ENGINE BUG:
0
```

Status:

```text
PENDING
```

## Campaign 46BETA - Pancanga Engine RC1 Experience

Priority: HIGH.

Objective:

```text
Use Pancanga Engine privately as an end-user experience before release.
```

This campaign does not re-open doctrine, astronomy, or engine behavior. It
tests whether the maintainer can understand and trust real outputs quickly.

Required result:

```text
The maintainer has used Pancanga Engine for personal Ekādaśī decisions and no
longer needs another calendar for ordinary use.
```

Allowed:

```text
minimal private application
single-screen daily result
UX notes
explanation view
oracle comparison view
documentation of release-blocking usability issues
```

Forbidden:

```text
engine rule changes
Knowledge Base changes
astronomy changes
calendar semantics changes
new v1.0 scope
```

Details:

[Pancanga Engine RC1 Experience](RC1-Experience.md)

## Campaign 46R - Release Audit

Priority: HIGH.

Objective:

```text
Verify that the repository is ready for v1.0 release.
```

Checklist:

```text
□ Knowledge Base frozen
□ Astronomy certified
□ Swiss Certification PASS
□ PureBhakti Validation PASS
□ SCS Math Validation PASS
□ Internal Consistency Certification PASS
□ Deterministic Certification PASS
□ RC1 Experience validated
□ PROJECT_PRINCIPLES.md updated
□ CERTIFICATION.md updated
□ CHANGELOG.md updated
□ RELEASE_NOTES.md prepared
□ README.md updated
□ All tests PASS
□ cargo fmt PASS
□ cargo clippy PASS
□ cargo doc PASS
□ No critical TODO
□ No critical FIXME
□ No confirmed ENGINE BUG
□ No open technical campaigns
□ No open doctrinal issues
```

Expected result:

```text
RC1 APPROVED
```

## Campaign 47 - v1.0 Release

Objective:

```text
Publish Pancanga Engine v1.0.
```

Release state:

```text
Version:
1.0.0

Tag:
v1.0.0

Status:
STABLE
```

Frozen for v1.0:

```text
Knowledge Base
Astronomy
Vaiṣṇava Engine
Public API
```

## Stabilization Window

After the v1.0 release:

```text
7-14 days
```

If a confirmed engine bug appears:

```text
v1.0.1
```

If no confirmed engine bug appears:

```text
v1.0 remains officially stable.
```

## Scope Lock

No new functionality enters this stage.

Allowed:

```text
bug fixes
documentation
certification
release preparation
```

Forbidden:

```text
new rules
new APIs
new features
new engines
scope expansion
```
