from __future__ import annotations
import argparse
from pathlib import Path
from .app import do_clean, do_drain, do_init, do_start, do_stop

COMMANDS = {
    "init"  : do_init,
    "start" : do_start,
    "stop"  : do_stop,
    "drain" : do_drain,
    "clean" : do_clean,
}

HELP = {
    "init"  : "scaffold Agentx.toml, agents/, and .agentx/ in the current directory",
    "start" : "resolve the project root and run a full orchestration cycle",
    "stop"  : "kill the running cycle and its agents immediately",
    "drain" : "stop the running cycle cleanly after the current turn",
    "clean" : "delete the .agentx cache entirely",
}

def main () -> None:

    parser = argparse.ArgumentParser(prog="agentx")
    commands = parser.add_subparsers(dest="command", required=True)

    for name, help_text in HELP.items():
        commands.add_parser(name, help=help_text)

    args = parser.parse_args()

    COMMANDS[args.command](Path.cwd())
