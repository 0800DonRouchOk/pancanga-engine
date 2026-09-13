# FEST-001 Janmāṣṭamī Primary Source Matrix

Campaign: 61.0T primary-source audit; 61.0U policy resolution  
Primary source: Gopāla Bhaṭṭa Gosvāmī, *Śrī Hari-bhakti-vilāsa*,
Vilāsa 15, mūla with Dig-darśinī-ṭīkā, Śrī Haridāsa Śāstrī edition,
Gadādhara-Gaurahari Press, Vṛndāvana, 1986  
Source verification: PASS

## Audited Range

The complete Janmāṣṭamī treatment occupies HBV 15.247-542. The portion that
can alter the computed observance date or pāraṇa is HBV 15.328-406. HBV
15.407-542 governs ritual procedure and completion rather than candidate-date
selection.

Line references below point to:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/
hbv-vilasa-15-mula-dig-darsini.txt
```

## Rule Matrix

| Rule ID | HBV verses | Sanskrit rule | Dig-darśinī interpretation | Older source cited by HBV | Conclusion | Confidence | Can change civil date |
|---|---|---|---|---|---|---|---|
| JR-01 | 15.262-263, 268, 328-331, 347-349 | `bhādre ... kṛṣṇāṣṭamī`; cited verses also use `śrāvaṇa`, `nabhas`, and `prauṣṭhapada` | The synthesis at 15.328 states Bhādra Kṛṣṇa Aṣṭamī. The commentary glosses `nabhas` as Śrāvaṇa and `prauṣṭhapada` as Bhādra. Campaign 61.0U resolves these as names for the same dark fortnight in different month conventions. | Bhaviṣyottara, Skanda, Viṣṇu-rahasya; amānta/pūrṇimānta calendrical correspondence | Engine-facing ID is pūrṇimānta Bhādrapada. Amānta Śrāvaṇa Kṛṣṇa is retained as an equivalent source label. This mapping does not change the date. | HIGH | NO |
| JR-02 | 15.262, 268, 328, 331, 334, 347-349 | `kṛṣṇapakṣe` / `asitāṣṭamī` / `bahule pakṣe` | No competing bright-half rule is admitted. | Bhaviṣyottara, Skanda, Viṣṇu-rahasya | Required pakṣa is Kṛṣṇa. | HIGH | YES |
| JR-03 | 15.247, 262, 268, 328-331 | `janmāṣṭamīvratam`; `kṛṣṇopāsyāṣṭamī` | The basic vrata remains required even when Rohiṇī and the other praised conjunctions are absent. | HBV synthesis; Bhaviṣyottara; Skanda; Viṣṇu-rahasya | Required tithi is Aṣṭamī. | HIGH | YES |
| JR-04 | 15.358-370, especially ṭīkā to 361 | `tyājyā viddhā ca saptamyā` | For tithis other than Ekādaśī, completeness is assessed from sunrise; the commentary rejects transferring the Ekādaśī Aruṇodaya threshold to Janmāṣṭamī. | Brahma-vaivarta, Padma, Yājñavalkya-smṛti | Sunrise matters as the Saptamī-viddhā boundary. | HIGH | YES |
| JR-05 | 15.328-406 | No sunset criterion stated | The audited determination and pāraṇa passages do not assign sunset a candidate-selection role. | None | Sunset is not part of the sourced selection rule. This is an absence finding, not a general doctrinal prohibition. | HIGH | NO |
| JR-06 | 15.328, 340-349, 379 | `niśīthe`; `arddharātre` | Aṣṭamī with Rohiṇī at the middle of the night is especially meritorious and is a worship/birth window. The commentary to 357 says midnight and Rohiṇī conjunctions enhance merit and are not mandatory for the vrata itself. | Agni, Bhaviṣya, Viṣṇudharma, Vahni, Bhaviṣyottara, Gautamīya Tantra; Calendar Reform Committee for the operational time definition | Niśītha is relevant but not universally required. Campaign 61.0U represents it as the eighth of fifteen local sunset-to-next-sunrise muhūrtas. | HIGH for role; MEDIUM-HIGH for the traditional computational representation | POSSIBLY, in tie cases |
| JR-07 | 15.328, 341-349 | `niśītha` / `arddharātra` | The commentary uses traditional night terminology and never equates it with `00:00` civil time. The external calendar definition identifies madhyarātri with the midpoint of the local night. | Same as JR-06 | Civil 00:00 must not be substituted for niśītha. Use the location-dependent central night muhūrta. | HIGH | YES if used as a priority fact |
| JR-08 | 15.329-340, 357 | `rohiṇī ... mahāphalā`; `rohiṇyādervyuktāpi sopoṣyā kevalāṣṭamī` | Rohiṇī makes the conjunction especially meritorious and gives it the name Jayantī, but absence of Rohiṇī does not cancel Janmāṣṭamī. | Viṣṇu-rahasya, Viṣṇudharmottara, Skanda, Viṣṇu Purāṇa, Agni | Rohiṇī is merit-enhancing and classificatory, not mandatory for the base vrata. | HIGH | NO for base eligibility; YES in disputed priority cases |
| JR-09 | 15.329-341 | `muhūrtam api`; `kalaikā`; `kalayā vā` | Even a small Aṣṭamī-Rohiṇī overlap qualifies the Jayantī conjunction. The commentary to 340-341 says the point is conjunction, not necessarily midnight, although midnight overlap is separately praised. | Viṣṇu-rahasya, Viṣṇu Purāṇa, Agni, Bhaviṣya, Viṣṇudharma | Any actual overlap can establish Jayantī; overlap at niśītha is a stronger merit condition. | MEDIUM-HIGH | YES only if the project adopts it as a tiebreaker |
| JR-10 | 15.352-355, 374 and ṭīkā | `udaye cāṣṭamī kiñcit ... navamī sakalā`; `śuddhā ca rohiṇīyuktā pūrva'hani paratra ca` | HBV 15.374 selects the earlier day when pure Aṣṭamī and Rohiṇī extend across both days. The commentary also describes cases selecting the later day and records a Vaiṣṇava practice that prefers the later day, concluding `atra sampradāyācāra eva gatiḥ`. | Skanda, Bhaviṣya, Padma; project synthesis | No single complete two-day priority algorithm is fixed for all subcases. | HIGH that a conflict exists | YES |
| JR-11 | 15.358-374 | `saptamīsaṁyutāṣṭamī` is to be rejected, even `saṛkṣā` | Saptamī-viddhā Aṣṭamī is rejected even with Rohiṇī. A very small Saptamī touch is condemned. The later, Navamī-connected day is preferred. The commentary locates the relevant completeness boundary at sunrise rather than Aruṇodaya. | Brahma-vaivarta, Padma, Yājñavalkya-smṛti, Skanda | FEST-001-specific Saptamī-viddhā rule is resolved: reject the sunrise-contaminated earlier candidate. | HIGH | YES |
| JR-12 | 15.328, 352-357, 370-374 | Multiple praised conditions: purity, Rohiṇī, niśītha, Navamī, Monday, Wednesday | Purity overrides Saptamī-viddhā. Beyond that, the commentary presents alternative priorities for clean competing days and refers the final choice to sampradāya practice. | Skanda, Bhaviṣya, Padma, Yājñavalkya-smṛti | Partial hierarchy only: purity is mandatory; the ordering of the remaining merits is unresolved as a project policy. | HIGH | YES |
| JR-13 | 15.247, 266-282, 377, 408-414, 464-465, 500 | `sarvair avaśyaṁ kartavyam`; `nirāhāraḥ sarvabhogavivarjitaḥ` | The vrata is annual and obligatory in the text. The vow may be taken after one meal on Saptamī or on Aṣṭamī morning; the votary remains without food and gives up enjoyment until the subsequent break. | Bhaviṣyottara, Skanda, Viṣṇu-rahasya, Gautamīya Tantra, Bhaviṣya | The full vrata includes fasting, not merely a midnight observance. Modern `fast until midnight, then anukalpa` remains operational practice unless separately sourced. | HIGH | NO for date; YES for fasting metadata |
| JR-14 | 15.374, 385-406 | `pare'hni pāraṇaṁ`; `aṣṭamyāmatha rohiṇyāṁ na kuryāt pāraṇam`; `tithyante cotsavānte vā` | Pāraṇa is on the following day. The commentary preserves multiple methods: first relevant ending, both endings for the capable, first ending for the unable, the tithi-end Rohiṇī exception, or festival end. | Gautamīya Tantra, Vahni, Yājñavalkya, Brahma-vaivarta, Garuḍa | Campaign 61.0U preserves these as explicit `PARANA_POLICY_PROFILE` values with no silent default. A universal latest clock time is not established. | HIGH | NO for observance date; YES for pāraṇa |
| JR-15 | 15.371-374, 382-395, 401-406 | Competing viddhā and pāraṇa statements are explicitly discussed | The Dig-darśinī rejects Saptamī-viddhā for Vaiṣṇavas, distinguishes non-Vaiṣṇava statements, and retains more than one Vaiṣṇava practice for clean two-day cases and pāraṇa. | Vahni, Agni, Padma, Viṣṇudharmottara, Skanda, Gautamīya Tantra, Yājñavalkya, Brahma-vaivarta, Garuḍa | Viddhā is resolved. Pāraṇa multiplicity is represented by profiles. The clean two-day date priority still requires a declared Gauḍīya project profile. | HIGH | YES for two-day cases |

