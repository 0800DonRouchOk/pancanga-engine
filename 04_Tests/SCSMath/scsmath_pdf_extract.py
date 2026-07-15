#!/usr/bin/env python3
"""Extract SCS Math Ekadasi fixtures from the annual PDF calendar.

This script is test infrastructure only. It does not define normative rules
and it does not modify Pancanga Engine behavior.
"""

from __future__ import annotations

import csv
import re
import sys
from dataclasses import dataclass
from pathlib import Path

try:
    import pdfplumber
except ImportError as exc:  # pragma: no cover - environment guard
    raise SystemExit("pdfplumber is required to extract SCS Math PDF fixtures") from exc


DEFAULT_INPUT = Path("02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf")
DEFAULT_OUTPUT = Path("04_Tests/SCSMath/scsmath-fixtures.csv")

LOCATION = "Sri Chaitanya Saraswat Math, Nabadwip, India"
LATITUDE = "23.4071"
LONGITUDE = "88.3679"
TIMEZONE = "Asia/Kolkata"

HEADERS = [
    "case_id",
    "date",
    "location",
    "latitude",
    "longitude",
    "timezone",
    "name",
    "is_mahadvadasi",
    "mahadvadasi_type",
    "parana_start",
    "parana_end",
    "masa",
    "paksha",
    "source_file",
    "status",
]

MONTHS = {
    "JANUARY": 1,
    "FEBRUARY": 2,
    "MARCH": 3,
    "APRIL": 4,
    "MAY": 5,
    "JUNE": 6,
    "JULY": 7,
    "AUGUST": 8,
    "SEPTEMBER": 9,
    "OCTOBER": 10,
    "NOVEMBER": 11,
    "DECEMBER": 12,
}

KNOWN_OBSERVANCES = [
    "Pāpa Vimochanī Ekādaśī",
    "Kāmadā Ekādaśī",
    "Varuthinī Ekādaśī",
    "Mohinī Ekādaśī",
    "Aparā Ekādaśī",
    "Padminī Ekādaśī",
    "Paramā Ekādaśī",
    "Pāṇḍavā Nirjalā Ekādaśī",
    "Yoginī Ekādaśī",
    "Śayan Ekādaśī",
    "Kāmikā Ekādaśī",
    "Vyañjulī Mahādvādaśī",
    "Annadā Ekādaśī",
    "Pārśva Ekādaśī",
    "Indirā Ekādaśī",
    "Pāpāṅkuśā Ekādaśī",
    "Śrī Ramā Ekādaśī",
    "Śrī Utthān Ekādaśī",
    "Utpannā Ekādaśī",
    "Mokṣadā Ekādaśī",
    "Saphalā Ekādaśī",
    "Kurma Dvādaśī",
    "Ṣaṭ-tilā Ekādaśī",
    "Bhaimī Ekādaśī",
    "Vijayā Ekādaśī",
    "Āmalakī Ekādaśī",
]


@dataclass
class DateBlock:
    page: int
    year: int
    month: int
    day: int
    masa: str
    text: str


@dataclass
class Fixture:
    case_id: str
    date: str
    name: str
    is_mahadvadasi: str
    mahadvadasi_type: str
    parana_start: str
    parana_end: str
    masa: str
    paksha: str
    source_file: str
    status: str

    def row(self) -> list[str]:
        return [
            self.case_id,
            self.date,
            LOCATION,
            LATITUDE,
            LONGITUDE,
            TIMEZONE,
            self.name,
            self.is_mahadvadasi,
            self.mahadvadasi_type,
            self.parana_start,
            self.parana_end,
            self.masa,
            self.paksha,
            self.source_file,
            self.status,
        ]


def main(argv: list[str]) -> int:
    input_path = Path(argv[0]) if argv else DEFAULT_INPUT
    output_path = Path(argv[1]) if len(argv) > 1 else DEFAULT_OUTPUT

    if not input_path.is_file():
        write_pending(output_path, input_path, "SCS Math PDF source is not available")
        print("External Oracle Pending: SCS Math PDF not found.")
        print(f"Expected PDF: {input_path}")
        return 0

    blocks = extract_date_blocks(input_path)
    fixtures = extract_fixtures(input_path, blocks)

    if not fixtures:
        write_pending(output_path, input_path, "No Ekadasi fixtures extracted from SCS Math PDF")
        print("External Oracle Pending: no Ekadasi fixtures were extracted.")
        return 0

    output_path.parent.mkdir(parents=True, exist_ok=True)
    with output_path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(HEADERS)
        for fixture in fixtures:
            writer.writerow(fixture.row())

    print(f"SCS Math fixtures extracted: {len(fixtures)}")
    print(f"CSV: {output_path}")
    return 0


