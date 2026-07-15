# Pancanga Engine Principles

## Release Freeze

Pancanga Engine is in release freeze for v1.0.

```text
Knowledge Base:
FROZEN

Astronomy:
CERTIFIED / FROZEN

Architecture:
FROZEN

Scope:
FROZEN

API:
FROZEN
```

Allowed changes during release freeze:

```text
ENGINE BUG
DOCUMENTATION
CERTIFICATION
UX without semantic changes
RELEASE
```

No new functionality, new rules, new APIs, new optimizations, or new engines
are added to v1.0.

Release freeze rule:

```text
If an improvement does not increase trust in the engine,
it does not belong to v1.0.
```

## Release Candidate Rule

A release candidate does not incorporate new functionality.

Allowed RC changes:

```text
bug fixes
documentation improvements
certification completion
release preparation
```

Forbidden RC changes:

```text
new rules
new APIs
new features
new engines
scope expansion
```

Pancanga Engine RC1 Experience is allowed during RC only as a release-protection
activity. It may add a minimal private application, local reports, UX notes,
and screenshots, but it must not alter certified engine semantics or expand
v1.0 scope.

## Definition Of v1.0

Pancanga Engine v1.0 means that the original objective has been fulfilled.

```text
Astronomy certified
Calendar Engine stable
Knowledge Base complete
Vaiṣṇava Engine implemented
External-oracle certification performed
No confirmed ENGINE BUG
Release Freeze respected
```

v1.0 does not mean that no future work exists. Future work belongs to later
release lines.

```text
v1.x:
bug fixes, compatibility, documentation

v2.0:
new functionality
```

## Release Archive

Before publishing v1.0, the project creates an immutable release archive:

```text
Pancanga Engine Archive

Version:
1.0.0

Status:
Release Archive

Mutability:
Immutable
```

The archive must preserve:

- source code;
- Knowledge Base;
- certification documents;
- fixtures;
- validation reports;
- release notes;
- changelog.

The release archive is the audit copy of v1.0.

## Stabilization Window

After v1.0 is published, the project enters a short stabilization window:

```text
7-14 days
```

During that window:

```text
If a confirmed ENGINE BUG appears:
release v1.0.1

If no confirmed ENGINE BUG appears:
v1.0 remains officially stable
```

## Principles

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

11. The greatest victory of Pancanga Engine is not that it contains code, but
    that every important code decision can be justified by walking the path
    backward to śāstra.
