from __future__ import annotations

import subprocess

from .config import CWD, GATE_CMD, PATHS

GATE_LOG = str(PATHS.root / "gate.log")


def run_gate() -> tuple[str, str]:
    # Deterministic verification. The model never self-reports pass/fail.
    if not GATE_CMD:
        return "pass", "no GATE_CMD set; gate skipped"

    proc = subprocess.run(GATE_CMD, shell=True, cwd=CWD, capture_output=True, text=True)
    log = (proc.stdout or "") + (proc.stderr or "")

    with open(GATE_LOG, "w", encoding="utf-8") as fh:
        fh.write(log)

    return ("pass" if proc.returncode == 0 else "fail"), log
