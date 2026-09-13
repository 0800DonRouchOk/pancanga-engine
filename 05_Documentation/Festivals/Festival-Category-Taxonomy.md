# Festival Category Taxonomy

Campaign: 62.0  
Status: STRUCTURAL BASELINE  
Scope: non-Ekadasi festivals and observances discovered in the current local corpus

## Purpose

This taxonomy organizes candidates for research and product planning. A category
does not establish a date rule, doctrinal authority, fasting duty, or priority of
one tradition over another.

The existing 24 Ekadasis and 8 Mahadvadasis remain in the Observance Catalog and
are linked as an already implemented domain. They are not duplicated with new
`FEST-*` identifiers here.

## Categories

| Code | Category | Includes | Boundary |
| --- | --- | --- | --- |
| A | Bhagavan / avatara festivals | Appearance and major lila days of Bhagavan and avataras | Gaura-lila uses C as its primary product category, with A as a cross-category |
| B | Srimati Radharani / Vraja festivals | Radha, Vraja-lila, rasa, jhulan and Vraja vrata observances | General Karttika vrata mechanics remain in G |
| C | Caitanya-lila festivals | Panca-tattva, Gaura-lila and directly related associates/events | Acarya commemorations whose principal identity is broader use D |
| D | Gosvami / acarya observances | Appearance and disappearance days of major Gaudiya teachers | Institution-specific modern calendars are discovery evidence only |
| E | Nityananda Vamsa / parivara | Family, lineage, guru and parivara observances | Requires an explicit lineage profile; never populated from model memory |
| F | Jagannatha festival cycle | Snana, Anavasara, Gundica, Ratha, Hera, Bahuda and associated events | Relative-day and interval relations must be researched as one cycle |
| G | Karttika / Damodara cycle | Month opening/closing, dipa observances, rasa, Govardhana and related days | A festival may cross-reference B or I without receiving a second ID |
| H | Caturmasya | Opening, four vrata phases, transitions and ending | Calendar detection is separate from dietary or devotional content |
| I | Special vratas | Important vratas distinct from ordinary Ekadasi | Fasting and parana remain `UNKNOWN` until sourced |
| J | Solar / seasonal events | Events potentially dependent on solar month, sankranti or season | A solar marker observed in a calendar is not yet a normative rule |

## Subcategories

The minimum subcategory vocabulary is:

```text
APPEARANCE
DISAPPEARANCE
LILA
PROCESSION
PREPARATION
RETURN
VRATA_BEGIN
VRATA_PHASE
VRATA_END
RELATIVE_DAY
LINEAGE_PROFILE
SOLAR_MARKER
SEASONAL
```

Subcategories are descriptive. They do not determine calculation behavior.

## Cross-Category Policy

One observance has one primary category and one canonical ID. Related categories
are recorded as cross-references, not duplicate records. Examples:

```text
Gaura Purnima       primary C; cross-category A
Govardhana Puja     primary B; cross-category G
Rasa Purnima        primary B; cross-category G
Nityananda events   primary C or E according to scope, never both as duplicates
```

## Alias Control

Aliases include translations, common transliteration variants and names used by
the two local operational calendars. An alias cannot receive an independent ID
unless research later proves that it denotes a distinct observance.

Examples consolidated in Campaign 62.0:

```text
Nrsimha / Narasimha
Ratha-yatra / Car Festival
Bahuda / Return Ratha
Dipavali / Dipanvita / Festival of Lights
Jhulan-yatra / Swing Festival
Bahulastami / Radha-kunda appearance
Rasa Purnima / Krishna Rasa-yatra
```

## Status Rule

Every calendar rule not already governed by an approved local specification is:

```text
RESEARCH_REQUIRED
```

Nityananda Vamsa or other profile-specific observances use:

```text
LINEAGE_RESEARCH_REQUIRED
```

## Result

```text
Category taxonomy: PASS
Calculation rules added: 0
Invented doctrine: 0
```
