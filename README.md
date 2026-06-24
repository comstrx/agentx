# ✨ AgentX

<div align="center">
    <img height="350" src="https://github.com/user-attachments/assets/3d70694c-db2b-40e2-acd3-1016523a91c5" />
</div>

[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](./LICENSE)
[![Rust 1.92+](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![edition 2024](https://img.shields.io/badge/edition-2024-green.svg)](https://doc.rust-lang.org/edition-guide/)

**Drop requirements into a repo. Get production-grade code back.**

A competing team of CLI coding agents (`claude`, optionally `codex`) driven to
convergence — **fast, gated, resumable, self-training.** One Rust crate: library
*and* binary.

```
Requirements.md ─▶ intake ─▶ requires ─▶ tasks ─▶ tests ─▶ finalize ─▶ ~/.agentx/train/<id>/history/
   (your input)    manager   architects  executors verifiers  manager     the lesson, fed back next time
```

## Install

One line installs the right prebuilt binary for your platform (x86_64 & arm64),
checksum-verified, onto your `PATH`.

**Linux · macOS · WSL**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/comstrx/agentx/releases/latest/download/agentx-installer.sh | sh
```

**Windows** (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/comstrx/agentx/releases/latest/download/agentx-installer.ps1 | iex"
```

**With Cargo** — prebuilt, or from source:

```sh
cargo binstall agentx                                   # prebuilt, no compile
cargo install --git https://github.com/comstrx/agentx   # from source (Rust 1.92+)
```

> agentx drives external agent CLIs — install `claude` (and `codex` if you use
> it) first; `agentx doctor` verifies them. macOS may quarantine an unsigned
> binary: `xattr -d com.apple.quarantine "$(command -v agentx)"`.
>
> **Windows is via WSL2 for now** — native Windows support is on the roadmap, and
> the PowerShell installer above goes live with it.

## Build from source

```sh
git clone https://github.com/comstrx/agentx && cd agentx
cargo install --path .     # → agentx on your PATH
```

## Quickstart

```sh
agentx init                                  # scaffold Agentx.toml + .agentx/
echo "build X that does Y" > Requirements.md # one file or many; root or agentx/requires/
agentx start                                 # detects archetype + gate, then the team builds it
agentx start --bg                            # or detached — drive it with status / drain / stop
```

## Commands

| command | what it does |
|---|---|
| `init` | scaffold `Agentx.toml` + `.agentx/`; detect archetype + gate |
| `start` | run or **resume** a full cycle; clears `.agentx/` on success |
| `restart` | `clear` + `start` — a fresh cycle from scratch |
| `stop` | kill the running cycle now — resumable |
| `drain` | stop after the current turn — resumable |
| `clear` | delete `.agentx/` runtime files, keep the layout |
| `ignore` / `include` | skip or force-in paths during classification (persisted) |
| `refresh` | reset the ignore/include lists and re-classify |
| `info` | read-only snapshot: config, paths, classification, journey |
| `status` | live run state, progress, workers, pids — `-f/--tail` for a live dashboard |
| `doctor` | check every required agent CLI + tool is installed and runnable |
| `sync` | refresh the shipped training, **keep** learned history |
| `reset` | wipe and re-seed the training center from the binary |

## Flags

| flag | applies to | effect |
|---|---|---|
| `-i, --inspire <NAME\|N>` | init · start · restart | bind a training archetype (name or menu number) |
| `-g, --gate <COMMAND>` | init · start · restart | set the quality-gate command |
| `-t, --tests <BOOL>` | init · start · restart | verifiers write real project tests (`true/false`, `1/0`, `yes/no`) |
| `-b, --background` (`--bg`) | start · restart | run detached; drive with `status`/`drain`/`stop` |
| `--ignore` / `--include <PATH>…` | start · restart | curate classification (merged + persisted) |
| `-C, --dir <DIR>` | any | operate as if started in `DIR` |

Per-backend `model`/`effort` live only in the `[claude]`/`[codex]` tables of `Agentx.toml`.

## How a run works

- **Prime** — the whole team studies the project once and confirms the bar (training only).
- **Intake** — the manager turns your requirements into an ordered, de-duplicated backlog.
- **Requires** — architects write ordered task contracts: path, interface, invariants, acceptance criteria.
- **Tasks** — executors build them one at a time; the gate runs after every turn (≤ `max_fixes` repairs).
- **Tests** — verifiers attack the finished code; the manager reviews every round (≤ `max_rounds`).
- **Resumable** — the cursor is checkpointed after every action; `stop`/`drain`/`Ctrl+C` are safe and `start` resumes.
- **Resilient** — faults are classified and retried; a lost session is rebuilt; quota/auth stops cleanly.

## Self-training

A global, per-archetype knowledge base at `~/.agentx/train/<id>/`: an `about.md`
identity card plus `overview · contracts · skills · requires · history`.

- **Bound** — `start` matches your project's stack to the best archetype and writes it to `Agentx.toml`.
- **Injected** — its knowledge prepends every agent's briefing; on conflict, **your files win**.
- **Learned** — each finished run appends one lesson, so the next project of that kind starts smarter.

## Config

`agentx init` writes `Agentx.toml` and fills defaults — run `agentx info` to see the resolved config.

| table | keys (defaults) |
|---|---|
| `[project]` | `inspire` · `tests` (true) · `max_rounds` (5) · `max_fixes` (5) |
| `[gate]` | `command` · `timeout` (900s) |
| `[agent]` | `timeout` (10000s) · `manager` · `architects` · `executors` · `testers` |
| `[claude]` / `[codex]` | `model` · `effort` (empty = CLI default) |

A roster like `["claude", "claude", "codex"]` expands to `claude_1 claude_2 codex_1` — each a persistent, independently-briefed agent.

## As a library

Every command is a thin wrapper over a method on `App`, so the library does
exactly what the CLI does — blocking, returning `agentx::AppResult<()>`:

```rust
use agentx::{App, Flags};
use std::path::Path;

fn main() -> agentx::AppResult<()> {
    App::start(Path::new("."), &Flags::default())
}
// App::{init, start, restart, stop, drain, clear, ignore, include,
//       info, status, doctor, sync, reset} — the full CLI surface.
```

## Platforms

**Linux · macOS · WSL2.** Native Windows isn't supported yet — the OS-specific
calls (process groups, POSIX signals, `termios`) are isolated to
`core/support/{proc,term}`, so a contained `#[cfg(windows)]` port is planned
after the tool proves itself in production.

## License

**AGPL-3.0-only** — see [LICENSE](./LICENSE). Run a modified Agentx as a hosted
service and you must offer users its source. For other terms, contact the author.
