# ✨ AgentX

<div align="center">
    <img height="350" src="https://github.com/user-attachments/assets/3d70694c-db2b-40e2-acd3-1016523a91c5" />
</div>

[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](./LICENSE)
[![Rust 1.92+](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![edition 2024](https://img.shields.io/badge/edition-2024-green.svg)](https://doc.rust-lang.org/edition-guide/)

**Drop requirements into a repo. Get production-grade code back.**

One Rust crate — library *and* binary — that drives a competing team of CLI
coding agents (`claude`, optionally `codex`) to convergence: **fast, gated,
resumable, self-training.** Architects turn your requirements into ordered task
contracts, executors build them one at a time, verifiers attack the result, a
manager judges every round, and a gate stays green throughout.

```
agents/requires/ ─▶ requires ─▶ tasks ─▶ tests ─▶ finalize ─▶ ~/.agentx/train/<type>/history/
                    architects  executors verifiers  manager     the lesson, fed back to next time
```

## Quickstart

```sh
cargo install --path .                      # → agentx on your PATH
agentx init                                 # scaffold + bind a training archetype
echo "build X that does Y" > agents/requires/0001-x.md
agentx start                                # the team builds it
```

> Agents are external CLIs — install `claude` (and `codex` if you use it).

## Commands

| command | what it does |
|---------|--------------|
| `init`  | scaffold `Agentx.toml` + `agents/` + `.agentx/`, then detect the project type and gate command and write them to `Agentx.toml` |
| `start` | resolve the project root, then **run or resume** a full cycle; clears `.agentx/` on success |
| `stop`  | kill the running cycle and its agents immediately — resumable |
| `drain` | stop cleanly after the current turn — resumable |
| `clean` | clear the `.agentx/` runtime files, keeping the directory layout |
| `info`  | print a clean snapshot — config, paths, classification, journey, sessions (read-only) |
| `reset` | wipe and re-seed the global training center (`~/.agentx`) from the binary |

Global flags: `-C, --dir <DIR>` operate as if started in `DIR`; `-t, --type
<TYPE>` bind a training archetype explicitly (skips auto-detection).

`start` resolves the root by walking up for the nearest `Agentx.toml`; if none
is found it uses the current dir — the git root is never used, so a sub-project
in a monorepo (e.g. `repo/server`) stays its own project. `init` always targets
the current dir.

## How a run works

- **Prime** — before any work, every agent and the manager open a session, study
  the project (your `agents/` files + the archetype knowledge + the real
  codebase) and confirm the bar. Briefing happens **once**; later turns are
  lightweight — work plus reading reports.
- **Architects** write ordered task contracts (`0001-*.md`, …) under
  `.agentx/tasks/`, each fixing a path, public interface, invariants, and
  testable acceptance criteria.
- **Executors** build the tasks **one at a time** — the whole roster works each
  task in turn, reading the prior reports. The gate runs after every executor
  turn; a red gate gets up to `max_fixes` repair turns.
- **Manager** reviews each round and ships it or returns concrete notes, up to
  `max_rounds`. **Verifiers** then exercise the finished code for real.

Runs are **resumable**: the cursor (phase, task, agent, round) is checkpointed
atomically to `.agentx/state.json` after every action, so `stop`/`drain` are
safe and `start` picks up exactly where it left off. Agents never write under
`agents/`.

## How it trains itself

agentx keeps a global, per-archetype knowledge base at `~/.agentx/train/<id>/`.
Each archetype carries an `about.md` — its identity card: stack, architecture,
and exactly what it fits — plus five buckets: `overview/` (how this kind of
system is built), `contracts/` (enforceable law), `skills/` (reusable
playbooks), `requires/` (baseline requirements), `history/` (lessons from past
runs).

- **Seeded** — archetypes ship embedded in the binary and extract on first run
  (copy-if-absent, so upgrades sync new material without clobbering learned
  history).
- **Bound** — on `init`/`start`, if `project_type` is empty, a one-shot agent
  reads every archetype's `about.md`, matches your project's stack and shape to
  the best fit (and proposes a `gate_cmd`), and writes both to `Agentx.toml`.
- **Injected** — every agent's briefing prepends the archetype's overview +
  contracts + skills + history before your own `agents/` files; on conflict,
  **your files win**. A fresh project of a known kind starts at staff level.
- **Learned** — when a run completes, the manager writes one generalized lesson
  into the archetype's `history/`. The next project of that kind starts smarter.

That is the loop: each run teaches the archetype; each new project inherits
everything the archetype has learned.

## Layout

```
agents/            durable, committed — your intent (agents never write here)
  overview.md        how the system must be built   (a file, or overview/)
  contracts/         LAW — overrides agent preferences
  skills/            reusable know-how for this codebase
  requires/          requirements to build           (your only required input)
.agentx/           ephemeral runtime (gitignored), cleared on success
  state.json         the resumable cursor: phase, current task/agent, round
  requires/          frozen snapshot of the requirements
  tasks/             architect-generated task contracts
  reports/           per-agent reports per phase + manager/ reviews
  rounds/            per-round report archive (task-scoped under tasks/)
  sessions.json  gate.log  *.pid  drain
Agentx.toml        config — created by init
~/.agentx/train/<id>/   global training center (shared across all projects)
  about.md             archetype identity card — stack + what it fits (drives binding)
  overview/ contracts/ skills/ requires/ history/
```

## Config (`[project]`)

| key                | default    | meaning                                          |
|--------------------|------------|--------------------------------------------------|
| `project_type`     | `""`       | training archetype to inherit (auto-detected)    |
| `gate_cmd`         | `""`       | shell command that must pass (auto-suggested; empty = skip) |
| `gate_timeout`     | 900        | seconds before the gate is force-failed          |
| `max_rounds`       | 5          | manager review rounds per phase / task           |
| `max_fixes`        | 5          | gate-repair turns per executor                   |
| `manager_model`    | `claude`   | reviewer model                                   |
| `architect_models` | `[claude]` | architect roster (duplicates allowed)            |
| `executor_models`  | `[claude]` | executor roster                                  |
| `tester_models`    | `[claude]` | verifier roster                                  |

A roster of `["claude", "claude", "codex"]` expands to `claude_1 claude_2
codex_1` — each a persistent, independently-briefed agent.

```toml
# Agentx.toml — every key is optional; init fills project_type + gate_cmd for you.
[project]
project_type     = "laravel-octane-tenancy-api"
gate_cmd         = "vendor/bin/phpstan --no-progress && php artisan test"
gate_timeout     = 900
max_rounds       = 5
max_fixes        = 5
manager_model    = "claude"
architect_models = ["claude"]
executor_models  = ["claude", "codex"]
tester_models    = ["claude"]
```

## As a library

Agentx is a library *and* a binary from one crate — embed it in any Rust project.
Every entry point is blocking and returns `agentx::AppResult<()>`:

```rust
use agentx::App;

fn main() -> agentx::AppResult<()> {
    App::start(std::path::Path::new("."))   // also: init · stop · drain · clean · info
}
```

## License

Licensed under the **GNU Affero General Public License v3.0** — see [LICENSE](./LICENSE).

AGPL's network clause is the point: if you run a modified Agentx as a hosted
service, you must offer its users the corresponding modified source. For terms
outside the AGPL, contact the author.
