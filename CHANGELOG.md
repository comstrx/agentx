# Changelog

## v0.2.0 — 2026-06-23

Ground-up rebuild: a resumable state machine, a per-task council of agents, an explicit priming phase, and a self-training knowledge base shared across projects of the same kind.

### Orchestration
- Fixed pipeline **requires → tasks → tests → finalize** replaces the configurable `steps`. Drop requirements under `agents/requires/`; architects turn them into ordered task contracts under `.agentx/tasks/`, executors build them one at a time, verifiers attack the result.
- **Per-task council:** every executor model works the same task in turn (reading the prior reports); the gate runs after each turn (up to `max_fixes` repair turns) before the manager reviews it and the run advances.
- **Resumable runs:** `.agentx/state.json` tracks the live cursor (phase, current task, agent, round, per-task status) and is written atomically after every action; `start` resumes a drained or stopped journey from exactly where it left off.
- **Priming:** before any phase, every agent and the manager open a session and study the project once — context, contracts, skills, history, and the real codebase — then confirm the bar; the manager also receives a standing-duties charter. Every later turn is lightweight (work plus reports only).

### Training center (new)
- A global, per-archetype knowledge base at `~/.agentx/train/<id>/` — an `about.md` identity card (stack + what it fits) plus five buckets (`overview` · `contracts` · `skills` · `requires` · `history`) — seeded from archetypes embedded in the binary and extracted copy-if-absent (upgrades sync new material without clobbering learned history).
- **Binding:** `init`/`start` detect an empty `project_type` and `gate_cmd` in a single throwaway agent session — matching the project's stack against each archetype's `about.md` — and write both back to `Agentx.toml`, so later runs skip detection.
- **Injection:** each briefing prepends the archetype's overview/contracts/skills/history before the project's own `agents/` files — the project wins on conflict, so a fresh project of a known kind starts at staff level.
- **Learning:** on a successful run the manager writes one generalized lesson into the archetype's `history/`, so the next project of that kind starts smarter.

### Layout & config
- Two trees: `agents/` is durable and human-owned (overview, contracts, skills, requires); `.agentx/` is pure runtime, cleared on success while keeping its directory layout. Agents never write under `agents/`; the journey record goes to the training center, not the project.
- **tasks** are runtime artifacts under `.agentx/tasks/`, not human input under `agents/`.
- Config keys: `project_type`, `gate_cmd`, `gate_timeout`, `max_rounds`, `max_fixes`, `manager_model`, and the `architect_models` / `executor_models` / `tester_models` rosters (the `steps` key is gone).
- Root resolution now lets the nearest `Agentx.toml` win over the git root (monorepo-safe).
- Commands: added `info` (a read-only snapshot of config, paths, classification, journey, and sessions) and `reset` (wipe and re-seed `~/.agentx`); added global `-t/--type` and `-C/--dir` flags.

### Internals
- The main embeddable type is `App` (was `Agentx`); workers live in `core::worker::Worker`.
- Atomic writes (tmp + fsync + rename) for state, sessions, reports, reviews, and the journey record; a liveness-checked pid lock prevents two concurrent runs.

## v0.1.0 — 2026-06-21

First release. Foundations laid; the orchestration loop runs end to end.

### Added
- Layered Rust crate (`core` → `config` → `app` → `cli`) shipping as both a library and a binary.
- Orchestration cycle: **arch → work → test**, each driven to convergence (`ship it`) under a reviewing **manager** and a green **gate**.
- Roster expansion (`[claude, claude, codex]` → `claude_1 claude_2 codex_1`), each a persistent, independently-briefed agent.
- External CLI workers: `claude` and `codex`, with per-agent sessions persisted to `sessions.json`.
- Commands: `init`, `start`, `stop`, `drain`, `clean` — with process-group isolation, drain checkpoints, and run archival to `.agentx/runs/`.
- Neutral `support` std-lib (fs, parse, proc, text, time, thread, rt, …) and a global app context.
