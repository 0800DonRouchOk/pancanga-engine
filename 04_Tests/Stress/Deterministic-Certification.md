# Deterministic Certification

Campaign: 46DETERMINISM

Status: PASS (2026-09-15) — with the scope limitation below

## Objective

Demonstrate that Pancanga Engine produces identical results for identical
inputs across repeated executions of the same version.

This certification protects the v1.0 release from hidden state, accidental
non-determinism, order-dependent behavior, or unreproducible validation output.

## Scope Limitation — What This Certification Does and Does Not Prove

**Owner decision, recorded 2026-09-15: this certification runs at the
aggregate level, not the per-day level, and this is a deliberate v1.0 scope
choice, not an oversight.**

`examples/internal_stress_test.rs` does not write or print a per-day trace of
the 73,414 individual computed results (sunrise instant, tithi, candidate,
viddha, disposition for every civil day). It only ever produces four
artifacts: a 5-line stdout summary, an issues CSV, a 6-row extremes CSV, and
a markdown report built from the same underlying counters. No such per-day
artifact was added for this campaign — doing so would mean modifying
`internal_stress_test.rs`, which this campaign's scope forbids.

**What this certification DOES prove:** across 10 consecutive executions of
one release build, the following are byte-for-bit identical every time:
the 14 aggregate `Stats` fields (total days, completed days, panics,
crashes, NaN/infinite values, invalid dates, negative intervals, overflow
suspicions, contradictions, missing sunrise, observance days, standard
Parāṇa windows, short Dvādaśī contingencies, confirmed engine bugs), the 6
extreme-value rows (shortest/longest Parāṇa, Hari-vāsara, Dvādaśī, each with
a 12-decimal Julian Date and a 6-decimal hour figure), and the full issues
list (empty on this run, since 46STRESS records 0 engine bugs over this
range). This is a real determinism check: it would catch an
iteration-order bug, an accumulation-order float drift, or an
engine-path non-determinism that changed any of these counts or extreme
values.

**What this certification does NOT prove:** that every one of the 73,414
individual days produces a bit-identical result across runs. It is possible
in principle for two days' individual outputs to swap or shift in a way
that leaves every aggregate count and every extreme value unchanged; this
method would not detect that. A full per-day hash (e.g., hashing a
per-day CSV dump of sunrise/tithi/candidate/viddha/disposition for all
73,414 days) is deferred to v1.1 as a certification-instrumentation
enhancement. It is not implemented here.

## Scope

Use the same range and location as Campaign 46STRESS:

```text
Location:
Valencia, España

Range:
1900-01-01 -> 2100-12-31

Cases:
73,414 consecutive civil days
```

## Required Method

Run the complete certification range ten consecutive times.

For each pass:

```text
Input range
↓
Pancanga Engine calculations
↓
Canonical output stream
↓
Stable hash
```

The canonical output must exclude timestamps, elapsed seconds, filesystem paths,
or any other runtime-dependent metadata.

## PASS Criterion

```text
Hash 1 == Hash 2 == Hash 3 == Hash 4 == Hash 5
      == Hash 6 == Hash 7 == Hash 8 == Hash 9 == Hash 10
```

Additional required result:

```text
Confirmed ENGINE BUG:
0
```

## Non-Determinism Risk Classes Checked (2026-09-15)

Checked by reading the full source of `examples/internal_stress_test.rs`
before running anything, per campaign discipline (patching a suppressed
trap into the binary was out of scope regardless of what was found):

```text
Timestamp / elapsed time
FOUND — Internal-Stress-Test.md contains a live "Elapsed seconds: {:.3}"
line from Instant::now(). Not a bug: excluded by construction from the
hash (stripped with `sed` before hashing), never patched out of the
binary itself.

Absolute file paths
NONE — OUTPUT_DIR / ISSUE_CSV / EXTREMES_CSV / REPORT_MD are compile-time
relative-string constants ("../../04_Tests/Stress/..."), identical on
every invocation from 03_Source/rust regardless of machine.

HashMap / HashSet iteration order
NONE — the binary uses only Vec and Option; Extremes::all() returns a
fixed-order literal array of 6 slots; the issues Vec is pushed in strict
date order inside one sequential loop. No hash-based collection anywhere
in this file.

Threading / parallelism
NONE — one sequential `while current <= end` loop from 1900-01-01 to
2100-12-31. No std::thread, no rayon, no par_iter.

Panic-hook source-path risk (documented, theoretical, not live this run)
Rust's default panic hook prints a source-file-and-line message to
stderr if a panic occurs. This did not fire on any of the 10 runs (0
panics, matching 46STRESS's established 0/73,414), and stderr is
excluded from the hash regardless (only stdout + the two CSVs + the
elapsed-seconds-stripped report are hashed). Recorded as a documented
risk class per campaign instruction, not because it affected this
result.
```

## Method Used (2026-09-15)

```text
cd 03_Source/rust
cargo build --release -p pancanga-engine --example internal_stress_test
```

Then, 10 times, from `03_Source/rust` (relative output paths depend on
this working directory):

```text
{
  ./target/release/examples/internal_stress_test
  cat ../../04_Tests/Stress/internal-stress-issues.csv
  cat ../../04_Tests/Stress/internal-stress-extremes.csv
  sed '/^Elapsed seconds:/d' ../../04_Tests/Stress/Internal-Stress-Test.md
} | sha256sum
```

One build, ten executions of the same binary — this isolates run-to-run
engine/tooling determinism from build determinism, which is a separate
question this method does not address.

## Result (2026-09-15)

```text
Cases executed per run: 73,414
Engine bugs per run: 0
Panics per run: 0
```

10 hashes (SHA-256 of stdout + internal-stress-issues.csv +
internal-stress-extremes.csv + Internal-Stress-Test.md with the
Elapsed-seconds line removed):

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

All 10 identical. PASS criterion met at the aggregate scope defined above.

Incidental cross-check: apart from the excluded Elapsed-seconds line, this
run's `Internal-Stress-Test.md` is byte-identical to the version already
committed from the original 46STRESS run — the only diff in
`git diff -- 04_Tests/Stress/Internal-Stress-Test.md` is the elapsed-seconds
value (225.966 → 200.449). This extends the reproducibility evidence beyond
just these 10 runs, to reproducibility against the previously recorded
46STRESS evidence as well.

## Failure Classification

Any mismatch must be classified before any engine change:

```text
VALIDATION_TOOL_NONDETERMINISM
ENGINE_NONDETERMINISM
RUNTIME_ENVIRONMENT_DIFFERENCE
UNKNOWN
```

## Methodological Rule

Deterministic certification confirms reproducibility.

It does not redefine:

```text
Astronomy
Calendar logic
Vaiṣṇava rules
Knowledge Base
```

No engine behavior may be changed merely to make hashes match until the
non-deterministic layer has been identified.
