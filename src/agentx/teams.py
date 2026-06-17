from __future__ import annotations

from pathlib import Path

from langgraph.graph import END, START, StateGraph

from .config import CWD, MANAGER, MAX_STEPS, PATHS
from .gate import GATE_LOG, run_gate
from .io_files import (
    CONTROL,
    parse_control,
    snapshot_report,
    sorted_md,
    write,
)
from .prompts import arch_prompt, exec_prompt, manager_prompt
from .state import State
from .workers import run_worker


def _ask_manager(state: State, mode: str, workers: list[str]) -> tuple[str, str, str]:
    sid = state.get("sessions", {}).get(MANAGER)
    prompt = manager_prompt(
        mode,
        state.get("current_task", ""),
        workers,
        state.get("last_worker", "none"),
        state.get("gate", "none"),
        GATE_LOG,
    )
    _, new_sid = run_worker(MANAGER, prompt, sid, CWD)
    action, pick = parse_control()
    return action, pick, new_sid


# ------------------------------------------------------------------ architecture

def build_arch_team(architects: list[str]):
    def supervisor(state: State) -> dict:
        action, pick, sid = _ask_manager(state, "arch", architects)
        step = state.get("step", 0) + 1
        if MAX_STEPS and step >= MAX_STEPS:
            action = "complete"
        return {
            "action": action,
            "pick": pick,
            "step": step,
            "sessions": {MANAGER: sid},
        }

    def worker(state: State) -> dict:
        pick = state["pick"]
        if pick not in architects:
            return {}
        primed = state.get("primed", {})
        init = pick not in primed
        sid = state.get("sessions", {}).get(pick)

        _, new_sid = run_worker(pick, arch_prompt(pick, init), sid, CWD)

        snapshot_report(
            PATHS.reports_requires / f"{pick}.md",
            PATHS.history_reports / "requires",
            pick,
        )
        return {
            "last_worker": pick,
            "primed": {pick: True},
            "sessions": {pick: new_sid},
        }

    def route(state: State) -> str:
        return "worker" if state.get("action") == "assign" else "done"

    g = StateGraph(State)
    g.add_node("supervisor", supervisor)
    g.add_node("worker", worker)
    g.add_edge(START, "supervisor")
    g.add_conditional_edges("supervisor", route, {"worker": "worker", "done": END})
    g.add_edge("worker", "supervisor")
    return g.compile()


# ------------------------------------------------------------------ execution

def build_exec_team(executors: list[str]):
    def next_task(state: State) -> dict:
        tasks = sorted_md(PATHS.tasks)
        idx = state.get("task_index", 0)
        if idx >= len(tasks):
            return {"current_task": "", "task_index": idx}
        # fresh per-task: clear live task reports, reset step/last/gate
        for p in PATHS.reports_tasks.glob("*.md"):
            p.unlink()
        return {
            "current_task": tasks[idx],
            "task_index": idx + 1,
            "step": 0,
            "last_worker": "",
            "gate": "none",
        }

    def supervisor(state: State) -> dict:
        action, pick, sid = _ask_manager(state, "exec", executors)
        step = state.get("step", 0) + 1
        if MAX_STEPS and step >= MAX_STEPS:
            action = "complete"
        return {
            "action": action,
            "pick": pick,
            "step": step,
            "sessions": {MANAGER: sid},
        }

    def worker(state: State) -> dict:
        pick = state["pick"]
        if pick not in executors:
            return {}
        primed = state.get("primed", {})
        init = pick not in primed
        sid = state.get("sessions", {}).get(pick)

        _, new_sid = run_worker(
            pick,
            exec_prompt(pick, state["current_task"], init, state.get("gate", "none"), GATE_LOG),
            sid,
            CWD,
        )

        snapshot_report(
            PATHS.reports_tasks / f"{pick}.md",
            PATHS.history_reports / "tasks",
            pick,
        )
        verdict, _ = run_gate()
        return {
            "last_worker": pick,
            "primed": {pick: True},
            "sessions": {pick: new_sid},
            "gate": verdict,
        }

    def has_task(state: State) -> str:
        return "supervisor" if state.get("current_task") else "done"

    def route(state: State) -> str:
        a = state.get("action")
        if a == "assign":
            return "worker"
        if a == "complete":
            return "next"
        return "done"

    g = StateGraph(State)
    g.add_node("next", next_task)
    g.add_node("supervisor", supervisor)
    g.add_node("worker", worker)
    g.add_edge(START, "next")
    g.add_conditional_edges("next", has_task, {"supervisor": "supervisor", "done": END})
    g.add_conditional_edges("supervisor", route, {"worker": "worker", "next": "next", "done": END})
    g.add_edge("worker", "supervisor")
    return g.compile()
