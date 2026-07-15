# Internal Stress Test

Campaign: 46STRESS

Status: ✅ PASS

## Scope

This stress test attempts to break Pancanga Engine without modifying the motor,
Knowledge Base, architecture, or supported-location scope.

Configuration:

```text
Location: Valencia, España
Latitude: 39.4699
Longitude: -0.3763
Range: 1900-01-01 to 2100-12-31 inclusive
```

## Summary

| Metric | Count |
|---|---:|
| Cases executed | 73414 |
| Completed days | 73414 |
| Panics captured | 0 |
| Crashes | 0 |
| NaN / infinite values | 0 |
| Invalid dates | 0 |
| Negative intervals | 0 |
| Overflow suspicions | 0 |
| Contradictions | 0 |
| Missing sunrise | 0 |
| Observance days detected | 4823 |
| Standard Parāṇa windows | 4326 |
| Short Dvādaśī contingencies | 497 |
| Confirmed ENGINE BUG | 0 |

Elapsed seconds: 225.966

## Invariants Checked

- no panic per civil day;
- sunrise and Aruṇodaya are finite;
- Aruṇodaya precedes sunrise;
- tithi index remains in `0..29`;
- traditional tithi number remains in `1..15`;
- nakṣatra index remains in `0..26`;
- `NotCandidate` does not produce an observance;
- Parāṇa is calculated only for observance days;
- Dvādaśī interval is positive;
- Hari-vāsara end is inside Dvādaśī;
- standard Parāṇa windows do not have negative duration;
- Mahādvādaśī is not classified without observance.

## Extremes

| Metric | Date | Days | Hours | Details |
|---|---|---:|---:|---|
| Parāṇa duration | 2037-12-04 | 0.000446514692 | 0.010716 | start_jd=2465396.795244395267; end_jd=2465396.795690909959 |
| Parāṇa duration | 1910-12-27 | 0.837548793759 | 20.101171 | start_jd=2419033.047133531887; end_jd=2419033.884682325646 |
| Hari-vāsara duration | 1915-06-24 | 0.215239092242 | 5.165738 | dvadasi_start_jd=2420672.280826608650; hari_vasara_end_jd=2420672.496065700892 |
| Hari-vāsara duration | 1929-01-07 | 0.279221476056 | 6.701315 | dvadasi_start_jd=2425618.098495273851; hari_vasara_end_jd=2425618.377716749907 |
| Dvādaśī duration | 1915-06-24 | 0.860956368502 | 20.662953 | start_jd=2420672.280826608650; end_jd=2420673.141782977153 |
| Dvādaśī duration | 1929-01-07 | 1.116885903291 | 26.805262 | start_jd=2425618.098495273851; end_jd=2425619.215381177142 |

## Issues

No issues recorded.

## Artifacts

```text
04_Tests/Stress/Internal-Stress-Test.md
04_Tests/Stress/internal-stress-issues.csv
04_Tests/Stress/internal-stress-extremes.csv
```

## Diagnostic Note

An initial stress pass used a stricter sunrise-only invariant for displaced
observance days: `ObserveOnDvadasi` was expected to imply Dvādaśī at the
following sunrise. The first flagged case, 1904-09-19, showed that this was a
validation-tool assumption rather than an engine result: Dvādaśī began after
the following sunrise and continued into the next civil day. The stress test now
validates the actual Dvādaśī interval used for Parāṇa instead of requiring
Dvādaśī at a specific sunrise.

## Methodological Note

This campaign is internal certification. It does not redefine the engine. If a
difference or impossible state appears, it must be documented before any code is
changed.
