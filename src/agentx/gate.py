from __future__ import annotations

import subprocess
from pathlib import Path

def run_gate ( gate_cmd: str, cwd: str, log_path: Path ) -> tuple[bool, str]:

    if not gate_cmd:
        return True, "no gate command set; gate skipped"

    result = subprocess.run(gate_cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    log = (result.stdout or "") + (result.stderr or "")

    log_path.parent.mkdir(parents=True, exist_ok=True)
    log_path.write_text(log, encoding="utf-8")

    return result.returncode == 0, log
