# Deterministic Certification

Campaign: 46DETERMINISM

Status: PENDING

## Objective

Demonstrate that Pancanga Engine produces identical results for identical
inputs across repeated executions of the same version.

This certification protects the v1.0 release from hidden state, accidental
non-determinism, order-dependent behavior, or unreproducible validation output.

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
