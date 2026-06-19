from __future__ import annotations
from dataclasses import dataclass, field
from pathlib import Path

CONFIG_FILE = "Agentx.toml"

DOCS_DIR = "agents"

CACHE_DIR = ".agentx"

CONFIG_SECTION = "project"

ROOT_MARKERS = ( CONFIG_FILE, ".git" )

ROOT_FALLBACK_FILES = ( "agents.md", "claude.md", "codex.md" )

STEPS = ( "arch", "work", "test" )

ROLES = { "arch": "ARCHITECT", "work": "EXECUTOR", "test": "VERIFIER" }

STEP_MODELS_KEY = { "arch": "arch_models", "work": "work_models", "test": "test_models" }

BUCKETS = ( "overview", "contracts", "history", "tasks", "requires" )

CONVERGENCE = "ship it"

DECISION_BUCKET = "history"

DISCOVERY_STEMS = {
    "overview":  ( "overview", "workflow", "index", "agent", "agents" ),
    "contracts": ( "contract", "contracts", "style", "styles", "instruction", "instructions" ),
    "history":   ( "history", "decision", "decisions" ),
    "tasks":     ( "task", "tasks" ),
    "requires":  ( "require", "requires", "requirement", "requirements" ),
}

DEFAULTS = {
    "max_rounds": 5,
    "max_fixes": 5,
    "gate_cmd": "",
    "manager_model": "claude",
    "steps": list(STEPS),
    "arch_models": ["claude"],
    "work_models": ["claude"],
    "test_models": ["claude"],
}

DEFAULT_TOML = """[project]
max_rounds    = 5
max_fixes     = 5
gate_cmd      = ""
manager_model = "claude"
steps         = [ "arch", "work", "test" ]
arch_models   = [ "claude" ]
work_models   = [ "claude" ]
test_models   = [ "claude" ]
"""


@dataclass(frozen=True)
class Paths:

    root: Path
    docs: Path
    cache: Path
    overview: Path
    contracts: Path
    history: Path
    tasks: Path
    requires: Path
    reports: Path
    rounds: Path
    tests: Path
    probes: Path
    prompts: Path
    runs: Path
    review: Path
    control: Path
    gate_log: Path
    pid: Path
    drain: Path
    gitignore: Path
    config_file: Path

    def reports_of ( self, step: str ) -> Path:

        return self.reports / step

    def rounds_of ( self, step: str ) -> Path:

        return self.rounds / step

@dataclass(frozen=True)
class Spec:

    max_rounds: int
    max_fixes: int
    gate_cmd: str
    manager_model: str
    steps: list[str]
    arch_models: list[str]
    work_models: list[str]
    test_models: list[str]

    def models ( self, step: str ) -> list[str]:

        return getattr(self, STEP_MODELS_KEY[step])

    def roster ( self, step: str ) -> list[str]:

        return expand_roster(self.models(step))

@dataclass
class Context:

    overview: list[Path] = field(default_factory=list)
    contracts: list[Path] = field(default_factory=list)
    history: list[Path] = field(default_factory=list)
    tasks: list[Path] = field(default_factory=list)
    requires: list[Path] = field(default_factory=list)

    def bucket ( self, name: str ) -> list[Path]:

        return getattr(self, name)

    def add ( self, name: str, path: Path ) -> None:

        target = getattr(self, name)

        if path not in target:
            target.append(path)

    def is_empty ( self ) -> bool:

        return not any(getattr(self, name) for name in BUCKETS)

@dataclass(frozen=True)
class Config:

    root: Path
    spec: Spec
    paths: Paths
    context: Context

    def manager ( self ) -> str:

        return self.spec.manager_model

def layout ( root: Path ) -> Paths:

    docs = root / DOCS_DIR
    cache = root / CACHE_DIR

    return Paths(
        root=root,
        docs=docs,
        cache=cache,
        overview=docs / "overview.md",
        contracts=docs / "contracts",
        history=docs / "history",
        tasks=docs / "tasks",
        requires=docs / "requires",
        reports=cache / "reports",
        rounds=cache / "rounds",
        tests=cache / "tests",
        probes=cache / "probes",
        prompts=cache / "prompts",
        runs=cache / "runs",
        review=cache / "review.md",
        control=cache / "control.md",
        gate_log=cache / "gate.log",
        pid=cache / "agentx.pid",
        drain=cache / "drain",
        gitignore=cache / ".gitignore",
        config_file=root / CONFIG_FILE,
    )

def expand_roster ( models: list[str] ) -> list[str]:

    roster: list[str] = []
    seen: dict[str, int] = {}

    for raw in ( str(m).strip() for m in models if str(m).strip() ):

        seen[raw] = seen.get(raw, 0) + 1
        roster.append(f"{raw}_{seen[raw]}")

    return roster
