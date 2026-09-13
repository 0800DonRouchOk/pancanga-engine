# Existing Engine Audit for Future Festival Engine

Campaign: 61.0  
Status: PASS  
Mode: Audit only

## Objective

Determine what astronomical and calendrical information Pancanga Engine already
exposes, and what a future Festival Engine would need before implementing
additional Gauḍīya Vaiṣṇava festivals.

This audit does not modify the motor.

## Audit Matrix

| Required input | Current status | Existing source | Notes for Festival Engine |
| --- | --- | --- | --- |
| Current tithi | AVAILABLE | `astronomy::astronomical_tithi(elongation)` | Available for any Julian date once solar and lunar longitudes are computed. |
| Tithi start/end | PARTIAL | `calendar::tithi_transition_between_sunrises` | Current API returns the first transition between two sunrises. Festivals need reusable arbitrary-window interval search. |
| Tithi at sunrise | AVAILABLE | `calendar::tithi_at_sunrise` | Directly reusable for sunrise-based festival rules. |
| Sunrise | AVAILABLE | `calendar::sunrise` | Directly reusable. |
| Sunset | NOT EXPOSED | None found | Required for festivals observed until sunset or dusk. Must be added as a general calendar utility, not festival-specific logic. |
| Lunar month / māsa | AVAILABLE | `calendar::vaishnava_masa_at` | Native Vaiṣṇava māsa is available, with provisional documented ayanāṁśa status. |
| Pakṣa | AVAILABLE | `AstronomicalTithi::paksha` | Directly derived from tithi index. |
| Nakṣatra | AVAILABLE | `astronomy::nakshatra::index` | Instantaneous nakṣatra is available from lunar longitude. |
| Nakṣatra start/end | NOT EXPOSED | None found | Required for nakṣatra-combination rules such as Vāmana Dvādaśī / Śravaṇa. |
| Solar longitude | AVAILABLE | `astronomy::solar::apparent_longitude` | Stable Solar Engine API. |
| Lunar longitude | AVAILABLE | `astronomy::moon::apparent_longitude` | Stable Lunar Engine API. |
| Timezone | PARTIAL | RC1 city/config layer | Core location does not carry timezone/DST rules. Festival windows need explicit civil-time modeling. |
| Location | AVAILABLE | `GeoLocation` | Latitude and longitude are modeled in core types. |
| Events crossing midnight | PARTIAL | JulianDate intervals | Engine can represent instants, but no dedicated ritual-window model exists for midnight/noon/sunset crossing cases. |

## Main Finding

The current engine is strong enough to support a future Festival Engine, but the
new layer should first introduce a reusable `Pañcāṅga State` or equivalent
domain object rather than letting each festival recompute tithi, nakṣatra,
sunrise, māsa, pakṣa, and civil windows independently.

## Gaps for Campaign 61.1

```text
1. Reusable tithi interval finder for arbitrary windows.
2. Public sunset calculation.
3. Instantaneous nakṣatra exposed through a calendar state.
4. Reusable nakṣatra interval finder.
5. Civil ritual window model: sunrise, noon, sunset, midnight.
6. Timezone/DST source attached to a civil location configuration.
7. A read-only Pañcāṅga State object consumed by future festival rules.
```

## Certification

```text
Existing engine audit:
PASS

Engine modified:
NO

Astronomy modified:
NO

Ekādaśī logic modified:
NO
```
