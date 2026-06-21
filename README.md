# ✨ Agentx

<div align="center">
    <img height="350" src="https://github.com/user-attachments/assets/3d70694c-db2b-40e2-acd3-1016523a91c5" />
</div>

[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](./LICENSE)
[![Rust 1.92+](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![edition 2024](https://img.shields.io/badge/edition-2024-green.svg)](https://doc.rust-lang.org/edition-guide/)

**Drop requirements into a repo. Get production-grade code back.**

A single Rust crate — library *and* binary — that orchestrates a competing team
of CLI coding agents to convergence. Fast, gated, zero ceremony.

Agentx is a hierarchical multi-agent orchestrator. It runs a *competing team* of
CLI agents (`claude`, optionally `codex`) through three steps —
**arch** (plan) → **work** (build) → **test** (verify) — with a **manager**
judging every step and a **gate** kept green throughout.

```
requirements ─▶ arch ─▶ work ─▶ test ─▶ decision record  +  archived run
```

## How it works

Three loops drive every step to convergence:

- **Roster** — each step runs a team of agents in rounds; a turn converges only when its report's last line is exactly `ship it`.
- **Manager** — reviews each finished step and either ships it or writes concrete notes back for another round, up to `max_rounds`.
- **Gate** — in **work** only, runs after every executor turn; if it's red, the executor gets up to `max_fixes` repair turns before the step can ship.

On success the manager writes one decision record to `agents/history/` and the
whole run is archived to `.agentx/runs/<stamp>/`. If a step can't converge, the
pipeline stops there and records the open issues — nothing is destroyed.

## Quickstart

```sh
cargo install --path .                         # → agentx on your PATH
agentx init                                    # scaffold the layout
echo "build X that does Y" > agents/requires/0001-x.md
agentx start                                   # the team builds it
```

> Agents are external CLIs — install `claude` (and `codex` if you use it).

## Commands

| command | what it does |
|---------|--------------|
| `init`  | scaffold `Agentx.toml`, `agents/`, `.agentx/` in the current dir |
| `start` | resolve the project root, run a full cycle |
| `stop`  | kill the running cycle and its agents immediately |
| `drain` | stop cleanly after the current turn (state kept, resumable) |
| `clean` | delete the `.agentx` cache |

`start` resolves the root by: `Agentx.toml` in cwd → nearest `.git` upward →
nearest `Agentx.toml` upward → cwd. `init` always targets the current dir.

## Layout it owns

```
agents/            durable, committed — your intent + the run's memory
  overview.md        how the system must be built
  contracts/         LAW — overrides agent preferences
  requires/          requirements to build
  tasks/             task contracts (frozen if you authored them)
  history/           one decision record per completed cycle
.agentx/           ephemeral scratch (gitignored)
  reports/ rounds/ prompts/ tests/ probes/ runs/
  sessions.json  control.md  review.md  gate.log  *.pid  drain
Agentx.toml        config — optional, sane defaults if absent
```

## Config (`[project]`)

| key             | default            | meaning                                   |
|-----------------|--------------------|-------------------------------------------|
| `max_rounds`    | 5                  | manager review rounds per step            |
| `max_fixes`     | 5                  | gate-repair turns per executor            |
| `gate_cmd`      | `""`               | shell command that must pass (empty = ok) |
| `gate_timeout`  | 900                | seconds before the gate is force-failed   |
| `manager_model` | `claude`           | reviewer model                            |
| `steps`         | `arch, work, test` | which steps to run                        |
| `arch_models`   | `[claude]`         | architect roster (duplicates allowed)     |
| `work_models`   | `[claude]`         | executor roster                           |
| `test_models`   | `[claude]`         | verifier roster                           |

A roster of `["claude", "claude", "codex"]` expands to `claude_1 claude_2
codex_1` — each a persistent, independently-briefed agent.

```toml
# Agentx.toml — every key is optional; shown with a real gate and a mixed roster.
[project]
max_rounds    = 5
max_fixes     = 5
gate_cmd      = "cargo clippy -- -D warnings && cargo test"
gate_timeout  = 900
manager_model = "claude"
steps         = ["arch", "work", "test"]
arch_models   = ["claude"]
work_models   = ["claude", "codex"]
test_models   = ["claude"]
```

## As a library

Agentx is a library *and* a binary from one crate — embed it in any Rust project.
Every entry point is blocking and returns `agentx::AppResult<()>`:

```rust
use agentx::Agentx;

fn main() -> agentx::AppResult<()> {
    Agentx::start(std::path::Path::new("."))   // also: init · stop · drain · clean
}
```

## License

Licensed under the **GNU Affero General Public License v3.0** — see [LICENSE](./LICENSE).

AGPL's network clause is the point: if you run a modified Agentx as a hosted
service, you must offer its users the corresponding modified source. For terms
outside the AGPL, contact the author.
