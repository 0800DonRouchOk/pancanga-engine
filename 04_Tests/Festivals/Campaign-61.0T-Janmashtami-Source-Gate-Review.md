# Campaign 61.0T Janmāṣṭamī Source Gate Review

Status: SOURCE ACQUISITION AND RULE AUDIT COMPLETE  
Mode: Source and documentation only  
Code changes: 0

## 1. Source Acquisition

```text
Authorized external sources acquired:
1

Source:
Hari-bhakti-vilāsa, Vilāsa 15, divyāvirbhāvaḥ

Host:
Gaudiya Grantha Mandira / Jiva Institute

Mūla:
PRESENT

Ṭīkā:
PRESENT

Source verification:
PASS
```

The acquired DOCX is preserved without modification. Its SHA-256 is:

```text
bceb71d94ed274318dfe32c17b1d6f3069eefbff1b846659672328e578cae1c6
```

## 2. Completeness

| Check | Result |
|---|---|
| Correct work and vilāsa | PASS |
| HBV 15.1 present | PASS |
| HBV 15.672 and closing colophon present | PASS |
| Janmāṣṭamī HBV 15.247-542 present | PASS |
| Determination HBV 15.328-396 present | PASS |
| Pāraṇa HBV 15.397-407 present | PASS |
| Mūla and commentary aligned by verse | PASS |
| Truncation | 0 detected |

## 3. Rule Coverage

| Component | Result | Evidence |
|---|---|---|
| Māsa | PARTIAL | HBV 15.328 says Bhādra; quoted sources also use Śrāvaṇa/Nabhas. |
| Pakṣa | PASS | Kṛṣṇa stated repeatedly. |
| Tithi | PASS | Aṣṭamī stated repeatedly. |
| Sunrise role | PASS | Determines Saptamī-viddhā purity; Aruṇodaya analogy rejected. |
| Sunset role | NOT REQUIRED | No selection role in HBV 15.328-406. |
| Niśītha role | PASS | Merit and worship window, not mandatory eligibility. |
| Niśītha exact boundary | FAIL | No computable interval definition in the acquired passage. |
| Rohiṇī role | PASS | Merit-enhancing and Jayantī-classifying, not mandatory. |
| Saptamī-viddhā | PASS | Earlier contaminated candidate rejected even with Rohiṇī. |
| Two-day priority | FAIL | Dig-darśinī preserves multiple practices. |
| Vrata / fast | PASS | Annual full vrata and fasting are explicit. |
| Pāraṇa day | PASS | Following day. |
| Pāraṇa termination | FAIL | Multiple source-backed policies remain. |

## 4. Source Chain

The source chain from HBV to the older quoted works was recorded in:

```text
05_Documentation/Festivals/Rule-Analysis/
FEST-001-Janmashtami-Primary-Source-Matrix.md
```

No second external primary work was downloaded. Consequently, every older
quoted work remains marked `externally verified: NO`. This does not invalidate
the Dig-darśinī's adjudication where it is explicit; it prevents claiming an
independent edition-level verification that Campaign 61.0T did not perform.

## 5. Conflicts Capable Of Changing Output

```text
RULE_CONFLICT-JAN-001:
Bhādra / Śrāvaṇa source nomenclature mapping
Can change observance search range: YES

RULE_CONFLICT-JAN-002:
Two clean candidate dates and competing Gauḍīya priorities
Can change civil observance date: YES

RULE_CONFLICT-JAN-004:
Exact niśītha boundary when used by a priority policy
Can change a tie result: YES

RULE_CONFLICT-JAN-003:
Pāraṇa termination policy
Can change pāraṇa output: YES
Can change observance date: NO
```

## 6. Existing Engine Inputs

| Input | Status after source audit |
|---|---|
| Sunrise | AVAILABLE NOW |
| Tithi at sunrise | AVAILABLE NOW |
| Tithi at arbitrary instant | AVAILABLE NOW |
| Tithi interval | DERIVABLE FROM EXISTING ENGINE |
| Pakṣa | AVAILABLE NOW |
| Native Vaiṣṇava māsa | AVAILABLE PROVISIONALLY; source-name mapping unresolved |
| Nakṣatra at arbitrary instant | AVAILABLE NOW |
| Rohiṇī interval | DERIVABLE FROM EXISTING ENGINE |
| Aṣṭamī-Rohiṇī overlap | DERIVABLE FROM EXISTING ENGINE |
| Exact niśītha interval | NOT READY; normative boundary missing |
| Janmāṣṭamī pāraṇa policy | NOT READY; policy choice missing |

No astronomical calculation needs to be duplicated.

## 7. Final Implementation Gate

```text
Source acquisition:
PASS

External source verification:
PASS

Complete Janmāṣṭamī section:
PASS

Primary source matrix:
PASS

Canonical rule:
FAIL

Source authority:
PASS

Rule conflicts:
UNRESOLVED

Astronomical inputs:
NOT READY

Existing engine reuse:
PASS

Test strategy:
PASS AS A PLAN

Invented doctrine:
0

Recommendation:
DO NOT IMPLEMENT YET
```

The failure is no longer caused by missing primary material. It is caused by
explicit policy alternatives preserved in that material. Campaign 61.1 must
remain closed until the project records:

1. the adopted Bhādra/Śrāvaṇa mapping;
2. the adopted two-day Gauḍīya priority;
3. the computable definition of niśītha if that priority uses it;
4. the adopted Janmāṣṭamī pāraṇa policy.

```text
ENGINE MODIFIED:
NO

ASTRONOMY MODIFIED:
NO

EKĀDAŚĪ ENGINE MODIFIED:
NO

KNOWLEDGE BASE RULE PROMOTED:
NO
```
