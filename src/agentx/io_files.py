from __future__ import annotations
import os, re, shutil, datetime as dt
from pathlib import Path

def sorted_md ( directory: Path ) -> list[Path]:

    files = [path for path in directory.glob("*.md") if path.is_file()]
    files.sort(key=lambda path: [int(part) if part.isdigit() else part for part in re.split(r"(\d+)", path.name)])

    return files

def parse_control ( control: Path ) -> tuple[str, str]:

    if not control.exists():
        return "", ""

    action = note = ""

    for line in control.read_text(encoding="utf-8").splitlines():

        matched = re.match(r"\s*ACTION:\s*(.+)", line, re.I)

        if matched and not action:
            action = re.sub(r"[^a-z]", "", matched.group(1).lower())

        matched = re.match(r"\s*NOTE:\s*(.+)", line, re.I)

        if matched and not note:
            note = matched.group(1).strip()

    return action, note

def all_shipped ( directory: Path, agents: list[str], token: str ) -> bool:

    needle = re.compile(rf"\s*{re.escape(token)}\s*", re.I)

    for agent in agents:

        report = directory / f"{agent}.md"

        if not report.exists():
            return False

        lines = [line for line in report.read_text(encoding="utf-8").splitlines() if line.strip()]

        if not lines or not needle.fullmatch(lines[-1].lower()):
            return False

    return True

def snap_seq ( directory: Path ) -> str:

    directory.mkdir(parents=True, exist_ok=True)
    highest = 0

    for path in directory.iterdir():

        matched = re.match(r"^(\d+)", path.name)

        if matched:
            highest = max(highest, int(matched.group(1)))

    return f"{highest + 1:03d}"

def next_stamp ( directory: Path ) -> str:

    directory.mkdir(parents=True, exist_ok=True)
    day = dt.date.today().isoformat()
    highest = 0

    for path in directory.glob(f"{day}-*"):

        matched = re.match(rf"^{day}-(\d+)", path.name)

        if matched:
            highest = max(highest, int(matched.group(1)))

    return f"{day}-{highest + 1:04d}"

def snapshot_one ( report: Path, rounds_dir: Path ) -> None:

    if not report.exists():
        return

    rounds_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy(report, rounds_dir / f"{snap_seq(rounds_dir)}-{report.name}")

def dump_prompt ( prompts_dir: Path, label: str, prompt: str ) -> None:

    prompts_dir.mkdir(parents=True, exist_ok=True)
    target = prompts_dir / f"{snap_seq(prompts_dir)}-{label}.md"
    target.write_text(prompt, encoding="utf-8")

def clear_all ( *dirs: Path ) -> None:

    for directory in dirs:

        if not directory.exists():
            continue

        for path in directory.iterdir():

            if path.is_file():
                path.unlink()

            elif path.is_dir():
                shutil.rmtree(path)

def make_run_dir ( runs: Path ) -> Path:

    run = runs / next_stamp(runs)
    run.mkdir(parents=True, exist_ok=True)

    return run

def harvest ( source: Path, run_dir: Path, label: str ) -> None:

    if not source.exists():
        return

    entries = [path for path in source.iterdir()]

    if not entries:
        return

    target = run_dir / label
    target.mkdir(parents=True, exist_ok=True)

    for path in entries:

        if path.is_file():
            shutil.copy(path, target / path.name)

        else:
            shutil.copytree(path, target / path.name, dirs_exist_ok=True)

def harvest_file ( source: Path, run_dir: Path ) -> None:

    if source.exists():
        shutil.copy(source, run_dir / source.name)

def relative ( paths: list[Path], root: Path ) -> list[str]:

    rendered: list[str] = []

    for path in paths:

        try:
            rendered.append(str(path.relative_to(root)))

        except ValueError:
            rendered.append(str(path))

    return rendered

def task_snapshot ( tasks_dir: Path ) -> set[str]:

    if not tasks_dir.is_dir():
        return set()

    return { path.name for path in tasks_dir.glob("*.md") if path.is_file() }

def write_pid ( pid_file: Path ) -> None:

    pid_file.parent.mkdir(parents=True, exist_ok=True)
    pid_file.write_text(str(os.getpid()), encoding="utf-8")

def read_pid ( pid_file: Path ) -> int | None:

    if not pid_file.is_file():
        return None

    try:
        return int(pid_file.read_text(encoding="utf-8").strip())

    except ValueError:
        return None

def request_drain ( drain_file: Path ) -> None:

    drain_file.parent.mkdir(parents=True, exist_ok=True)
    drain_file.write_text("true\n", encoding="utf-8")

def drain_requested ( drain_file: Path ) -> bool:

    return drain_file.exists()

def remove ( *targets: Path ) -> None:

    for target in targets:

        if target.exists():
            target.unlink()
