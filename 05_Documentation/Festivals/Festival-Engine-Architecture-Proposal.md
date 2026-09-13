# Festival Engine Architecture Proposal

Campaign: 61.0  
Status: PROPOSED  
Mode: No refactor

## Objective

Define the minimum architecture needed to add future Gauḍīya Vaiṣṇava festivals
without breaking the certified Ekādaśī and Mahādvādaśī pipeline.

## Current Stable Layers

```text
Astronomy Engine
        ↓
Calendar Engine
        ↓
Native Vaiṣṇava Māsa
        ↓
Observance Resolver
        ↓
ObservanceContent
        ↓
RC1 / API / Library
```

## Proposed Future Layers

```text
Astronomy Engine
        ↓
Calendar Engine
        ↓
Pañcāṅga State
        ↓
Observance Rule Engine
        ↓
Ekādaśī Resolver / Festival Resolver
        ↓
Knowledge Base / Festival Registry / Observance Library
        ↓
UI / API / Desktop
```

## Design Principle

The Festival Engine must consume already certified astronomical and calendrical
facts. It must not duplicate solar, lunar, tithi, nakṣatra, māsa, or pakṣa
calculations.

## Proposed Domain Objects

### Pañcāṅga State

Read-only state for a civil day and location:

```text
civil_date
location
timezone
sunrise
sunset
moon_longitude
sun_longitude
tithi_at_sunrise
tithi_intervals
nakṣatra_at_sunrise
nakṣatra_intervals
pakṣa
vaiṣṇava_māsa
```

### Festival Rule

Declarative rule object:

```text
festival_id
required_māsa
required_pakṣa
required_tithi
rule_class
required_window
nakṣatra_condition
fasting_policy
parāṇa_policy
source_status
```

### Festival Result

Output object:

```text
festival_id
display_name
observance_date
rule_class
fasting_requirement
parāṇa_requirement
source
rule_status
conflicts
```

## Integration Rule

Ekādaśī should remain the certified specialized resolver until a future campaign
proves that a generalized rule engine can reproduce it exactly.

In v1 terms:

```text
Do not migrate Ekādaśī into a generic festival engine yet.
```

## Minimum Campaign 61.1 Direction

The next campaign should introduce only reusable read-only infrastructure:

```text
Pañcāṅga State audit implementation
arbitrary tithi interval utility
sunset utility
optional nakṣatra interval utility only if required by approved rules
festival rule status model
```

No Janmāṣṭamī calculation should be implemented before the normative rule is
approved.

## Result

```text
Architecture proposal:
PASS

Refactor performed:
NO

Ekādaśī touched:
NO
```
