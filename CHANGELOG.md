# Changelog

All notable changes to Pancanga Engine are documented in this file. The
format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

A separate, earlier and partial changelog exists at
`00_Project/CHANGELOG.md`, covering only the pre-implementation Knowledge
Base v1.0 freeze (2026-07-14) and the initial `0.1.0 Alpha` project
bootstrap. It was not superseded or edited by this document; this file is
the authoritative changelog for the engine and certification work that
followed it.

Every entry below is sourced from `PROJECT_STATUS.md`, `CERTIFICATION.md`,
and the campaign evidence files they cite. Nothing here is asserted beyond
what those documents record.

## [Unreleased]

- Release documentation (this file, `RELEASE_NOTES.md`, and the
  corresponding `README.md`/`PROJECT_PRINCIPLES.md` review) prepared under
  Campaign 46R Phase 2, ahead of the still-pending Campaign 47 v1.0.0
  publication.

## [1.0.0] - 2026-09-15

This date reflects when external and deterministic certification closed
(`CERTIFICATION.md`'s own closing dates); it is not a claim that Campaign
47 (the v1.0.0 tag/publication) has run. Update this heading's date if it
diverges from the actual publication date.

### Added

- **Astronomy Engine**: solar apparent longitude (VSOP87D, aberration,
  nutation in longitude); lunar apparent longitude (ELP2000, the 12
  integrated longitude families `ELP1, ELP4, ELP7, ELP10, ELP13, ELP16,
  ELP19, ELP22, ELP25, ELP28, ELP31, ELP34`, plus the AST-L006 frame
  transformation to bring ELP2000 longitude into the mean/date frame);
  Sun-Moon elongation; astronomical tithi index and `AstronomicalTithi`.
- **Calendar Engine**: `calendar::sunrise(date, location)`,
  `calendar::tithi_at_sunrise(sunrise_jd)`, tithi transition and presence
  across the civil day (`tithi_transition_between_sunrises`,
  `tithi_presence_between_sunrises`), and the Nakṣatra index
  (`astronomy::nakshatra::index`).
- **Vaiṣṇava Engine**, implementing Hari-bhakti-vilāsa rules HBV-EK-001
  through HBV-EK-005: candidate Ekādaśī generation, Viddhā invalidation
  via Aruṇodaya, observance displacement to Dvādaśī, the eight
  Mahādvādaśī classifications (tithi- and nakṣatra-based), and the
  Parāṇa/Hari-vāsara window (including the `ShortDvadasi` contingency);
  unified under `vaishnava::classify_vaishnava_day(...)`.
- **Native Vaiṣṇava Māsa**: provisional pūrṇimānta / saṅkrānti-count
  specification (Campaigns 50.1-51.0), and the Native Observance
  Resolver with typed `ObservanceId`, unified `ObservanceContent`, and
  `fasting_for` modeled as a relation between observances rather than
  text (Campaign 52.0).
- **Knowledge Base v1.0** (frozen 2026-07-14): HBV-EK-001 to HBV-EK-005,
  the source hierarchy (Hari-bhakti-vilāsa / Digdarśinī-ṭīkā / Navadvīpa
  Pañjikā / Tradition Notes), and 24/24 published ordinary Ekādaśī
  content records with 8/8 Mahādvādaśī identifiers resolvable; RC1
  content server exposing 32/32 cataloged routes.
- **External certification**:
  - Swiss Ephemeris: 1000/1000 PASS, 0 tithi mismatches, 0 confirmed
    engine bugs.
  - PureBhakti (Buenos Aires, Argentina): 16/16 observance dates PASS, 0
    confirmed engine bugs; Parāṇa differences documented as 13 editorial
    and 2 rounding differences, not engine bugs.
  - Sri Chaitanya Saraswat Math (Navadvīpa, India): 23/26 observance
    dates PASS (Campaign 46C Final rerun, 2026-09-14, against the
    ORDEN-6 engine state), 3 remaining differences analyzed with 0
    confirmed engine bugs.
- **Internal Consistency Certification** (Campaign 46STRESS): every civil
  day from 1900-01-01 to 2100-12-31 (73,414 days) at Valencia, España — 0
  crashes, panics, NaN/infinite values, invalid dates, negative
  intervals, or logical contradictions.
- **Deterministic Certification** (Campaign 46DETERMINISM): the release
  build of `examples/internal_stress_test` produced an identical SHA-256
  hash across 10 consecutive runs. Scope: the 14 aggregate stats fields,
  6 extreme-value rows, and issues list — not a per-day trace (see Known
  Limitations).
- **RC1 Experience** (Campaign 46BETA): a minimal local web application
  (`examples/rc1_experience.rs`) serving a single-screen Ekādaśī decision
  view, a "Why?" HBV-rule explanation trace, an astronomy view, and a
  certification-status summary, for three shipped locations: Valencia
  (España), Buenos Aires (Argentina), and Navadvīpa (India).
- **macOS packaging** (Campaign 60.0/60.1): a `Pancanga Engine.app`
  bundle starting the embedded RC1 backend without a terminal or manual
  browser navigation.

### Fixed

- **ORDEN-6 — `calendar::sunrise()` day-carry defect.** The function
  normalized the almanac local mean time into `[0, 24)` and attached it
  to the requested civil date, which silently returned the *following*
  local day's sunrise instant for any location whose local sunrise falls
  before 0h UT (at Navadvīpa, roughly early April to early October every
  year). Fixed by building the returned instant from the requested
  date's 0h UT plus an unnormalized UT offset, letting the day carry
  fall onto the correct adjacent UTC date (commit `2a6d5fd`). Four
  Sydney golden-test expectations were corrected under explicit owner
  authorization to match (commit `ed9450a`); no other expected value was
  changed. This moved SCS Math (Navadvīpa) observance-date certification
  from 11/26 to 23/26 PASS after a full rerun (Campaign 46C Final,
  2026-09-14); Valencia and Buenos Aires results were unaffected (0
  observance-date changes, confirmed by rerun, not assumed).

### Changed

- SCS Math certification decision: `OPEN` with 15 analyzed differences
  (Campaigns 46F/46F.1) → `PASS` at 23/26 observance dates with 3
  remaining differences, following the ORDEN-6 rerun and the subsequent
  evidence-reconciliation campaigns that annotated the 12 now-superseded
  `DOC-001` classifications without erasing their original text.
- Release Gate disposition (`CERTIFICATION.md § Release Gate`, all 4
  items closed 2026-09-15): GCal fixtures declared deliberately unfilled
  for v1.0 — PureBhakti and SCS Math, two bibliographically distinct and
  measurably non-redundant Tier 5 oracles, are treated as sufficient for
  the v1.0 validation set; GCal acquisition deferred to v1.1.

### Known limitations (deferred to v1.0.1 / v1.1)

- **GCal oracle not populated.** No real fixture source has been
  acquired; the importer is ready but has no local data to import.
- **Civil-configuration defects in `examples/`, outside `src/`**,
  documented but not remediated for v1.0 (Campaign 46F.2): a duplicated
  `sunset()` day-carry bug, and incomplete UTC-offset/DST handling in
  `is_europe_dst`. Neither reaches an Ekādaśī date for the three shipped
  locations. Remediation drafted as Phase 2 of ORDEN-3A.
- **Deterministic Certification covers aggregate statistics only.** It
  does not prove bit-for-bit identity of every individual day's computed
  result across runs, because the certified binary emits no per-day
  trace. A full per-day hash is deferred to v1.1.