## Source Chain Register

No second external work was acquired in Campaign 61.0T. The following older
texts are identified because HBV 15 quotes or names them. `Externally verified`
therefore means independently checked against a separate edition, not merely
present inside the acquired HBV source.

| Chain ID | HBV reference | Older source cited | Wording or rule represented in HBV | Externally verified |
|---|---|---|---|---|
| SC-JAN-001 | 15.329, 331 | Viṣṇu-rahasya | Aṣṭamī joined with Rohiṇī, even briefly, is especially meritorious and called Jayantī. | NO |
| SC-JAN-002 | 15.332-333; contrary quotation in ṭīkā to 371 | Viṣṇudharmottara | Jayantī definition and merit; a contrary viddhā/tithi-end position is recorded and delimited by the commentary. | NO |
| SC-JAN-003 | 15.334-335, 352, 372; ṭīkā to 361 | Skanda Purāṇa | Rohiṇī conjunction, rare Navamī-linked conjunction, and rejection of Saptamī-viddhā for the adopted Vaiṣṇava rule. | NO |
| SC-JAN-004 | 15.336-339 | Viṣṇu Purāṇa | Even a small Aṣṭamī-Rohiṇī overlap forms Jayantī and is observed with fasting. | NO |
| SC-JAN-005 | 15.340; contrary quotation in ṭīkā to 371 | Agni Purāṇa | Rohiṇī near midnight forms Jayantī; a contrary viddhā position is recorded but not adopted for Vaiṣṇavas. | NO |
| SC-JAN-006 | 15.341-345 | Bhaviṣya and Viṣṇudharma | Aṣṭamī-Rohiṇī around midnight, worship, fast, vigil, and Jayantī terminology. | NO |
| SC-JAN-007 | 15.346; 398-400; contrary quotation in ṭīkā to 371 | Vahni Purāṇa | Midnight conjunction and pāraṇa when a relevant tithi/nakṣatra factor ends; contrary viddhā wording is delimited. | NO |
| SC-JAN-008 | 15.347-349; 409-463 | Bhaviṣyottara Purāṇa | Bhādrapada Kṛṣṇa Aṣṭamī at midnight with Rohiṇī; worship and vrata procedure. | NO |
| SC-JAN-009 | 15.350-351, 355, 360-368 | Padma Purāṇa | Navamī-linked merit and strict rejection of Saptamī-viddhā even with Rohiṇī. | NO |
| SC-JAN-010 | 15.353; 464-495 | Bhaviṣya Purāṇa | Navamī/Rohiṇī/weekday conjunction and detailed vow procedure. | NO |
| SC-JAN-011 | 15.359, 403 | Brahma-vaivarta Purāṇa | Reject Saptamī-joined Aṣṭamī; do not perform pāraṇa while Aṣṭamī or Rohiṇī remains under the strict reading. | NO |
| SC-JAN-012 | 15.370, 402 | Yājñavalkya-smṛti | Prefer complete midnight Rohiṇī over a prior-contaminated day; special pāraṇa treatment for Rohiṇī. | NO |
| SC-JAN-013 | 15.375-386 | Gautamīya Tantra | Pure later-connected Aṣṭamī, rejection of prior contamination, and next-day daytime pāraṇa. | NO |
| SC-JAN-014 | 15.388 | Bhojarājīya | Explains the Umā-Māheśvarī name for Aṣṭamī joined with Navamī. | NO |
| SC-JAN-015 | 15.396 | Agneya and Viṣṇudharma | Vaiṣṇava/non-Vaiṣṇava distinction used in the viddhā discussion. | NO |
| SC-JAN-016 | 15.406 | Garuḍa Purāṇa | Pāraṇa at tithi end or festival end. | NO |
| SC-JAN-017 | 15.407 | Vāyu Purāṇa | Taking Jagannātha's food after the festival. | NO |

