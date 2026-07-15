# Pancanga Engine

Version: v1.0 RC1

> An offline, auditable Gaudiya Calendar Engine.

Pancanga Engine is a Rust-based offline engine for calculating and serving
Gaudiya calendar data with traceability from normative sources to
implementation and external certification.

## Release Candidate

Pancanga Engine is in v1.0 Release Candidate status.

```text
Development:
🔒 CLOSED

Certification:
🟡 FINAL STAGE

Release:
PENDING

Knowledge Base:
🔒 FROZEN

Astronomy:
🔒 FROZEN

Architecture:
🔒 FROZEN

Scope:
🔒 FROZEN
```

v1.0 means the original objective has been fulfilled:

```text
✓ Astronomy certified
✓ Calendar Engine stable
✓ Knowledge Base complete
✓ Vaiṣṇava Engine implemented
✓ External-oracle certification performed
✓ Open doctrinal issues: 0
✓ No confirmed ENGINE BUG to date
✓ Release Freeze respected
```

## Astronomical Engine v1.0

Certified capabilities:

```text
✅ astronomy::solar::apparent_longitude(jd)

✅ astronomy::moon::apparent_longitude(jd)

✅ astronomy::lunar_solar_elongation(moon_longitude, sun_longitude)

✅ astronomy::tithi_index(elongation)

✅ astronomy::astronomical_tithi(elongation)
```

Swiss Ephemeris certification:

```text
1000 / 1000 PASS
Tithi mismatches: 0
Confirmed ENGINE BUG: 0
```

## Calendar And Vaiṣṇava Engine

The base calendar engine and Vaiṣṇava engine are implemented and locally
validated. The Vaiṣṇava engine implements the frozen Hari-bhakti-vilāsa
Knowledge Base:

```text
HBV-EK-001  Candidate Ekādaśī
HBV-EK-002  Viddhā
HBV-EK-003  Observance displacement
HBV-EK-004  Mahādvādaśī
HBV-EK-005  Parāṇa / Hari-vāsara
```

External certification has established a cross-oracle pattern with no confirmed
engine bug so far:

```text
Swiss Ephemeris
PureBhakti
Sri Chaitanya Saraswat Math

Confirmed ENGINE BUG:
0
```

Remaining v1.0 work is limited to final external certification and release
publication, with the RC1 Experience before the release audit.

The objective exit criterion for v1.0 is documented in
[Final Certification Stage](00_Project/Final-Certification-Stage.md).

RC1 Experience:

[Campaign 46BETA](00_Project/RC1-Experience.md)

The RC1 Experience is a minimal application, not a developer-only command. It
must answer:

```text
Do I fast today?
When is Parāṇa?
Why did the engine decide this?
```

RC rule:

```text
No new functionality.

Allowed:
bug fixes
documentation
certification
release preparation
```

## Rust Toolchain

Pancanga Engine uses Rust `1.85.0` as its Minimum Supported Rust Version
(MSRV). The repository pins this toolchain in `rust-toolchain.toml` and installs
the `rustfmt` and `clippy` components.

Development checks:

```text
cd 03_Source/rust
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo doc --workspace --all-features --no-deps
```

## Repository Layout

- `00_Project/` - project vision, roadmap, changelog, and task tracking.
- `01_Specification/` - formal specifications, separated into `PES/` and
  `ARCH/`.
- `02_Research/` - source material and reference research, including one
  research folder per PES.
- `03_Source/rust/` - Rust workspace and source crates.
- `04_Tests/` - future astronomical, regression, and reference tests.
- `05_Documentation/` - API, mathematics, architecture, and image assets.
- `06_Data/` - future strings, time zone, festival, and reference data.
- `07_Sources/` - source authority and validation traceability.
- `07_AI/` - AI-assisted research and development notes.
- `08_Examples/` - future usage examples.
- `09_Benchmarks/` - future performance benchmarks.
- `10_Tools/` - future developer and data tooling.
- `99_Archive/` - historical or retired material.

## Project Documents

- [Project Principles](PROJECT_PRINCIPLES.md)
- [Why Pancanga Engine Exists](WHY_PANCANGA_ENGINE_EXISTS.md)
- [Certification](CERTIFICATION.md)
- [Specification Index](01_Specification/INDEX.md)
- [Project Status](PROJECT_STATUS.md)
- [Roadmap Oficial v1.0](00_Project/Roadmap-v1.0.md)
- [Component Maturity](00_Project/Component-Maturity.md)
- [Milestone 01 - Solar Engine v1.0](00_Project/Milestone-01-Solar-Engine.md)
- [Milestone 03 - Astronomical Engine v1.0](00_Project/Milestone-03-Astronomical-Engine-v1.0.md)
- [Solar Engine API Review](00_Project/Solar-Engine-API-Review.md)
- [PES-0001](01_Specification/PES/PES-0001.md) - 🔒 Congelado
- [PES-0002](01_Specification/PES/PES-0002.md) - 🟡 En revisión
- [ARCH-0001](01_Specification/ARCH/ARCH-0001.md) - 🔒 Congelado
- [ARCH-0002](01_Specification/ARCH/ARCH-0002.md) - 🟡 En revisión final
- [ADR](05_Documentation/ADR)
- [ADR-0004](05_Documentation/ADR/ADR-0004.md) - VSOP87D para la cadena solar
- [ADR-0007](05_Documentation/ADR/ADR-0007.md) - Referencia externa de validación científica
- [RFC](05_Documentation/RFC/README.md)
- [Algorithm Catalog](05_Documentation/Algorithms/README.md)
- [Knowledge Base](05_Documentation/Knowledge-Base/README.md)
- [HBV Extraction Protocol](05_Documentation/Knowledge-Base/HBV-Extraction-Protocol.md)
- [Sources](07_Sources/README.md)
- [Technical Book](05_Documentation/Book/README.md)
