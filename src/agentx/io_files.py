from __future__ import annotations

import re
import shutil
import datetime as dt
from pathlib import Path

def sorted_md ( directory: Path ) -> list[Path]:

    files = [path for path in directory.glob("*.md") if path.is_file()]
    files.sort(key=lambda path: [int(part) if part.isdigit() else part for part in re.split(r"(\d+)", path.name)])

    return files

def parse_control ( control: Path ) -> tuple[str, str]:

    if not control.exists():
        return "", ""

    action = note = ""

    for line in control.read_text(encoding="utf-8").splitlines():

        matched = re.match(r"\s*ACTION:\s*(.+)", line, re.I)

        if matched and not action:
            action = re.sub(r"[^a-z]", "", matched.group(1).lower())

        matched = re.match(r"\s*NOTE:\s*(.+)", line, re.I)

        if matched and not note:
            note = matched.group(1).strip()

    return action, note

def all_shipped ( directory: Path, agents: list[str] ) -> bool:

    for agent in agents:

        report = directory / f"{agent}.md"

        if not report.exists():
            return False

        lines = [line for line in report.read_text(encoding="utf-8").splitlines() if line.strip()]

        if not lines or not re.fullmatch(r"\s*ship\s+it\s*", lines[-1].lower()):
            return False

    return True

def snap_seq ( directory: Path ) -> str:

    directory.mkdir(parents=True, exist_ok=True)
    highest = 0

    for path in directory.glob("*.md"):

        matched = re.match(r"^(\d+)", path.name)

        if matched:
            highest = max(highest, int(matched.group(1)))

    return f"{highest + 1:03d}"

def next_stamp ( directory: Path ) -> str:

    directory.mkdir(parents=True, exist_ok=True)
    day = dt.date.today().isoformat()
    highest = 0

    for path in directory.glob(f"{day}-*.md"):

        matched = re.match(rf"^{day}-(\d+)", path.name)

        if matched:
            highest = max(highest, int(matched.group(1)))

    return f"{day}-{highest + 1:04d}"

def strip_prefix ( name: str ) -> str:

    return re.sub(r"^\d+-", "", name)

def snapshot_one ( report: Path, history_dir: Path ) -> None:

    if not report.exists():
        return

    history_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy(report, history_dir / f"{snap_seq(history_dir)}-{report.name}")

def clear_files ( *dirs: Path ) -> None:

    for directory in dirs:

        if not directory.exists():
            continue

        for path in directory.glob("*.md"):
            path.unlink()

def archive ( sources: list[Path], history_dir: Path ) -> None:

    history_dir.mkdir(parents=True, exist_ok=True)

    for source in sources:

        name = strip_prefix(source.name)
        source.rename(history_dir / f"{next_stamp(history_dir)}-{name}")
