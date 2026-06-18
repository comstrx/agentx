from __future__ import annotations

import argparse
from pathlib import Path

from .app import run_cycle
from .config import build_config
from .io_files import clear_files

def main () -> None:

    parser = argparse.ArgumentParser(prog="agentx")
    commands = parser.add_subparsers(dest="command", required=True)

    start = commands.add_parser("start", help="run a full orchestration cycle")
    start.add_argument("--archs")
    start.add_argument("--exec", dest="execs")
    start.add_argument("--phase")
    start.add_argument("--root")

    clean = commands.add_parser("clean", help="clear old tasks and reports")
    clean.add_argument("--root")

    args = parser.parse_args()

    if args.command == "start":

        config = build_config(args.archs, args.execs, args.phase, args.root)
        run_cycle(config, str(Path.cwd()))

        return

    config = build_config(None, None, None, args.root)
    paths = config.paths

    clear_files(paths.tasks, paths.reports_requires, paths.reports_tasks)
    print("[agentx] cleaned tasks and reports")
