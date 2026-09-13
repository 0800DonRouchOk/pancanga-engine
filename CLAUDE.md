# Pancanga Engine — Claude Code Instructions

## 0. Orientation

Root governing documents:

```
PANCANGA_ENGINE_MASTER_REFERENCE.md
PROJECT_PRINCIPLES.md
PROJECT_STATUS.md          <- current state, single source of truth
KB-REF-001                 <- bibliographic authority, v2
```

This file contains only persistent invariants, limits and commands.
It contains no project state. `PROJECT_STATUS.md` always wins on state.

If this file, campaign documents, `PROJECT_STATUS.md`, the tests and actual
code behaviour appear to contradict each other:

```
DO NOT reconcile them.
REPORT the contradiction and stop.
```

You never have authority to update project state because you infer a campaign
"must be complete", nor to declare `PROJECT_STATUS.md` stale. Repository
behaviour that disagrees with it is a finding to report, not a licence to
overwrite it.

---

## 1. Mission

An auditable Gauḍīya Vaiṣṇava calendar engine.

Accuracy and traceability have priority over speed, elegance and coverage.
A wrong date that looks clean is a failure. A missing date marked as requiring
research is a success.

```
Astronomy determines WHEN astronomical conditions occur.
Normative authority determines WHICH conditions define an observance.
```

Never derive the second from the first.

---

## 2. Test Integrity — Critical Invariant

Golden tests, regression vectors and expected astronomical/calendar results
are part of the audited specification.

```
IF CODE OUTPUT != GOLDEN TEST
THEN CODE IS WRONG
```

Do not modify expected values to make tests pass. Do not relax a tolerance,
skip a case, or mark a test ignored to reach green.

Expected values may change only when a campaign explicitly authorizes a
golden-data correction and supplies documentary or mathematical evidence.

If a test appears wrong: STOP and report TEST_CONFLICT, including test,
expected value, actual value, affected rule, evidence, and date-changing
impact.

Any change to a test expectation must appear explicitly in your final report.

---

## 3. Frozen Audited Surface

Do not modify unless the active campaign authorizes it by name:

- Astronomy Engine
- Ekādaśī Engine
- Mahādvādaśī rules
- validated parāṇa logic
- Observance Resolver
- Knowledge Base schema (`08_Examples/RC1-Experience/content/schema/observance.schema.json`)
- golden datasets and regression vectors
- astronomical constants, ayanāṁśa handling
- `Cargo.lock` (in `03_Source/rust/`)
- the toolchain pin, which lives in TWO places that must change together:
  `rust-toolchain.toml` at the **repository root**, and the explicit
  `rustup toolchain install 1.85.0` in `.github/workflows/ci.yml`. CI does not
  read the toml. Changing one without the other silently desynchronizes local
  and CI builds.

**The Rust workspace has zero third-party dependencies.** `Cargo.lock`
contains only the workspace crates. Ephemeris (ELP2000, VSOP87D), Julian date
arithmetic, sunrise, sunset, timezone offsets and calendar conversion are all
implemented in-house.

This is a deliberate auditability property and the strongest one the project
has: no dependency bump can move a date, because there are no dependencies to
bump.

The property is about Cargo, not about all tooling. Exactly two auxiliary
scripts exist outside the workspace: `04_Tests/SCSMath/scsmath_pdf_extract.py`
and `Deploy/macOS/build_rc1_macos_app.sh`. Neither computes dates, so neither
affects the property — but do not claim the project is Rust-only.

Consequences:

1. **Introducing any third-party crate is an architectural decision requiring
   owner approval.** It is not a routine addition, and "it is a well-known
   crate" is not an argument. Ask first, always.
2. The risk that a dependency list would normally carry now sits on the
   in-house implementations instead. Treat the in-house ephemeris, Julian
   arithmetic, sunrise, sunset, timezone and DST code as audit-sensitive
   wherever it lives — including code currently outside `src/`.
3. Calendrical or civil-time computation that lives in `examples/` rather than
   `src/` is outside the freeze by accident, not by design. Never add more of
   it, and never treat its absence from `src/` as permission to modify it
   freely.

Never refactor a frozen component for elegance, consistency or readability.
If one looks wrong, report it and stop.

Release Freeze is currently declared over Knowledge Base, Astronomy,
Architecture, API and Scope. Confirm its status in `PROJECT_STATUS.md` before
any work.

---

## 4. Bibliographic Authority

`KB-REF-001` v2 governs source authority.

```
WEBSITE != SOURCE
```

Web is permitted only to: discover a source, locate an edition, download a
source, verify bibliographic metadata.

Web is never permitted to: establish doctrine, create a calendar rule, resolve
a conflict by popularity, or serve as an authority because it appeared in a
search result.

