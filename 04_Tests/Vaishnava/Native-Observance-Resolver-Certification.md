# Native Observance Resolver Certification

Campaign: 52.1

Mode: certification only. No Rust source, Knowledge Base, architecture, or rule logic was modified.

## Configuration

- Location: Buenos Aires, Argentina
- Latitude: -34.6037
- Longitude: -58.3816
- Range: 2000-01-01 to 2049-12-31
- Civil days scanned: 18263
- Observance days detected in range: 1188

## Scope Boundary

The chronological scan certifies ordinary Ekādaśī identity resolution from calculated civil days. Mahādvādaśī identity coverage is certified directly against the existing HBV-EK-004 rule output enum, because automatic derivation of every Mahādvādaśī fact from a civil date is outside this resolver-only campaign.

## Results

- Ordinary Ekādaśīs detected by date scan: 24 / 24
- Ordinary Ekādaśīs covered by direct resolver domain: 24 / 24
- Mahādvādaśīs covered by direct resolver domain: 8 / 8
- Metadata / relationship inconsistencies: 0
- Native māsa errors during date scan: 0
- Confirmed ENGINE BUG: 0

## Ekādaśī Coverage

Date scan:

- ✓ EK-001
- ✓ EK-002
- ✓ EK-003
- ✓ EK-004
- ✓ EK-005
- ✓ EK-006
- ✓ EK-007
- ✓ EK-008
- ✓ EK-009
- ✓ EK-010
- ✓ EK-011
- ✓ EK-012
- ✓ EK-013
- ✓ EK-014
- ✓ EK-015
- ✓ EK-016
- ✓ EK-017
- ✓ EK-018
- ✓ EK-019
- ✓ EK-020
- ✓ EK-021
- ✓ EK-022
- ✓ EK-023
- ✓ EK-024

Direct resolver:

- ✓ EK-001
- ✓ EK-002
- ✓ EK-003
- ✓ EK-004
- ✓ EK-005
- ✓ EK-006
- ✓ EK-007
- ✓ EK-008
- ✓ EK-009
- ✓ EK-010
- ✓ EK-011
- ✓ EK-012
- ✓ EK-013
- ✓ EK-014
- ✓ EK-015
- ✓ EK-016
- ✓ EK-017
- ✓ EK-018
- ✓ EK-019
- ✓ EK-020
- ✓ EK-021
- ✓ EK-022
- ✓ EK-023
- ✓ EK-024

## Mahādvādaśī Coverage

Direct resolver:

- ✓ MD-001
- ✓ MD-002
- ✓ MD-003
- ✓ MD-004
- ✓ MD-005
- ✓ MD-006
- ✓ MD-007
- ✓ MD-008

## fasting_for Integrity

PASS

- Ordinary Ekādaśī records have no `fasting_for` relation.
- Mahādvādaśī records with supplied context point to existing Ekādaśī identifiers.
- No self-reference or cycle was detected.

## Certification Status

PASS
