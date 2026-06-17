from __future__ import annotations

from typing import Annotated, TypedDict


def merge(a: dict, b: dict) -> dict:
    out = dict(a)
    out.update(b)
    return out


class State(TypedDict, total=False):
    phase: str

    requirements: list[str]
    tasks: list[str]
    task_index: int
    current_task: str

    last_worker: str
    gate: str                      # none | pass | fail

    sessions: Annotated[dict[str, str], merge]   # agent_name -> session_id
    primed: Annotated[dict[str, bool], merge]    # agent_name -> already initialized

    action: str                    # assign | complete | stop
    pick: str                      # chosen worker for this step
    step: int
