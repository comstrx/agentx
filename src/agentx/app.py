from __future__ import annotations
import os, shutil, signal
from pathlib import Path
from .config import CACHE_DIR, DEFAULT_TOML, STEPS, Config, layout
from .discovery import discover, resolve_root
from .io_files import read_pid, remove, request_drain, write_pid
from .runner import DrainSignal, Orchestrator
from .spec import load_spec

def assemble ( root: Path ) -> Config:

    paths = layout(root)
    spec = load_spec(paths.config_file)
    context = discover(paths)

    return Config(root=root, spec=spec, paths=paths, context=context)

def scaffold ( root: Path ) -> None:

    paths = layout(root)

    for directory in ( paths.contracts, paths.history, paths.tasks, paths.requires ):
        directory.mkdir(parents=True, exist_ok=True)

    if not paths.overview.exists():
        paths.overview.write_text("", encoding="utf-8")

    cache_dirs = (
        paths.reports_of("arch"), paths.reports_of("work"), paths.reports_of("test"),
        paths.rounds_of("arch"), paths.rounds_of("work"), paths.rounds_of("test"),
        paths.tests, paths.probes, paths.prompts, paths.runs,
    )

    for directory in cache_dirs:
        directory.mkdir(parents=True, exist_ok=True)

    if not paths.gitignore.exists():
        paths.gitignore.write_text("*\n", encoding="utf-8")

    if not paths.config_file.exists():
        paths.config_file.write_text(DEFAULT_TOML, encoding="utf-8")

def _resolve_steps ( config: Config ) -> list[str]:

    steps = config.spec.steps
    has_requires = bool(config.context.requires)
    has_tasks = bool(config.context.tasks)

    if not has_requires and not has_tasks:
        return []

    run: list[str] = []

    if has_requires and "arch" in steps:
        run.append("arch")

    if "work" in steps:
        run.append("work")

    if "test" in steps:
        run.append("test")

    return [step for step in STEPS if step in run]

def run_cycle ( config: Config, cwd: str ) -> None:

    orchestrator = Orchestrator(config, cwd)

    steps = _resolve_steps(config)

    if not steps:
        raise SystemExit("nothing to do: add a requirement under agents/requires/ or a task under agents/tasks/")

    orchestrator.brief_manager()

    try:

        for step in steps:

            print(f"[agentx] step: {step}")
            orchestrator.run_phase(step)

        print("[agentx] writing decision record")
        orchestrator.write_decision()

        orchestrator.archive_run()

    except DrainSignal:
        print("[agentx] drained - stopped after the current turn, state left intact for inspection or resume")
        return

    if orchestrator.blocked:
        joined = ", ".join(orchestrator.blocked)
        print(f"[agentx] CYCLE FINISHED WITH OPEN ISSUES in: {joined} - review the decision record before shipping")
        return

    print("[agentx] cycle finished clean")

def do_init ( cwd: Path ) -> None:

    scaffold(cwd)
    print(f"[agentx] initialised at {cwd}")

def do_start ( cwd: Path ) -> None:

    root = resolve_root(cwd)
    scaffold(root)

    config = assemble(root)
    paths = config.paths

    os.setpgrp()
    write_pid(paths.pid)

    try:
        run_cycle(config, str(root))

    finally:
        remove(paths.pid, paths.drain)

def do_stop ( cwd: Path ) -> None:

    root = resolve_root(cwd)
    paths = layout(root)
    pid = read_pid(paths.pid)

    if pid is None:
        print("[agentx] no running cycle found")
        return

    try:
        os.killpg(pid, signal.SIGTERM)
        print(f"[agentx] stopped run {pid} immediately")

    except ProcessLookupError:
        print("[agentx] no live process for the recorded run")

    remove(paths.pid)

def do_drain ( cwd: Path ) -> None:

    root = resolve_root(cwd)
    paths = layout(root)

    request_drain(paths.drain)
    print("[agentx] drain requested - the run will stop cleanly after the current turn")

def do_clean ( cwd: Path ) -> None:

    root = resolve_root(cwd)
    cache = root / CACHE_DIR

    shutil.rmtree(cache, ignore_errors=True)
    print(f"[agentx] removed {cache}")
