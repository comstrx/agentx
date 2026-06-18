from __future__ import annotations

from .config import Config
from .io_files import sorted_md
from .runner import Orchestrator

def _ensure_tree ( config: Config ) -> None:

    paths = config.paths

    targets = (
        paths.contracts, paths.requires, paths.tasks, paths.decisions,
        paths.reports_requires, paths.reports_tasks,
        paths.history_requires, paths.history_tasks,
        paths.history_reports / "requires", paths.history_reports / "tasks",
    )

    for path in targets:
        path.mkdir(parents=True, exist_ok=True)

def _resolve_phases ( config: Config ) -> list[str]:

    if config.phases:
        return config.phases

    paths = config.paths
    has_requires = bool(sorted_md(paths.requires))
    has_tasks = bool(sorted_md(paths.tasks))

    if has_tasks and not has_requires:
        return ["exec"]

    if has_requires:
        return ["arch", "exec"]

    if has_tasks:
        return ["exec"]

    return []

def run_cycle ( config: Config, cwd: str ) -> None:

    _ensure_tree(config)

    paths = config.paths

    if not paths.overview.exists():
        raise SystemExit(f"missing {paths.overview}")

    if not any(paths.contracts.glob("*.md")):
        raise SystemExit(f"missing contracts in {paths.contracts}")

    phases = _resolve_phases(config)

    if not phases:
        raise SystemExit("nothing to do: no requirements and no tasks")

    orchestrator = Orchestrator(config, cwd)
    orchestrator.brief_manager()

    if "arch" in phases:
        print("[agentx] phase: architecture")
        orchestrator.run_phase("arch")

    if "exec" in phases:

        if not sorted_md(paths.tasks):
            raise SystemExit("no tasks to execute")

        print("[agentx] phase: execution")
        orchestrator.run_phase("exec")

    print("[agentx] writing decision record")
    orchestrator.write_decision()

    orchestrator.archive_run()
    print("[agentx] cycle finished")
