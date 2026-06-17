from __future__ import annotations

import datetime as dt
import re
import shutil
from pathlib import Path

from .config import PATHS

CONTROL = PATHS.root / "control.md"


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def sorted_md(dir_: Path) -> list[str]:
    files = [p for p in dir_.glob("*.md") if p.is_file()]
    files.sort(key=lambda p: [int(x) if x.isdigit() else x
                              for x in re.split(r"(\d+)", p.name)])
    return [str(p) for p in files]


def parse_control() -> tuple[str, str]:
    if not CONTROL.exists():
        return "", ""
    action = pick = ""
    for line in CONTROL.read_text(encoding="utf-8").splitlines():
        m = re.match(r"\s*ACTION:\s*(.+)", line, re.I)
        if m and not action:
            action = re.sub(r"[^a-z]", "", m.group(1).lower())
        m = re.match(r"\s*AGENT:\s*(.+)", line, re.I)
        if m and not pick:
            pick = m.group(1).strip()
    return action, pick


def read_ship(report: Path) -> bool:
    if not report.exists():
        return False
    lines = [ln for ln in report.read_text(encoding="utf-8").splitlines() if ln.strip()]
    return bool(lines) and re.fullmatch(r"\s*ship\s+it\s*", lines[-1].lower())


def snap_seq(dir_: Path) -> str:
    dir_.mkdir(parents=True, exist_ok=True)
    mx = 0
    for p in dir_.glob("*.md"):
        m = re.match(r"^(\d+)", p.name)
        if m:
            mx = max(mx, int(m.group(1)))
    return f"{mx + 1:03d}"


def next_stamp(dir_: Path) -> str:
    dir_.mkdir(parents=True, exist_ok=True)
    day = dt.date.today().isoformat()
    mx = 0
    for p in dir_.glob(f"{day}-*.md"):
        m = re.match(rf"^{day}-(\d+)", p.name)
        if m:
            mx = max(mx, int(m.group(1)))
    return f"{day}-{mx + 1:04d}"


def strip_prefix(name: str) -> str:
    return re.sub(r"^\d+-", "", name)


def snapshot_report(live: Path, hist_dir: Path, agent: str) -> None:
    if live.exists():
        shutil.copy(live, hist_dir / f"{snap_seq(hist_dir)}-{agent}.md")


def clear_files(*dirs: Path) -> None:
    for d in dirs:
        if d.exists():
            for p in d.glob("*.md"):
                p.unlink()
