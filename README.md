# ✨ AgentX

<div align="center">
    <img height="350" src="https://github.com/user-attachments/assets/3d70694c-db2b-40e2-acd3-1016523a91c5" />
</div>

[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](./LICENSE)
[![Rust 1.92+](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![edition 2024](https://img.shields.io/badge/edition-2024-green.svg)](https://doc.rust-lang.org/edition-guide/)
[![CI](https://github.com/comstrx/agentx/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/comstrx/agentx/actions/workflows/ci.yaml)
[![Release](https://img.shields.io/github/v/release/comstrx/agentx?sort=semver)](https://github.com/comstrx/agentx/releases/latest)

**You stay the architect. AgentX becomes your execution army.**

AgentX is a **contract-driven engineering operating system.** You encode your
architecture, contracts, skills, and taste once — then a disciplined team of CLI
coding agents (`claude`, optionally `codex`) executes your requirements *in your
own style*, reviewed and gate-kept at every step. One Rust crate: library *and* binary.

> It does **not** build a whole app from one magic prompt. It builds **one
> requirement, end-to-end** — architected, executed, audited, verified, and
> manager-reviewed — so you can ship a system of dozens of small, deliberate
> requirements and inspect every single one. AgentX scales your judgment; it
> doesn't replace it.

```
your requirement ─▶ intake ─▶ requires ─▶ tasks ─▶ audit ─▶ verify ─▶ train ─▶ you review ─▶ next requirement
   (your intent)    manager   architects  executors auditors  tests…   manager     (human)      (the loop)
```

## Why it's different

Most tools wrap an LLM and ask it to *"build the app."* AgentX runs on a different
engineering philosophy:

- **A review relay, not a vote.** Agents work each phase **sequentially** — every one
  inherits the previous one's report and refines it. Progressive refinement and
  context inheritance instead of parallel drafts that contradict each other. Slower
  by design; far higher quality.
- **Warm live workers, not cold prompts.** Each agent is a real, long-lived process
  kept warm for the whole journey (claude over streaming I/O, codex over its MCP
  server) — no respawn, no cold start, no replaying the transcript every turn. The
  agent keeps its train of thought, like an engineer who never left the terminal.
- **The training center is your mind, made executable.** `contracts · skills ·
  overview · designs` are a durable clone of *your* architecture, standards, and
  taste; agents retrain on them every run. The code comes out as if **you** wrote it
  — at the AI's raw speed — and each finished run feeds its lessons back, so the next
  project of the same kind starts smarter.
- **The manager owns the work, not just the review.** It rewrites your requirements
  into its own clean, ordered backlog first — taking real ownership of intent — then
  judges every round against the contracts with authority: it sends work back for
  *drifting from the intent*, not only for a bug.
- **Human-led, one requirement at a time.** You write small, sharp requirements;
  AgentX takes each from A to Z; you review the result and, on any gap, write the next
  one. You stay in command of the architecture — the agents are the execution army.

## Install

One line installs the right prebuilt binary for your platform (x86_64 & arm64),
checksum-verified, onto your `PATH`.

**Linux · macOS · WSL**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/comstrx/agentx/releases/latest/download/agentx-installer.sh | sh
```

> **Windows:** run it under **WSL2** for now — native Windows support (and a
> PowerShell installer) is on the roadmap.

**With Cargo** — prebuilt, or from source:

```sh
cargo binstall agentx                                   # prebuilt, no compile
cargo install --git https://github.com/comstrx/agentx   # from source (Rust 1.92+)
```

> agentx drives external agent CLIs — install `claude` (and `codex` if you use
> it) first; `agentx doctor` verifies them. macOS may quarantine an unsigned
> binary: `xattr -d com.apple.quarantine "$(command -v agentx)"`.

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
| `init` | scaffold `Agentx.toml` + `.agentx/` + `agentx/` from flags (no detection) |
| `new <dir>` | create a fresh project of a chosen archetype — the manager builds the skeleton (mandatory `--inspire`) |
| `start` | run or **resume** a full cycle; detect archetype + gate; on a clean cycle auto-records + clears `.agentx/` |
| `restart` | `clear` + `start` — a fresh cycle from scratch |
| `stop` | kill the running cycle now — resumable |
| `drain` | stop after the current turn — resumable |
| `train` | record the finished run into the training center (manager writes a report per requirement) — auto-runs after a clean cycle |
| `clear` | delete `.agentx/` runtime files, keep the layout |
| `ignore` / `include` | skip or force-in paths during classification (persisted) |
| `refresh` | reset the ignore/include lists and re-classify |
| `info` | read-only snapshot: config, paths, classification, journey |
| `status` | live run state, progress, workers, pids, and a tail of the live log — `-f/--tail` for a live dashboard |
| `doctor` | check every required agent CLI + tool is installed and runnable |
| `sync` | refresh the shipped training, **keep** learned history |
| `reset` | wipe and re-seed the training center from the binary |

## Flags

| flag | applies to | effect |
|---|---|---|
| `-i, --inspire <NAME\|N>` | init · new · start · restart | bind a training archetype (name or menu number; **required** for `new`) |
| `-g, --gate <COMMAND>` | init · new · start · restart | set the quality-gate command |
| `-d, --description <TEXT>` | init · new · start · restart | a short project description to guide the manager (classify + create) |
| `--lint <BOOL>` | init · new · start · restart | gate includes a lint / static-analysis pillar |
| `--format <BOOL>` | init · new · start · restart | gate format check + executors keep code formatted |
| `--audits <BOOL>` | init · new · start · restart | run the audit phase after tasks (auditors hunt integration/quality defects, raise remediation tasks) |
| `-t, --tests <BOOL>` | init · new · start · restart | run the tests phase + gate test pillar (`true/false`, `1/0`, `yes/no`) |
| `--benches <BOOL>` | init · new · start · restart | run the benches phase — real benchmarks for the executed work |
| `--examples <BOOL>` | init · new · start · restart | run the examples phase — real runnable examples |
| `--fuzzes <BOOL>` | init · new · start · restart | run the fuzzes phase — real fuzzing of the executed work |
| `--comments <BOOL>` | init · new · start · restart | executors add inline comments on non-obvious logic; off = none |
| `--doc-blocks <BOOL>` | init · new · start · restart | document every public item in the native doc format |
| `--doc-contracts <BOOL>` | init · new · start · restart | document non-obvious units that don't return explicit types |
| `-b, --background` (`--bg`) | start · restart | run detached; drive with `status`/`drain`/`stop` |
| `--no-train` | init · new · start · restart | don't auto-record the finished run into the training center (sets `[option].train = false`) |
| `--no-clear` | init · new · start · restart | don't auto-clear `.agentx/` when the run finishes (sets `[option].clear = false`) |
| `--ignore` / `--include <PATH>…` | start · restart | curate classification (merged + persisted) |
| `-C, --dir <DIR>` | any | operate as if started in `DIR` |

Per-backend `model`/`effort` live only in the `[claude]`/`[codex]` tables of `Agentx.toml`.

Shell completions and a man page are generated on demand:

```sh
agentx completions zsh > ~/.zfunc/_agentx      # bash · zsh · fish · elvish · powershell
agentx man > /usr/local/share/man/man1/agentx.1
```

## How a run works

The philosophy above, made concrete — every phase is a relay of warm, contract-bound agents the manager rules on:

- **Prime** — the whole team studies the project once and confirms the bar (training only).
- **Discover** — if the archetype or gate isn't set, the now-primed manager classifies the project (binding a training archetype) and composes the gate command — always a `check` baseline, plus the `lint`/`format`/`tests` pillars you switched on.
- **Intake** — the manager turns your requirements into an ordered, de-duplicated backlog.
- **Requires** — architects write ordered task contracts: path, interface, invariants, acceptance criteria.
- **Tasks** — executors build them one at a time; the gate runs after every turn (≤ `max_fixes` repairs; a gate still red after the last repair stops the run with a clear, resumable error).
- **Audit** — when `audits` is on, a council of auditors examines the WHOLE built system for integration, layering, abstraction, providers/adaptors, dangerous dependencies, performance, and secrets, and raises each real defect as an explained remediation task; the executors build those, then it audits again — up to `max_audits` rounds, or until the system is clean.
- **Verify** — then up to four ordered phases, each run only when its `[option]` switch is on and skipped otherwise: **tests** → **benches** → **examples** → **fuzzes**. Each has its own roster, works on the executed tasks for real (the language's idiomatic tooling, run and measured), and is manager-reviewed every round (≤ `max_rounds`). Because these phases write real code into the project, the gate runs after every turn here too: a producer repairs its own broken artifact, but never edits project source — a real defect a test surfaces is reported, and a gate left red blocks the phase for review rather than halting the run.
- **Train & clear** — a clean cycle auto-records the run into the training center (manager writes a decision report per requirement) and clears `.agentx/`; both are also manual commands (`agentx train`, `agentx clear`) for when you stop early.
- **Warm** — each agent runs as one long-lived session kept warm for the whole journey (claude over streaming I/O, codex over its MCP server), so turns have no cold start and never lose context.
- **Resumable** — the cursor is checkpointed after every action; `stop`/`drain`/`Ctrl+C` are safe and `start` resumes (re-warming each agent once).
- **Resilient** — faults are classified and retried; a lost session is rebuilt; quota/auth stops cleanly.

## Self-training

Your engineering judgment, stored once and reused on every project of the same kind.
A global, per-archetype knowledge base at `~/.agentx/train/<id>/`: an `about.md`
identity card, the study buckets `overview · contracts · skills · designs ·
references` (a `references/` of prior projects to learn taste from), a `manifests/`
tree merged into a new project's root, and a growing
`history/{requires, tasks, reports}` archive.

Each bucket is matched flexibly — file **or** folder, singular **or** plural,
case-insensitive — from the training center first, then your project's `agentx/`
dir or root (your files win on conflict).

- **Bound** — the primed manager matches your project's stack to the best archetype and writes it to `Agentx.toml`.
- **Injected** — its knowledge prepends every agent's briefing; on conflict, **your files win**.
- **Learned** — `train` archives the requirements it built, the tasks they became, and one manager decision
  report per requirement into `history/` — so the next project of that kind starts smarter.

## Config

`agentx init` writes `Agentx.toml` and fills defaults — run `agentx info` to see the resolved config.

| table | keys (defaults) |
|---|---|
| `[project]` | `inspire` · `description` |
| `[option]` | `lint` · `format` · `audits` · `tests` · `fuzzes` · `benches` · `examples` · `comments` · `doc_blocks` · `doc_contracts` (default off) · `train` · `clear` (default on) — each a flexible bool. `lint`/`format`/`tests` add gate pillars; `audits`/`tests`/`benches`/`examples`/`fuzzes` switch their phase on; `comments`/`doc_blocks`/`doc_contracts` shape how executors document; `train`/`clear` toggle the post-run auto-record and auto-clear (disable via `--no-train`/`--no-clear`) |
| `[gate]` | `command` · `timeout` (1000s) |
| `[agent]` | `max_audits` (3) · `max_rounds` (3) · `max_fixes` (3) · `timeout` (10000s) · `manager` (exactly one) · per-phase rosters `requires` · `tasks` · `audits` · `tests` · `benches` · `examples` · `fuzzes` |
| `[claude]` / `[codex]` | `model` · `effort` (empty = CLI default) |

Each phase has its own roster. A roster value can be a single name (`"claude"`), a list (`["claude", "claude", "codex"]` → `claude_1 claude_2 codex_1`, each a persistent, independently-briefed agent), or empty (`""`/`[]` → the default agent when that phase runs). `manager` must be exactly one agent.

## As a library

Every command is a thin wrapper over a method on `App`, so the library does
exactly what the CLI does — blocking, returning `agentx::AppResult<()>`:

```rust
use agentx::{App, Flags};
use std::path::Path;

fn main() -> agentx::AppResult<()> {
    App::start(Path::new("."), &Flags::default())
}
// App::{init, create, start, restart, stop, drain, train, clear, ignore, include,
//       refresh, info, status, doctor, sync, reset} — the full CLI surface.
```

## Platforms

**Linux · macOS · WSL2.** Native Windows isn't supported yet — the OS-specific
calls (process groups, POSIX signals, `termios`) are isolated to
`core/{proc,term}`, so a contained `#[cfg(windows)]` port is planned
after the tool proves itself in production.

## License

**AGPL-3.0-only** — see [LICENSE](./LICENSE). Run a modified Agentx as a hosted
service and you must offer users its source. For other terms, contact the author.
