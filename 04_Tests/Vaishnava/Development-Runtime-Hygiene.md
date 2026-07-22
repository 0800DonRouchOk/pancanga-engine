# Development Runtime Hygiene

Campaign: 52.4

Status: PASS

## Objective

Prevent stale local RC1 instances from being mistaken for the current
implementation during development and beta testing.

## Background

Campaign 52.3 identified the source of an apparent content mismatch as:

```text
STALE_LOCAL_INSTANCE
```

Older local servers were still listening on ports 7878 and 7879 while the
current RC1 implementation was served from port 7892.

## Change

The RC1 local server now generates runtime metadata in `rc1-config.js`.

The RC1 footer displays:

```text
Engine
Knowledge Base
Astronomy
Build
Commit
Port
Started
```

The issue report draft also includes:

```text
Commit
Port
Started
```

## Runtime Identity

The local RC1 server resolves:

```text
commit:
current git short commit, or RC1_COMMIT / GIT_COMMIT when supplied

port:
the local server port serving the current request

started:
the UTC timestamp for the current server session
```

## Methodological Boundary

This campaign only improves development and beta visibility.

It does not modify:

```text
Astronomy Engine
Calendar Engine
Vaiṣṇava Engine
Observance Engine
Knowledge Base
Festival Catalog
Certified rules
```

## Verification

```text
cargo fmt --all --check
PASS

cargo check -p pancanga-engine --example rc1_experience
PASS

cargo test -p pancanga-engine
PASS
```

Note:

```text
The Codex sandbox compiled the local RC1 example successfully but did not permit
opening a fresh listening socket from this session. Existing user-launched RC1
instances remain under the user's control.
```

## Certification Result

```text
Campaign 52.4

Development Runtime Hygiene:
PASS

Footer runtime identity:
PASS

Issue report runtime identity:
PASS

Stale instance visibility:
PASS

Motor:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS

Architecture:
SIN CAMBIOS
```