## Supplementary Calendrical Authorities

These sources do not replace HBV or the Dig-darśinī. They supply the
traditional calendar definitions needed to translate source terms into
deterministic calendar facts.

| Source ID | Source | Classification | Component | Finding | Normative use |
|---|---|---|---|---|---|
| CAL-JAN-001 | Robert Sewell and Śaṅkara Bālakṛṣṇa Dīkṣita, *The Indian Calendar* (London: Swan Sonnenschein & Co., Ltd., 1896), Table II Part I and article 51, printed p. 30 | Tier 4 scholarly/technical bibliography | Bhādra/Śrāvaṇa | States that bright fortnights retain the same name while the pūrṇimānta dark fortnight receives the following amānta month name. Thus amānta Śrāvaṇa Kṛṣṇa and pūrṇimānta Bhādrapada Kṛṣṇa are the same fortnight. | Resolve nomenclature only; never override HBV. |
| CAL-JAN-002 | Government of India, Calendar Reform Committee, *Report of the Calendar Reform Committee* (New Delhi: CSIR, 1955), printed pp. 107-108 and 160 | Tier 4 scholarly/technical bibliography | Niśītha | Defines niśītha/madhyarātri as two ghaṭikās covering midnight and night muhūrtas as fifteenths of the sunset-to-sunrise night. | Define the traditional interval used to operationalize the HBV term. |
| CAL-JAN-003 | PanchangTime, “How Nishita Muhurta Is Calculated” | Tier 6 web content | Niśītha corroboration | Describes the eighth of fifteen local night muhūrtas, centered on the sunset-to-next-sunrise midpoint. | Discovery/corroboration only. `NORMATIVE AUTHORITY: PROHIBITED`. |

