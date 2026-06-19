from __future__ import annotations
import tomllib
from pathlib import Path
from .config import CONFIG_SECTION, DEFAULTS, STEPS, Spec

def _as_int ( value, fallback: int ) -> int:

    try:
        return int(value)

    except (TypeError, ValueError):
        return fallback

def _as_str ( value, fallback: str ) -> str:

    return value if isinstance(value, str) else fallback

def _as_list ( value, fallback: list[str] ) -> list[str]:

    if isinstance(value, list) and value:
        return [str(item) for item in value]

    return list(fallback)

def _read ( config_file: Path ) -> dict:

    if not config_file.is_file():
        return {}

    try:
        data = tomllib.loads(config_file.read_text(encoding="utf-8"))

    except tomllib.TOMLDecodeError as error:
        raise SystemExit(f"invalid {config_file.name}: {error}") from error

    section = data.get(CONFIG_SECTION, data)

    return section if isinstance(section, dict) else {}

def load_spec ( config_file: Path ) -> Spec:

    table = _read(config_file)

    steps = [step for step in _as_list(table.get("steps"), DEFAULTS["steps"]) if step in STEPS]

    if not steps:
        steps = list(DEFAULTS["steps"])

    def models ( key: str ) -> list[str]:

        return _as_list(table.get(key), DEFAULTS[key])

    return Spec(
        max_rounds=_as_int(table.get("max_rounds"), DEFAULTS["max_rounds"]),
        max_fixes=_as_int(table.get("max_fixes"), DEFAULTS["max_fixes"]),
        gate_cmd=_as_str(table.get("gate_cmd"), DEFAULTS["gate_cmd"]),
        manager_model=_as_str(table.get("manager_model"), DEFAULTS["manager_model"]),
        steps=steps,
        arch_models=models("arch_models"),
        work_models=models("work_models"),
        test_models=models("test_models"),
    )
