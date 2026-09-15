# Campaign Report — 46DETERMINISM Deterministic Certification

Date: 2026-09-15

Scope: fill in the pre-existing `PENDING` `04_Tests/Stress/Deterministic-
Certification.md` per the method established in Phase 1 and approved by the
owner. No engine or example code changed; no golden test changed;
`CERTIFICATION.md`, `PROJECT_STATUS.md`, and `Final-Certification-Stage.md`
not touched.

## PHASE 0 — write-permission check

`04_Tests/Stress/Deterministic-Certification.md`'s own header declares no
write restriction (`Campaign: 46DETERMINISM / Status: PENDING`, nothing
more). Neither it nor `Final-Certification-Stage.md` is named in CLAUDE.md
§ 9's single-writer list (`PROJECT_STATUS.md`, `CERTIFICATION.md`, the
Festival Master Registry, the Research Backlog, `KB-REF-001`, "canonical
rule documents"), nor in `PROJECT_STATUS.md` or `KB-REF-001`.

Stronger than absence: CLAUDE.md § 9 explicitly names this exact file as
the required precedent for this campaign series — *"Certification (the 46
series)... Its precedent is evidence under `04_Tests/<area>/` — e.g.
`04_Tests/Stress/Internal-Stress-Test.md`, `04_Tests/Stress/Deterministic-
Certification.md` — plus a summary section in the root `CERTIFICATION.md`,
which the owner writes."*

**Conclusion: editable.** `CERTIFICATION.md` and `PROJECT_STATUS.md` were
not touched regardless of this finding, per direct instruction; `Final-
Certification-Stage.md` also not touched, per this campaign's explicit
forbidden list.

## PHASE 1 — hashing method (reported and approved before execution)

1. **Output format**: `internal_stress_test.rs` does not emit a per-day
   trace anywhere. It produces exactly four artifacts: a 5-line stdout
   summary, stderr progress pings (diagnostic only), an issues CSV (empty
   on a clean run), a 6-row extremes CSV, and a markdown report built from
   the same counters plus one live timing line. No full 73,414-day record
   exists to hash.
2. **Non-determinism traps checked** (by reading the source, not by
   running anything): timestamp (found — `Elapsed seconds` line, excluded
   by construction from the hash); absolute file paths (none — compile-time
   relative constants); HashMap/HashSet iteration order (none — `Vec`/
   `Option` only, fixed-order collection in `Extremes::all()`); threading
   (none — single sequential loop). Panic-hook source-path risk noted as a
   theoretical, non-live class.
3. **Proposed method**: build once in release mode, then 10 times pipe
   stdout + `internal-stress-issues.csv` + `internal-stress-extremes.csv`
   + `Internal-Stress-Test.md` (with the `Elapsed seconds` line stripped)
   through `sha256sum`.

The owner approved this method as-is and recorded, as an explicit v1.0
scope decision, that it certifies the aggregate/summary level, not a
bit-for-bit per-day result, with a full per-day hash deferred to v1.1.

## PHASE 2 — execution

```text
cd 03_Source/rust
cargo build --release -p pancanga-engine --example internal_stress_test
```

Build: `Finished \`release\` profile [optimized] target(s)` — clean, no
warnings.

Then, from `03_Source/rust`, 10 consecutive times:

```text
{
  ./target/release/examples/internal_stress_test
  cat ../../04_Tests/Stress/internal-stress-issues.csv
  cat ../../04_Tests/Stress/internal-stress-extremes.csv
  sed '/^Elapsed seconds:/d' ../../04_Tests/Stress/Internal-Stress-Test.md
} | sha256sum
```

Each run: `Cases executed: 73414`, `Engine bugs: 0`, exit code 0. No code
was modified.

## PHASE 3 — evidence written

`04_Tests/Stress/Deterministic-Certification.md` updated with:

- Status line: `PENDING` → `PASS (2026-09-15) — with the scope limitation
  below`.
- A new, prominent (not footnoted) "Scope Limitation" section placed
  directly after the Objective, stating plainly what the certification
  does and does not prove, and naming the v1.1 per-day-hash deferral
  explicitly.
