# Simple Tithi Rule Comparison

Campaign: 62.1
Status: PRELIMINARY - NO IMPLEMENTATION AUTHORITY

| Festival | Masa | Paksa | Tithi | Sunrise requirement | Viddha/contact rule | Two-day rule | Fast | Parana | Special exception | Primary authority status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Radhastami | Bhadrapada candidate | Sukla | Astami | UNRESOLVED | UNRESOLVED | UNRESOLVED | a lineage source says HBV does not prescribe fasting; direct HBV passage not located | UNRESOLVED | reference-location policy may change civil date | PARTIAL; exact primary passage missing |
| Govardhana Puja | Karttika | Sukla | Pratipada | morning is explicit | prefer Amavasya-Pratipada contact; reject Dvitiya presence for the rite | para/purva algorithm still requires formal extraction | UNKNOWN | UNKNOWN | complete before noon | HBV 16.231-236 located; edition metadata/commentary review incomplete |
| Jagannatha Ratha-yatra | Asadha | Sukla | Dvitiya candidate | institutional source cites arunodaya procedure; selection role unresolved | disputed scope of alternative tithis | UNRESOLVED | UNKNOWN | UNKNOWN | nine-day cycle and Bahuda relation | Skanda/Brahma Purana leads; exact editions and conflict disposition pending |
| Lalita Saptami | Bhadrapada candidate | Sukla candidate | Saptami candidate | UNRESOLVED | Sanskrit lead mentions `trisandhya-vyapini`, interpretation unresolved | UNRESOLVED | UNKNOWN | UNKNOWN | verse identity may denote a different Lalita tradition | Bhavisya Purana 2.2.7.40 lead; accepted edition/context missing |
| Guru/Vyasa Purnima | Asadha candidate | Sukla | Purnima | UNRESOLVED | UNRESOLVED | UNRESOLVED | UNKNOWN | UNKNOWN | Gaudiya scope differs from general and lineage Vyasa-puja usage | no accepted Gaudiya Tier 1-3 date rule acquired |

## Cross-Festival Conflicts

### ST-CONFLICT-001 - Location Policy

Position A:

```text
Calculate the festival from local tithi and local sunrise.
```

Status: architectural expectation only; no universal festival policy approved.

Position B:

```text
SCS Math states that non-Ekadasi observances worldwide follow the Kolkata-based
Sri Gaudiya Parva Talika date, while Ekadasi and parana are calculated locally.
```

Source: SCS Math calendar introduction, Tier 3 lineage policy delivered by a
website. Impact: the same astronomical tithi can produce different civil dates
for distant locations.

Disposition: `UNRESOLVED`.

### ST-CONFLICT-002 - Ratha-yatra Scope

Position A: annual Puri Ratha-yatra begins on Asadha Sukla Dvitiya and concludes
through its prescribed cycle.

Position B: a documented opposing reading cites additional tithis for Ratha
yatra outside that single annual anchor.

Source status: an institutional response reproduces both positions and cites
Skanda Purana, Brahma Purana and related works, but the underlying editions have
not been acquired and classified.

Disposition: `UNRESOLVED`.

## Family Decision

```text
GENERIC_SIMPLE_TITHI_RULE: NOT ESTABLISHED
DATE-CHANGING UNRESOLVED CONFLICTS: 2
IMPLEMENTATION_READY: 0
```