def extract_date_blocks(input_path: Path) -> list[DateBlock]:
    month_re = re.compile(
        r"^(JANUARY|FEBRUARY|MARCH|APRIL|MAY|JUNE|JULY|AUGUST|SEPTEMBER|OCTOBER|NOVEMBER|DECEMBER)\s+(20\d{2})$"
    )
    date_re = re.compile(
        r"^(\d{1,2})\s+(JANUARY|FEBRUARY|MARCH|APRIL|MAY|JUNE|JULY|AUGUST|SEPTEMBER|OCTOBER|NOVEMBER|DECEMBER)\s+\([A-Z]{3}\)\.\s*(.*)$"
    )

    current_year: int | None = None
    current_masa = ""
    blocks: list[DateBlock] = []
    current: DateBlock | None = None

    with pdfplumber.open(input_path) as pdf:
        for page_no, page in enumerate(pdf.pages, 1):
            text = page.extract_text(x_tolerance=1, y_tolerance=3) or ""
            for raw_line in text.splitlines():
                line = " ".join(raw_line.strip().split())
                if not line or line.isdigit():
                    continue

                month_match = month_re.match(line)
                if month_match:
                    current_year = int(month_match.group(2))
                    continue

                if is_masa_heading(line):
                    current_masa = line

                date_match = date_re.match(line)
                if date_match:
                    if current:
                        blocks.append(current)
                    day = int(date_match.group(1))
                    month = MONTHS[date_match.group(2)]
                    year = current_year or 2026
                    current = DateBlock(
                        page=page_no,
                        year=year,
                        month=month,
                        day=day,
                        masa=current_masa,
                        text=date_match.group(3),
                    )
                    continue

                if current:
                    current.text = f"{current.text} {line}"

    if current:
        blocks.append(current)

    return blocks


def is_masa_heading(line: str) -> bool:
    if any(month in line for month in MONTHS):
        return False
    if len(line) > 40:
        return False
    letters = [ch for ch in line if ch.isalpha()]
    if not letters:
        return False
    uppercase = sum(1 for ch in letters if ch.upper() == ch)
    return uppercase / len(letters) > 0.75


def extract_fixtures(input_path: Path, blocks: list[DateBlock]) -> list[Fixture]:
    fixtures: list[Fixture] = []

    for index, block in enumerate(blocks):
        text = " ".join(block.text.split())
        if "Fast on" not in text:
            continue
        if not any(marker in text for marker in ["Ekādaśī", "Dvādaśī", "Mahādvādaśī"]):
            continue

        name = observance_name(text)
        if not name:
            continue

        parana_start, parana_end = parana_after(index, blocks)
        is_mahadvadasi = "true" if "Mahādvādaśī" in name else "false"
        mahadvadasi_type = (
            name.replace("Mahādvādaśī", "").strip() if is_mahadvadasi == "true" else "None"
        )

        fixtures.append(
            Fixture(
                case_id=f"SCS-{len(fixtures) + 1:04}",
                date=f"{block.year:04}-{block.month:02}-{block.day:02}",
                name=name,
                is_mahadvadasi=is_mahadvadasi,
                mahadvadasi_type=mahadvadasi_type or "None",
                parana_start=parana_start,
                parana_end=parana_end,
                masa=clean_masa(block.masa),
                paksha=paksha(text),
                source_file=f"{input_path}#page={block.page}",
                status="IMPORTED_REQUIRES_REVIEW",
            )
        )

    return fixtures


def observance_name(text: str) -> str:
    after = text.split("Fast on", 1)[1]
    raw = after.split(".", 1)[0].strip()
    raw = re.split(r"\s+and\s+in\s+honour|\s+and\s+", raw)[0].strip()

    for known in KNOWN_OBSERVANCES:
        if known in raw:
            return known

    return raw


def parana_after(index: int, blocks: list[DateBlock]) -> tuple[str, str]:
    for block in blocks[index : index + 4]:
        text = " ".join(block.text.split())
        match = re.search(
            r"Ekādaśī\s+pāraṇ\s+is\s+at\s+(\d{1,2}:\d{2})\s*[–-]\s*(\d{1,2}:\d{2})",
            text,
        )
        if match:
            return match.group(1), match.group(2)
    return "", ""


def clean_masa(value: str) -> str:
    return value.replace("\ue047", "Ṛ")


def paksha(text: str) -> str:
    if "Kṛṣṇa" in text:
        return "Krishna"
    if "Gaura" in text:
        return "Gaura"
    return "Unknown"


def write_pending(output_path: Path, input_path: Path, notes: str) -> None:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with output_path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(HEADERS)
        writer.writerow(
            [
                "SCS-0000",
                "",
                LOCATION,
                LATITUDE,
                LONGITUDE,
                TIMEZONE,
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                str(input_path),
                f"EXTERNAL_ORACLE_PENDING: {notes}",
            ]
        )


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