- A "Non-Determinism Risk Classes Checked" section recording all four
  ruled-out classes plus the theoretical panic-hook class, exactly as
  established in Phase 1.
- A "Method Used" section with the exact build and hash commands.
- A "Result" section with the cases/bugs/panics counts, all 10 hashes,
  the PASS determination, and an incidental cross-check note (see below).

## HASHES

```text
Run  1: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  2: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  3: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  4: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  5: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  6: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  7: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  8: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run  9: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
Run 10: d34bf15753fb566af46b88afd02a18fa14ad8f1d2525958130b1d7d06a3e83c0
```

## ALL IDENTICAL: YES

Confirmed both by direct comparison of the 10 hash strings and by
`awk '{print $2}' hashes.txt | sort -u | wc -l` returning `1`.

## SCOPE LIMITATION RECORDED: YES

In `04_Tests/Stress/Deterministic-Certification.md`, immediately after the
`## Objective` section, under `## Scope Limitation — What This
Certification Does and Does Not Prove` — not a footnote, placed before the
Required Method/PASS Criterion sections so it is the second thing a reader
sees.

## Files modified

- `04_Tests/Stress/Deterministic-Certification.md` (evidence filled in, per
  Phase 3)
- `04_Tests/Stress/Internal-Stress-Test.md` (incidental side effect of
  running the existing binary 10 times — see note below; not a content
  edit made by me)

## Tests executed and results

`cargo build --release -p pancanga-engine --example internal_stress_test`:
clean. The binary itself was executed 10 times (73,414 days each,
matching 46STRESS's established range): 0 engine bugs, 0 panics, every
run. No `cargo fmt`/`clippy`/`test`/`doc` re-run — no `.rs` file was
touched by this campaign, so there is nothing new for them to check.

## Golden changes: NO

## Dependency or lockfile changes: NO

## Astronomy Engine impact: NONE

## Calendar-rule impact: NONE

## Knowledge Base impact: NONE

## Date-changing impact: NONE

This campaign only re-executes an existing, unmodified validation binary
and records the result; it computes no new dates and changes no rule.

## Location of computation: NO

No solar, calendrical, UTC-offset, or DST computation was added or
modified anywhere, in `src/` or outside it.

## Silent defaults: NONE introduced, encountered, or preserved

## Unresolved issues, contradictions and surprises

1. **`Internal-Stress-Test.md` shows as modified in `git status`, but the
   only line that changed is `Elapsed seconds` (225.966 → 200.449,
   confirmed via `git diff`).** Every other byte — the stats table, all 6
   extremes rows, the issues table, the static prose — is identical to the
   version already committed from the original 46STRESS run. This is an
   unavoidable side effect of running the existing binary (it always
   overwrites this file); I did not hand-edit it, and reverting it would
   mean putting a stale timing number back into an evidence file, which
   seemed worse than leaving the accurate one. Flagging it explicitly
   since it technically falls outside this campaign's named write scope
   (`Deterministic-Certification.md` plus the new report), even though it
   is not a content change I made and could not have avoided by any other
   invocation of the specified method. This byte-for-bit match against the
   already-committed 46STRESS evidence is also useful supporting evidence
   in its own right — it extends the reproducibility finding beyond these
   10 runs to reproducibility against a run performed on a separate
   occasion.
2. The scope limitation recorded here (aggregate-level, not per-day) is a
   real, load-bearing constraint on what "46DETERMINISM: PASS" means for
   v1.0. It is written prominently per instruction, but noting it again
   here so it isn't lost if this report is read independently of the
   certification document itself.

## `git diff --check`: clean

## CERTIFICATION.md / PROJECT_STATUS.md / Final-Certification-Stage.md TOUCHED: NO

Confirmed via `git status --short` — none of the three appears with new
changes attributable to this campaign. (`CERTIFICATION.md` and
`PROJECT_STATUS.md` already carried unrelated modifications from the prior
turn, applied by the owner, not by this campaign.)
