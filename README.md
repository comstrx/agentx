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

- It does **not** build a whole app from one magic prompt. It builds **one
- requirement, end-to-end** — architected, executed, audited, verified, and
- manager-reviewed — so you can ship a system of dozens of small, deliberate
- requirements and inspect every single one. AgentX scales your judgment; it
- doesn't replace it.

```
your requirement ─▶ intake ─▶ requires ─▶ tasks ─▶ audits ─▶ verify ─▶ train ─▶ you review
  (your intent)     manager   architects  executors auditors  tests…   manager    (human)
        ▲                                                                             │
        └──────────────────◀ the loop · your next sharp requirement ──────────────────┘
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
- **It objects before it guesses.** After the whole team is primed, intake runs three
  checkpoints — the project foundation (is there really something to build on?), the
  requirements (any breaking conflict?), then the quality gate (do its tools and
  scripts really exist and cover the pillars?). Each real flaw becomes a brutally
  short objection right in your terminal, and you rule on the spot: Enter continues
  (conflicts become visible `Assumption:` lines, never silent guesses), **fix** hands
  it to the manager — he repairs the foundation, resolves the requirements with
  visible `Decision:` lines, or composes the corrected gate, then the check re-runs —
  or stop resumably and fix it yourself. Clean ground feels nothing, and `-f/--force`,
  CI, or a background run never pause — the dialogue is a terminal-only privilege.
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
agentx start                                 # matches a project kind + gate, then the team builds it
agentx start --bg                            # or detached — drive it with status / drain / stop
```

## Commands

| command | what it does |
|---|---|
| `init` | scaffold `Agentx.toml` + `.agentx/` + `agentx/` from flags; on a terminal it offers the inspire menu (with `auto`) then a skippable gate prompt |
| `new <dir>` | create a fresh project of a chosen project node — the manager builds the skeleton (mandatory `--inspire`) |
| `start` | run or **resume** a full cycle; match a project kind + gate; on a clean cycle auto-records + clears `.agentx/` |
| `restart` | `clear` + `start` — a fresh cycle from scratch |
| `stop` | kill the running cycle now — resumable |
| `drain` | stop after the current turn — resumable |
| `train` | record the finished run into the training center (manager writes a report per requirement) — auto-runs after a clean cycle |
| `clear` | delete `.agentx/` runtime files, keep the layout |
| `ignore <PATH>…` | skip paths during classification (persisted) |
| `include <PATH>…` | force paths into classification, overriding ignore (persisted) |
| `refresh` | reset the ignore/include lists and re-classify |
| `inspire [NAME\|N]` | bind or switch the project's inspiration node — by name/number, or from the menu |
| `gate [COMMAND]` | set or switch the quality-gate command — as an argument, or typed at a prompt; `--show` prints the current gate and exits |
| `info` | read-only snapshot: config, pids, paths, classification, journey, sessions |
| `status` | one snapshot: state, per-seat engines, journey progress, live-log tail, a numbers-only stats block, and a closing "Now · what's happening" panel |
| `watch` | the same status as a live dashboard — full-screen refresh every second until you exit (Ctrl+C) |
| `doctor` | check every required agent CLI + tool is installed and runnable |
| `compose` | print the **exact assembled prompts** the agents receive, as titled blocks with real project values — zero agent calls; filter with `--role`/`--phase` down to a single block or fragment, `--out FILE` writes instead of printing |
| `sync` | re-extract the shipped knowledge nodes, **keep** learned history |
| `reset` | wipe and re-seed the training center from the binary — always asks first on a terminal; headless needs `-y` |

## Flags

| flag | applies to | effect |
|---|---|---|
| `-i, --inspire <NAME\|N>` | init · new · start · restart | bind a project node (name or menu number; **required** for `new`) |
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
| `-y, --yes` | any | assume yes on every confirmation prompt (e.g. `reset`) — for CI and scripts |
| `-f, --force` | start · restart | never pause intake on an objection — conflicts resolve to the narrowest assumption, recorded as a visible `Assumption:` line in the backlog (implied when headless: CI, pipes, `--bg`) |
| `-b, --background` (`--bg`) | new · start · restart | run detached; drive with `status`/`drain`/`stop` |
| `--no-train` | init · new · start · restart | don't auto-record the finished run into the training center (sets `[option].train = false`) |
| `--no-clear` | init · new · start · restart | don't auto-clear `.agentx/` when the run finishes (sets `[option].clear = false`) |
| `--ignore <PATH>…` | start · restart · refresh | skip paths during classification (merged + persisted) |
| `--include <PATH>…` | start · restart · refresh | force paths into classification, overriding ignore (merged + persisted) |
| `-C, --dir <DIR>` | any | operate as if started in `DIR` |