Authority tiers: 1 primary text, 2 traditional commentary, 3 lineage document,
4 scholarly bibliography, 5 operational calendars (validation only),
6 web content (prohibited as normative authority).

Every date-changing claim must answer: what is the rule, which text, which
edition, which verse/page, which commentary, how it became a computable rule.

Modern calendars validate. Disagreement with one is a finding to report, never
a bug to fix. Legacy provenance levels A–E are storage labels only; resolve
them against the v2 tiers by inspecting the source itself.

---

## 5. Doctrine Safety and Status Vocabulary

Invented doctrine must remain:

```
0
```

Never complete an unresolved rule by intuition, by analogy with another
festival, or by inference from astronomical data.

### 5.1 Schema-valid tokens — the only values writable to the KB

From `observance.schema.json`:

```
metadata.status        draft  sources_collected  review  approved
                       published  deprecated
textSection.status     present  not_applicable  pending
scripturalBasis.status confirmed  derived  pending  not_applicable
```

Other controlled enums in the same schema: `identity.observance_type`,
`classification.source`, `classification.masa_type`, `classification.paksha`,
`metadata.content_source`, `related_observances[].relationship`,
`reference.reference_type`, `provenanceSource.level`.

### 5.2 Reporting vocabulary — NEVER written to data or code

```
RESEARCH_REQUIRED      SOURCE_REQUIRED
RULE_CONFLICT          NORMATIVE_DECISION_REQUIRED
LINEAGE_RULE_CONFLICT  TEST_CONFLICT
```

These exist in campaign prose only. They are not in the schema and not in the
Rust source. Use them in reports. Never emit them into a JSON field, never
introduce them as a code enum unless a campaign explicitly creates them.

### 5.3 Silent defaults must remain 0

The doctrine rule above has a civil-time counterpart, and it is the same
failure in a different domain.

```
Silent civil-time defaults must remain:
0
```

A location, offset or time value that cannot be determined must fail
explicitly. It must never fall through to a default that produces a clean,
plausible, wrong number — `_ => 0.0` for an unknown timezone is the civil form
of invented doctrine, and it is worse in one respect: missing doctrine has a
gate and emits a status a human can see, while a silent default emits nothing.

This applies with particular force to any value shown to a user, and to any
value a validation harness runs through: a guessed offset inside a validation
path can both hide a real discrepancy and manufacture a false one.

Never introduce such a default. When you encounter one, report it; do not
quietly preserve it and do not quietly change it.

### 5.4 Known contract defect

The authoring guidelines use `reviewed`; the schema enumerates `review`.
Until a campaign resolves it, the schema value wins for anything written to
data. Do not "fix" either document on your own initiative.

---

## 6. Campaign Discipline

All substantial work is campaign-scoped.

Before modifying anything:

1. read `PROJECT_STATUS.md`
2. read the active campaign in full
3. inspect relevant normative documentation
4. inspect relevant source records
5. inspect affected tests
6. inspect git status
7. identify permitted files
8. identify frozen files and components

Do not extend scope opportunistically. Report unrelated problems; do not
repair them. If campaign scope is ambiguous, ask before starting rather than
choosing an interpretation.

Open gates are intentionally open. Do not resolve, implement or work around
them. If a campaign turns out to depend on one, stop and report the dependency.

Do not mass-implement festivals before their normative rules are documented.
A rule subfamily exists only when multiple observances provably share the same
documented normative algorithm.

---

## 7. Commands

All cargo commands run from `03_Source/rust`.
Toolchain is pinned at 1.85.0. CI sets `RUSTDOCFLAGS: -D warnings`.

### Canonical pipeline — matches `.github/workflows/ci.yml` (job `rust`)
```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo doc --workspace --all-features --no-deps
```

### Narrow regression variant — used in campaign reports
```bash
cargo fmt --all --check
cargo test -p pancanga-engine
cargo clippy -p pancanga-engine -- -D warnings
cargo doc --no-deps
```

### Development run
```bash
cargo run -p pancanga-engine --example rc1_experience
cargo run --release -p pancanga-engine --example rc1_experience
```

### Validation examples and tools
```bash
cargo run -p pancanga-engine --example swiss_validation -- <args>
cargo run -p pancanga-engine --example purebhakti_html_extract -- <args>
cargo run -p pancanga-engine --example gcal_fixture_import -- <args>
cargo run --release -p pancanga-engine --example internal_stress_test
cargo run -p elp2000-importer -- verify ../../06_Data/Reference/ELP2000/raw
cargo run -p elp2000-importer -- generate-all <args>
```

### Benchmarks
```bash
cargo bench --bench core_math
cargo bench --bench astronomy_algorithms
```

