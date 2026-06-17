from __future__ import annotations

import os
from pathlib import Path

from .config import CHECKPOINT_DB, PATHS
from .graph import build_app


def _ensure_tree() -> None:
    for p in (
        PATHS.contracts, PATHS.requires, PATHS.tasks, PATHS.decisions,
        PATHS.reports_requires, PATHS.reports_tasks,
        PATHS.history_requires, PATHS.history_tasks,
        PATHS.history_reports / "requires", PATHS.history_reports / "tasks",
    ):
        p.mkdir(parents=True, exist_ok=True)


def main() -> None:
    _ensure_tree()

    if not PATHS.overview.exists():
        raise SystemExit(f"missing {PATHS.overview}")
    if not any(PATHS.contracts.glob("*.md")):
        raise SystemExit(f"missing contracts in {PATHS.contracts}")

    Path(CHECKPOINT_DB).parent.mkdir(parents=True, exist_ok=True)

    from langgraph.checkpoint.sqlite import SqliteSaver

    with SqliteSaver.from_conn_string(CHECKPOINT_DB) as cp:
        app = build_app(checkpointer=cp)
        config = {"configurable": {"thread_id": os.environ.get("CYCLE_ID", "cycle")}}
        app.invoke({"phase": "init"}, config=config)


if __name__ == "__main__":
    main()
