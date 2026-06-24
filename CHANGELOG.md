# Changelog

## v0.1.0 — 2026-06-24

First release. A library and binary that drives a competing team of CLI coding agents to convergence — a resumable state machine, a per-task council of agents, an upfront priming phase, and a self-training knowledge base shared across projects of the same kind.

- **Pipeline:** fixed `requires → tasks → tests → finalize`. The manager turns your requirements into an ordered backlog, architects cut it into task contracts, executors build them one at a time (a council of models, gate after every turn), verifiers attack the result — each phase manager-reviewed until it ships.
- **Priming:** the whole team studies the project once up front and confirms the bar; every later turn is a light work prompt.
- **Resumable:** the cursor (phase · task · agent · round) is checkpointed atomically after every action. `Ctrl+C`, `stop`, and `drain` halt gracefully; `start` resumes exactly where it left off.
- **Training center** (`~/.agentx/train/<archetype>/`): shared `overview · contracts · skills · requires · history` per project kind, shipped in the binary. The kind is auto-detected and written to `Agentx.toml`; its knowledge is injected into every briefing (yours wins on conflict), and each finished run appends one generalized lesson — so the next project of that kind starts smarter.
- **Layout:** minimal input is `Agentx.toml` + a root `Requirements.md` (an optional `agentx/`/`agents/` tree is also read). `.agentx/` is pure runtime, cleared on success; agents never write outside it. Root = nearest `Agentx.toml` upward (monorepo-safe).
- **Config** in five tables: `[project]` (`inspire`, `tests`, `max_rounds`, `max_fixes`), `[gate]` (`timeout`, `command`), `[agent]` (`timeout`, `manager`, `architects`/`executors`/`testers`), and `[claude]`/`[codex]` (`model`, `effort`). `tests = true` makes verifiers write real project tests; `false` keeps throwaway probes.
- **Commands** (each a thin wrapper over the `App` library API): `init` · `start` · `restart` · `stop` · `drain` · `clear` · `ignore`/`include`/`refresh` (curate classification) · `info` · `status` (`-f/--tail` for a live dashboard) · `doctor` (preflight every required CLI) · `sync` (refresh shipped training, keep history) · `reset` · `completions` / `man` (shell completions + man page). Global flags `-i/--inspire`, `-g/--gate`, `-t/--tests`, `-b/--background`.
- **Resilience:** a CLI that reports an internal error is never mistaken for success. Faults are classified (transient / session / exhausted / fatal); transient faults retry with backoff, a lost session is re-trained and re-confirmed before resuming, and quota/auth failures stop cleanly and resumably.
- **Internals:** `#![forbid(unsafe_code)]` throughout; atomic state writes (tmp + fsync + rename); a liveness-checked pid lock; graceful signals via sigmask + sigwait. The embeddable type is `App`; workers live in `core::worker::Worker`.