### macOS packaging
```bash
./Deploy/macOS/build_rc1_macos_app.sh
```

### Repository sanity checks
```bash
git status --short
git diff --check
```

### NOT DOCUMENTED — do not assume, report before use

- **Single test.** No documented invocation. Standard cargo filtering applies
  but is not repo-verified.
- **Dedicated astronomy regression command.** None exists separately from the
  suites above. Do not claim to have run one.
- **macOS smoke test.** Recorded as passed in campaign 60.0/60.1; no command
  is documented.
- `examples/scsmath_validation.rs`, `purebhakti_validation.rs` and
  `parana_diagnostic.rs` exist as binaries with no documented invocation.

Never invent a test or build command. Never report work complete on the basis
of reasoning alone — run the tests.

---

## 8. Documentation Integrity

Normative documentation is part of the audited system.

Do not casually rewrite canonical rules, Evidence Logs, source matrices, the
Festival Master Registry, the Research Backlog, `KB-REF-001`, or
`PROJECT_STATUS.md`.

Any change to normative documentation must be explicitly within campaign
scope. Never make a prose "cleanup" that alters normative meaning.

---

## 9. Multi-Agent Rule

More than one coding agent may work on this repository.

- Agents must not share a working tree. Use `git worktree` plus separate
  branches, one directory per agent.
- Each worktree has its own environment and build artifacts. Never assume
  another worktree's `target/` or dependencies are valid here.
- Never assume another agent's uncommitted work is available or stable.
- Never touch another agent's branch or worktree. Never rewrite history,
  never force-push.
- If the working tree does not match what you expect, stop and report instead
  of reconciling it yourself.

**Normative documents have a single writer.** You do not edit
`PROJECT_STATUS.md`, `CERTIFICATION.md`, the Festival Master Registry, the
Research Backlog, `KB-REF-001` or canonical rule documents directly.

Note on `CERTIFICATION.md`: the certification line writes into it —
`Final-Certification-Stage.md § Campaign 46C Final` defines its objective as
closing external certification there. It is a root normative document and it
is covered by this rule.

Write your outcome following the precedent of the line of work you are in.
The naming convention `Campaign-<id>-<name>.md` is uniform; the **location**
is not, and the two lines do not share one:

- **Certification (the 46 series).** No `Campaign-46*.md` file exists. This
  series does not produce records under that name. Its precedent is evidence
  under `04_Tests/<area>/` — e.g. `04_Tests/Stress/Internal-Stress-Test.md`,
  `04_Tests/Stress/Deterministic-Certification.md` — plus a summary section in
  the root `CERTIFICATION.md`, which the owner writes. Follow that pattern.
  Do not invent a `Campaign-46*.md`.
- **Engine and feature campaigns (50–61).** `00_Project/`.
- **Festival research campaigns.** `04_Tests/Festivals/`.

If you cannot tell which applies, ask. Do not create a new directory for it.

**Do not renumber, rename or consolidate existing directories.** The numbering
is not clean — `07_AI` and `07_Sources` share a prefix, and `Deploy`, `dist`
and `99_Archive` sit outside it. That is the tree as it is. Tidying it would
break every path reference in the documentation.

The owner merges campaign outcomes into the normative documents.

Branches prevent code conflicts. They do not prevent competing documentary
truth.

---

## 10. Change Reporting

After any implementation campaign, report:

- files modified and created
- tests executed and their results
- golden changes: YES / NO
- dependency or lockfile changes: YES / NO
- Astronomy Engine impact
- calendar-rule impact
- Knowledge Base impact
- date-changing impact (which dates, which years?)
- unresolved issues, contradictions and surprises
- **location of computation**: did you add or modify any solar, calendrical,
  UTC-offset or DST computation outside `src/`? YES / NO. If YES, name the
  file and what it computes — this is prohibited by § 3 and must surface in
  the report, not be discovered later.
- **silent defaults**: did you introduce, encounter or preserve any default
  that returns a plausible value for an undetermined input? YES / NO, with
  location.
- `git diff --check` result

Then stop. Do not chain into the next campaign. Do not commit unless
explicitly requested or authorized by campaign policy.

---

## 11. Evidence Standard

Cite repository evidence by **stable anchor**, never by line number:

```
file + heading, or
file + rule ID (e.g. HBV-EK-005, FEST-001, ST-CONFLICT-001), or
file + exact quoted identifier
```

Line numbers drift and are trivially fabricated. An anchor is greppable.

If a claim cannot be supported from repository evidence, mark it `UNVERIFIED`.
Never present inferred project history as repository fact.

---

## 12. Reporting Style

Terse and factual. No restatement of the task. No claims of completeness you
have not verified by running something. Uncertainty stated explicitly, never
smoothed over.
