from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(os.environ.get("AGENTS_ROOT", "agents"))


@dataclass(frozen=True)
class Paths:
    root: Path = ROOT
    overview: Path = ROOT / "overview.md"
    contracts: Path = ROOT / "contracts"
    requires: Path = ROOT / "requires"
    tasks: Path = ROOT / "tasks"
    decisions: Path = ROOT / "decisions"
    reports_requires: Path = ROOT / "reports" / "requires"
    reports_tasks: Path = ROOT / "reports" / "tasks"
    history_requires: Path = ROOT / "history" / "requires"
    history_tasks: Path = ROOT / "history" / "tasks"
    history_reports: Path = ROOT / "history" / "reports"


PATHS = Paths()

MANAGER = os.environ.get("MANAGER", "claude")
ARCHITECTS = os.environ.get("ARCHITECTS", "claude:2,codex:1")
EXECUTORS = os.environ.get("EXECUTORS", "claude:2,codex:1")

GATE_CMD = os.environ.get("GATE_CMD", "")
MAX_STEPS = int(os.environ.get("MAX_STEPS", "0"))
CHECKPOINT_DB = os.environ.get("CHECKPOINT_DB", ".orchestrator/checkpoint.sqlite")

CWD = str(Path.cwd())


def expand_roster(spec: str) -> list[str]:
    # "claude:2,codex:1" -> ["claude_1", "claude_2", "codex_1"]
    out: list[str] = []
    for part in (p.strip() for p in spec.split(",") if p.strip()):
        base, _, n = part.partition(":")
        count = int(n) if n else 1
        out.extend(f"{base}_{i}" for i in range(1, count + 1))
    return out
