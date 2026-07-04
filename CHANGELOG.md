# Changelog

## v0.2.0 — 2026-06-28

- **Warm agents** — one long-lived process per agent for the whole journey; no cold starts, sessions survive `stop`/`start`.
- **Pluggable backends** — workers are a `Backend` trait; a new CLI is one file + one registry line.
- **Per-seat engines** — `model`/`effort` per roster entry (`{ agent, model, effort }`); `doctor` probes every distinct triple.
- **Audit phase** — auditors sweep the whole system and raise remediation tasks the executors then build.
- **Verify phases** — tests → benches → examples → fuzzes, each opt-in with its own roster, gated and reviewed.
- **`[option]` switches** — every run-shaping decision is an explicit bool; the gate is composed from `lint`/`format`/`tests` pillars.
- **Composable training center** — three zones (`base`/`project`/`history`), knowledge composed general → specific, transitive dependencies with cycle/diamond safety.
- **Per-requirement memory** — every clean run archives one decision report per requirement into the kind's history.
- **Manager-led discovery** — the primed manager classifies the project and composes the gate; the team trains after, never blind.
- **Anti-hallucination prompts** — evidence-or-open-item reports, closed write fences, positional seat briefings, two-lap active-recall priming.
- **New commands** — `new` (bootstrap a project skeleton) · `inspire` · `gate` · `watch` (alt-screen live dashboard).
- **New flags** — `-y/--yes` · `--no-train`/`--no-clear` · per-option globals · `--bg`.
- **Game-feel logs** — expressive icons with ASCII fallbacks, per-operation durations, breathing paragraphs, capability-graded terminal output.
- **Self-documenting scaffold** — `init` writes every seat explicit, offers the inspire menu + gate prompt; placeholders (`.gitkeep`) never classify or copy.
- **Hardened lifecycle** — resumable everywhere, fail-closed intake/verdicts, gate after every code-writing phase, blocked runs keep `.agentx/` for inspection.
- **Internals** — flat module tree, strict `core → config → app` layering, POSIX-portable (Linux · macOS · WSL), zero `unwrap`/`panic`.

## v0.1.0 — 2026-06-24

First release. A library and binary that drives a competing team of CLI coding agents to convergence — a resumable state machine, a per-task council of agents, an upfront priming phase, and a self-training knowledge base shared across projects of the same kind.

- **Pipeline:** fixed `requires → tasks → tests → finalize`. The manager turns your requirements into an ordered backlog, architects cut it into task contracts, executors build them one at a time (a council of models, gate after every turn), verifiers attack the result — each phase manager-reviewed until it ships.
- **Priming:** the whole team studies the project once up front and confirms the bar; every later turn is a light work prompt.
- **Resumable:** the cursor (phase · task · agent · round) is checkpointed atomically after every action. `Ctrl+C`, `stop`, and `drain` halt gracefully; `start` resumes exactly where it left off.
- **Training center** (`~/.agentx/train/<archetype>/`): shared `overview · contracts · skills · designs · requires · history` per project kind, shipped in the binary. `designs/` holds frontend visual references (images, style tokens, figma exports, platform links) studied by the agents when present. The kind is auto-detected and written to `Agentx.toml`; its knowledge is injected into every briefing (yours wins on conflict), and each finished run appends one generalized lesson — so the next project of that kind starts smarter.
- **Layout:** minimal input is `Agentx.toml` + a root `Requirements.md` (an optional `agentx/`/`agents/` tree is also read). `.agentx/` is pure runtime, cleared on success; agents never write outside it. Root = nearest `Agentx.toml` upward (monorepo-safe).
- **Config** in five tables: `[project]` (`inspire`, `real_tests`, `doc_blocks`, `doc_contracts`, `max_rounds`, `max_fixes`), `[gate]` (`timeout`, `command`), `[agent]` (`timeout`, `manager`, `architects`/`executors`/`testers`), and `[claude]`/`[codex]` (`model`, `effort`). `real_tests` makes verifiers write real project tests; `doc_blocks` documents every public item; `doc_contracts` documents only the non-obvious ones — each a flexible bool, overridable per run by a flag.
- **Commands** (each a thin wrapper over the `App` library API): `init` · `start` · `restart` · `stop` · `drain` · `clear` · `ignore`/`include`/`refresh` (curate classification) · `info` · `status` (`-f/--tail` for a live dashboard) · `doctor` (preflight every required CLI) · `sync` (refresh shipped training, keep history) · `reset` · `completions` / `man` (shell completions + man page). Global flags `-i/--inspire`, `-g/--gate`, `-t/--real-tests`, `--doc-blocks`, `--doc-contracts`, `-b/--background`.
- **Resilience:** a CLI that reports an internal error is never mistaken for success. Faults are classified (transient / session / exhausted / fatal); transient faults retry with backoff, a lost session is re-trained and re-confirmed before resuming, and quota/auth failures stop cleanly and resumably.
- **Internals:** `#![forbid(unsafe_code)]` throughout; atomic state writes (tmp + fsync + rename); a liveness-checked pid lock; graceful signals via sigmask + sigwait. The embeddable type is `App`; workers live in `core::worker::Worker`.
