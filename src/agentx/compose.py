from __future__ import annotations
from pathlib import Path
from .config import BUCKETS, CONVERGENCE, Config
from .io_files import relative
from . import prompts as P

_BUCKET_TITLE = {
    "overview"  :  "Overview / workflow",
    "contracts" : "Contracts (LAW - they override everything)",
    "history"   :   "History / past decisions",
    "tasks"     :     "Existing tasks",
    "requires"  :  "Requirements to build",
}

def _r ( path: Path, root: Path ) -> str:

    return relative([path], root)[0]

def _context_block ( cfg: Config ) -> str:

    lines = ["Project context (read once, internalise, comply):"]

    for name in BUCKETS:

        files = relative(cfg.context.bucket(name), cfg.root)
        lines.append("")
        lines.append(f"{_BUCKET_TITLE[name]}:")

        if files:

            for rendered in files:
                lines.append(f"  {rendered}")

        else:
            lines.append("  (none)")

    return "\n".join(lines)

def _values ( cfg: Config, step: str, agent: str ) -> dict:

    paths = cfg.paths
    root = cfg.root

    return {
        "token": CONVERGENCE,
        "cache": _r(paths.cache, root),
        "tasks": _r(paths.tasks, root),
        "requires": _r(paths.requires, root),
        "reports": _r(paths.reports_of(step), root),
        "rounds": _r(paths.rounds_of(step), root),
        "report": _r(paths.reports_of(step) / f"{agent}.md", root),
        "review": _r(paths.review, root),
        "control": _r(paths.control, root),
        "gate_log": _r(paths.gate_log, root),
        "tests": _r(paths.tests, root),
        "probes": _r(paths.probes, root),
        "agent": agent,
    }

def _assemble ( parts: list[str], values: dict ) -> str:

    return "\n\n".join(part.format(**values) for part in parts)

def _briefing ( cfg: Config ) -> str:

    return P.BRIEFING.replace("{context}", _context_block(cfg))

def architect ( cfg: Config, agent: str, init: bool, critique: bool, has_review: bool, frozen: list[str] ) -> str:

    values = _values(cfg, "arch", agent)

    if init:
        parts = [P.ARCH_ROLE, P.TEAM, _briefing(cfg)]

        if frozen:
            values = {**values, "frozen": "\n".join(f"  {name}" for name in frozen)}
            parts.append(P.ARCH_FROZEN)

        parts += [P.ARCH_MISSION, P.ARCH_FLAG, P.LAW]

    else:
        parts = [f"{agent}, your next architecture turn.", P.ROUND]

    if critique:
        parts.append(P.ARCH_CRITIQUE)

    if has_review:
        parts.append(P.REVIEW_HANDOFF)

    parts.append(P.ARCH_REPORT)

    return _assemble(parts, values)

def executor ( cfg: Config, agent: str, init: bool, gate_failed: bool, has_review: bool ) -> str:

    values = _values(cfg, "work", agent)

    if init:
        parts = [P.EXEC_ROLE, P.TEAM, _briefing(cfg), P.EXEC_IMPLEMENT, P.LAW]

    else:
        parts = [f"{agent}, your next execution turn.", P.ROUND]

    if gate_failed:
        parts.append(P.EXEC_GATE_FAIL)

    if has_review:
        parts.append(P.REVIEW_HANDOFF)

    parts.append(P.EXEC_REPORT)

    return _assemble(parts, values)

def verifier ( cfg: Config, agent: str, init: bool, has_review: bool ) -> str:

    values = _values(cfg, "test", agent)

    if init:
        parts = [P.VERIFY_ROLE, P.TEAM, _briefing(cfg), P.VERIFY_WORKSPACE, P.VERIFY_STRATEGY, P.LAW]

    else:
        parts = [f"{agent}, your next verification turn.", P.ROUND]

    if has_review:
        parts.append(P.REVIEW_HANDOFF)

    parts.append(P.VERIFY_REPORT)

    return _assemble(parts, values)

def manager_brief ( cfg: Config ) -> str:

    parts = [P.MANAGER_ROLE, P.MANAGER_INIT.replace("{context}", _context_block(cfg))]

    return _assemble(parts, {})

def manager_review ( cfg: Config, step: str, round_no: int ) -> str:

    body = {"arch": P.MANAGER_REVIEW_ARCH, "work": P.MANAGER_REVIEW_WORK, "test": P.MANAGER_REVIEW_TEST}[step]
    values = _values(cfg, step, "manager")
    counter = f"Review round {round_no} of at most {cfg.spec.max_rounds}."

    parts = [P.MANAGER_ROLE, counter, P.MANAGER_INTEGRATION, body, P.MANAGER_FLAG, P.MANAGER_VERDICT]

    return _assemble(parts, values)

def manager_decision ( cfg: Config, decision: Path ) -> str:

    values = {
        "rounds": _r(cfg.paths.rounds, cfg.root),
        "decision": _r(decision, cfg.root),
    }

    parts = [P.MANAGER_ROLE, P.MANAGER_DECISION]

    return _assemble(parts, values)
