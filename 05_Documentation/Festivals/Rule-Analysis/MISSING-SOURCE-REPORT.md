# Campaign 61.0T - Local Source Inventory

## Resolution Update

This report preserves the result of the local-only audit. The missing source
was subsequently acquired in Campaign 61.0T on 2026-09-12 from Gaudiya Grantha
Mandira / Jiva Institute and verified as complete.

```text
LOCAL-ONLY AUDIT RESULT:
LOCAL PRIMARY SOURCE MISSING

CURRENT SOURCE STATUS:
PRIMARY SOURCE ACQUIRED AND VERIFIED
```

Current source record:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/
hbv-vilasa-15-source-record.md
```

## Status

```text
LOCAL PRIMARY SOURCE MISSING

External downloads performed: 0
Janmastami implementation: NOT AUTHORIZED
Campaign 61.0T: SOURCE MATERIAL STILL REQUIRED
```

## Audit Scope

The complete project root was inspected recursively:

```text
/Users/donrouch/Documents/CALCULADORA EKADASI/Pancanga Engine/
```

The audit included:

- 4,785 physical files, including hidden files and Git storage;
- 499 files outside Git objects, Rust build output, and Wrangler cache;
- 201 candidate document files (`PDF`, `TXT`, `DOC`, `DOCX`, and Markdown);
- all text-bearing pages of both local PDFs;
- the internal contents of the local `DOCX` and converted text from the local
  `DOC`;
- all local Markdown, HTML, JSON, TXT, and source files by content, not only by
  filename;
- the complete Git object/path history, to detect a source that might have
  existed in an earlier revision;
- image and EPUB extensions throughout the project.

No EPUB files or standalone scanned-document images are present. The two PDFs
have extractable text on every page; neither hides an unindexed scanned edition
of Hari-bhakti-vilasa. No earlier Git revision contains a different HBV volume
or a Vilasa 15 source.

## Local Source Inventory

| File | Path | Format | Contents | Vilasas present | Vilasa 15 present | Dig-darsini present | Usable for 61.0T |
|---|---|---|---|---|---|---|---|
| `hbv-vilasa-12.docx` | `07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-12.docx` | DOCX | Sanskrit mula and Sanskrit commentary for the Ekadasi determination chapter | XII, complete | NO | YES, for XII only | NO |
| `hbv-vilasa-12.txt` | `07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-12.txt` | TXT | Text export of the same Sanskrit source | XII, complete | NO | YES, for XII only | NO |
| `hbv-vilasa-13.doc` | `07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.doc` | DOC | English translated compilation with DDT material: complete XII-XIII and selected passages from XIV-XVI | XII-XIII complete; XIV-XVI excerpts | PARTIAL EXCERPTS ONLY; FEST-001 block absent | YES, only for included passages | NO |
| `hbv-vilasa-13.txt` | `07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.txt` | TXT | Text export identical in content to the local DOC | XII-XIII complete; XIV-XVI excerpts | PARTIAL EXCERPTS ONLY; FEST-001 block absent | YES, only for included passages | NO |
| `204176529-Ekadashi-Es-Ramanuja-Dasa.pdf` | `07_Sources/Vaishnava/204176529-Ekadashi-Es-Ramanuja-Dasa.pdf` | PDF, 102 pages | Spanish secondary compilation of Ekadasi stories and selected HBV XII-XIII rules; one general note says the Janmastami fast lasts until midnight | No complete HBV vilasa | NO | Mentions Digdarsini secondarily; no primary commentary text for FEST-001 | NO |
| `GaudiyaCalendar_2026-2027.pdf` | `02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf` | PDF, 64 pages | Operational Gaudiya calendar; records the 2026 observance date, fast until midnight, and next-day parana | None | NO | NO | NO; validation oracle only |
| `Calendario-Vaishnava-2026.html` | `02_Research/PureBhakti/Calendario-Vaishnava-2026.html` | HTML | Operational calendar with a Janmastami date, fast instruction, and parana window | None | NO | NO | NO; validation oracle only |
| FEST-001 research set | `05_Documentation/Festivals/` and `04_Tests/Festivals/` | Markdown | Dossier, evidence log, rule analyses, architecture proposal, registry, and implementation gate produced from the sources above | References only | NO primary text | NO primary commentary | NO; derived documentation |
| Existing HBV/KB records | `05_Documentation/Knowledge-Base/` and `05_Documentation/Algorithms/` | Markdown | Extracted Ekadasi and Mahadvadasi rules based primarily on Vilasas XII-XIII | XII-XIII references | NO | Only XII-XIII-derived material | NO for FEST-001 |
| Project summaries | `PANCANGA_ENGINE_MASTER_REFERENCE.md`, `PROJECT_STATUS.md`, `CERTIFICATION.md`, `00_Project/` | Markdown | Status, roadmap, certification, and historical summaries | References only | NO | NO | NO |
| RC1/Desktop/code artifacts | `08_Examples/`, `dist/macOS/`, `03_Source/` | HTML, JSON, Rust, binary | Presentation, catalog metadata, resolver code, and packaged copies | None | NO | NO | NO |

`YES` under Dig-darsini means that commentary is present for the passages that
the corresponding local file actually includes. It does not mean that the
Dig-darsini commentary on the Janmastami section is present.

## Misleading Partial Vilasa 15 Finding

The file named `hbv-vilasa-13.doc` is broader than its filename suggests. Its
internal index and headings show:

```text
Vilasa XII                 complete
Vilasa XIII                complete
FROM VILASA XIV            selected passages
FROM VILASA XV             selected passages
FROM VILASA XVI            selected passages
```

The corresponding TXT has the same content. The local Vilasa XV extract begins
with Nirjala Ekadasi, includes Sayana and Pavitropana material, and later jumps
to Parsva-Parivartana and Vamana material. The numbering jumps from the early
Vilasa XV material through verse 240 to verse 543.

The complete converted document was searched for all requested variants of:

```text
Janmastami / Janmashtami / Janmastami with diacritics
Krishna-janmastami
nisitha / midnight
Rohini
Saptami-viddha Astami
```

No Janmastami rule passage occurs in the Vilasa XV extract. The occurrences of
Rohini and Jayanti in this local document belong to the Jayanti Mahadvadasi
discussion in Vilasa XIII, not to FEST-001. They cannot be transferred by
analogy.

## PDF Inspection Result

Both local PDFs were inspected page by page rather than by filename or partial
search:

| PDF | Pages | Pages with extractable text | Result |
|---|---:|---:|---|
| `204176529-Ekadashi-Es-Ramanuja-Dasa.pdf` | 102 | 102 | Secondary Ekadasi compilation; no HBV Vilasa 15 |
| `GaudiyaCalendar_2026-2027.pdf` | 64 | 64 | Operational calendar; no HBV text |

The Ramanuja Das PDF contains a general statement that Krishna Janmastami is
fasted until midnight. It does not provide the complete Janmastami decision
rule, the two-day hierarchy, the FEST-001 viddha rule, the role of Rohini, the
definition of nisitha, or the parana rule from HBV Vilasa 15. It therefore
cannot close any date-changing JR component.

## Git History Result

The complete Git history contains only the same two PDFs and the same four HBV
source artifacts listed above. No deleted or renamed copy of Vilasa 15, no
complete HBV edition, and no additional Dig-darsini source was found.

## Exact Missing Primary Source

Campaign 61.0T requires a traceable local edition containing:

1. the complete `Hari-bhakti-vilasa`, Vilasa XV (`divyavirbhava`), including
   the full Janmastami determination section;
2. the corresponding Sanskrit mula with stable verse numbering;
3. the Dig-darsini-tika for those same verses;
4. edition metadata sufficient to identify editor, publisher, publication
   date, volume, and page range;
5. a translation may accompany the source, but it cannot replace the Sanskrit
   mula and commentary.

The local translated compilation identifies the edition it used as:

```text
Sri Haribhaktivilasah, Vols. I-III
Editor: Sri Haridasa Sastri
Publisher: Gadadhara-Gaurahari Press, Vrindavana
Year: 1986
```

The volume containing the complete Vilasa XV, with Dig-darsini-tika, is the
minimum source acquisition needed to resume this campaign. An equivalent
traceable edition is acceptable if it preserves the complete mula, commentary,
and verse numbering.

## Final Finding

```text
LOCAL PRIMARY SOURCE MISSING

Vilasa 15 complete:
NO

Janmastami normative section:
NO

Dig-darsini for Janmastami:
NO

Rule-resolution authority:
FAIL

Invented doctrine:
0

Recommendation:
SOURCE MATERIAL STILL REQUIRED
```

Campaign 61.0T stops here. No FEST-001 rule may be promoted, and Campaign 61.1
must not implement Janmastami until the missing primary source has been added
to the local corpus and audited.
