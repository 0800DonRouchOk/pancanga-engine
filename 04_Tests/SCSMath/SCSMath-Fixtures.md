# SCS Math Fixtures

Campaign: 46F

Status: 🟡 IMPORTED / REVIEW REQUIRED

## Source

External oracle:

```text
Sri Chaitanya Saraswat Math International
Sri Gaudiya Calendar 540-541 Gaurabda (2026-2027)
```

Local source:

```text
02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf
```

The PDF states that it is calculated for:

```text
Sri Chaitanya Saraswat Math, Nabadwip, India
```

## Extractor

Extractor:

```text
04_Tests/SCSMath/scsmath_pdf_extract.py
```

Output:

```text
04_Tests/SCSMath/scsmath-fixtures.csv
```

## Extracted Fields

The fixture CSV records:

```text
case_id
date
location
latitude
longitude
timezone
name
is_mahadvadasi
mahadvadasi_type
parana_start
parana_end
masa
paksha
source_file
status
```

## Result

```text
Fixtures extracted:
26

Mahadvadasi fixtures:
1

Displaced/no-fast patterns:
included as published observance dates
```

The extractor uses the published observance date. For example, when the source
prints `No fast` on Ekādaśī and `Fast on ...` on the following date, the
fixture records the published fast date.

## Review Notes

The PDF does not publish numeric latitude and longitude. The fixture uses a
Nabadwip coordinate configuration for automated comparison and therefore keeps
fixture rows in:

```text
IMPORTED_REQUIRES_REVIEW
```

No engine behavior or Knowledge Base rule is derived from these coordinates.
They are validation configuration only.

## Status

```text
Source:
LOCAL PDF AVAILABLE

Extractor:
PASS

Fixtures:
REAL DATA GENERATED

Manual review:
PENDING

Motor changes:
NO

Knowledge Base changes:
NO
```
