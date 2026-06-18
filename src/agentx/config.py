from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path

@dataclass(frozen=True)
class Paths:

    root: Path
    overview: Path
    contracts: Path
    requires: Path
    tasks: Path
    decisions: Path
    review: Path
    control: Path
    gate_log: Path
    reports_requires: Path
    reports_tasks: Path
    history_requires: Path
    history_tasks: Path
    history_reports: Path

@dataclass(frozen=True)
class Config:

    manager: str
    architects: list[str]
    executors: list[str]
    phases: list[str]
    gate_cmd: str
    max_rounds: int
    paths: Paths

def _paths ( root: Path ) -> Paths:

    return Paths(
        root=root,
        overview=root / "overview.md",
        contracts=root / "contracts",
        requires=root / "requires",
        tasks=root / "tasks",
        decisions=root / "decisions",
        review=root / "review.md",
        control=root / "control.md",
        gate_log=root / "gate.log",
        reports_requires=root / "reports" / "requires",
        reports_tasks=root / "reports" / "tasks",
        history_requires=root / "history" / "requires",
        history_tasks=root / "history" / "tasks",
        history_reports=root / "history" / "reports",
    )

def _roster ( spec: str ) -> list[str]:

    roster: list[str] = []
    seen: dict[str, int] = {}

    for part in ( token.strip() for token in spec.split(",") if token.strip() ):

        seen[part] = seen.get(part, 0) + 1
        roster.append(f"{part}_{seen[part]}")

    return roster

def _pick ( flag: str | None, env_key: str, fallback: str ) -> str:

    if flag is not None:
        return flag

    return os.environ.get(env_key, fallback)

def build_config ( archs: str | None, execs: str | None, phase: str | None, root: str | None ) -> Config:

    base = Path(_pick(root, "AGENTS_ROOT", "agents"))

    manager = os.environ.get("MANAGER", "claude")
    architects = _roster(_pick(archs, "ARCHITECTS", "claude,codex,claude"))
    executors = _roster(_pick(execs, "EXECUTORS", "claude,codex,claude,codex,claude"))
    phases = [p.strip() for p in _pick(phase, "PHASES", "").split(",") if p.strip()]

    gate_cmd = os.environ.get("GATE_CMD", "")
    max_rounds = int(os.environ.get("MAX_ROUNDS", "5"))

    return Config(
        manager=manager,
        architects=architects,
        executors=executors,
        phases=phases,
        gate_cmd=gate_cmd,
        max_rounds=max_rounds,
        paths=_paths(base),
    )