`model`/`effort` live in `Agentx.toml` — inline on any seat, or per backend in the `[claude]`/`[codex]` tables (see Config).

Shell completions and a man page are generated on demand:

```sh
agentx completions zsh > ~/.zfunc/_agentx      # bash · zsh · fish · elvish · powershell
agentx man > /usr/local/share/man/man1/agentx.1
```

## How a run works

The philosophy above, made concrete — every phase is a relay of warm, contract-bound agents the manager rules on:

- **Prime** — the manager trains and confirms FIRST, alone.
- **Intake** — still before anyone else is primed, the manager verifies the ground in FOUR checkpoints: the project tree, the training-center binding, the quality gate, and the requirements — each with the objection dialogue (continue / fix / stop), and each re-armed on EVERY `start`, resumes included. There is no silent auto-discovery any more: choosing **fix** is what makes the manager classify the project into the center or compose the gate (`check` baseline plus the `lint`/`format`/`tests` pillars you switched on). Only after the checks pass is the team primed, and only after the team is primed does the manager write the final ordered backlog (skipped when resuming mid-journey). A final active-recall pass confirms the bar.
- **Requires** — architects write ordered task contracts: path, interface, invariants, acceptance criteria.
- **Tasks** — executors build them one at a time; the gate runs after every turn (≤ `max_fixes` repairs; a gate still red after the last repair stops the run with a clear, resumable error).
- **Audits** — when `audits` is on, a council of auditors examines the WHOLE built system for integration, layering, abstraction, providers/adaptors, dangerous dependencies, performance, and secrets, and raises each real defect as an explained remediation task; the executors build those, then it audits again — up to `max_audits` rounds, or until the system is clean.
- **Verify** — then up to four ordered phases, each run only when its `[option]` switch is on and skipped otherwise: **tests** → **benches** → **examples** → **fuzzes**. Each has its own roster, works on the executed tasks for real (the language's idiomatic tooling, run and measured), and is manager-reviewed every round (≤ `max_rounds`). Because these phases write real code into the project, the gate runs after every turn here too: a producer repairs its own broken artifact, but never edits project source — a real defect a test surfaces is reported, and a gate left red blocks the phase for review rather than halting the run.
- **Train & clear** — a clean cycle auto-records the run into the training center (manager writes a decision report per requirement) and clears `.agentx/`; both are also manual commands (`agentx train`, `agentx clear`) for when you stop early.
- **Warm** — each agent runs as one long-lived session kept warm for the whole journey (claude over streaming I/O, codex over its MCP server), so turns have no cold start and never lose context.
- **Resumable** — the cursor is checkpointed after every action; `stop`/`drain`/`Ctrl+C` are safe and `start` resumes (re-warming each agent once).
- **Resilient** — faults are classified and retried; a lost session is rebuilt; quota/auth stops cleanly.

## Self-training

Your engineering judgment, written once and **composed** per project — three zones
under `~/.agentx/`:

- **`base/`** — shared knowledge in seven numbered axes; the number **is** the
  composition order:
  `01_architecture · 02_pattern · 03_form · 04_lang · 05_framework · 06_standard · 07_domain`.
  Each axis holds nodes (`04_lang/rust`, `05_framework/laravel`, `07_domain/saas`, …),
  and every node carries the same optional buckets — `contracts · overview · skills ·
  references · designs` — so a Rust idiom, a web-API rule, or a tenancy skill lives in
  **one** node, reused by every project that needs it.
- **`project/`** — your curated project nodes (`NN_<name>/`): the five buckets plus
  `manifests/` and a `config.json`.
- **`history/`** — one flat, numbered folder per project kind, holding the stamped
  decision reports the tool accumulates run after run.

A project node's `config.json` is its identity card — a human `name`, an optional
`history` naming the folder under `history/` that holds this kind's accumulated
reports (empty → the node's own name, matched with or without the `NN_` prefix),
a one-line stack `description`, and a `dependency` map whose keys name **axes**
and whose values name **nodes** (a name **or** a list):

```json
{
  "name": "Laravel multi-tenant SaaS API",
  "history": "laravel-saas-tenancy-api",
  "description": "A monolithic multi-tenant SaaS HTTP/JSON API on Laravel Octane + FrankenPHP…",
  "dependency": {
    "lang": "php", "framework": "laravel",
    "standard": ["cache", "payment", "webhook"], "domain": ["saas", "tenancy"]
  }
}
```

Resolution is **transitive**: any node — base nodes included — may carry its own
`config.json`, and its dependencies are followed recursively, so `laravel` can itself
inherit `lang/php` and every kind that composes laravel gets php for free. One visited
set guards cycles and diamonds — a shared node lands exactly **once**, at its most
general position; a dependency that resolves to nothing is flagged with its full
parent chain, never dropped silently.

The tool composes every node's `.md` in **axis-ladder order** — `01_architecture →
… → 07_domain`, then your project node, then your **live repo last**. The later layer
always wins, so a project preference overrides an inherited default. Names and buckets
match flexibly: the `NN_` prefix is ordering metadata only (stripped on lookup),
case-insensitive, file **or** folder, singular **or** plural; `designs`/`references`
accept any file type (Figma, a whole prior project, a link).

- **Bound** — the primed manager matches your project to a curated node (or, if none
  fits, coins a clean kind-name and starts a fresh `history/` line — it never edits
  `project/`); the choice lands in `Agentx.toml`.
- **Injected** — the composed chain plus the kind's accumulated history prepend every
  agent's briefing, general → specific; on conflict **your live files win**.
- **Learned** — `train` writes one manager decision report per requirement into
  `history/<kind>/` — so the next project of that kind starts smarter.

`agentx info` prints the composed node chain and the accumulated report count, and
flags any dependency that points at a missing node.

## Config

`agentx init` writes `Agentx.toml` and fills defaults — run `agentx info` to see the resolved config.

| table | keys (defaults) |
|---|---|
| `[project]` | `inspire` · `stage` (`dev` \| `staging` \| `live`, default `dev` — the blast-radius contract: `dev` = break and reshape freely, `live` = published interfaces are promises, data stores untouchable without an explicit requirement) · `description` |
| `[option]` | `lint` · `format` · `audits` · `tests` · `fuzzes` · `benches` · `examples` · `comments` · `doc_blocks` · `doc_contracts` (default off) · `train` · `clear` (default on) — each a flexible bool. `lint`/`format`/`tests` add gate pillars; `audits`/`tests`/`benches`/`examples`/`fuzzes` switch their phase on; `comments`/`doc_blocks`/`doc_contracts` shape how executors document; `train`/`clear` toggle the post-run auto-record and auto-clear (disable via `--no-train`/`--no-clear`) |
| `[gate]` | `command` · `timeout` (1000s) |
| `[agent]` | `max_audits` (3) · `max_rounds` (3) · `max_fixes` (3) · `timeout` (10000s) · `manager` (exactly one) · per-phase rosters `requires` · `tasks` · `audits` · `tests` · `benches` · `examples` · `fuzzes` |
| `[claude]` / `[codex]` | `model` · `effort` — optional overrides for bare seats; absent = built-in defaults (claude `claude-opus-4-8`/`xhigh` · codex `gpt-5.5`/`high`). Not scaffolded — add them only when you want one place to steer every bare seat |

Each phase has its own roster. A roster entry is either a bare backend name (`"claude"`) or an inline table `{ agent = "claude", model = "fable-5", effort = "max" }`; a roster field is a single entry or a list of them. `manager` must resolve to exactly one entry. Entries expand to `claude_1 claude_2 codex_1 …` (backend + ordinal), each a persistent, independently-briefed agent. `init` scaffolds every seat explicit — `{ agent, model, effort }`, one architect — so the file teaches its own syntax; edit freely.

`model`/`effort` resolve **per seat**, three layers: the entry's own field → the `[<backend>]` section → the built-in default. So the manager can run `fable-5`/`max` while one architect runs `opus-4.8`/`high` and the next `fable-5`/`xhigh`:

```toml
[agent]
manager  = { agent = "claude", model = "fable-5", effort = "max" }
requires = [ { agent = "claude", model = "opus-4.8" }, { agent = "claude", model = "fable-5", effort = "xhigh" } ]
tasks    = [ "claude" ]                                # bare → inherits [claude] wholly
```

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
//       refresh, inspire, gate, info, status, watch, doctor, compose, sync, reset} — the full CLI surface.
```

## Platforms

**Linux · macOS · WSL2.** Native Windows isn't supported yet — the OS-specific
calls (process groups, POSIX signals, `termios`) are isolated to
`core/{proc,term}`, so a contained `#[cfg(windows)]` port is planned
after the tool proves itself in production.

## License

**AGPL-3.0-only** — see [LICENSE](./LICENSE). Run a modified Agentx as a hosted
service and you must offer users its source. For other terms, contact the author.