Bibliographic metadata, local artifacts, hashes, and acquisition URLs for
CAL-JAN-001 and CAL-JAN-002 are recorded in:

```text
07_Sources/Calendrical/janmashtami-technical-source-record.md
```

The CAL-JAN-003 URL is retained only in acquisition/discovery records. It is
not part of the normative evidence chain.

## Campaign 61.0U Policy Classification

| Policy ID | Conclusion | Classification | Status |
|---|---|---|---|
| JAN-POL-001 | Amānta Śrāvaṇa Kṛṣṇa and pūrṇimānta Bhādrapada Kṛṣṇa are the same lunation. | TRADITIONAL CALENDAR DEFINITION | RESOLVED |
| JAN-POL-002 | The engine-facing month ID is Bhādrapada. | IMPLEMENTATION POLICY aligned with the existing pūrṇimānta model | RESOLVED |
| JAN-POL-003 | Niśītha is the central/eighth local night muhūrta, not civil 00:00. | TRADITIONAL CALENDAR DEFINITION | RESOLVED |
| JAN-POL-004 | The niśītha interval is represented as `[S + 7N/15, S + 8N/15)`. | IMPLEMENTATION POLICY derived from the traditional definition | RESOLVED |
| JAN-POL-005 | Pāraṇa alternatives are explicit profiles with no default. | IMPLEMENTATION POLICY preserving textual alternatives | RESOLVED |
| JAN-POL-006 | A project two-day sampradāya profile must select among the DDT alternatives. | IMPLEMENTATION POLICY | NORMATIVE_DECISION_REQUIRED |

## Resolved Components

```text
Pakṣa:
Kṛṣṇa

Tithi:
Aṣṭamī

Rohiṇī:
Not mandatory for the base vrata; merit-enhancing and Jayantī-classifying

Saptamī-viddhā:
Rejected at the sunrise purity boundary, even when Rohiṇī is present

Vrata:
Annual fast and observance; the full HBV vow is not reduced to midnight only

Bhādra/Śrāvaṇa:
Same Kṛṣṇa pakṣa under pūrṇimānta/amānta naming; engine ID Bhādrapada

Niśītha:
Central eighth muhūrta of the local sunset-to-next-sunrise night

Pāraṇa:
Explicit profile family; no silent default and no universal latest limit
```

## Unresolved Components

```text
Complete priority order for two clean candidate dates
Project approval of the named two-day sampradāya policy profile
```

Result:

```text
PRIMARY SOURCE MATRIX:
PASS

CAMPAIGN 61.0U POLICY MATRIX:
PASS WITH ONE EXPLICIT NORMATIVE DECISION

INVENTED DOCTRINE:
0
```
