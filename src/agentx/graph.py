from __future__ import annotations

from pathlib import Path

from langgraph.graph import END, START, StateGraph

from .config import CWD, MANAGER, expand_roster, ARCHITECTS, EXECUTORS, PATHS
from .io_files import (
    clear_files,
    next_stamp,
    sorted_md,
    strip_prefix,
)
from .prompts import decision_prompt
from .state import State
from .teams import build_arch_team, build_exec_team
from .workers import run_worker


def _init(state: State) -> dict:
    return {
        "requirements": sorted_md(PATHS.requires),
        "tasks": sorted_md(PATHS.tasks),
        "task_index": 0,
        "sessions": state.get("sessions", {}),
        "primed": state.get("primed", {}),
        "gate": "none",
    }


def _route_after_init(state: State) -> str:
    # resume: if tasks already exist, skip architecture
    return "execution" if state.get("tasks") else "architecture"


def _decisions(state: State) -> dict:
    sid = state.get("sessions", {}).get(MANAGER)
    for require in sorted_md(PATHS.requires):
        name = strip_prefix(Path(require).stem)
        decision = PATHS.decisions / f"{next_stamp(PATHS.decisions)}-{name}.md"
        _, sid = run_worker(MANAGER, decision_prompt(require, str(decision)), sid, CWD)
    return {"sessions": {MANAGER: sid}}


def _archive(state: State) -> dict:
    for require in sorted_md(PATHS.requires):
        name = strip_prefix(Path(require).name)
        Path(require).rename(PATHS.history_requires / f"{next_stamp(PATHS.history_requires)}-{name}")
    for task in sorted_md(PATHS.tasks):
        name = strip_prefix(Path(task).name)
        Path(task).rename(PATHS.history_tasks / f"{next_stamp(PATHS.history_tasks)}-{name}")

    for sub in ("requires", "tasks"):
        d = PATHS.history_reports / sub
        if d.exists():
            for p in d.glob("*.md"):
                p.unlink()
    clear_files(PATHS.reports_requires, PATHS.reports_tasks)
    return {}


def build_app(checkpointer=None):
    architects = expand_roster(ARCHITECTS)
    executors = expand_roster(EXECUTORS)

    g = StateGraph(State)
    g.add_node("init", _init)
    g.add_node("architecture", build_arch_team(architects))
    g.add_node("execution", build_exec_team(executors))
    g.add_node("decisions", _decisions)
    g.add_node("archive", _archive)

    g.add_edge(START, "init")
    g.add_conditional_edges(
        "init", _route_after_init,
        {"architecture": "architecture", "execution": "execution"},
    )
    g.add_edge("architecture", "execution")
    g.add_edge("execution", "decisions")
    g.add_edge("decisions", "archive")
    g.add_edge("archive", END)

    return g.compile(checkpointer=checkpointer)
